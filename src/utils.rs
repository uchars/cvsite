use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

pub fn get_dhm_since(date: Option<NaiveDate>) -> String {
    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let default = NaiveDateTime::new(d, t);

    let target_date = date.unwrap().and_hms_opt(0, 0, 0).unwrap();
    let current_date = Local::now();
    let current_date_naive =
        NaiveDateTime::from_timestamp_opt(current_date.timestamp(), 0).unwrap_or(default);
    let duration = current_date_naive.signed_duration_since(target_date);
    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    format!("{} days, {} hours, {} minutes", days, hours, minutes)
}
