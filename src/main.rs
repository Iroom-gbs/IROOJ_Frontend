#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use IROOJ_Frontend::*;

use rocket::response::{content, Redirect};
use std::net::TcpStream;
use std::io::BufReader;

#[get("/")]
fn index() -> content::Html<String> {
    return DataManager::FileIO::ReadHTMLFile("HTML/index/index.html").unwrap();
}

#[get("/test/problem")]
fn test_problem() -> content::Html<String> {
    return DataManager::FileIO::ReadHTMLFile("HTML/test/problem.html").unwrap();
}

#[post("/test/problem", data="<code>")]
fn test_problem_post(code:String) -> Redirect {
    let mut codere = code.replace("+","%20").replace("code=","");
    let mut codes = urlencoding::decode(&codere[..]).unwrap();
    NetworkManager::SendCode(1, "C", &codes[..]).unwrap();
    return Redirect::to("/test/list");
}

#[get("/test/list")]
fn list() -> content::Html<String> {
    let ip_socket = DataManager::GetBackendSocket(8081);
    let stream = TcpStream::connect(ip_socket).unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    loop
    {
        let k = NetworkManager::TCPGetString(&mut reader).unwrap();
        println!("{}",k);
    }
    return DataManager::FileIO::ReadHTMLFile("HTML/test/list.html").unwrap();
}
fn main() {
    rocket::ignite().mount("/", routes![index, test_problem, test_problem_post,list]).launch();
}