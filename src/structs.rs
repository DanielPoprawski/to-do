use chrono::DateTime;

#[allow(dead_code)]
struct Event<Tz: chrono::TimeZone> {
    date: DateTime<Tz>, // * Event start
    duration: u16,      // * Represented in 15 minute chunks. Max length is 682 hours days.
    name: String,
}
