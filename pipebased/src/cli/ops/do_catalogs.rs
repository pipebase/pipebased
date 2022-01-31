use pipebased_common::{
    grpc::daemon::{
        daemon_client::DaemonClient, ListCatalogsRequest, ListCatalogsResponse,
        PullCatalogsRequest, PullCatalogsResponse, RemoveCatalogsRequest, RemoveCatalogsResponse,
    },
    Result,
};
use tonic::transport::Channel;

pub async fn pull_catalogs(
    client: &mut DaemonClient<Channel>,
    namespace: String,
    id: String,
    version: u64,
) -> Result<PullCatalogsResponse> {
    let request = PullCatalogsRequest {
        namespace,
        id,
        version,
    };
    let response = client.pull_catalogs(request).await?;
    Ok(response.into_inner())
}

pub async fn remove_catalogs(
    client: &mut DaemonClient<Channel>,
    namespace: String,
    id: String,
    version: u64,
) -> Result<RemoveCatalogsResponse> {
    let request = RemoveCatalogsRequest {
        namespace,
        id,
        version,
    };
    let response = client.remove_catalogs(request).await?;
    Ok(response.into_inner())
}

pub async fn list_catalogs(client: &mut DaemonClient<Channel>) -> Result<ListCatalogsResponse> {
    let request = ListCatalogsRequest {};
    let response = client.list_catalogs(request).await?;
    Ok(response.into_inner())
}
