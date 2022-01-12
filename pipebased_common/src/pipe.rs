use crate::{
    chown, create_directory, grpc, link, open_lock_file, path_error, pipe_error, read_yml,
    write_yml, PathBuilder, Result, PATH_CATALOGS, PATH_PIPE_LOCK, PATH_PIPE_REGISTER,
    SYSTEMD_DEFAULT_DESCRIPTION, SYSTEMD_DEFAULT_GROUP, SYSTEMD_DEFAULT_START_UNIT_MODE,
    SYSTEMD_DEFAULT_STOP_UNIT_MODE, SYSTEMD_DEFAULT_USER,
};
use fslock::LockFile;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};
use systemd_client::{
    build_blocking_client, create_unit_configuration_file, delete_unit_configuration_file,
    manager::blocking::OrgFreedesktopSystemd1Manager, models::IntoModel,
    unit::blocking::UnitProperties, ServiceConfiguration, ServiceUnitConfiguration,
    SystemdObjectType, UnitActiveStateType, UnitConfiguration, UnitLoadStateType, UnitProps,
    UnitSubStateType,
};
use tracing::warn;

#[derive(Debug)]
pub enum PipeOperation {
    Create,
    Deregister,
    Register,
    Start,
    Status,
    Stop,
    Delete,
}

impl Display for PipeOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            PipeOperation::Create => "create",
            PipeOperation::Deregister => "deregister",
            PipeOperation::Register => "register",
            PipeOperation::Start => "start",
            PipeOperation::Status => "status",
            PipeOperation::Stop => "stop",
            PipeOperation::Delete => "delete",
        };
        write!(f, "{}", op)
    }
}

pub enum PipeLoadStateType {
    Stub,
    Loaded,
    NotFound,
    Error,
    Merged,
    Masked,
    Other(String),
}

impl From<UnitLoadStateType> for PipeLoadStateType {
    fn from(origin: UnitLoadStateType) -> Self {
        match origin {
            UnitLoadStateType::Stub => PipeLoadStateType::Stub,
            UnitLoadStateType::Loaded => PipeLoadStateType::Loaded,
            UnitLoadStateType::NotFound => PipeLoadStateType::NotFound,
            UnitLoadStateType::Error => PipeLoadStateType::Error,
            UnitLoadStateType::Merged => PipeLoadStateType::Merged,
            UnitLoadStateType::Masked => PipeLoadStateType::Masked,
            UnitLoadStateType::Other(other) => PipeLoadStateType::Other(other),
        }
    }
}

impl ToString for PipeLoadStateType {
    fn to_string(&self) -> String {
        match self {
            PipeLoadStateType::Stub => String::from("stub"),
            PipeLoadStateType::Loaded => String::from("loaded"),
            PipeLoadStateType::NotFound => String::from("not-found"),
            PipeLoadStateType::Error => String::from("error"),
            PipeLoadStateType::Merged => String::from("merged"),
            PipeLoadStateType::Masked => String::from("masked"),
            PipeLoadStateType::Other(other) => other.clone(),
        }
    }
}

pub enum PipeActiveStateType {
    Active,
    Reloading,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Other(String),
}

impl From<UnitActiveStateType> for PipeActiveStateType {
    fn from(origin: UnitActiveStateType) -> Self {
        match origin {
            UnitActiveStateType::Active => PipeActiveStateType::Active,
            UnitActiveStateType::Reloading => PipeActiveStateType::Reloading,
            UnitActiveStateType::Inactive => PipeActiveStateType::Inactive,
            UnitActiveStateType::Failed => PipeActiveStateType::Failed,
            UnitActiveStateType::Activating => PipeActiveStateType::Activating,
            UnitActiveStateType::Deactivating => PipeActiveStateType::Deactivating,
            UnitActiveStateType::Other(other) => PipeActiveStateType::Other(other),
        }
    }
}

impl ToString for PipeActiveStateType {
    fn to_string(&self) -> String {
        match self {
            PipeActiveStateType::Active => String::from("active"),
            PipeActiveStateType::Reloading => String::from("reloading"),
            PipeActiveStateType::Inactive => String::from("inactive"),
            PipeActiveStateType::Failed => String::from("failed"),
            PipeActiveStateType::Activating => String::from("activating"),
            PipeActiveStateType::Deactivating => String::from("deactivating"),
            PipeActiveStateType::Other(other) => other.clone(),
        }
    }
}

