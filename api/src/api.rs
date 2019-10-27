extern crate reqwest;
extern crate ics;
use crate::les::Les;
use crate::rooster::RoosterType;


fn get_json(url: String) -> String {
    let req_url: &str = &*url;
    let response_text = reqwest::get(req_url)
        .expect("Couldnt make request")
        .text().expect("Couldnt make request");
    return response_text;
}

fn get_type_string(given_type: &RoosterType) -> &'static str {
    match given_type {
        RoosterType::Docent => return "docent/",
        RoosterType::Klas => return "klas/",
        RoosterType::Vak => return "vak/",
    }
}

pub fn get_lessen(klascode: &String, given_type: &RoosterType) -> Vec<Les> {
    let mut url: String = "http://api.windesheim.nl/api/".to_owned();
    let type_string: &str = get_type_string(&given_type);
    let closing_url: String = "/Les".to_owned();
    url.push_str(&type_string);
    url.push_str(&klascode);
    url.push_str(&closing_url);
    return parse_lessen(get_json(url));
}

fn parse_lessen(json: String) -> Vec<Les> {
    let rooster: Vec<Les> = serde_json::from_str(&json)
        .expect("Couldnt parse json");
    return rooster;
}
