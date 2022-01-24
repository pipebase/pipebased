use crate::{
    register_error, AppDescriptor, CatalogsDescriptor, EnvironmentVariable, PipeDescriptor,
    PipeManager, PipeManagerConfig, PipeState, RepositoryManager, RepositoryManagerConfig,
    ResourceType, Result,
};
use serde::Deserialize;
use std::path::PathBuf;
use tracing::warn;

#[derive(Deserialize)]
pub struct DaemonConfig {
    pub repository: RepositoryManagerConfig,
    pub pipe: PipeManagerConfig,
}

pub struct Daemon {
    repository_manager: RepositoryManager,
    pipe_manager: PipeManager,
}

pub struct DaemonBuilder {
    repository_manager: Option<RepositoryManager>,
    pipe_manager: Option<PipeManager>,
}

impl Default for DaemonBuilder {
    fn default() -> Self {
        DaemonBuilder::new()
    }
}

impl DaemonBuilder {
    pub fn new() -> Self {
        DaemonBuilder {
            repository_manager: None,
            pipe_manager: None,
        }
    }

    pub fn repository_manager(mut self, repository_manager: RepositoryManager) -> Self {
        self.repository_manager = Some(repository_manager);
        self
    }

    pub fn pipe_manager(mut self, pipe_manager: PipeManager) -> Self {
        self.pipe_manager = Some(pipe_manager);
        self
    }

    pub fn build(self) -> Daemon {
        let repository_manager = self
            .repository_manager
            .expect("repository manager undefined");
        let pipe_manager = self.pipe_manager.expect("pipe manager undefined");
        Daemon {
            repository_manager,
            pipe_manager,
        }
    }
}

// composite descriptor include all material to create a new pipe instance
pub struct Descriptor {
    pub id: String,
    pub description: Option<String>,
    pub user: Option<String>,
    pub group: Option<String>,
    pub envs: Vec<EnvironmentVariable>,
    pub app_descriptor: AppDescriptor,
    pub catalogs_descriptor: CatalogsDescriptor,
}

impl Descriptor {
    pub fn builder() -> DescriptorBuilder {
        DescriptorBuilder::new()
    }
}

pub struct DescriptorBuilder {
    pub id: Option<String>,
    pub description: Option<String>,
    pub user: Option<String>,
    pub group: Option<String>,
    pub envs: Vec<EnvironmentVariable>,
    pub app_descriptor: Option<AppDescriptor>,
    pub catalogs_descriptor: Option<CatalogsDescriptor>,
}