pub enum PipeSubStateType {
    AutoRestart,
    Dead,
    Exited,
    Failed,
    FinalSigterm,
    FinalSigkill,
    Reload,
    Running,
    Start,
    StartPre,
    StartPost,
    Stop,
    StopPost,
    StopSigabrt,
    StopSigterm,
    StopSigkill,
    Waiting,
    Other(String),
}

impl From<UnitSubStateType> for PipeSubStateType {
    fn from(origin: UnitSubStateType) -> Self {
        match origin {
            UnitSubStateType::AutoRestart => PipeSubStateType::AutoRestart,
            UnitSubStateType::Dead => PipeSubStateType::Dead,
            UnitSubStateType::Exited => PipeSubStateType::Exited,
            UnitSubStateType::Failed => PipeSubStateType::Failed,
            UnitSubStateType::FinalSigterm => PipeSubStateType::FinalSigterm,
            UnitSubStateType::FinalSigkill => PipeSubStateType::FinalSigkill,
            UnitSubStateType::Running => PipeSubStateType::Running,
            UnitSubStateType::Start => PipeSubStateType::Start,
            UnitSubStateType::StartPre => PipeSubStateType::StartPre,
            UnitSubStateType::StartPost => PipeSubStateType::StartPost,
            UnitSubStateType::Stop => PipeSubStateType::Stop,
            UnitSubStateType::StopPost => PipeSubStateType::StopPost,
            UnitSubStateType::StopSigabrt => PipeSubStateType::StopSigabrt,
            UnitSubStateType::StopSigterm => PipeSubStateType::StopSigterm,
            UnitSubStateType::StopSigkill => PipeSubStateType::StopSigkill,
            UnitSubStateType::Waiting => PipeSubStateType::Waiting,
            UnitSubStateType::Other(other) => PipeSubStateType::Other(other),
            _ => unreachable!(), // Active, Plugged, Mounted, Listening state is impossible for service unit
        }
    }
}

impl ToString for PipeSubStateType {
    fn to_string(&self) -> String {
        match self {
            PipeSubStateType::AutoRestart => String::from("auto-restart"),
            PipeSubStateType::Dead => String::from("dead"),
            PipeSubStateType::Exited => String::from("exited"),
            PipeSubStateType::Failed => String::from("failed"),
            PipeSubStateType::FinalSigterm => String::from("final-sigterm"),
            PipeSubStateType::FinalSigkill => String::from("final-sigkill"),
            PipeSubStateType::Reload => String::from("reload"),
            PipeSubStateType::Running => String::from("running"),
            PipeSubStateType::Start => String::from("start"),
            PipeSubStateType::StartPre => String::from("start-pre"),
            PipeSubStateType::StartPost => String::from("start-post"),
            PipeSubStateType::Stop => String::from("stop"),
            PipeSubStateType::StopPost => String::from("stop-post"),
            PipeSubStateType::StopSigabrt => String::from("stop-sigabrt"),
            PipeSubStateType::StopSigterm => String::from("stop-sigterm"),
            PipeSubStateType::StopSigkill => String::from("stop-sigkill"),
            PipeSubStateType::Waiting => String::from("waiting"),
            PipeSubStateType::Other(other) => other.clone(),
        }
    }
}

pub struct PipeState {
    // pipe id - systemd unit name
    pub id: String,
    pub load_state: PipeLoadStateType,
    pub active_state: PipeActiveStateType,
    pub sub_state: PipeSubStateType,
}

impl PipeState {
    pub fn is_inactive(&self) -> bool {
        matches!(self.active_state, PipeActiveStateType::Inactive)
    }

    pub fn is_dead(&self) -> bool {
        matches!(self.sub_state, PipeSubStateType::Dead)
    }
}

impl From<PipeState> for grpc::daemon::PipeState {
    fn from(origin: PipeState) -> Self {
        let id = origin.id;
        let load_state = origin.load_state.to_string();
        let active_state = origin.active_state.to_string();
        let sub_state = origin.sub_state.to_string();
        grpc::daemon::PipeState {
            id,
            load_state,
            active_state,
            sub_state,
        }
    }
}

