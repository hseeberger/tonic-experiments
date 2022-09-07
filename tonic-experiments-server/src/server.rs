mod pb {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("experiments");
}

use self::pb::{
    experiments_server::{Experiments, ExperimentsServer},
    EchoRequest, EchoResponse, PingRequest, PingResponse,
};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Deserialize)]
pub struct Config {
    addr: IpAddr,
    port: u16,
}

impl Config {
    fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.addr, self.port)
    }
}

pub async fn run(config: &Config) -> Result<()> {
    let app = Server::builder().add_service(ExperimentsServer::new(ExperimentsService));
    app.serve(config.socket_addr())
        .await
        .context("Server completed with error")
}

struct ExperimentsService;

#[tonic::async_trait]
impl Experiments for ExperimentsService {
    async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse::default()))
    }

    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let EchoRequest { text } = request.into_inner();
        Ok(Response::new(EchoResponse { text }))
    }
}
