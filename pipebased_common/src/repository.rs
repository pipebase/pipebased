use crate::{
    chmod, create_recursive_directory_with_permission, grpc, open_lock_file, read_yml,
    remove_directory, resource_error, write_file, write_yml, PathBuilder, Result, PATH_APP,
    PATH_APP_LOCK, PATH_APP_REGISTER, PATH_CATALOGS, PATH_CATALOGS_LOCK, PATH_CATALOGS_REGISTER,
};
use fslock::LockFile;
use pipebuilder_common::api::{
    client::{ApiClient as PbClient, ApiClientConfig as PbClientConfig},
    models::{GetAppRequest, GetCatalogsRequest},
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};
use tracing::warn;

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

impl Display for AppDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(namespace = {}, id = {}, version = {})",
            self.namespace, self.id, self.version
        )
    }
}

impl AppDescriptor {
    pub fn builder() -> AppDescriptorBuilder {
        AppDescriptorBuilder::default()
    }
}

pub struct AppDescriptorBuilder {
    pub namespace: Option<String>,
    pub id: Option<String>,
    pub version: Option<u64>,
}

impl AppDescriptorBuilder {
    pub fn new() -> Self {
        AppDescriptorBuilder {
            namespace: None,
            id: None,
            version: None,
        }
    }

    pub fn namespace(mut self, namespace: String) -> Self {
        self.namespace = Some(namespace);
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn version(mut self, version: u64) -> Self {
        self.version = Some(version);
        self
    }

    pub fn build(self) -> AppDescriptor {
        let namespace = self.namespace.expect("namespace undefined");
        let id = self.id.expect("id undefined");
        let version = self.version.expect("version undefined");
        AppDescriptor {
            namespace,
            id,
            version,
        }
    }
}

impl Default for AppDescriptorBuilder {
    fn default() -> Self {
        AppDescriptorBuilder::new()
    }
}

impl From<AppDescriptor> for grpc::daemon::AppDescriptor {
    fn from(origin: AppDescriptor) -> Self {
        let namespace = origin.namespace;
        let id = origin.id;
        let version = origin.version;
        grpc::daemon::AppDescriptor {
            namespace,
            id,
            version,
        }
    }
}

impl From<grpc::daemon::AppDescriptor> for AppDescriptor {
    fn from(origin: grpc::daemon::AppDescriptor) -> Self {
        let namespace = origin.namespace;
        let id = origin.id;
        let version = origin.version;
        AppDescriptor {
            namespace,
            id,
            version,
        }
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

impl Display for CatalogsDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(namespace = {}, id = {}, version = {})",
            self.namespace, self.id, self.version
        )
    }
}

impl CatalogsDescriptor {
    pub fn builder() -> CatalogsDescriptorBuilder {
        CatalogsDescriptorBuilder::default()
    }
}

pub struct CatalogsDescriptorBuilder {
    pub namespace: Option<String>,
    pub id: Option<String>,
    pub version: Option<u64>,
}

impl CatalogsDescriptorBuilder {
    pub fn new() -> Self {
        CatalogsDescriptorBuilder {
            namespace: None,
            id: None,
            version: None,
        }
    }

    pub fn namespace(mut self, namespace: String) -> Self {
        self.namespace = Some(namespace);
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn version(mut self, version: u64) -> Self {
        self.version = Some(version);
        self
    }

    pub fn build(self) -> CatalogsDescriptor {
        let namespace = self.namespace.expect("namespace undefined");
        let id = self.id.expect("id undefined");
        let version = self.version.expect("version undefined");
        CatalogsDescriptor {
            namespace,
            id,
            version,
        }
    }
}

impl Default for CatalogsDescriptorBuilder {
    fn default() -> Self {
        CatalogsDescriptorBuilder::new()
    }
}

impl From<CatalogsDescriptor> for grpc::daemon::CatalogsDescriptor {
    fn from(origin: CatalogsDescriptor) -> Self {
        let namespace = origin.namespace;
        let id = origin.id;
        let version = origin.version;
        grpc::daemon::CatalogsDescriptor {
            namespace,
            id,
            version,
        }
    }
}

impl From<grpc::daemon::CatalogsDescriptor> for CatalogsDescriptor {
    fn from(origin: grpc::daemon::CatalogsDescriptor) -> Self {
        let namespace = origin.namespace;
        let id = origin.id;
        let version = origin.version;
        CatalogsDescriptor {
            namespace,
            id,
            version,
        }
    }
}

#[derive(Deserialize)]
pub struct RepositoryManagerConfig {
    pub app_directory: String,
    pub catalogs_directory: String,
    pub pb_client: PbClientConfig,
}

pub struct RepositoryManagerBuilder {
    app_directory: Option<PathBuf>,
    catalogs_directory: Option<PathBuf>,
    pb_client: Option<PbClient>,
}

impl Default for RepositoryManagerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RepositoryManagerBuilder {
    pub fn new() -> Self {
        RepositoryManagerBuilder {
            app_directory: None,
            catalogs_directory: None,
            pb_client: None,
        }
    }

