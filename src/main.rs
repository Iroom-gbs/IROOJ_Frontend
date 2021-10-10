#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::response::{content, Redirect};
use IROOJ_Frontend::*;

#[get("/")]
fn index() -> content::Html<String> {
    let mut s = FileManager::ReadHTMLFile("HTML/index/index.html");
    return content::Html(s);
}

#[get("/test/problem?<code>")]
fn test_problem_code(code:Option<String>) -> content::Html<String> {
    if code.is_none()
    {
        let mut s = FileManager::ReadHTMLFile("HTML/test/problem.html");
        return content::Html(s);
    }
    else
    {
        let codestr = if let Some(String) = code {String};
        NetworkManager::SendCode(1,"C",&codestr[..]);
        let mut s = FileManager::ReadHTMLFile("HTML/index/index.html");
        return content::Html(s);
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, test_problem_code]).launch();
}