#[derive(Clone)]
pub struct EnvironmentVariable {
    pub key: String,
    pub value: String,
}

#[derive(Clone)]
pub struct PipeDescriptor<'a> {
    // pipe id
    pub id: String,
    pub description: String,
    pub user: String,
    pub group: String,
    pub envs: Vec<EnvironmentVariable>,
    pub app_path: &'a Path,
    pub catalogs_path: &'a Path,
}

impl<'a> PartialEq for PipeDescriptor<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> PipeDescriptor<'a> {
    pub fn builder() -> PipeDescriptorBuilder<'a> {
        PipeDescriptorBuilder::default()
    }
}

pub struct PipeDescriptorBuilder<'a> {
    pub id: Option<String>,
    pub description: String,
    pub user: String,
    pub group: String,
    pub envs: Vec<EnvironmentVariable>,
    pub app_path: Option<&'a Path>,
    pub catalogs_path: Option<&'a Path>,
}

impl<'a> Default for PipeDescriptorBuilder<'a> {
    fn default() -> Self {
        PipeDescriptorBuilder {
            id: None,
            description: String::from(SYSTEMD_DEFAULT_DESCRIPTION),
            user: String::from(SYSTEMD_DEFAULT_USER),
            group: String::from(SYSTEMD_DEFAULT_GROUP),
            envs: vec![],
            app_path: None,
            catalogs_path: None,
        }
    }
}

impl<'a> PipeDescriptorBuilder<'a> {
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = user;
        self
    }

    pub fn group(mut self, group: String) -> Self {
        self.group = group;
        self
    }

    pub fn env(mut self, env: EnvironmentVariable) -> Self {
        self.envs.push(env);
        self
    }

    pub fn envs(mut self, envs: Vec<EnvironmentVariable>) -> Self {
        self.envs.extend(envs);
        self
    }

    pub fn app_path(mut self, app_path: &'a Path) -> Self {
        self.app_path = Some(app_path);
        self
    }

    pub fn catalogs_path(mut self, catalogs_path: &'a Path) -> Self {
        self.catalogs_path = Some(catalogs_path);
        self
    }

    pub fn build(self) -> PipeDescriptor<'a> {
        let id = self.id.expect("id undefined");
        let description = self.description;
        let user = self.user;
        let group = self.group;
        let envs = self.envs;
        let app_path = self.app_path.expect("app path undefined");
        let catalogs_path = self.catalogs_path.expect("catalogs path undefined");
        PipeDescriptor {
            id,
            description,
            user,
            group,
            envs,
            app_path,
            catalogs_path,
        }
    }
}

#[derive(Default)]
pub struct PipeUnitNameBuilder<'a> {
    id: Option<&'a str>,
}

impl<'a> PipeUnitNameBuilder<'a> {
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn build(self) -> String {
        let id = self.id.expect("pipe id undefined");
        format!("{}.service", id)
    }
}

pub struct PipeManager {
    pub workspace: PathBuf,
}

impl PipeManager {
    // create service configuration file and add pipe id into register
    pub(crate) fn create(&self, desc: PipeDescriptor<'_>) -> Result<()> {
        let mut lock_file = self.open_pipe_lock()?;
        lock_file.lock()?;
        let id = desc.id.as_str();
        let registered = self.do_check_pipe_registered(id)?;
        if registered {
            return Err(pipe_error(
                PipeOperation::Create,
                format!("conflict pipe id '{}'", id),
            ));
        }
        let unit_name = PipeUnitNameBuilder::default().id(id).build();
        if Self::do_get_unit(unit_name.as_str()).is_ok() {
            return Err(pipe_error(
                PipeOperation::Create,
                format!("invalid pipe unit name given id '{}'", id),
            ));
        }
        // init working directory
        let working_directory = self.do_create_working_directory(id)?;
        // link catalogs
        Self::do_link_catalogs(working_directory.as_path(), desc.catalogs_path)?;
        Self::do_create_ownership(
            desc.user.as_str(),
            desc.group.as_str(),
            working_directory.as_path(),
        )?;
        // create service configuration file
        Self::do_create_pipe_configuration_file(&desc, working_directory.as_path())?;
        self.do_register_pipe(id)
    }

