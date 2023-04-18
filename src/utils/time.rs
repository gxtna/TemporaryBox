use chrono::{FixedOffset, NaiveDateTime, Utc};

pub fn get_local_time() -> NaiveDateTime {
    let china_timezone = FixedOffset::east_opt(8 * 3600).unwrap();
    let temp = Utc::now().with_timezone(&china_timezone);
    let date_time = temp.naive_local();
    date_time
}
