use pipebased_common::{
    grpc::daemon::{
        daemon_client::DaemonClient, ListAppRequest, ListAppResponse, PullAppRequest,
        PullAppResponse, RemoveAppRequest, RemoveAppResponse,
    },
    Result,
};
use tonic::transport::Channel;

pub async fn pull_app(
    client: &mut DaemonClient<Channel>,
    namespace: String,
    id: String,
    version: u64,
) -> Result<PullAppResponse> {
    let request = PullAppRequest {
        namespace,
        id,
        version,
    };
    let response = client.pull_app(request).await?;
    Ok(response.into_inner())
}

pub async fn remove_app(
    client: &mut DaemonClient<Channel>,
    namespace: String,
    id: String,
    version: u64,
) -> Result<RemoveAppResponse> {
    let request = RemoveAppRequest {
        namespace,
        id,
        version,
    };
    let response = client.remove_app(request).await?;
    Ok(response.into_inner())
}

pub async fn list_app(client: &mut DaemonClient<Channel>) -> Result<ListAppResponse> {
    let request = ListAppRequest {};
    let response = client.list_app(request).await?;
    Ok(response.into_inner())
}
