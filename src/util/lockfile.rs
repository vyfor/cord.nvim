use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use std::{env, fs};

pub struct ServerLock {
    path: PathBuf,
    _file: File,
}

impl ServerLock {
    pub fn new() -> Result<Self> {
        let path = Self::get_lock_path()?;
        let file = Self::open_file(&path)?;

        if !file.try_lock()? {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "Could not acquire a file lock while another instance is \
                 running",
            ));
        }

        Ok(ServerLock { path, _file: file })
    }

    fn get_lock_path() -> Result<PathBuf> {
        let mut path = env::temp_dir();
        path.push("cord-server.lock");

        Ok(path)
    }

    fn open_file(lock_path: &PathBuf) -> Result<File> {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(lock_path)
    }
}

impl Drop for ServerLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
