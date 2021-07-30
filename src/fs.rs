//! Module for handling filesystem utility command primarily.
//! Mostly a matter of convenience for future projects.
//!
use std::{env, fs::{self, DirEntry, ReadDir}, io::{self, prelude::*}, path::PathBuf};
use super::{RecolError, RecolResult};

pub fn pwd() -> RecolResult<PathBuf> {
    let p = env::current_dir()?;
    return Ok(p)
}

pub fn cd<P: Into<PathBuf>>(dir: P) -> RecolResult<()> {
    env::set_current_dir(dir.into())?;
    return Ok(())
}

pub fn ls() -> RecolResult<()> {
    for d in fs::read_dir(pwd()?) {

    }
    Ok(())
}
pub fn search_file<P: Into<PathBuf>>(path: P, inp: &str) -> RecolResult<Vec<String>> {
    let f = path.into();
    let qu = fs::read_to_string(f)?;
    let r = qu.lines()
        .filter(|l| l.contains(inp))
        .map(|s| s.to_string());
    let r = r.collect::<Vec<String>>();
    Ok(r)
}

pub fn write_to_file<B>(content: B, path: &str) -> RecolResult<()> 
where
    B: Into<Vec<u8>> 
{
    let _ = fs::write(path, content.into())?;
    Ok(())
}

pub fn rm<P: Into<PathBuf>>(path: P) -> RecolResult<()> {
    let pb = path.into();
    if pb.is_file() { fs::remove_file(&pb)?; }
    else if pb.is_dir() { fs::remove_dir_all(&pb)?; }
    else { return Err(RecolError::GeneralError("No such file or directory".into())); }
    Ok(())
}

pub fn touch<P: Into<PathBuf>>(path: P) -> RecolResult<fs::File> {
    let pb = path.into();
    if pb.exists() && pb.is_file() {
        return Err(RecolError::GeneralError("File already exists".into()));
    } else {
        let file = fs::File::create(&pb)?;
        return Ok(file);
    }
}

pub fn mkdir<P: Into<PathBuf>>(path: P) -> RecolResult<ReadDir> {
    let pb = path.into();
    if pb.exists() && pb.is_dir() {
        return Err(RecolError::GeneralError("Dir already exists".into()));
    } else {
        fs::create_dir_all(&pb)?;
        let di = fs::read_dir(&pb)?;
        return Ok(di);
    }
}

pub fn cat<P: Into<PathBuf>>(path: P) -> RecolResult<String> {
    let pb = path.into();
    Ok(fs::read_to_string(&pb)?
        .to_string())
}
pub fn _lines(path: &str) -> io::Result<std::io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    let buf_reader = io::BufReader::new(file);
    Ok(buf_reader.lines())
}

fn _append_to_file(path: &str, content: &str) -> io::Result<()> {
    let file = fs::OpenOptions::new().append(true).open(path)?;
    let mut buf_writer = io::BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}


pub fn copy<P: Into<PathBuf>, Q: Into<PathBuf>>(from: P, dest: Q) -> RecolResult<()> {
    let (src, dest) = (from.into(), dest.into());
    if !src.exists() {
        return Err(RecolError::GeneralError("Src file doesn't exists".into()));
    }
    fs::copy(&src, &dest)?;
    Ok(())
}
