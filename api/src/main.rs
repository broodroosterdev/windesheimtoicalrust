#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate web_ical;
//use rocket_contrib::{Json, Value};

mod les;
mod api;

use rocket::http::ContentType;
use rocket::response::Content;


/*fn getLessen(klas: String) -> Result<(), reqwest::Error>{
    let rooster: Vec<Les> = reqwest::get("http://api.windesheim.nl/api/klas/ICTM1o/Les")?
    .json()?;
    println!("{:#?}", rooster[0]);
    Ok(())
}*/

#[get("/<klas>")]
fn index(klas: String) -> rocket::response::Content<std::fs::File>{
    Content(
        ContentType::Calendar, 
        api::get_ical(klas)
    )
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}