#[derive(Serialize, Deserialize)]
pub struct Les {
    pub id: String,
    pub lokaal: String,
    pub starttijd: u64,
    pub eindtijd: u64,
    pub changed: bool,
    pub docentcode: Option<String>,
    pub roosterdatum: String,
    pub commentaar: String,
    pub status: bool,
    pub groepcode: String,
    pub vaknaam: String,
    pub vakcode: String,
    pub docentnamen: Vec<String>
}

/*
{
    "id": "6e9578ee-0ad9-4a6c-a231-3fadcbb07ce9",
    "lokaal": "F2.39",
    "starttijd": 1573727400000,
    "eindtijd": 1573734600000,
    "changed": false,
    "docentcode": null,
    "roosterdatum": "2019-11-14T00:00:00Z",
    "commentaar": "Werkcollege - LMvM3 Applied Operations Management-Werken aan de verkenningsopdracht",
    "status": false,
    "groepcode": "LM3B",
    "vaknaam": "LM M3.2 Applied Operations Management 2019-2020",
    "vakcode": "LM M3.2 Applied Operations Management 2019-2020",
    "docentnamen": [
      "HMJ Grolleman"
    ]
  }
*/