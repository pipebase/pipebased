use crate::{
    chown, create_directory, link, path_error, pipe_error, PathBuilder, Result, PATH_CATALOGS,
    SYSTEMD_DEFAULT_DESCRIPTION, SYSTEMD_DEFAULT_GROUP, SYSTEMD_DEFAULT_START_UNIT_MODE,
    SYSTEMD_DEFAULT_STOP_UNIT_MODE, SYSTEMD_DEFAULT_USER,
};
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

#[derive(Debug)]
pub enum PipeOperation {
    Create,
    Start,
    Stop,
    Delete,
}

impl Display for PipeOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            PipeOperation::Create => "create",
            PipeOperation::Start => "start",
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

#[derive(Clone, Copy)]
pub struct PipeDescriptor<'a> {
    // pipe id
    pub id: &'a str,
    pub description: &'a str,
    pub user: &'a str,
    pub group: &'a str,
    pub app_path: &'a Path,
    pub catalogs_path: &'a Path,
}

impl<'a> PartialEq for PipeDescriptor<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct PipeDescriptorBuilder<'a> {
    pub id: Option<&'a str>,
    pub description: &'a str,
    pub user: &'a str,
    pub group: &'a str,
    pub app_path: Option<&'a Path>,
    pub catalogs_path: Option<&'a Path>,
}

impl<'a> Default for PipeDescriptorBuilder<'a> {
    fn default() -> Self {
        PipeDescriptorBuilder {
            id: None,
            description: SYSTEMD_DEFAULT_DESCRIPTION,
            user: SYSTEMD_DEFAULT_USER,
            group: SYSTEMD_DEFAULT_GROUP,
            app_path: None,
            catalogs_path: None,
        }
    }
}

impl<'a> PipeDescriptorBuilder<'a> {
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    pub fn user(mut self, user: &'a str) -> Self {
        self.user = user;
        self
    }

    pub fn group(mut self, group: &'a str) -> Self {
        self.group = group;
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
        let app_path = self.app_path.expect("app path undefined");
        let catalogs_path = self.catalogs_path.expect("catalogs path undefined");
        PipeDescriptor {
            id,
            description,
            user,
            group,
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

pub struct PipeManager<'a> {
    pub workspace: &'a Path,
}

impl<'a> PipeManager<'a> {
    pub fn create(&self, desc: PipeDescriptor<'_>) -> Result<()> {
        let id = desc.id;
        // verify pipe id conflict
        let unit_name = PipeUnitNameBuilder::default().id(id).build();
        if Self::get_unit(unit_name.as_str()).is_ok() {
            return Err(pipe_error(
                PipeOperation::Create,
                format!("conflict pipe id '{}'", id),
            ));
        }
        // TODO: register pipe id
        // init working directory
        let working_directory = self.create_working_directory(id)?;
        // link catalogs
        Self::link_catalogs(working_directory.as_path(), desc.catalogs_path)?;
        Self::create_ownership(desc.user, desc.group, working_directory.as_path())?;
        // create service configuration file
        Self::create_pipe_configuration_file(desc, working_directory.as_path())
    }

    pub fn start(&self, id: &str) -> Result<()> {
        let unit_name = PipeUnitNameBuilder::default().id(id).build();
        let client = build_blocking_client(SystemdObjectType::Manager)?;
        let _ = client.start_unit(unit_name.as_str(), SYSTEMD_DEFAULT_START_UNIT_MODE)?;
        Ok(())
    }

    pub fn stop(&self, id: &str) -> Result<()> {
        let unit_name = PipeUnitNameBuilder::default().id(id).build();
        let client = build_blocking_client(SystemdObjectType::Manager)?;
        let _ = client.stop_unit(unit_name.as_str(), SYSTEMD_DEFAULT_STOP_UNIT_MODE)?;
        Ok(())
    }

    pub fn status(&self, id: &str) -> Result<PipeState> {
        let unit_name = PipeUnitNameBuilder::default().id(id).build();
        let unit_path = Self::get_unit(unit_name.as_str())?;
        let unit_props = Self::get_unit_properties(unit_path)?;
        Ok(PipeState {
            id: id.to_owned(),
            load_state: unit_props.load_state.into(),
            active_state: unit_props.active_state.into(),
            sub_state: unit_props.sub_state.into(),
        })
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        let state = self.status(id)?;
        // before pipe deletion, the process should be stopped first
        if !state.is_inactive() {
            return Err(pipe_error(
                PipeOperation::Delete,
                format!("pipe {} is not inactive", id),
            ));
        }
        if !state.is_dead() {
            return Err(pipe_error(
                PipeOperation::Delete,
                format!("pipe {} is not dead", id),
            ));
        }
        let unit_name = PipeUnitNameBuilder::default().id(id).build();
        delete_unit_configuration_file(unit_name.as_str())?;
        // TODO: deregister pipe id
        Ok(())
    }

    fn create_working_directory(&self, id: &str) -> Result<PathBuf> {
        let working_directory = PathBuilder::default().push(self.workspace).push(id).build();
        create_directory(working_directory.as_path())?;
        Ok(working_directory)
    }

    fn link_catalogs(working_directory: &Path, catalogs_path: &Path) -> Result<()> {
        let catalogs_link_path = PathBuilder::default()
            .push(working_directory)
            .push(PATH_CATALOGS)
            .build();
        link(catalogs_path, catalogs_link_path.as_path(), true)
    }

    fn create_ownership(user: &str, group: &str, working_directory: &Path) -> Result<()> {
        // assume user and group created
        // grant ownership
        chown(user, group, working_directory, true)
    }

    // systemd service configuration file
    fn create_pipe_configuration_file(
        desc: PipeDescriptor<'_>,
        working_directory: &Path,
    ) -> Result<()> {
        let unit = UnitConfiguration::builder().description(desc.description);
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
            .user(desc.user)
            .group(desc.group);
        let service_unit = ServiceUnitConfiguration::builder()
            .unit(unit)
            .service(service)
            .build();
        let unit_name = PipeUnitNameBuilder::default().id(desc.id).build();
        let buffer = format!("{}", service_unit);
        create_unit_configuration_file(unit_name.as_str(), buffer.as_bytes())?;
        Ok(())
    }

    fn get_unit(unit_name: &str) -> Result<dbus::Path> {
        let client = build_blocking_client(SystemdObjectType::Manager)?;
        let unit_path = client.get_unit(unit_name)?;
        Ok(unit_path)
    }

    fn get_unit_properties(unit_path: dbus::Path) -> Result<UnitProps> {
        let client = build_blocking_client(SystemdObjectType::Unit(unit_path))?;
        let unit_props = client.get_unit_properties()?;
        let unit_props: UnitProps = unit_props.into_model()?;
        Ok(unit_props)
    }
}