    pub(crate) fn start(&self, id: &str) -> Result<()> {
        let mut lock_file = self.open_pipe_lock()?;
        lock_file.lock()?;
        let registered = self.do_check_pipe_registered(id)?;
        if !registered {
            return Err(pipe_error(
                PipeOperation::Start,
                format!("pipe '{}' not registered", id),
            ));
        }
        let unit_name = PipeUnitNameBuilder::default().id(id).build();
        let client = build_blocking_client(SystemdObjectType::Manager)?;
        let _ = client.start_unit(unit_name.as_str(), SYSTEMD_DEFAULT_START_UNIT_MODE)?;
        Ok(())
    }

    pub(crate) fn stop(&self, id: &str) -> Result<()> {
        let mut lock_file = self.open_pipe_lock()?;
        lock_file.lock()?;
        let registered = self.do_check_pipe_registered(id)?;
        if !registered {
            return Err(pipe_error(
                PipeOperation::Stop,
                format!("pipe '{}' not registered", id),
            ));
        }
        let unit_name = PipeUnitNameBuilder::default().id(id).build();
        let client = build_blocking_client(SystemdObjectType::Manager)?;
        let _ = client.stop_unit(unit_name.as_str(), SYSTEMD_DEFAULT_STOP_UNIT_MODE)?;
        Ok(())
    }

    pub(crate) fn status(&self, id: &str) -> Result<PipeState> {
        let mut lock_file = self.open_pipe_lock()?;
        lock_file.lock()?;
        let registered = self.do_check_pipe_registered(id)?;
        if !registered {
            return Err(pipe_error(
                PipeOperation::Status,
                format!("pipe '{}' not registered", id),
            ));
        }
        let unit_name = PipeUnitNameBuilder::default().id(id).build();
        let unit_path = Self::do_get_unit(unit_name.as_str())?;
        let unit_props = Self::do_get_unit_properties(unit_path)?;
        Ok(PipeState {
            id: id.to_owned(),
            load_state: unit_props.load_state.into(),
            active_state: unit_props.active_state.into(),
            sub_state: unit_props.sub_state.into(),
        })
    }

    // delete service configuration file and remove pipe id from register
    pub(crate) fn delete(&self, id: &str) -> Result<()> {
        let mut lock_file = self.open_pipe_lock()?;
        lock_file.lock()?;
        let registered = self.do_check_pipe_registered(id)?;
        if !registered {
            warn!("pipe '{}' not registered", id);
            return Ok(());
        }
        let state = self.status(id)?;
        // before pipe deletion, the process should be stopped first
        if !state.is_inactive() {
            return Err(pipe_error(
                PipeOperation::Delete,
                format!("pipe '{}' is not inactive", id),
            ));
        }
        if !state.is_dead() {
            return Err(pipe_error(
                PipeOperation::Delete,
                format!("pipe '{}' is not dead", id),
            ));
        }
        Self::do_delete_pipe_configuration_file(id)?;
        self.do_deregister_pipe(id)?;
        Ok(())
    }

    pub(crate) fn list_pipe_register(&self) -> Result<Vec<String>> {
        let mut lock_file = self.open_pipe_lock()?;
        lock_file.lock()?;
        self.do_read_pipe_register()
    }

    fn do_create_working_directory(&self, id: &str) -> Result<PathBuf> {
        let working_directory = PathBuilder::default()
            .push(self.workspace.as_path())
            .push(id)
            .build();
        create_directory(working_directory.as_path())?;
        Ok(working_directory)
    }

    fn do_link_catalogs(working_directory: &Path, catalogs_path: &Path) -> Result<()> {
        let catalogs_link_path = PathBuilder::default()
            .push(working_directory)
            .push(PATH_CATALOGS)
            .build();
        link(catalogs_path, catalogs_link_path.as_path(), true)
    }

    fn do_create_ownership(user: &str, group: &str, working_directory: &Path) -> Result<()> {
        // assume user and group created
        // grant ownership
        chown(user, group, working_directory, true)
    }

