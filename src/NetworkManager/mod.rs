use std::net::TcpStream;
use std::io::{Write, BufReader, BufRead, Read};
use std::io;
use std::str;

use crate::DataManager;

///tcp 소켓 통신을 통해 채점 서버로 입력된 코드 데이터를 보냅니다.
///
/// # Example
/// ```
/// NetworkManager::SendCode(1,"C","#include <stdio.h>...").unwrap();
/// ```
///
pub fn SendCode(questionNumber: i32, language: &str, code: &str) -> Result<i32, io::Error>
{
    let ip_socket = DataManager::GetBackendSocket(5000);
    let mut stream = TcpStream::connect(ip_socket)?;

    let data = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><root><question_number>{}</question_number><language>{}</language><code_size>{}</code_size></root>",questionNumber, language, code.len());

    stream.write(data.as_bytes())?;
    stream.flush()?;
    stream.write(code.as_bytes())?;
    stream.flush()?;

    return Ok(0);
}

//TODO: panic!으로 주어지는 인코딩 에러 정보를 Result로 전환
///tcp 소켓 통신을 통해 String 데이터를 수신합니다.
///
/// # Example
/// ```
/// let mut reader = BufReader::new(tcpstream.try_clone().unwrap());
/// let k = NetworkManager::TCPGetString(&mut reader).unwrap();
/// ```
///
/// # Panics
/// * **Can not read size of string** : tcp통신의 첫 줄에 주어지는 문자열의 사이즈를 읽을 수 없습니다. 백엔드의 문제일 가능성이 있습니다.
/// * **Can not conver data to utf8** : 받은 데이터를 utf8로 인코딩 할 수 없습니다. 받은 데이터에 문제가 있을 가능성이 있습니다.
///
pub fn TCPGetString(reader: &mut BufReader<TcpStream>) -> Result<String, io::Error>
{
    let mut x = String::new();
    reader.read_line(&mut x)?;
    x.pop();
    let y = x.parse::<usize>().expect("Can not read size of string");
    let mut z: Vec<u8> = vec![0; y];
    reader.read(&mut z)?;
    let k = str::from_utf8(&mut z).expect("Can not convert data to utf8");

    return Ok(String::from(k));
}