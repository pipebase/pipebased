use crate::{
    chmod, create_recursive_directory_with_permission, open_lock_file, read_yml, resource_error,
    write_file, write_yml, PathBuilder, Result, PATH_APP, PATH_APP_LOCK, PATH_APP_REGISTER,
    PATH_CATALOGS, PATH_CATALOGS_LOCK, PATH_CATALOGS_REGISTER,
};
use fslock::LockFile;
use pipebuilder_common::api::{
    client::ApiClient as BuilderClient,
    models::{GetAppRequest, GetCatalogsRequest},
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::Path};

#[derive(Debug)]
pub enum ResourceType {
    App,
    Catalogs,
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::App => write!(f, "app"),
            ResourceType::Catalogs => write!(f, "catalogs"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AppDescriptor {
    pub namespace: String,
    pub id: String,
    pub version: u64,
}

impl PartialEq for AppDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace && self.id == other.id && self.version == other.version
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CatalogsDescriptor {
    pub namespace: String,
    pub id: String,
    pub version: u64,
}

impl PartialEq for CatalogsDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace && self.id == other.id && self.version == other.version
    }
}

pub struct RepositoryManager<'a> {
    app_directory: &'a Path,
    catalogs_directory: &'a Path,
    client: BuilderClient,
}

impl<'a> RepositoryManager<'a> {
    async fn do_pull_app(&self, desc: &AppDescriptor) -> Result<Vec<u8>> {
        let request = GetAppRequest {
            namespace: desc.namespace.clone(),
            id: desc.id.clone(),
            build_version: desc.version,
        };
        match self.client.pull_app(&request).await {
            Ok(resp) => Ok(resp.buffer),
            Err(err) => Err(resource_error(ResourceType::App, err)),
        }
    }

    async fn do_pull_catalogs(&self, desc: &CatalogsDescriptor) -> Result<Vec<u8>> {
        let request = GetCatalogsRequest {
            namespace: desc.namespace.clone(),
            id: desc.id.clone(),
            version: desc.version,
        };
        match self.client.pull_catalogs(&request).await {
            Ok(resp) => Ok(resp.buffer),
            Err(err) => Err(resource_error(ResourceType::Catalogs, err)),
        }
    }

    fn do_save_app(&self, desc: &AppDescriptor, buffer: &[u8]) -> Result<()> {
        let version = desc.version.to_string();
        create_recursive_directory_with_permission(
            &[
                self.app_directory,
                Path::new(desc.namespace.as_str()),
                Path::new(desc.id.as_str()),
                Path::new(version.as_str()),
            ],
            "+r",
        )?;
        let path = PathBuilder::default()
            .push(self.app_directory)
            .push(desc.namespace.as_str())
            .push(desc.id.as_str())
            .push(version.as_str())
            .push(PATH_APP)
            .build();
        write_file(path.as_path(), buffer)?;
        chmod("+x", path.as_path(), false)
    }

    async fn do_save_catalogs(&self, desc: &CatalogsDescriptor, buffer: &[u8]) -> Result<()> {
        let version = desc.version.to_string();
        create_recursive_directory_with_permission(
            &[
                self.catalogs_directory,
                Path::new(desc.namespace.as_str()),
                Path::new(desc.id.as_str()),
                Path::new(version.as_str()),
            ],
            "+r",
        )?;
        let path = PathBuilder::default()
            .push(self.app_directory)
            .push(desc.namespace.as_str())
            .push(desc.id.as_str())
            .push(version.as_str())
            .push(PATH_CATALOGS)
            .build();
        match BuilderClient::dump_catalogs(buffer, path.as_path()).await {
            Ok(_) => chmod("+r", path.as_path(), true),
            Err(err) => Err(resource_error(ResourceType::Catalogs, err)),
        }
    }

    fn open_app_lock(&self) -> Result<LockFile> {
        let lock_file_path = PathBuilder::default()
            .push(self.app_directory)
            .push(PATH_APP_LOCK)
            .build();
        open_lock_file(lock_file_path.as_path())
    }

    fn open_catalogs_lock(&self) -> Result<LockFile> {
        let lock_file_path = PathBuilder::default()
            .push(self.catalogs_directory)
            .push(PATH_CATALOGS_LOCK)
            .build();
        open_lock_file(lock_file_path.as_path())
    }

    fn do_register_app(&self, desc: &AppDescriptor) -> Result<()> {
        let mut lock_file = self.open_app_lock()?;
        // unlock when lock_file dropped
        // https://docs.rs/fslock/latest/fslock/struct.LockFile.html#method.unlock
        lock_file.lock()?;
        // read registered app
        let register_file_path = PathBuilder::default()
            .push(self.app_directory)
            .push(PATH_APP_REGISTER)
            .build();
        let mut descs = match register_file_path.as_path().exists() {
            true => read_yml::<&Path, Vec<AppDescriptor>>(register_file_path.as_path())?,
            false => vec![],
        };
        descs.push(desc.clone());
        write_yml(register_file_path.as_path(), &descs)
    }

    fn do_register_catalogs(&self, desc: &CatalogsDescriptor) -> Result<()> {
        let mut lock_file = self.open_catalogs_lock()?;
        // unlock when lock_file dropped
        // https://docs.rs/fslock/latest/fslock/struct.LockFile.html#method.unlock
        lock_file.lock()?;
        // read registered app
        let register_file_path = PathBuilder::default()
            .push(self.catalogs_directory)
            .push(PATH_CATALOGS_REGISTER)
            .build();
        let mut descs = match register_file_path.as_path().exists() {
            true => read_yml::<&Path, Vec<CatalogsDescriptor>>(register_file_path.as_path())?,
            false => vec![],
        };
        descs.push(desc.clone());
        write_yml(register_file_path.as_path(), &descs)
    }

    pub async fn pull_app(&self, desc: &AppDescriptor) -> Result<()> {
        let buffer = self.do_pull_app(desc).await?;
        self.do_save_app(desc, buffer.as_slice())?;
        self.do_register_app(desc)
    }

    pub async fn pull_catalogs(&self, desc: &CatalogsDescriptor) -> Result<()> {
        let buffer = self.do_pull_catalogs(desc).await?;
        self.do_save_catalogs(desc, buffer.as_slice()).await?;
        self.do_register_catalogs(desc)
    }

    pub fn is_app_local(&self, desc: &AppDescriptor) -> bool {
        let path = PathBuilder::default()
            .push(self.app_directory)
            .push(desc.id.as_str())
            .push(desc.version.to_string())
            .push(PATH_APP)
            .build();
        path.as_path().exists()
    }

    pub fn is_catalogs_local(&self, desc: &CatalogsDescriptor) -> bool {
        let path = PathBuilder::default()
            .push(self.catalogs_directory)
            .push(desc.id.as_str())
            .push(desc.version.to_string())
            .push(PATH_CATALOGS)
            .build();
        path.as_path().exists()
    }
}
