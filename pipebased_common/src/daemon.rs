use crate::{
    pipe_error, AppDescriptor, CatalogsDescriptor, PipeDescriptor, PipeManager, PipeOperation,
    PipeState, RepositoryManager, Result,
};
use std::path::PathBuf;

pub struct Daemon<'a> {
    repository_manager: RepositoryManager<'a>,
    pipe_manager: PipeManager<'a>,
}

// composite descriptor include all material to create a new pipe instance
pub struct Descriptor {
    pub id: String,
    pub description: String,
    pub user: Option<String>,
    pub group: Option<String>,
    pub app_descriptor: AppDescriptor,
    pub catalogs_descriptor: CatalogsDescriptor,
}

impl<'a> Daemon<'a> {
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
    pub fn create_pipe(&self, desc: &Descriptor) -> Result<()> {
        let app_descriptor = &desc.app_descriptor;
        let app_path = match self.check_app_registered(app_descriptor)? {
            Some(path) => path,
            None => {
                return Err(pipe_error(
                    PipeOperation::Create,
                    format!("app {} not found", app_descriptor),
                ))
            }
        };
        let catalogs_descriptor = &desc.catalogs_descriptor;
        let catalogs_path = match self.check_catalogs_registered(catalogs_descriptor)? {
            Some(path) => path,
            None => {
                return Err(pipe_error(
                    PipeOperation::Create,
                    format!("catalogs {} not found", catalogs_descriptor),
                ))
            }
        };
        let builder = PipeDescriptor::builder()
            .id(desc.id.as_str())
            .description(desc.description.as_str())
            .app_path(app_path.as_path())
            .catalogs_path(catalogs_path.as_path());
        let builder = match desc.user.as_ref() {
            Some(user) => builder.user(user.as_str()),
            None => builder,
        };
        let builder = match desc.group.as_ref() {
            Some(group) => builder.group(group.as_str()),
            None => builder,
        };
        let pipe_descriptor = builder.build();
        self.pipe_manager.create(pipe_descriptor)
    }

    pub fn start_pipe(&self, id: &str) -> Result<()> {
        self.pipe_manager.start(id)
    }

    pub fn stop_pipe(&self, id: &str) -> Result<()> {
        self.pipe_manager.stop(id)
    }

    pub fn delete_pipe(&self, id: &str) -> Result<()> {
        self.pipe_manager.delete(id)
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
            let pipe_state = self.pipe_status(pipe_id.as_str())?;
            pipe_states.push(pipe_state);
        }
        Ok(pipe_states)
    }
}