    pub fn app_directory(mut self, app_directory: String) -> Self {
        self.app_directory = Some(PathBuf::from(app_directory));
        self
    }

    pub fn catalogs_directory(mut self, catalogs_directory: String) -> Self {
        self.catalogs_directory = Some(PathBuf::from(catalogs_directory));
        self
    }

    pub fn pb_client(mut self, pb_client: PbClient) -> Self {
        self.pb_client = Some(pb_client);
        self
    }

    pub fn build(self) -> RepositoryManager {
        let app_directory = self.app_directory.expect("app directory undefined");
        let catalogs_directory = self
            .catalogs_directory
            .expect("catalogs directory undefined");
        let pb_client = self.pb_client.expect("pb client undefined");
        RepositoryManager {
            app_directory,
            catalogs_directory,
            pb_client,
        }
    }
}

pub struct RepositoryManager {
    app_directory: PathBuf,
    catalogs_directory: PathBuf,
    pb_client: PbClient,
}

impl RepositoryManager {
    pub fn builder() -> RepositoryManagerBuilder {
        RepositoryManagerBuilder::default()
    }

    pub(crate) async fn pull_app(&self, desc: &AppDescriptor) -> Result<()> {
        let buffer = self.do_pull_app(desc).await?;
        let mut lock_file = self.open_app_lock()?;
        lock_file.lock()?;
        let path = self.do_check_app_registered(desc)?;
        if path.is_some() {
            warn!(
                namespace = desc.namespace.as_str(),
                id = desc.id.as_str(),
                version = desc.version,
                "pull app already exists"
            );
            return Ok(());
        }
        self.do_save_app(desc, buffer.as_slice())?;
        self.do_register_app(desc)
    }

    pub(crate) async fn pull_catalogs(&self, desc: &CatalogsDescriptor) -> Result<()> {
        let buffer = self.do_pull_catalogs(desc).await?;
        let mut lock_file = self.open_catalogs_lock()?;
        lock_file.lock()?;
        let path = self.do_check_catalogs_registered(desc)?;
        if path.is_some() {
            warn!(
                namespace = desc.namespace.as_str(),
                id = desc.id.as_str(),
                version = desc.version,
                "pull catalogs already exists"
            );
            return Ok(());
        }
        self.do_save_catalogs(desc, buffer.as_slice()).await?;
        self.do_register_catalogs(desc)
    }

    pub(crate) fn remove_app(&self, desc: &AppDescriptor) -> Result<()> {
        let mut lock_file = self.open_app_lock()?;
        lock_file.lock()?;
        let path = self.do_check_app_registered(desc)?;
        if path.is_none() {
            warn!(
                resource = "app",
                namespace = desc.namespace.as_str(),
                id = desc.id.as_str(),
                version = desc.version,
                "remove resource not exists"
            );
            return Ok(());
        }
        self.do_remove_app(desc)?;
        self.do_deregister_app(desc)
    }

    pub(crate) fn remove_catalogs(&self, desc: &CatalogsDescriptor) -> Result<()> {
        let mut lock_file = self.open_catalogs_lock()?;
        lock_file.lock()?;
        let path = self.do_check_catalogs_registered(desc)?;
        if path.is_none() {
            warn!(
                resource = "catalogs",
                namespace = desc.namespace.as_str(),
                id = desc.id.as_str(),
                version = desc.version,
                "remove resource not exists"
            );
            return Ok(());
        }
        self.do_remove_catalogs(desc)?;
        self.do_deregister_catalogs(desc)
    }

    pub(crate) fn list_catalogs_register(&self) -> Result<Vec<CatalogsDescriptor>> {
        let mut lock_file = self.open_catalogs_lock()?;
        lock_file.lock()?;
        self.do_read_catalogs_register()
    }

