use chrono::DateTime;

struct Event {
    date: DateTime, // * Event start
    duration: u16,  // * Represented in 15 minute chunks. Max length is 682 hours days.
}
