#![feature(proc_macro_hygiene, decl_macro)]

mod common;
mod data;

use percent_encoding::utf8_percent_encode;
use std::io::Cursor;

use common::HopType;
use rocket::{
    get,
    http::ContentType,
    response::{self, Redirect},
    routes, Response,
};

#[get("/opensearch.xml")]
fn opensearch_file<'r>() -> Response<'r> {
    Response::build()
        .header(ContentType::new("application", "opensearchdescription+xml"))
        .sized_body(Cursor::new(include_str!("opensearch.xml")))
        .finalize()
}

#[get("/")]
fn index<'a>() -> response::Result<'a> {
    Response::build()
        .header(ContentType::HTML)
        .sized_body(Cursor::new(&*data::HELP_PAGE))
        .ok()
}

#[get("/info/healthcheck")]
fn healthcheck() -> &'static str {
    "imok"
}

#[get("/search?<args>")]
fn search(args: String) -> Redirect {
    let (cmd, argv, wholecmdstr) = common::validate(&args);
    if let Ok(e) = cmd.parse::<HopType>() {
        e.to_redirect(argv)
    } else {
        Redirect::to(format!(
            "https://google.com/search?q={}",
            utf8_percent_encode(&wholecmdstr, common::FRAGMENT).to_string()
        ))
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, search, opensearch_file, healthcheck])
        .launch();
}
