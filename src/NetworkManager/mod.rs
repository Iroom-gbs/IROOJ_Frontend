use std::net::TcpStream;
use std::net::TcpListener;
use std::io::Write;

///tcp 소켓 통신을 통해 채점 서버로 입력된 코드 데이터를 보냅니다.
///
/// # Example
/// ```
/// NetworkManager::SendCode(1,"C","#include <stdio.h>...");
/// ```
///# Panics
/// * *Can not connect Judge Server* : 서버에 연결 할 수 없습니다.
/// * *Can not write data* : 서버에 보낼 데이터를 작성할 수 없습니다.
/// * *Can not flush data* : 서버에 데이터를 보낼 수 없습니다.
/// * *Can not write code* : 서버에 보낼 코드를 작성할 수 없습니다.
/// * *Can not flush code* : 서버에 코드를 보낼 수 없습니다.
///
pub fn SendCode(questionNumber: i32, language: &str, code: &str)
{
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Can not connect judge server");

    let data = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><root><question_number>{}</question_number><language>{}</language><code_size>{}</code_size></root>",questionNumber, language, code.len());

    stream.write(data.as_bytes()).expect("Can not write data");
    stream.flush().expect("Can not flush data");
    stream.write(code.as_bytes()).expect("Can not write code");
    stream.flush().expect("Can not flush code");
}