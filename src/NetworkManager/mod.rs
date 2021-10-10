use std::net::TcpStream;
use std::net::TcpListener;
use std::io::Write;

pub fn SendCode(questionNumber: i32, language: &str, code: &str)
{
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Can not connect judge server");

    let data = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><root><question_number>{}</question_number><language>{}</language><code_size>{}</code_size></root>",questionNumber, language, code.len());

    stream.write(data.as_bytes()).expect("Can not write data");
    stream.flush().expect("Can not flush data");
    stream.write(code.as_bytes()).expect("Can not write code");
    stream.flush().expect("Can not flush code");
}