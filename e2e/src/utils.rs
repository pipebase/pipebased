use pipebased_common::{
    grpc::{
        client::{DaemonClientBuilder, RpcClientConfig},
        daemon::daemon_client::DaemonClient,
    },
    read_yml, Result,
};
use std::path::Path;
use tonic::transport::Channel;

#[allow(dead_code)]
pub(crate) async fn build_client<P>(path: P) -> Result<DaemonClient<Channel>>
where
    P: AsRef<Path>,
{
    let config: RpcClientConfig = read_yml(path)?;
    let protocol = config.protocol;
    let address = config.address;
    DaemonClientBuilder::default()
        .protocol(protocol)
        .address(address.as_str())
        .build()
        .await
}
