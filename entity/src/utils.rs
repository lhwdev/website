use chrono::TimeZone;
use rocket::serde::{Serialize, Deserialize};


pub type Time = chrono::DateTime<chrono::Utc>;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RawTime(i64);

impl Into<RawTime> for Time {
    fn into(self) -> RawTime {
        RawTime(self.timestamp_millis())
    }
}

impl Into<Time> for RawTime {
    fn into(self) -> Time {
        chrono::Utc
            .timestamp_opt(self.0 / 1000, ((self.0 % 1000) * 1_000_000) as u32)
            .unwrap()
    }
}



