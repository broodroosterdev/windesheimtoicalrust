extern crate rocket;
use rocket::response::Responder;
use rocket::response::Content;
use rocket::http::Header;
use rocket::http::ContentType;

use chrono::{DateTime, Utc, SecondsFormat};
use std::time::{UNIX_EPOCH, Duration};
use ics::properties::{Categories, DtEnd, DtStart, Status, Summary, Location};
use ics::{Event, ICalendar};

use crate::api;
use crate::les::Les;

#[derive(Responder)]
#[response(status = 200, content_type = "text/calendar")]
pub struct IcalResponder {
    inner: Content<String>,
    header: Header<'static>
}

pub enum RoosterType {
    Klas,
    Docent,
    Vak 
}

pub struct Rooster {
    lessen: Vec<Les>,
    klascode: String,
    roostertype: RoosterType
}

impl Rooster {
    pub fn new(lessen: Vec<Les>, klascode: String, roostertype: RoosterType) -> Rooster {
        Rooster{
            lessen: lessen,
            klascode: klascode,
            roostertype: roostertype
        }
    }

    pub fn from_klascode(klascode: String, roostertype: RoosterType) -> Rooster {
        Rooster{
            lessen: api::get_lessen(&klascode, &roostertype),
            klascode: klascode,
            roostertype: roostertype
        }
    }

    pub fn to_ical(&self) -> ICalendar {
        assert!(!&self.lessen.is_empty());
        let mut ical = ICalendar::new("2.0", "-//WindesheimtoiCal//WindesheimiCal 1.0//EN");
        for i in &self.lessen {
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
        return ical;
    }

    pub fn to_responder(&self) -> IcalResponder{
        IcalResponder{
            inner: Content(
            ContentType::Calendar, 
            self.to_ical().to_string()
            ),
            header: Header::new("Content-Disposition", String::from("attachment; filename=\"{}.ics\"").replace("{}", &self.klascode))
        }
        
    }

    
}

fn format_date(string: String) -> String {
        return string.replace("-", "").replace(":", "");
}