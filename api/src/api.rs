extern crate reqwest;
extern crate ics;
use crate::les::Les;

use web_ical::Calendar;
use chrono::{DateTime, Utc, SecondsFormat};
use chrono::offset::Local;
use web_ical::Events;
use std::time::{UNIX_EPOCH, Duration};
use std::fs::File;
use ics::properties::{Categories, Description, DtEnd, DtStart, Organizer, Status, Summary, Location};
use ics::{escape_text, Event, ICalendar};

pub enum Rooster {
    Klas,
    Docent,
    Vak 
}

fn get_text(url: String) -> String {
    let req_url: &str = &*url;
    let response_text = reqwest::get(req_url)
        .expect("Couldnt make request")
        .text().expect("Couldnt make request");
    return response_text;
}

fn get_type_string(given_type: Rooster) -> &'static str {
    match given_type {
        Rooster::Docent => return "docent/",
        Rooster::Klas => return "klas/",
        Rooster::Vak => return "vak/",
    }
}

fn get_lessen(klascode: String, given_type: Rooster) -> Vec<Les> {
    let mut url: String = "http://api.windesheim.nl/api/".to_owned();
    let type_string: &str = get_type_string(given_type);
    let closing_url: String = "/Les".to_owned();
    url.push_str(&type_string);
    url.push_str(&klascode);
    url.push_str(&closing_url);
    return parse_lessen(get_text(url));
}

fn parse_lessen(json: String) -> Vec<Les> {
    let rooster: Vec<Les> = serde_json::from_str(&json)
        .expect("Couldnt parse json");
    return rooster;
}

fn format_date(string: String) -> String {
    return string.replace("-", "").replace(":", "");
}

pub fn get_ical(klascode: String, rooster_type: Rooster) -> File {
    let rooster = get_lessen(klascode, rooster_type);
    let mut ical = ICalendar::new("2.0", "-//WindesheimtoiCal//WindesheimiCal 1.0//EN");
    for i in &rooster {
        let mut event = Event::new(i.id.to_string(), format_date(i.roosterdatum.to_string()));
            event.push(DtStart::new(
                format_date(DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_millis(i.starttijd) - Duration::from_secs(7200)).to_rfc3339_opts(SecondsFormat::Secs, true))
                ));
            event.push(DtEnd::new(
                format_date(DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_millis(i.eindtijd) - Duration::from_secs(7200)).to_rfc3339_opts(SecondsFormat::Secs, true))
                ));
            event.push(Status::confirmed());
            event.push(Categories::new(i.vaknaam.to_string()));
            event.push(Summary::new(i.commentaar.to_string()));
            event.push(Location::new(i.lokaal.to_string()));
        ical.add_event(event);
    }
    
    ical.save_file("ical.ics").expect("Couldnt save ics file");
    let file = File::open("ical.ics").expect("Unable to open the file");
    return file
}