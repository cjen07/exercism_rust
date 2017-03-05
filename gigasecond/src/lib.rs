extern crate chrono;

use chrono::*;

pub fn after(time: DateTime<UTC>) -> DateTime<UTC>{
    time + Duration::seconds(1_000_000_000)
}