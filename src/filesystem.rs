#![allow(dead_code)]

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub enum PathType {
    File,
    Directory,
    Invalid,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct Path {
    path: String,
}

impl Path {
    pub fn new(user_path: &str) -> Self {
        let mut path = user_path.to_string();

        if std::env::consts::OS == "windows" {
            path = path.replace("/", "\\");
        }

        else {
            path = path.replace("\\", "/");
        }

        return Self {
            path,
        };
    }

    pub fn exists(&self) -> bool {
        return std::path::Path::new(self.path.as_str()).exists();
    }

    pub fn to_string(&self) -> String {
        return self.path.to_string();
    }

    pub fn basename(&self) -> String {
        let mut segments: Vec<String> = Vec::new();

        for i in self.to_string().split(Self::split_char()) {
            segments.push(i.to_string());
        }

        return segments[segments.len() - 1].to_string();
    }

    pub fn path_type(&self) -> PathType {
        if self.exists() == false {
            return PathType::Invalid;
        }

        if std::path::Path::new(self.to_string().as_str()).is_file() {
            return PathType::File;
        }

        else {
            return PathType::Directory;
        }
    }

    pub fn split_char() -> char {
        if std::env::consts::OS == "windows" {
            return '\\';
        }

        else {
            return '/';
        }
    }
}

pub mod directory {
    use std::io;
    use walkdir::WalkDir;
    use super::Path;

    pub fn list_items(path: Path) -> Result<Vec<Path>, io::Error> {
        let mut items: Vec<Path> = Vec::new();

        for i in match std::fs::read_dir(path.to_string()) {
            Ok(o) => o,
            Err(e) => return Err(e),
        } {
            items.push(match i {
                Ok(o) => Path::new(o.path().display().to_string().as_str()),
                Err(e) => return Err(e),
            });
        }

        return Ok(items);
    }

    pub fn list_items_recursive(path: Path) -> Result<Vec<Path>, io::Error> {
        let mut items: Vec<Path> = Vec::new();

        for i in WalkDir::new(path.to_string()) {
            items.push(Path::new(&match i {
                Ok(o) => o,
                Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Failed in WalkDir somewhere!")),
            }.path().display().to_string()));
        }

        return Ok(items);
    }

    pub fn create(path: Path) -> Result<(), io::Error> {
        return match std::fs::create_dir_all(path.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
}

pub mod file {
    use std::fs::File;
    use std::io::{self, Read, Write};
    use super::Path;

    pub fn read(path: Path) -> Result<String, io::Error> {
        let mut file = match File::open(path.to_string()) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        return Ok(contents);
    }

    pub fn write(contents: &str, path: Path) -> Result<(), io::Error> {
        let mut file = match File::create(path.to_string()) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };

        match file.write_all(contents.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        return Ok(());
    }
}

pub mod fs_action {
    use std::io;
    use std::fs;
    use super::{Path, PathType};

    /// Move a file or directory from point A, to point B!
    pub fn mv(path_from: Path, path_to: Path) -> Result<(), io::Error> {
        match copy(path_from.clone(), path_to.clone()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        match delete(path_from.clone()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        return Ok(());
    }

    /// Copy a file.
    pub fn copy(path_from: Path, path_to: Path) -> Result<(), io::Error> {
        if path_from.path_type() == PathType::File {
            match fs::copy(path_from.to_string(), path_to.to_string()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }

        else if path_from.path_type() == PathType::Directory {
            let mut options = fs_extra::dir::CopyOptions::new();

            options.copy_inside = true;

            match fs_extra::dir::copy(path_from.to_string(), path_to.to_string(), &options) {
                Ok(_) => (),
                Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Failed to copy directory!")),
            };
        }

        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid path!"));
        }

        return Ok(());
    }

    /// Delete a file. (Be careful!)
    pub fn delete(path: Path) -> Result<(), io::Error> {
        if path.path_type() == PathType::File {
            match fs::remove_file(path.to_string()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }

        else if path.path_type() == PathType::Directory {
            match fs::remove_dir_all(path.to_string()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }

        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid path!"));
        }

        return Ok(());
    }
}

pub mod archive {
    pub mod zip {
        use std::io;
        use std::fs;
        use std::path::PathBuf;
        use super::super::Path;

        pub fn extract(archive_path: Path, target_path: Path, strip_toplevel: bool) -> Result<(), io::Error> {
            let archive = fs::File::open(archive_path.to_string())?;
            let target = PathBuf::from(target_path.to_string());

            match zip_extract::extract(archive, &target, strip_toplevel) {
                Ok(_) => (),
                Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Failed to unzip archive!")),
            };

            return Ok(());
        }
    }
}
