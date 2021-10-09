#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::response::content;
use IROOJ_Frontend::*;

#[get("/")]
fn index() -> content::Html<String> {
    let mut s = FileManager::ReadHTMLFile("HTML/index/index.html");
    return content::Html(s);
}

#[get("/list")]
pub fn list() -> & 'static str {
    "Problem list"
}

#[get("/user/<name>")]
fn user(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}


fn main() {
    rocket::ignite().mount("/", routes![index, list, user]).launch();
}