    pub(crate) fn list_app_register(&self) -> Result<Vec<AppDescriptor>> {
        let mut lock_file = self.open_app_lock()?;
        lock_file.lock()?;
        self.do_read_app_register()
    }

    pub(crate) fn check_catalogs_registered(
        &self,
        desc: &CatalogsDescriptor,
    ) -> Result<Option<PathBuf>> {
        let mut lock_file = self.open_catalogs_lock()?;
        lock_file.lock()?;
        self.do_check_catalogs_registered(desc)
    }

    pub(crate) fn check_app_registered(&self, desc: &AppDescriptor) -> Result<Option<PathBuf>> {
        let mut lock_file = self.open_app_lock()?;
        lock_file.lock()?;
        self.do_check_app_registered(desc)
    }

    async fn do_pull_app(&self, desc: &AppDescriptor) -> Result<Vec<u8>> {
        let request = GetAppRequest {
            namespace: desc.namespace.clone(),
            id: desc.id.clone(),
            build_version: desc.version,
        };
        match self.pb_client.pull_app(&request).await {
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
        match self.pb_client.pull_catalogs(&request).await {
            Ok(resp) => Ok(resp.buffer),
            Err(err) => Err(resource_error(ResourceType::Catalogs, err)),
        }
    }

    fn do_save_app(&self, desc: &AppDescriptor, buffer: &[u8]) -> Result<()> {
        let version = desc.version.to_string();
        create_recursive_directory_with_permission(
            &[
                self.app_directory.as_path(),
                Path::new(desc.namespace.as_str()),
                Path::new(desc.id.as_str()),
                Path::new(version.as_str()),
            ],
            "+r",
        )?;
        let path = PathBuilder::default()
            .push(self.app_directory.as_path())
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
                self.catalogs_directory.as_path(),
                Path::new(desc.namespace.as_str()),
                Path::new(desc.id.as_str()),
                Path::new(version.as_str()),
            ],
            "+r",
        )?;
        let path = PathBuilder::default()
            .push(self.catalogs_directory.as_path())
            .push(desc.namespace.as_str())
            .push(desc.id.as_str())
            .push(version.as_str())
            .push(PATH_CATALOGS)
            .build();
        match PbClient::dump_catalogs(buffer, path.as_path()).await {
            Ok(_) => chmod("+r", path.as_path(), true),
            Err(err) => Err(resource_error(ResourceType::Catalogs, err)),
        }
    }

    fn do_remove_catalogs(&self, desc: &CatalogsDescriptor) -> Result<()> {
        let path = PathBuilder::default()
            .push(self.catalogs_directory.as_path())
            .push(desc.namespace.as_str())
            .push(desc.id.as_str())
            .push(desc.version.to_string())
            .build();
        remove_directory(path.as_path())
    }

    fn do_remove_app(&self, desc: &AppDescriptor) -> Result<()> {
        let path = PathBuilder::default()
            .push(self.app_directory.as_path())
            .push(desc.namespace.as_str())
            .push(desc.id.as_str())
            .push(desc.version.to_string())
            .build();
        remove_directory(path.as_path())
    }

    fn open_app_lock(&self) -> Result<LockFile> {
        let lock_file_path = PathBuilder::default()
            .push(self.app_directory.as_path())
            .push(PATH_APP_LOCK)
            .build();
        open_lock_file(lock_file_path.as_path())
    }

    fn open_catalogs_lock(&self) -> Result<LockFile> {
        let lock_file_path = PathBuilder::default()
            .push(self.catalogs_directory.as_path())
            .push(PATH_CATALOGS_LOCK)
            .build();
        open_lock_file(lock_file_path.as_path())
    }

    // read app register
    fn do_read_app_register(&self) -> Result<Vec<AppDescriptor>> {
        let register_file_path = PathBuilder::default()
            .push(self.app_directory.as_path())
            .push(PATH_APP_REGISTER)
            .build();
        match register_file_path.as_path().exists() {
            true => read_yml::<&Path, Vec<AppDescriptor>>(register_file_path.as_path()),
            false => Ok(vec![]),
        }
    }

    fn do_write_app_register(&self, descs: Vec<AppDescriptor>) -> Result<()> {
        let register_file_path = PathBuilder::default()
            .push(self.app_directory.as_path())
            .push(PATH_APP_REGISTER)
            .build();
        write_yml(register_file_path.as_path(), &descs)
    }

