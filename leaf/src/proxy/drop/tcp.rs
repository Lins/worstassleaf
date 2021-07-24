use std::io;

use async_trait::async_trait;

use crate::{
    proxy::{OutboundConnect, ProxyStream, TcpOutboundHandler},
    session::Session,
};

pub struct Handler;

#[async_trait]
impl TcpOutboundHandler for Handler {
    fn connect_addr(&self) -> Option<OutboundConnect> {
        None
    }

    async fn handle<'a>(
        &'a self,
        _sess: &'a Session,
        _stream: Option<Box<dyn ProxyStream>>,
    ) -> io::Result<Box<dyn ProxyStream>> {
        Err(io::Error::new(io::ErrorKind::Other, "dropped"))
    }
}
