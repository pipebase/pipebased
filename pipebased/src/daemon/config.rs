use pipebased_common::DaemonConfig;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub daemon: DaemonConfig,
}