impl DescriptorBuilder {
    pub fn new() -> Self {
        DescriptorBuilder {
            id: None,
            description: None,
            user: None,
            group: None,
            envs: vec![],
            app_descriptor: None,
            catalogs_descriptor: None,
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

    pub fn group(mut self, group: String) -> Self {
        self.group = Some(group);
        self
    }

    pub fn env(mut self, key: String, value: String) -> Self {
        self.envs.push(EnvironmentVariable { key, value });
        self
    }

    pub fn app_descriptor(mut self, desc: AppDescriptor) -> Self {
        self.app_descriptor = Some(desc);
        self
    }

    pub fn catalogs_descriptor(mut self, desc: CatalogsDescriptor) -> Self {
        self.catalogs_descriptor = Some(desc);
        self
    }

    pub fn build(self) -> Descriptor {
        let id = self.id.expect("id undefined");
        let description = self.description;
        let user = self.user;
        let group = self.group;
        let envs = self.envs;
        let app_descriptor = self.app_descriptor.expect("app descriptor undefined");
        let catalogs_descriptor = self
            .catalogs_descriptor
            .expect("catalogs descriptor undefined");
        Descriptor {
            id,
            description,
            user,
            group,
            envs,
            app_descriptor,
            catalogs_descriptor,
        }
    }
}

impl Default for DescriptorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Daemon {
    pub fn builder() -> DaemonBuilder {
        DaemonBuilder::default()
    }

    // repository operations
    fn check_app_registered(&self, desc: &AppDescriptor) -> Result<Option<PathBuf>> {
        self.repository_manager.check_app_registered(desc)
    }

    fn check_catalogs_registered(&self, desc: &CatalogsDescriptor) -> Result<Option<PathBuf>> {
        self.repository_manager.check_catalogs_registered(desc)
    }

    pub fn list_app_register(&self) -> Result<Vec<AppDescriptor>> {
        self.repository_manager.list_app_register()
    }

    pub fn list_catalogs_register(&self) -> Result<Vec<CatalogsDescriptor>> {
        self.repository_manager.list_catalogs_register()
    }

    pub async fn pull_app(&self, desc: &AppDescriptor) -> Result<()> {
        self.repository_manager.pull_app(desc).await
    }

    pub async fn pull_catalogs(&self, desc: &CatalogsDescriptor) -> Result<()> {
        self.repository_manager.pull_catalogs(desc).await
    }

    pub fn remove_app(&self, desc: &AppDescriptor) -> Result<()> {
        self.repository_manager.remove_app(desc)
    }

    pub fn remove_catalogs(&self, desc: &CatalogsDescriptor) -> Result<()> {
        self.repository_manager.remove_catalogs(desc)
    }

    // pipe operations
    pub fn create_pipe(&self, desc: Descriptor) -> Result<()> {
        let app_descriptor = &desc.app_descriptor;
        let app_path = match self.check_app_registered(app_descriptor)? {
            Some(path) => path,
            None => {
                return Err(register_error(
                    ResourceType::App,
                    format!("app {} not found", app_descriptor),
                ))
            }
        };
        let catalogs_descriptor = &desc.catalogs_descriptor;
        let catalogs_path = match self.check_catalogs_registered(catalogs_descriptor)? {
            Some(path) => path,
            None => {
                return Err(register_error(
                    ResourceType::Catalogs,
                    format!("catalogs {} not found", catalogs_descriptor),
                ))
            }
        };
        let mut builder = PipeDescriptor::builder()
            .id(desc.id)
            .app_path(app_path.as_path())
            .catalogs_path(catalogs_path.as_path());
        for env in desc.envs {
            builder = builder.env(env);
        }
        let builder = match desc.description {
            Some(description) => builder.description(description),
            None => builder,
        };
        let builder = match desc.user {
            Some(user) => builder.user(user),
            None => builder,
        };
        let builder = match desc.group {
            Some(group) => builder.group(group),
            None => builder,
        };
        let pipe_descriptor = builder.build();
        self.pipe_manager.init(&pipe_descriptor)
    }

    pub fn start_pipe(&self, id: &str) -> Result<()> {
        self.pipe_manager.start(id)
    }

    pub fn stop_pipe(&self, id: &str) -> Result<()> {
        self.pipe_manager.stop(id)
    }

    pub fn remove_pipe(&self, id: &str) -> Result<()> {
        self.pipe_manager.remove(id)
    }

    pub fn pipe_status(&self, id: &str) -> Result<PipeState> {
        self.pipe_manager.status(id)
    }

    pub fn list_pipe_register(&self) -> Result<Vec<String>> {
        self.pipe_manager.list_pipe_register()
    }

    pub fn list_pipe_status(&self) -> Result<Vec<PipeState>> {
        let pipe_ids = self.list_pipe_register()?;
        let mut pipe_states: Vec<PipeState> = vec![];
        for pipe_id in pipe_ids.iter() {
            let pipe_state = match self.pipe_status(pipe_id.as_str()) {
                Ok(pipe_state) => pipe_state,
                Err(err) => {
                    warn!(
                        pipe_id = pipe_id.as_str(),
                        "get pipe status failed, error: {:#?}", err
                    );
                    continue;
                }
            };
            pipe_states.push(pipe_state);
        }
        Ok(pipe_states)
    }
}
