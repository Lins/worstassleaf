use std::io;
use std::sync::Arc;

use async_trait::async_trait;
use log::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    app::SyncDnsClient,
    proxy::{
        DatagramTransportType, OutboundConnect, OutboundDatagram, OutboundHandler,
        OutboundTransport, UdpOutboundHandler,
    },
    session::Session,
};

pub struct Handler {
    pub actors: Vec<Arc<dyn OutboundHandler>>,
    pub dns_client: SyncDnsClient,
}

#[async_trait]
impl UdpOutboundHandler for Handler {
    fn connect_addr(&self) -> Option<OutboundConnect> {
        None
    }

    fn transport_type(&self) -> DatagramTransportType {
        DatagramTransportType::Undefined
    }

    async fn handle<'a>(
        &'a self,
        sess: &'a Session,
        _transport: Option<OutboundTransport>,
    ) -> io::Result<Box<dyn OutboundDatagram>> {
        let mut rng = StdRng::from_entropy();
        let i: usize = rng.gen_range(0..self.actors.len());
        debug!(
            "random handles udp [{}] to [{}]",
            sess.destination,
            self.actors[i].tag()
        );
        let transport =
            crate::proxy::connect_udp_outbound(sess, self.dns_client.clone(), &self.actors[i])
                .await?;
        UdpOutboundHandler::handle(self.actors[i].as_ref(), sess, transport).await
    }
}
