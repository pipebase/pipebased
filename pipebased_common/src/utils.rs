use crate::{chmod_error, chown_error, link_error, Result, CHARSET, ID_LEN};
use fslock::{LockFile, ToOsStr};
use rand::Rng;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    ffi::{OsStr, OsString},
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
};

pub fn generate_random_id() -> String {
    let mut rng = rand::thread_rng();
    let id: String = (0..ID_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    id
}

// fs ops
pub fn create_directory<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn create_recursive_directory_with_permission<P>(paths: &[P], permission: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut buffer = PathBuf::new();
    for path in paths {
        buffer.push(path);
        if !buffer.as_path().exists() {
            fs::create_dir(buffer.as_path())?;
            chmod(permission, buffer.as_path(), true)?;
        }
    }
    Ok(())
}

pub fn create_file<P>(path: P) -> Result<fs::File>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::create(path)?;
    Ok(file)
}

pub fn read_file<P>(path: P) -> Result<Vec<u8>>
where
    P: AsRef<std::path::Path>,
{
    let buffer = fs::read(path)?;
    Ok(buffer)
}

pub fn read_yml<P, T>(path: P) -> Result<T>
where
    P: AsRef<std::path::Path>,
    T: DeserializeOwned,
{
    let buffer = read_file(path)?;
    let t = serde_yaml::from_slice::<T>(buffer.as_slice())?;
    Ok(t)
}

pub fn write_yml<P, T>(path: P, t: &T) -> Result<()>
where
    P: AsRef<std::path::Path>,
    T: Serialize,
{
    let buffer = serde_yaml::to_vec(t)?;
    write_file(path, buffer.as_slice())
}

pub fn write_file<P>(path: P, buffer: &[u8]) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    let file = create_file(path)?;
    let mut wrt = BufWriter::new(file);
    wrt.write_all(buffer)?;
    wrt.flush()?;
    Ok(())
}

pub fn open_lock_file<P>(path: &P) -> Result<LockFile>
where
    P: ToOsStr + ?Sized,
{
    let file = LockFile::open(path)?;
    Ok(file)
}

// os command
// run cmd and collect status and output
fn cmd_status_output(mut cmd: Command) -> Result<(i32, String)> {
    let output = cmd.output()?;
    match output.status.success() {
        true => {
            let stderr = String::from_utf8(output.stderr)?;
            Ok((0, stderr))
        }
        false => {
            let stderr = String::from_utf8(output.stderr)?;
            let err_code = output.status.code().unwrap_or(1);
            Ok((err_code, stderr))
        }
    }
}

// run cmd and collect status
/*
fn cmd_status(mut cmd: Command) -> Result<i32> {
    let status = cmd.status()?;
    match status.success() {
        true => Ok(0),
        false => Ok(status.code().unwrap_or(1)),
    }
}
*/

fn link_binary() -> OsString {
    "ln".to_owned().into()
}

pub fn link<P>(from: P, to: P, soft: bool) -> Result<()>
where
    P: AsRef<OsStr>,
{
    let mut cmd = Command::new(link_binary());
    if soft {
        cmd.arg("-s");
    }
    cmd.arg(&from).arg(&to);
    let (code, out) = cmd_status_output(cmd)?;
    match code == 0 {
        true => Ok(()),
        false => Err(link_error(from, to, out)),
    }
}

fn chown_binary() -> OsString {
    "chown".to_owned().into()
}

pub fn chown<P>(user: &str, group: &str, path: P, recursive: bool) -> Result<()>
where
    P: AsRef<OsStr>,
{
    let mut cmd = Command::new(chown_binary());
    if recursive {
        cmd.arg("-R");
    }
    cmd.arg(format!("{}:{}", user, group)).arg(&path);
    let (code, out) = cmd_status_output(cmd)?;
    match code == 0 {
        true => Ok(()),
        false => Err(chown_error(user.to_owned(), group.to_owned(), path, out)),
    }
}

fn chmod_binary() -> OsString {
    "chmod".to_owned().into()
}

pub fn chmod<P>(permission: &str, path: P, recursive: bool) -> Result<()>
where
    P: AsRef<OsStr>,
{
    let mut cmd = Command::new(chmod_binary());
    if recursive {
        cmd.arg("-R");
    }
    cmd.arg(permission).arg(&path);
    let (code, out) = cmd_status_output(cmd)?;
    match code == 0 {
        true => Ok(()),
        false => Err(chmod_error(permission.to_owned(), path, out)),
    }
}

// path builder
#[derive(Default)]
pub struct PathBuilder {
    buffer: PathBuf,
}

impl PathBuilder {
    pub fn push<P>(mut self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.buffer.push(path);
        self
    }

    pub fn build(self) -> PathBuf {
        self.buffer
    }

    pub fn clone_from(path: &Path) -> Self {
        PathBuilder {
            buffer: path.to_path_buf(),
        }
    }
}
