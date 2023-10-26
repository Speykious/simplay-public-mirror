#![allow(unused_macros)]
#![allow(unused_imports)]

macro_rules! file_setup {
    (
        $digest: ident,
        $path: ident
    ) => {
        let bytes = std::fs::read($path.to_string())?;
    
        let checksum = {
            let mut string = String::new();
    
            for i in bytes.iter() {
                string.push_str(i.to_string().as_str());
                string.push_str(" ");
            }
    
            $digest(string)
        };
    
        return Ok(checksum);
    };
}

pub(crate) use file_setup;
