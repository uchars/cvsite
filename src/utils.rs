use chrono::{Local, NaiveDate, NaiveDateTime, Timelike};
pub fn get_time() -> String {
    let target_date = NaiveDate::from_ymd_opt(1999, 10, 14)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let current_date = Local::now();
    let current_date_naive = NaiveDateTime::from_timestamp(current_date.timestamp(), 0);
    let duration = current_date_naive.signed_duration_since(target_date);
    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    format!("{} days, {} hours, {} minutes", days, hours, minutes)
}
