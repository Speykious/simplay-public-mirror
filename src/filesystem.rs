#![allow(dead_code)]

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
