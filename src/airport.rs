pub struct Airport {
    pub name: String,
    pub icao: String
}

impl Airport {
    pub fn new(name: String, icao: String) -> Airport {
        Airport { name, icao }
    }
}