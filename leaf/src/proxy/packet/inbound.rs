use std::sync::Arc;

use anyhow::{anyhow, Result};
use futures::{sink::SinkExt, stream::StreamExt};
use log::*;
use protobuf::Message;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex as TokioMutex;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio_util::codec::Framed;

use crate::{
    app::dispatcher::Dispatcher,
    app::fake_dns::{FakeDns, FakeDnsMode},
    app::nat_manager::NatManager,
    config::{Inbound, PacketInboundSettings},
    option, Runner,
};

use crate::proxy::tun::netstack::NetStack;

use bytes::{BufMut, Bytes, BytesMut};
use std::net::{SocketAddr, TcpStream};
#[cfg(unix)]
use std::os::unix::io::FromRawFd;
use std::pin::Pin;
use tokio::fs::File;
use tokio::net::TcpListener;
use tun::{AsyncDevice, TunPacket, TunPacketCodec};
use crate::config::TunInboundSettings;

use crate::config::PacketInboundSettings_Sink;
use crate::proxy::packet::{TcpSink, UdpSink};
use crate::proxy::packet::Sink;

#[cfg(unix)]
use crate::proxy::packet::FdSink;

#[cfg(unix)]
fn sink_from_fd(fd: i32) -> Result<Pin<Box<dyn Sink>>> {
    Ok(Box::pin(FdSink::new(fd)))
}

fn sink_from_udp(local_port: u32, remote_port: u32) -> Result<Pin<Box<dyn Sink>>>
{
    let local_addr: SocketAddr = format!("127.0.0.1:{}", local_port).parse()?;
    let remote_addr: SocketAddr = format!("127.0.0.1:{}", remote_port).parse()?;
    let sock = std::net::UdpSocket::bind(&local_addr)?;
    debug!("udp sink listen on {}", sock.local_addr()?);
    // sock.connect(remote_addr)?;
    Ok(Box::pin(UdpSink::new(sock)))
}

fn sink_from_tcp(port: u32) -> Result<Pin<Box<dyn Sink>>>
{
    let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port))?;
    let (stream, _) = listener.accept()?;
    debug!("tcp sink listen on {}", port);
    Ok(Box::pin(TcpSink::new(stream)))
}

// impl Sink for AsyncDevice {}

fn sink_from_tun() -> Result<Pin<Box<dyn Sink>>>
{
    let mut cfg = tun::Configuration::default();
    cfg
        .address((240, 255, 0, 2))
        .netmask((255, 255, 255, 0))
        .destination((240, 255, 0, 1))
        .up();
    #[cfg(target_os = "linux")]
        cfg.platform(|config| {
        cfg.packet_information(true);
    });
    let dev = tun::create_as_async(&cfg).unwrap();
    Ok(Box::pin(dev))
}

impl Sink for File {}

#[cfg(unix)]
fn sink_from_pipe(pipe: &str) -> Result<Pin<Box<dyn Sink>>> {
    Ok(Box::pin(unsafe { File::from_raw_fd(21) }))
}

impl Sink for AsyncDevice {}

pub fn new(
    inbound: Inbound,
    dispatcher: Arc<Dispatcher>,
    nat_manager: Arc<NatManager>,
) -> Result<Runner> {
    let settings = PacketInboundSettings::parse_from_bytes(&inbound.settings)?;
    let port = settings.local_port;
    // // FIXME it's a bad design to have 2 lists in config while we need only one
    // let fake_dns_exclude = settings.fake_dns_exclude;
    // let fake_dns_include = settings.fake_dns_include;
    // if !fake_dns_exclude.is_empty() && !fake_dns_include.is_empty() {
    //     return Err(anyhow!(
    //         "fake DNS run in either include mode or exclude mode"
    //     ));
    // }
    // let (fake_dns_mode, fake_dns_filters) = if !fake_dns_include.is_empty() {
    //     (FakeDnsMode::Include, fake_dns_include)
    // } else {
    //     (FakeDnsMode::Exclude, fake_dns_exclude)
    // };
    let mut fakedns = FakeDns::new(FakeDnsMode::Exclude);
    let fakedns = Arc::new(TokioMutex::new(fakedns));
    // for filter in fake_dns_filters.into_iter() {
    //     fakedns.add_filter(filter);
    // }
    // let tun = sink_from_tun(&inbound.settings).unwrap();
    // let stack =  NetStack::new(inbound.tag, dispatcher, nat_manager, Arc::new(TokioMutex::new(fakedns)));
    // let stack = Arc::new(stack);

    Ok(Box::pin(async move {
        let listen_addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&listen_addr).await.unwrap();
        info!("packet inbound listening tcp {}", &listen_addr);
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let dispatcher = dispatcher.clone();
                    let nat_manager = nat_manager.clone();
                    let fakedns = fakedns.clone();
                    let tag = inbound.clone().tag;
                    let stack =  NetStack::new(tag, dispatcher, nat_manager, fakedns);
                    let (mut stack_reader, mut stack_writer) = io::split(stack);
                    // let tun = sink_from_tun().unwrap();
                    let tun = stream;
                    let pi = true;
                    let mtu = 1504;
                    let codec = TunPacketCodec::new(pi, mtu);
                    let framed = Framed::new(tun, codec);
                    let (mut tun_sink, mut tun_stream) = framed.split();

                    let s2t = Box::pin(async move {
                        let mut buf = vec![0; mtu as usize];
                        loop {
                            match stack_reader.read(&mut buf).await {
                                Ok(0) => {
                                    debug!("read stack eof");
                                    return;
                                }
                                Ok(n) => match tun_sink.send(TunPacket::new((&buf[..n]).to_vec())).await {
                                    Ok(_) => (),
                                    Err(e) => {
                                        warn!("send pkt to tun failed: {}", e);
                                        return;
                                    }
                                },
                                Err(err) => {
                                    warn!("read stack failed {:?}", err);
                                    return;
                                }
                            }
                        }
                    });

                    let t2s = Box::pin(async move {
                        while let Some(packet) = tun_stream.next().await {
                            match packet {
                                Ok(packet) => match stack_writer.write(packet.get_bytes()).await {
                                    Ok(_) => (),
                                    Err(e) => {
                                        warn!("write pkt to stack failed: {}", e);
                                        return;
                                    }
                                },
                                Err(err) => {
                                    warn!("read tun failed {:?}", err);
                                    return;
                                }
                            }
                        }
                    });

                    info!("packet inbound started");
                    futures::future::select(t2s, s2t).await;
                    info!("packet inbound exited");
                }
                Err(e) => {
                    error!("accept connection failed: {}", e);
                    break;
                }
            }
        }
    }))
}
