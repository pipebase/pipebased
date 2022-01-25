use pipebased_common::{grpc, AppDescriptor, CatalogsDescriptor, Daemon, Descriptor};
use tracing::{error, info};

pub struct DaemonServiceBuilder {
    pub daemon: Option<Daemon>,
}

impl Default for DaemonServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DaemonServiceBuilder {
    pub fn new() -> Self {
        DaemonServiceBuilder { daemon: None }
    }

    pub fn daemon(mut self, daemon: Daemon) -> Self {
        self.daemon = Some(daemon);
        self
    }

    pub fn build(self) -> DaemonService {
        let daemon = self.daemon.expect("daemon undefined");
        DaemonService { daemon }
    }
}

pub struct DaemonService {
    daemon: Daemon,
}

impl DaemonService {
    pub fn builder() -> DaemonServiceBuilder {
        DaemonServiceBuilder::default()
    }
}

#[tonic::async_trait]
impl grpc::daemon::daemon_server::Daemon for DaemonService {
    async fn list_app(
        &self,
        _request: tonic::Request<grpc::daemon::ListAppRequest>,
    ) -> Result<tonic::Response<grpc::daemon::ListAppResponse>, tonic::Status> {
        match self.daemon.list_app_register() {
            Ok(apps) => {
                let apps: Vec<grpc::daemon::AppDescriptor> = apps
                    .into_iter()
                    .map(|app| {
                        let app: grpc::daemon::AppDescriptor = app.into();
                        app
                    })
                    .collect();
                Ok(tonic::Response::new(grpc::daemon::ListAppResponse { apps }))
            }
            Err(err) => {
                error!("list app error {:#?}", err);
                Err(tonic::Status::internal(format!("{:#?}", err)))
            }
        }
    }

    async fn list_catalogs(
        &self,
        _request: tonic::Request<grpc::daemon::ListCatalogsRequest>,
    ) -> Result<tonic::Response<grpc::daemon::ListCatalogsResponse>, tonic::Status> {
        match self.daemon.list_catalogs_register() {
            Ok(catalogss) => {
                let catalogss: Vec<grpc::daemon::CatalogsDescriptor> = catalogss
                    .into_iter()
                    .map(|catalogs| {
                        let catalogs: grpc::daemon::CatalogsDescriptor = catalogs.into();
                        catalogs
                    })
                    .collect();
                Ok(tonic::Response::new(grpc::daemon::ListCatalogsResponse {
                    catalogss,
                }))
            }
            Err(err) => {
                error!("list catalogs failed, error: {:#?}", err);
                Err(tonic::Status::internal(format!(
                    "list catalogs failed, error: {:#?}",
                    err
                )))
            }
        }
    }

    async fn pull_app(
        &self,
        request: tonic::Request<grpc::daemon::PullAppRequest>,
    ) -> Result<tonic::Response<grpc::daemon::PullAppResponse>, tonic::Status> {
        let request = request.into_inner();
        let namespace = request.namespace;
        let id = request.id;
        let version = request.version;
        info!(
            namespace = namespace.as_str(),
            id = id.as_str(),
            version = version,
            "pull app"
        );
        let desc = AppDescriptor::builder()
            .namespace(namespace)
            .id(id)
            .version(version)
            .build();
        match self.daemon.pull_app(&desc).await {
            Ok(_) => Ok(tonic::Response::new(grpc::daemon::PullAppResponse {})),
            Err(err) => {
                error!("pull app error {:#?}", err);
                Err(tonic::Status::invalid_argument(format!("{:#?}", err)))
            }
        }
    }

    async fn pull_catalogs(
        &self,
        request: tonic::Request<grpc::daemon::PullCatalogsRequest>,
    ) -> Result<tonic::Response<grpc::daemon::PullCatalogsResponse>, tonic::Status> {
        let request = request.into_inner();
        let namespace = request.namespace;
        let id = request.id;
        let version = request.version;
        info!(
            namespace = namespace.as_str(),
            id = id.as_str(),
            version = version,
            "pull catalogs"
        );
        let desc = CatalogsDescriptor::builder()
            .namespace(namespace)
            .id(id)
            .version(request.version)
            .build();
        match self.daemon.pull_catalogs(&desc).await {
            Ok(_) => Ok(tonic::Response::new(grpc::daemon::PullCatalogsResponse {})),
            Err(err) => {
                error!("pull catalogs error {:#?}", err);
                Err(tonic::Status::invalid_argument(format!("{:#?}", err)))
            }
        }
    }

    async fn remove_app(
        &self,
        request: tonic::Request<grpc::daemon::RemoveAppRequest>,
    ) -> Result<tonic::Response<grpc::daemon::RemoveAppResponse>, tonic::Status> {
        let request = request.into_inner();
        let namespace = request.namespace;
        let id = request.id;
        let version = request.version;
        info!(
            namespace = namespace.as_str(),
            id = id.as_str(),
            version = version,
            "remove app"
        );
        let desc = AppDescriptor::builder()
            .namespace(namespace)
            .id(id)
            .version(version)
            .build();
        match self.daemon.remove_app(&desc) {
            Ok(_) => Ok(tonic::Response::new(grpc::daemon::RemoveAppResponse {})),
            Err(err) => {
                error!("remove app error {:#?}", err);
                Err(tonic::Status::invalid_argument(format!("{:#?}", err)))
            }
        }
    }

