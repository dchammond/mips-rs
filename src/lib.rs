//use lazy_static::lazy_static;
//use regex::Regex;

//use std::{fs, io, path};
//use std::io::Read;

mod machine;
mod assembler;
mod instructions;
/*

pub use machine::state::State;

pub fn load_file(state: &mut State, p: &path::Path, offset: Option<u32>) {
    lazy_static! {
        static ref COMMENT_RE: Regex = Regex::new(r"\s*#.*\s*\n").unwrap();
        static ref LABEL_CODE_RE: Regex = Regex::new(r"\s*(?P<label>\w+:)[^\n](?P<code>.+)").unwrap();
    }
    let mut file: fs::File;
    {
        let r: io::Result<fs::File> = fs::File::open(p);
        file = r.unwrap();
    }
    let mut file_contents: String = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    file_contents = COMMENT_RE.replace_all(&file_contents, "\n").to_string();
    file_contents = LABEL_CODE_RE.replace_all(&file_contents, "\n$label\n$code").to_string(); // dumb hack to put label on its own line
    let file_contents = file_contents.split("\n");
    let file_contents: Vec<&str> = file_contents.collect();
    state.load_text_instructions(&file_contents[..], offset);
}
*/

