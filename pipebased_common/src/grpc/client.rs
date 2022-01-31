use crate::{errors::Result, grpc::daemon::daemon_client::DaemonClient};
use serde::Deserialize;
use std::fmt;
use tonic::transport::Channel;

#[derive(Deserialize)]
pub enum RpcProtocolType {
    Http,
    Https,
}

impl fmt::Display for RpcProtocolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RpcProtocolType::Http => write!(f, "http"),
            RpcProtocolType::Https => write!(f, "https"),
        }
    }
}

#[derive(Deserialize)]
pub struct RpcClientConfig {
    pub protocol: RpcProtocolType,
    pub address: String,
}

impl RpcClientConfig {
    pub fn endpoint(&self) -> String {
        format!("{}://{}", self.protocol, self.address)
    }
}

impl Default for RpcClientConfig {
    fn default() -> Self {
        RpcClientConfig {
            protocol: RpcProtocolType::Http,
            address: String::from("127.0.0.1:10000"),
        }
    }
}

#[derive(Default)]
pub struct DaemonClientBuilder<'a> {
    pub protocol: Option<RpcProtocolType>,
    pub address: Option<&'a str>,
}

impl<'a> DaemonClientBuilder<'a> {
    pub fn protocol(mut self, protocol: RpcProtocolType) -> Self {
        self.protocol = Some(protocol);
        self
    }

    pub fn address(mut self, address: &'a str) -> Self {
        self.address = Some(address);
        self
    }

    pub async fn build(self) -> Result<DaemonClient<Channel>> {
        let client = DaemonClient::connect(format!(
            "{}://{}",
            self.protocol.expect("protocol undefined"),
            self.address.expect("address undefined")
        ))
        .await?;
        Ok(client)
    }
}
