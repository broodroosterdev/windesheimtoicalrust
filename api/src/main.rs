#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate reqwest;

mod les;
mod api;
mod rooster;
use rooster::*;


#[get("/<code>")]
fn default(code: String) -> IcalResponder {
    Rooster::from_klascode(code, RoosterType::Klas).to_responder()
}

#[get("/klas/<code>")]
fn klas(code: String) -> IcalResponder{
    Rooster::from_klascode(code, RoosterType::Klas).to_responder()
}

#[get("/docent/<code>")]
fn docent(code: String) -> IcalResponder{
    Rooster::from_klascode(code, RoosterType::Docent).to_responder()
}

#[get("/vak/<code>")]
fn vak(code: String) -> IcalResponder{
    Rooster::from_klascode(code, RoosterType::Vak).to_responder()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![default])
        .mount("/", routes![klas])
        .mount("/", routes![docent])
        .mount("/", routes![vak])
        .launch();
}