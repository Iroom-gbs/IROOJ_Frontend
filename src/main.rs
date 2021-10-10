#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content;
use IROOJ_Frontend::*;
use std::net::TcpStream;
use std::io::BufReader;
use IROOJ_Frontend::DataManager::GetBackendIP;

#[get("/")]
fn index() -> content::Html<String> {
    return DataManager::FileIO::ReadHTMLFile("HTML/index/index.html").unwrap();
}

#[get("/test/problem?<code>")]
fn test_problem_code(code:Option<String>) -> content::Html<String> {
    if code.is_none()
    {
        return DataManager::FileIO::ReadHTMLFile("HTML/test/problem.html").unwrap();
    }
    else
    {
        let codestr = match code {Some(s)=>s, _=>String::from("error")};
        NetworkManager::SendCode(1,"C",&codestr[..]).unwrap();
        return DataManager::FileIO::ReadHTMLFile("HTML/test/re_list.html").unwrap();
    }
}

#[get("/test/list")]
fn list() -> content::Html<String> {
    let ip_socket = format!("{}:8081",GetBackendIP());
    let stream = TcpStream::connect(ip_socket).unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    loop{
    let k = NetworkManager::TCPGetString(&mut reader).unwrap();
    println!("{}",k);}
    return DataManager::FileIO::ReadHTMLFile("HTML/test/list.html").unwrap();
}
fn main() {
    rocket::ignite().mount("/", routes![index, test_problem_code, list]).launch();
}