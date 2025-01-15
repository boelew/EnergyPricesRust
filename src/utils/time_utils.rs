// filepath: src/utils/time_utils.rs
use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};

pub fn get_time_in_history(days: i64) -> DateTime<Utc> {
    Utc::now() - Duration::days(days)
}

pub fn get_local_time() -> DateTime<Local> {
    Local::now()
}

pub fn set_to_midnight(dt: DateTime<Local>) -> DateTime<Local> {
    let naive = NaiveDateTime::new(
        dt.date().naive_local(),
        chrono::NaiveTime::from_hms(0, 0, 0),
    );
    DateTime::from_utc(naive, *dt.offset())
}

pub fn set_to_end_of_day(dt: DateTime<Local>) -> DateTime<Local> {
    let naive = NaiveDateTime::new(
        dt.date().naive_local(),
        chrono::NaiveTime::from_hms(23, 59, 59),
    );
    DateTime::from_utc(naive, *dt.offset())
}

pub fn format_date(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339()
}