    fn do_register_app(&self, desc: &AppDescriptor) -> Result<()> {
        // read registered app
        let mut apps = self.do_read_app_register()?;
        apps.push(desc.clone());
        self.do_write_app_register(apps)
    }

    fn do_deregister_app(&self, desc: &AppDescriptor) -> Result<()> {
        let mut apps = self.do_read_app_register()?;
        let mut i: usize = 0;
        for app in apps.iter() {
            if app == desc {
                break;
            }
            i += 1;
        }
        let len = apps.len();
        assert!(i < len, "app descriptor {} not found in register", desc);
        apps.swap(i, len - 1);
        apps.remove(len - 1);
        self.do_write_app_register(apps)
    }

    // read catalogs register
    fn do_read_catalogs_register(&self) -> Result<Vec<CatalogsDescriptor>> {
        let register_file_path = PathBuilder::default()
            .push(self.catalogs_directory.as_path())
            .push(PATH_CATALOGS_REGISTER)
            .build();
        match register_file_path.as_path().exists() {
            true => read_yml::<&Path, Vec<CatalogsDescriptor>>(register_file_path.as_path()),
            false => Ok(vec![]),
        }
    }

    fn do_write_catalogs_register(&self, descs: Vec<CatalogsDescriptor>) -> Result<()> {
        let register_file_path = PathBuilder::default()
            .push(self.catalogs_directory.as_path())
            .push(PATH_CATALOGS_REGISTER)
            .build();
        write_yml(register_file_path.as_path(), &descs)
    }

    fn do_register_catalogs(&self, desc: &CatalogsDescriptor) -> Result<()> {
        let mut catalogs = self.do_read_catalogs_register()?;
        catalogs.push(desc.clone());
        self.do_write_catalogs_register(catalogs)
    }

    fn do_deregister_catalogs(&self, desc: &CatalogsDescriptor) -> Result<()> {
        let mut catalogs = self.do_read_catalogs_register()?;
        let mut i: usize = 0;
        for catalog in catalogs.iter() {
            if catalog == desc {
                break;
            }
            i += 1;
        }
        let len = catalogs.len();
        assert!(
            i < len,
            "catalogs descriptor {} not found in register",
            desc
        );
        catalogs.swap(i, len - 1);
        catalogs.remove(len - 1);
        self.do_write_catalogs_register(catalogs)
    }

    // app exists at local repository
    fn do_check_app_registered(&self, desc: &AppDescriptor) -> Result<Option<PathBuf>> {
        let apps = self.do_read_app_register()?;
        let mut i: usize = 0;
        for app in apps.iter() {
            if app == desc {
                break;
            }
            i += 1;
        }
        let exists_in_register = i < apps.len();
        let path = PathBuilder::default()
            .push(self.app_directory.as_path())
            .push(desc.namespace.as_str())
            .push(desc.id.as_str())
            .push(desc.version.to_string())
            .push(PATH_APP)
            .build();
        let exists_path = path.as_path().exists();
        if exists_in_register != exists_path {
            warn!(
                resource = "app",
                namespace = desc.namespace.as_str(),
                id = desc.id.as_str(),
                version = desc.version,
                "exists in register({}) != exists path({}), conflict",
                exists_in_register,
                exists_path
            );
        }
        match exists_in_register {
            true => Ok(Some(path)),
            false => Ok(None),
        }
    }

    // catalogs exists at local repository
    fn do_check_catalogs_registered(&self, desc: &CatalogsDescriptor) -> Result<Option<PathBuf>> {
        let catalogs = self.do_read_catalogs_register()?;
        let mut i: usize = 0;
        for catalog in catalogs.iter() {
            if catalog == desc {
                break;
            }
            i += 1;
        }
        let exists_in_register = i < catalogs.len();
        let path = PathBuilder::default()
            .push(self.catalogs_directory.as_path())
            .push(desc.namespace.as_str())
            .push(desc.id.as_str())
            .push(desc.version.to_string())
            .push(PATH_CATALOGS)
            .build();
        let exists_path = path.as_path().exists();
        if exists_in_register != exists_path {
            warn!(
                resource = "catalogs",
                namespace = desc.namespace.as_str(),
                id = desc.id.as_str(),
                version = desc.version,
                "exists in register({}) != exists path({}), conflict",
                exists_in_register,
                exists_path
            );
        }
        match exists_in_register {
            true => Ok(Some(path)),
            false => Ok(None),
        }
    }
}
