use pipebased_common::{grpc::client::RpcClientConfig, read_yml, Result};
use serde::Deserialize;

const DEFAULT_CONFIG_FILE: &str = "~/.pd/config";

#[derive(Default, Deserialize)]
pub struct Config {
    // rpc client config
    pub rpc: RpcClientConfig,
}

impl Config {
    pub fn parse(path: Option<&str>) -> Result<Self> {
        let path = path.unwrap_or(DEFAULT_CONFIG_FILE);
        read_yml(path)
    }

    pub fn parse_or_default(path: Option<&str>) -> Self {
        Self::parse(path).unwrap_or_default()
    }
}
