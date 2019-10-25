extern crate reqwest;
use crate::les::Les;

use web_ical::Calendar;
use chrono::{DateTime, Utc};
use web_ical::Events;
use std::time::{UNIX_EPOCH, Duration};
use std::fs::File;

pub fn get_text(url: String) -> String {
    let req_url: &str = &*url;
    let response_text = reqwest::get(req_url)
        .expect("Couldnt make request")
        .text().expect("Couldnt make request");
    return response_text;
}
pub fn get_lessen(klascode: String) -> Vec<Les> {
    let mut url: String = "http://api.windesheim.nl/api/klas/".to_owned();
    let closing_url: String = "/Les".to_owned();
    url.push_str(&klascode);
    url.push_str(&closing_url);
    return parse_lessen(get_text(url));
}
pub fn parse_lessen(json: String) -> Vec<Les> {
    let rooster: Vec<Les> = serde_json::from_str(&json)
        .expect("Couldnt parse json");
    return rooster;
}

pub fn get_ical(klascode: String) -> File {
    let rooster = get_lessen(klascode);
    let mut ical =  Calendar::create(
    "-//My Business Inc//My Calendar 70.9054//EN",
    "2.0",
    "GREGORIAN",
    "PUBLISH",
    "www.broodrooster.dev/windesheim",
    "America/New_York");
    let date_tz: DateTime<Utc> = Utc::now();
    for i in &rooster {
        
        let own_event = Events{
            dtsart:         DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_millis(i.starttijd) - Duration::from_secs(7200)),
            dtend:          DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_millis(i.eindtijd) - Duration::from_secs(7200)), 
            dtstamp:        date_tz,
            uid:            i.id.to_string(),
            created:        date_tz,
            description:    i.docentnamen.join(","),
            last_modified:  date_tz,
            location:       i.lokaal.to_string(),
            sequence:       0,
            status:         i.status.to_string(),
            summary:        i.commentaar.to_string(),
            transp:         "OPAQUE".to_string()
        };
        ical.add_event(own_event);
    }
    match ical.export_ics("ical.ics"){
       Ok(_) => println!("OK"),
       Err(_) => panic!("Err")
    };
    let file = File::open("ical.ics").expect("Unable to open the file");
    file
}