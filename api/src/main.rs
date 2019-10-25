#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate web_ical;

mod les;
mod api;

use rocket::http::ContentType;
use rocket::response::Content;
use rocket::response::Redirect;

#[get("/<code>")]
fn default(code: String) -> Redirect {
    Redirect::to(uri!(klas: code))
}

#[get("/<code>")]
fn klas(code: String) -> rocket::response::Content<std::fs::File>{
    Content(
        ContentType::Calendar, 
        api::get_ical(code, api::Rooster::Klas)
    )
}

#[get("/<code>")]
fn docent(code: String) -> rocket::response::Content<std::fs::File>{
    Content(
        ContentType::Calendar, 
        api::get_ical(code, api::Rooster::Docent)
    )
}

#[get("/<code>")]
fn vak(code: String) -> rocket::response::Content<std::fs::File>{
    Content(
        ContentType::Calendar, 
        api::get_ical(code, api::Rooster::Vak)
    )
}


fn main() {
    rocket::ignite()
        .mount("/", routes![default])
        .mount("/klas", routes![klas])
        .mount("/docent", routes![docent])
        .mount("/vak", routes![vak])
        .launch();
}