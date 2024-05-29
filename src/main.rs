#![feature(proc_macro_hygiene, decl_macro)]

mod common;
mod data;

use percent_encoding::utf8_percent_encode;

use common::HopType;
use rocket::response::{content::RawHtml, Redirect};
use rocket::{get, launch, routes};

#[get("/opensearch.xml")]
fn opensearch_file() -> &'static str {
    include_str!("opensearch.xml")
}

#[get("/")]
fn index<'a>() -> RawHtml<&'static str> {
    RawHtml(data::HELP_PAGE.as_str())
}

#[get("/info/healthcheck")]
fn healthcheck() -> &'static str {
    "imok"
}

#[get("/search/<args>")]
fn search(args: &str) -> Redirect {
    let (cmd, argv, wholecmdstr) = common::validate(args);
    print!("Command is {}", cmd);
    if let Ok(e) = cmd.parse::<HopType>() {
        e.to_redirect(argv)
    } else {
        Redirect::to(format!(
            "https://google.com/search?q={}",
            utf8_percent_encode(&wholecmdstr, common::FRAGMENT).to_string()
        ))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search, opensearch_file, healthcheck])
}
