use crate::{chown_error, link_error, Result, CHARSET, ID_LEN};
use rand::Rng;
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

pub fn create_file<P>(path: P) -> Result<fs::File>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::create(path)?;
    Ok(file)
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
