#![allow(dead_code)]

use crate::filesystem::Path;

macro_rules! return_path {
    (
        $directory: ident
    ) => {
        return Path::new(&dirs::$directory().unwrap().display().to_string());
    }
}

pub fn home() -> Path {
    return_path!(home_dir);
}

pub fn config() -> Path {
    return_path!(config_dir);
}

pub fn cache() -> Path {
    return_path!(cache_dir);
}
