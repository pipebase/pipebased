use crate::{PipeOperation, ResourceType};
use std::{
    env,
    ffi::OsStr,
    fmt::{Debug, Display},
    net,
    path::Path,
    result,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub struct Error(Box<ErrorImpl>);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum ErrorImpl {
    #[error("addr parse error, detail: {0:?}")]
    AddrParse(#[from] net::AddrParseError),
    #[error("chmod error, permission: {permission:?}, path: {path:?}, detail: {message:?}")]
    Chmod {
        permission: String,
        path: String,
        message: String,
    },
    #[error("chown error, user: {user:?}, group: {group:?}, path: {path:?}, detail: {message:?}")]
    Chown {
        user: String,
        group: String,
        path: String,
        message: String,
    },
    #[error("env error, detail: {0:?}")]
    Env(#[from] env::VarError),
    #[error("io error, detail: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("link error, from: {from:?}, to: {to:?}, detail: {message:?}")]
    Link {
        from: String,
        to: String,
        message: String,
    },
    #[error("path error, operation: {operation:?}, detail: {message:?}")]
    Path { operation: String, message: String },
    #[error("pipe error, operation: {operation:?}, detail: {message:?}")]
    Pipe {
        operation: PipeOperation,
        message: String,
    },
    #[error("pull repository error, resource: {resource:?}, detail: {error:?}")]
    Resource {
        resource: ResourceType,
        error: pipebuilder_common::Error,
    },
    #[error("systemd client error, detail: {0:?}")]
    Systemd(#[from] systemd_client::Error),
    #[error("tonic transport error, detail: {0:?}")]
    TonicTransport(#[from] tonic::transport::Error),
    #[error("utf8 error, detail: {0:?}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("yaml error, detail: {0:?}")]
    Yaml(#[from] serde_yaml::Error),
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Self {
        Error(Box::new(ErrorImpl::AddrParse(err)))
    }
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Self {
        Error(Box::new(ErrorImpl::Env(err)))
    }
}

impl From<std::io::Error> for Error {
    fn from(origin: std::io::Error) -> Self {
        Error(Box::new(ErrorImpl::Io(origin)))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(origin: std::string::FromUtf8Error) -> Self {
        Error(Box::new(ErrorImpl::Utf8(origin)))
    }
}

impl From<systemd_client::Error> for Error {
    fn from(origin: systemd_client::Error) -> Self {
        Error(Box::new(ErrorImpl::Systemd(origin)))
    }
}

impl From<dbus::Error> for Error {
    fn from(origin: dbus::Error) -> Self {
        let error: systemd_client::Error = origin.into();
        Error(Box::new(ErrorImpl::Systemd(error)))
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(origin: serde_yaml::Error) -> Self {
        Error(Box::new(ErrorImpl::Yaml(origin)))
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(err: tonic::transport::Error) -> Self {
        Error(Box::new(ErrorImpl::TonicTransport(err)))
    }
}

pub fn link_error<P, M>(from: P, to: P, message: M) -> Error
where
    P: AsRef<OsStr>,
    M: Display,
{
    let from = Path::new(&from).to_str().unwrap_or_default().to_owned();
    let to = Path::new(&to).to_str().unwrap_or_default().to_owned();
    let message = format!("{}", message);
    Error(Box::new(ErrorImpl::Link { from, to, message }))
}

pub fn chown_error<P, M>(user: String, group: String, path: P, message: M) -> Error
where
    P: AsRef<OsStr>,
    M: Display,
{
    let path = Path::new(&path).to_str().unwrap_or_default().to_owned();
    let message = format!("{}", message);
    Error(Box::new(ErrorImpl::Chown {
        user,
        group,
        path,
        message,
    }))
}

pub fn chmod_error<P, M>(permission: String, path: P, message: M) -> Error
where
    P: AsRef<OsStr>,
    M: Display,
{
    let path = Path::new(&path).to_str().unwrap_or_default().to_owned();
    let message = format!("{}", message);
    Error(Box::new(ErrorImpl::Chmod {
        permission,
        path,
        message,
    }))
}

pub fn path_error(operation: String, message: String) -> Error {
    Error(Box::new(ErrorImpl::Path { operation, message }))
}

pub fn pipe_error(operation: PipeOperation, message: String) -> Error {
    Error(Box::new(ErrorImpl::Pipe { operation, message }))
}

pub fn resource_error(resource: ResourceType, error: pipebuilder_common::Error) -> Error {
    Error(Box::new(ErrorImpl::Resource { resource, error }))
}
