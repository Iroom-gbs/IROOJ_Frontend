use std::fs::File;
use std::io::prelude::*;

///입력된 주소에서 HTML파일을 찾아 읽어서 String으로 반환합니다.
///
/// # Example
/// ```
/// let mut s = FileManager::ReadHTMLFile("HTML/index/index.html");
/// ```
/// # Panics
/// * **Can not open HTML file** : 경로에 맞는 HTML 파일이 존재하지 않아 열 수 없습니다.
/// * **Can not read HTML file** : HTML파일을 찾았으나 문제가 생겨서 String으로 읽어낼 수 없습니다.
///
pub fn ReadHTMLFile(path: &str) -> String
{
    let mut file = File::open(path).expect("Can not open HTML file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can not read HTML file");

    return String::from(contents);
}

