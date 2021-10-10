use std::fs::File;
use std::io::prelude::*;
use rocket::response::content;
use std::io;

///입력된 주소에서 파일을 찾아 읽어서 String으로 반환합니다.
///
/// # Example
/// ```
/// let mut s = DataManager::FileIO::ReadHTMLFile("HTML/index/index.html").unwrap();
/// ```
///
pub fn ReadFile(path: &str) -> Result<String, io::Error>
{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(String::from(contents));
}

///입력된 주소에서 HTML파일을 찾아 읽어서 content::Html로 반환합니다.
///
/// # Example
/// ```
/// return DataManager::FileIO::ReadHTMLFile("HTML/index/index.html").unwrap();
/// ```
///
pub fn ReadHTMLFile(path: &str) -> Result<content::Html<String>, io::Error>
{
    let s = ReadFile(path)?;
    return Ok(content::Html(s));
}