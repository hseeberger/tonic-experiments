mod pb {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("experiments");
}

use crate::pb::{experiments_client::ExperimentsClient, PingRequest};
use anyhow::{Context as AnyhowContext, Result};
use configured::Configured;
use pb::PingResponse;
use serde::Deserialize;
use std::task::{Context, Poll};
use tonic::{
    body::BoxBody,
    codegen::{
        http::{Request as HttpRequest, Response as HttpResponse},
        Service,
    },
    transport::{channel::ResponseFuture, Channel, Error},
};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub server_endpoint: String,
}

pub fn init_tracing() -> Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .try_init()
        .context("Cannot initialize tracing subscriber")
}

pub async fn run() -> Result<()> {
    // Load configuration
    let config = Config::load().context("Cannot load configuration")?;
    debug!(message = "Starting", config = debug(&config));

    // let _ = use_default_client(config.server_endpoint).await?;
    // let _ = use_channel(config.server_endpoint).await?;
    let _ = use_grpc_service(config.server_endpoint).await?;
    info!("Received ping response");

    Ok(())
}

#[derive(Debug)]
struct ChannelService(Channel);

impl Service<HttpRequest<BoxBody>> for ChannelService {
    type Response = HttpResponse<hyper::Body>;
    type Error = Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        info!("Intercepting poll_ready");
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: HttpRequest<BoxBody>) -> Self::Future {
        info!("Intercepting call");
        self.0.call(req)
    }
}

#[allow(dead_code)]
async fn use_default_client(server_endpoint: String) -> Result<PingResponse> {
    let mut client = ExperimentsClient::connect(server_endpoint.clone())
        .await
        .context(format!(
            "Cannot create client for endpoint {server_endpoint}",
        ))?;
    client
        .ping(PingRequest::default())
        .await
        .context("Error sending ping request")
        .map(|response| response.into_inner())
}

#[allow(dead_code)]
async fn use_channel(server_endpoint: String) -> Result<PingResponse> {
    let channel = Channel::from_shared(server_endpoint.clone())
        .context(format!(
            "Cannot create channel for endpoint {server_endpoint}"
        ))?
        .connect()
        .await
        .context(format!(
            "Cannot connect channel for endpoint {server_endpoint}"
        ))?;
    let mut client = ExperimentsClient::new(channel);
    client
        .ping(PingRequest::default())
        .await
        .context("Error sending ping request")
        .map(|response| response.into_inner())
}

#[allow(dead_code)]
async fn use_grpc_service(server_endpoint: String) -> Result<PingResponse> {
    let channel = Channel::from_shared(server_endpoint.clone())
        .context(format!(
            "Cannot create channel for endpoint {server_endpoint}"
        ))?
        .connect()
        .await
        .context(format!(
            "Cannot connect channel for endpoint {server_endpoint}"
        ))?;
    let channel = ChannelService(channel);
    let mut client = ExperimentsClient::new(channel);
    client
        .ping(PingRequest::default())
        .await
        .context("Error sending ping request")
        .map(|response| response.into_inner())
}
