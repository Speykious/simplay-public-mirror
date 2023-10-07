#![allow(dead_code)]

use std::io;
use std::fs;
use sha256::*;
use crate::filesystem::*;

pub fn file(path: Path) -> Result<String, io::Error> {
    let bytes = match fs::read(path.to_string()) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let checksum = {
        let mut string = String::new();

        for i in bytes.iter() {
            string.push_str(i.to_string().as_str());
            string.push_str(" ");
        }

        digest(string)
    };

    return Ok(checksum);
}
