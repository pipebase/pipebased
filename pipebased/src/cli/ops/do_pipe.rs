use pipebased_common::{
    grpc::daemon::{
        daemon_client::DaemonClient, CreatePipeRequest, CreatePipeResponse, ListPipeRequest,
        ListPipeResponse, RemovePipeRequest, RemovePipeResponse, StartPipeRequest,
        StartPipeResponse, StopPipeRequest, StopPipeResponse,
    },
    read_yml, Result,
};
use tonic::transport::Channel;

pub async fn remove_pipe(
    client: &mut DaemonClient<Channel>,
    id: String,
) -> Result<RemovePipeResponse> {
    let request = RemovePipeRequest { id };
    let response = client.remove_pipe(request).await?;
    Ok(response.into_inner())
}

pub async fn start_pipe(
    client: &mut DaemonClient<Channel>,
    id: String,
) -> Result<StartPipeResponse> {
    let request = StartPipeRequest { id };
    let response = client.start_pipe(request).await?;
    Ok(response.into_inner())
}

pub async fn stop_pipe(client: &mut DaemonClient<Channel>, id: String) -> Result<StopPipeResponse> {
    let request = StopPipeRequest { id };
    let response = client.stop_pipe(request).await?;
    Ok(response.into_inner())
}

pub async fn list_pipe(client: &mut DaemonClient<Channel>) -> Result<ListPipeResponse> {
    let request = ListPipeRequest {};
    let response = client.list_pipe(request).await?;
    Ok(response.into_inner())
}

fn parse_create_pipe_request<P>(path: P) -> Result<CreatePipeRequest>
where
    P: AsRef<std::path::Path>,
{
    let request: models::CreatePipeRequest = read_yml(path)?;
    Ok(request.into())
}

pub async fn create_pipe<P>(
    client: &mut DaemonClient<Channel>,
    path: P,
) -> Result<CreatePipeResponse>
where
    P: AsRef<std::path::Path>,
{
    let request = parse_create_pipe_request(path)?;
    let response = client.create_pipe(request).await?;
    Ok(response.into_inner())
}

mod models {
    use pipebased_common::grpc::daemon;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct AppDescriptor {
        pub namespace: String,
        pub id: String,
        pub version: u64,
    }

    #[derive(Deserialize)]
    pub struct CatalogsDescriptor {
        pub namespace: String,
        pub id: String,
        pub version: u64,
    }

    #[derive(Deserialize)]
    pub struct EnvironmentVariable {
        pub key: String,
        pub value: String,
    }

    #[derive(Deserialize)]
    pub struct CreatePipeRequest {
        pub id: String,
        pub description: Option<String>,
        pub user: Option<String>,
        pub group: Option<String>,
        pub envs: Option<Vec<EnvironmentVariable>>,
        pub app: AppDescriptor,
        pub catalogs: CatalogsDescriptor,
    }

    impl From<AppDescriptor> for daemon::AppDescriptor {
        fn from(origin: AppDescriptor) -> Self {
            let namespace = origin.namespace;
            let id = origin.id;
            let version = origin.version;
            daemon::AppDescriptor {
                namespace,
                id,
                version,
            }
        }
    }

    impl From<CatalogsDescriptor> for daemon::CatalogsDescriptor {
        fn from(origin: CatalogsDescriptor) -> Self {
            let namespace = origin.namespace;
            let id = origin.id;
            let version = origin.version;
            daemon::CatalogsDescriptor {
                namespace,
                id,
                version,
            }
        }
    }

    impl From<EnvironmentVariable> for daemon::EnvironmentVariable {
        fn from(origin: EnvironmentVariable) -> Self {
            let key = origin.key;
            let value = origin.value;
            daemon::EnvironmentVariable { key, value }
        }
    }

    impl From<CreatePipeRequest> for daemon::CreatePipeRequest {
        fn from(origin: CreatePipeRequest) -> Self {
            let id = origin.id;
            let description = origin.description;
            let user = origin.user;
            let group = origin.group;
            let envs: Vec<daemon::EnvironmentVariable> = match origin.envs {
                Some(envs) => envs
                    .into_iter()
                    .map(|env| {
                        let env: daemon::EnvironmentVariable = env.into();
                        env
                    })
                    .collect(),
                None => vec![],
            };
            let app: daemon::AppDescriptor = origin.app.into();
            let catalogs: daemon::CatalogsDescriptor = origin.catalogs.into();
            daemon::CreatePipeRequest {
                id,
                description,
                user,
                group,
                envs,
                app: Some(app),
                catalogs: Some(catalogs),
            }
        }
    }
}
