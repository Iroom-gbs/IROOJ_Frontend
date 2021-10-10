#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::response::{content, Redirect};
use IROOJ_Frontend::*;
use std::net::TcpStream;
use std::io::Read;

#[get("/")]
fn index() -> content::Html<String> {
    return DataManager::FileIO::ReadHTMLFile("HTML/index/index.html");
}

#[get("/test/problem?<code>")]
fn test_problem_code(code:Option<String>) -> content::Html<String> {
    if code.is_none()
    {
        return DataManager::FileIO::ReadHTMLFile("HTML/test/problem.html");
    }
    else
    {
        let mut codestr = match code {Some(s)=>s, _=>String::from("error")};
        NetworkManager::SendCode(1,"C",&codestr[..]);
        return DataManager::FileIO::ReadHTMLFile("HTML/test/re_list.html");
    }
}

#[get("/test/list")]
fn list() -> content::Html<String> {
    let mut stream = TcpStream::connect("222.237.120.237:8081").unwrap();
    println!("a");
    let mut s = String::new();
    println!("b");
    let i = stream.read_to_string(&mut s).unwrap();
    println!("{}",i);
    return DataManager::FileIO::ReadHTMLFile("HTML/test/list.html");
}
fn main() {
    rocket::ignite().mount("/", routes![index, test_problem_code, list]).launch();
}