    // systemd service configuration file
    fn do_create_pipe_configuration_file(
        desc: &PipeDescriptor<'_>,
        working_directory: &Path,
    ) -> Result<()> {
        let unit = UnitConfiguration::builder().description(desc.description.as_str());
        let app_path = match desc.app_path.to_str() {
            Some(app_path) => app_path,
            None => {
                return Err(path_error(
                    String::from("path to str"),
                    String::from("app path is not valid unicode (UTF-8)"),
                ))
            }
        };
        let working_directory = match working_directory.to_str() {
            Some(working_directory) => working_directory,
            None => {
                return Err(path_error(
                    String::from("path to str"),
                    String::from("working_directory is not valid unicode (UTF-8)"),
                ))
            }
        };
        let service = ServiceConfiguration::builder()
            .exec_start(vec![app_path])
            .working_directory(working_directory)
            .user(desc.user.as_str())
            .group(desc.group.as_str());
        let service_unit = ServiceUnitConfiguration::builder()
            .unit(unit)
            .service(service)
            .build();
        let unit_name = PipeUnitNameBuilder::default().id(desc.id.as_str()).build();
        let buffer = format!("{}", service_unit);
        create_unit_configuration_file(unit_name.as_str(), buffer.as_bytes())?;
        Ok(())
    }

    fn do_delete_pipe_configuration_file(id: &str) -> Result<()> {
        let unit_name = PipeUnitNameBuilder::default().id(id).build();
        delete_unit_configuration_file(unit_name.as_str())?;
        Ok(())
    }

    fn do_get_unit(unit_name: &str) -> Result<dbus::Path> {
        let client = build_blocking_client(SystemdObjectType::Manager)?;
        let unit_path = client.get_unit(unit_name)?;
        Ok(unit_path)
    }

    fn do_get_unit_properties(unit_path: dbus::Path) -> Result<UnitProps> {
        let client = build_blocking_client(SystemdObjectType::Unit(unit_path))?;
        let unit_props = client.get_unit_properties()?;
        let unit_props: UnitProps = unit_props.into_model()?;
        Ok(unit_props)
    }

    // read pipe register
    fn do_read_pipe_register(&self) -> Result<Vec<String>> {
        let register_file_path = PathBuilder::default()
            .push(self.workspace.as_path())
            .push(PATH_PIPE_REGISTER)
            .build();
        match register_file_path.as_path().exists() {
            true => read_yml::<&Path, Vec<String>>(register_file_path.as_path()),
            false => Ok(vec![]),
        }
    }

    // write pipe register
    fn do_write_pipe_register(&self, ids: Vec<String>) -> Result<()> {
        let register_file_path = PathBuilder::default()
            .push(self.workspace.as_path())
            .push(PATH_PIPE_REGISTER)
            .build();
        write_yml(register_file_path.as_path(), &ids)
    }

    // register pipe id
    fn do_register_pipe(&self, id: &str) -> Result<()> {
        let mut ids = self.do_read_pipe_register()?;
        // append new pipe id
        ids.push(id.to_owned());
        self.do_write_pipe_register(ids)
    }

    fn do_deregister_pipe(&self, id: &str) -> Result<()> {
        let mut ids = self.do_read_pipe_register()?;
        let mut i: usize = 0;
        for pipe_id in ids.iter() {
            if pipe_id.as_str() == id {
                break;
            }
            i += 1;
        }
        let len = ids.len();
        assert!(i < len, "deregister pipe with invalid id '{}'", id);
        // avoid O(N) removal
        ids.swap(i, len - 1);
        ids.remove(len - 1);
        self.do_write_pipe_register(ids)
    }

    fn do_check_pipe_registered(&self, id: &str) -> Result<bool> {
        let ids = self.do_read_pipe_register()?;
        let mut i: usize = 0;
        for pipe_id in ids.iter() {
            if pipe_id.as_str() == id {
                break;
            }
            i += 1;
        }
        Ok(i < ids.len())
    }

    fn open_pipe_lock(&self) -> Result<LockFile> {
        let lock_file_path = PathBuilder::default()
            .push(self.workspace.as_path())
            .push(PATH_PIPE_LOCK)
            .build();
        open_lock_file(lock_file_path.as_path())
    }
}
