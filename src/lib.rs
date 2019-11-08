use std::{fs, io, path};
use std::io::Read;

mod machine;
pub mod assembler;
mod instructions;

pub fn load_file(p: &path::Path) -> String {
    let mut file: fs::File;
    {
        let r: io::Result<fs::File> = fs::File::open(p);
        file = r.unwrap();
    }
    let mut file_contents: String = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    file_contents
}
