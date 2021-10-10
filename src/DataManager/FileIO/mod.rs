use std::fs::File;
use std::io::prelude::*;
use rocket::response::content;

//TODO: ReadFile에서 Panic말고 Result를 호출한 함수로 넘기는 방식으로 예외처리
///입력된 주소에서 파일을 찾아 읽어서 String으로 반환합니다.
///
/// # Example
/// ```
/// let mut s = DataManager::FileIO::ReadHTMLFile("HTML/index/index.html");
/// ```
/// # Panics
/// * **Can not open HTML file** : 경로에 맞는 파일이 존재하지 않아 열 수 없습니다.
/// * **Can not read HTML file** : 파일을 찾았으나 String으로 읽어낼 수 없습니다.
///
pub fn ReadFile(path: &str) -> String
{
    let mut file = File::open(path).expect("Can not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can not read file");

    return String::from(contents);
}

///입력된 주소에서 HTML파일을 찾아 읽어서 content::Html로 반환합니다.
///
/// # Example
/// ```
/// return ReadHTMLFile("HTML/index/index.html");
/// ```
///
/// # Panics
/// ReadFile을 호출합니다.
///
pub fn ReadHTMLFile(path: &str) -> content::Html<String>
{
    let mut s = ReadFile(path);
    return content::Html(s);
}