    async fn remove_catalogs(
        &self,
        request: tonic::Request<grpc::daemon::RemoveCatalogsRequest>,
    ) -> Result<tonic::Response<grpc::daemon::RemoveCatalogsResponse>, tonic::Status> {
        let request = request.into_inner();
        let namespace = request.namespace;
        let id = request.id;
        let version = request.version;
        info!(
            namespace = namespace.as_str(),
            id = id.as_str(),
            version = version,
            "remove catalogs"
        );
        let desc = CatalogsDescriptor::builder()
            .namespace(namespace)
            .id(id)
            .version(version)
            .build();
        match self.daemon.remove_catalogs(&desc) {
            Ok(_) => Ok(tonic::Response::new(
                grpc::daemon::RemoveCatalogsResponse {},
            )),
            Err(err) => {
                error!("remove catalogs error {:#?}", err);
                Err(tonic::Status::invalid_argument(format!("{:#?}", err)))
            }
        }
    }

    async fn create_pipe(
        &self,
        request: tonic::Request<grpc::daemon::CreatePipeRequest>,
    ) -> Result<tonic::Response<grpc::daemon::CreatePipeResponse>, tonic::Status> {
        let request = request.into_inner();
        let id = request.id;
        let app: AppDescriptor = match request.app {
            Some(app) => app.into(),
            None => return Err(tonic::Status::invalid_argument("app descriptor undefined")),
        };
        let catalogs: CatalogsDescriptor = match request.catalogs {
            Some(catalogs) => catalogs.into(),
            None => {
                return Err(tonic::Status::invalid_argument(
                    "catalogs descriptor undefined",
                ))
            }
        };
        let mut builder = Descriptor::builder()
            .id(id)
            .app_descriptor(app)
            .catalogs_descriptor(catalogs);
        builder = match request.user {
            Some(user) => builder.user(user),
            None => builder,
        };
        builder = match request.group {
            Some(group) => builder.group(group),
            None => builder,
        };
        for env in request.envs {
            builder = builder.env(env.key, env.value);
        }
        let desc = builder.build();
        match self.daemon.create_pipe(desc) {
            Ok(_) => Ok(tonic::Response::new(grpc::daemon::CreatePipeResponse {})),
            Err(err) => Err(tonic::Status::invalid_argument(format!(
                "create pipe failed, error: {:#?}",
                err
            ))),
        }
    }

    async fn start_pipe(
        &self,
        request: tonic::Request<grpc::daemon::StartPipeRequest>,
    ) -> Result<tonic::Response<grpc::daemon::StartPipeResponse>, tonic::Status> {
        let request = request.into_inner();
        match self.daemon.start_pipe(request.id.as_str()) {
            Ok(_) => Ok(tonic::Response::new(grpc::daemon::StartPipeResponse {})),
            Err(err) => Err(tonic::Status::invalid_argument(format!(
                "start pipe failed, error: {:#?}",
                err
            ))),
        }
    }

    async fn stop_pipe(
        &self,
        request: tonic::Request<grpc::daemon::StopPipeRequest>,
    ) -> Result<tonic::Response<grpc::daemon::StopPipeResponse>, tonic::Status> {
        let request = request.into_inner();
        match self.daemon.stop_pipe(request.id.as_str()) {
            Ok(_) => Ok(tonic::Response::new(grpc::daemon::StopPipeResponse {})),
            Err(err) => Err(tonic::Status::invalid_argument(format!(
                "stop pipe failed, error: {:#?}",
                err
            ))),
        }
    }

    async fn remove_pipe(
        &self,
        request: tonic::Request<grpc::daemon::RemovePipeRequest>,
    ) -> Result<tonic::Response<grpc::daemon::RemovePipeResponse>, tonic::Status> {
        let request = request.into_inner();
        match self.daemon.remove_pipe(request.id.as_str()) {
            Ok(_) => Ok(tonic::Response::new(grpc::daemon::RemovePipeResponse {})),
            Err(err) => Err(tonic::Status::invalid_argument(format!(
                "remove pipe failed, error: {:#?}",
                err
            ))),
        }
    }

    async fn list_pipe(
        &self,
        _request: tonic::Request<grpc::daemon::ListPipeRequest>,
    ) -> Result<tonic::Response<grpc::daemon::ListPipeResponse>, tonic::Status> {
        match self.daemon.list_pipe_status() {
            Ok(pipes) => {
                let pipes: Vec<grpc::daemon::PipeState> = pipes
                    .into_iter()
                    .map(|pipe| {
                        let pipe: grpc::daemon::PipeState = pipe.into();
                        pipe
                    })
                    .collect();
                Ok(tonic::Response::new(grpc::daemon::ListPipeResponse {
                    pipes,
                }))
            }
            Err(err) => {
                error!("list pipe failed, error {:#?}", err);
                Err(tonic::Status::internal(format!(
                    "list pipe failed, error: {:#?}",
                    err
                )))
            }
        }
    }
}
