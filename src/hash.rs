#![allow(dead_code)]

pub mod sha256 {
    use std::io;
    use sha256::*;
    use crate::filesystem::*;
    use crate::hash_boilerplate::*;

    pub fn file(path: &Path) -> Result<String, io::Error> {
        file_setup!(digest, path);
    }
}
