//! Module for handling filesystem utility command primarily.
//! Mostly a matter of convenience for future projects.
//!
use std::{
    env, 
    fs::{self, DirEntry, ReadDir}, 
    io::{self, BufReader, prelude::*}, 
    path::{Path, PathBuf}
};
use super::{RecolError, RecolResult};

#[doc = "Wrapper function for the env::current_dir() stdlib function"]
pub fn cwd() -> RecolResult<PathBuf> {
    env::current_dir().map_err(|e| RecolError::Io(e))
}

#[doc = "Simple wrapper function for the std::env::set_current_dir stdlib fn"]
pub fn cd<P: AsRef<Path>>(directory_path: P) -> RecolResult<()> {
    env::set_current_dir(directory_path).map_err(|e| RecolError::Io(e))
}

#[doc = "Return lines of a file in a buf reader"]
pub fn lines<P: AsRef<Path>>(file_path: P) -> RecolResult<Vec<String>> {
    fs::read_to_string(file_path)
        .map(|s| s.lines().map(|s| s.to_string()).collect())
        .map_err(|e| RecolError::Io(e))
}

#[doc = ""]
pub fn ls<P: Into<PathBuf>>(dir_path: Option<P>) -> RecolResult<Vec<DirEntry>> {
    let entries = dir_path.map_or(cwd()?, |p| p.into())
        .read_dir()
        .map_err(|e| RecolError::Io(e))?
        .filter_map(|dir| dir.ok())
        .collect();
    Ok(entries)
}

#[doc = ""]
pub fn search_file<P: AsRef<Path>>(file_path: P, inp: &str) -> RecolResult<Vec<String>> {
    let qu = fs::read_to_string(file_path)?;
    let r = qu.lines()
        .filter(|l| l.contains(inp))
        .map(|s| s.to_string());
    let r = r.collect::<Vec<String>>();
    Ok(r)
}

#[doc = ""]
pub fn write_to_file<B, P>(path: P, content: B) -> RecolResult<()> 
where 
    B: AsRef<[u8]>,
    P: AsRef<Path>
{
    fs::write(path, content).map_err(|e| RecolError::Io(e))
}

#[doc = ""]
pub fn rm<P: AsRef<Path>>(path: P) -> RecolResult<()> {
    if !path.as_ref().exists() {
        return Err(RecolError::Io(io::Error::from(io::ErrorKind::NotFound)));
    }
    if path.as_ref().is_file() { 
        fs::remove_file(path).map_err(|e| RecolError::Io(e))?;
    } else {
        fs::remove_dir_all(path).map_err(|e| RecolError::Io(e))?;
    }
    Ok(())
}

#[doc = ""]
pub fn touch<P: AsRef<Path>>(path: P) -> RecolResult<fs::File> {
    if path.as_ref().is_file() {
        return Err(RecolError::from(io::ErrorKind::AlreadyExists));
    } 
    fs::File::create(path).map_err(|e| RecolError::Io(e))
}

#[doc = ""]
pub fn mkdir<P: AsRef<Path>>(path: P) -> RecolResult<ReadDir> {
    if path.as_ref().is_file() {
        return Err(RecolError::from(io::ErrorKind::AlreadyExists));
    } 
    fs::create_dir_all(&path).map_err(|e| RecolError::Io(e))?;
    fs::read_dir(&path).map_err(|e| RecolError::Io(e))
}

#[doc = ""]
pub fn cat<P: AsRef<Path>>(path: P) -> RecolResult<String> {
    fs::read_to_string(path).map_err(|e| RecolError::Io(e))
}

#[doc = ""]
fn append<P: AsRef<Path>, B: AsRef<[u8]>>(path: P, content: B) -> RecolResult<()> {
    fs::OpenOptions::new()
        .append(true)
        .open(path)
        .map_err(|e| RecolError::Io(e))
        .map(|f| -> RecolResult<()> { 
            io::BufWriter::new(f)
                .write_all(&content.as_ref())
                .map_err(|e| RecolError::Io(e)) 
        })?
}


#[doc = ""]
pub fn copy<P: AsRef<Path>>(from: P, dest: P) -> RecolResult<u64> {
    fs::copy(from, dest).map_err(|e| RecolError::Io(e))
}
