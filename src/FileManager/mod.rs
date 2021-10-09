use std::fs::File;
use std::io::prelude::*;

pub fn ReadHTMLFile(path: &str) -> String
{
    let mut file = File::open(path).expect("Can not open HTML file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can not read HTML file");

    return String::from(contents);

}