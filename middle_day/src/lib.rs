use chrono::NaiveDate;
use chrono::Datelike;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let date = NaiveDate::from_ymd_opt(year as i32, 7, 2);
    if !date?.leap_year() {
        Some(date?.weekday())
    } else {
        None
    }
}