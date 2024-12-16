// use std::io;
use chrono::{DateTime, Datelike, Local, NaiveDate};
use time::{self, Month};

fn main() {
    let current_date: DateTime<Local> = Local::now();
    // * Format: 2024-12-15 15:33:04.278536300 -05:00
    let current_month_number: u8 = current_date.month() as u8;
    // * Returns current month as a number (December as 12 for example)
    let month = match Month::try_from(current_month_number) {
        Ok(month) => month,
        Err(_month) => {
            println!("Invalid month");
            return;
        } // * Attempts to match the numerical month to a string, returning an error if one is one isn't found.
    };

    let days_in_month = month.length(current_date.year()); // * Gets the number of days in a month
    let first_day_of_the_week: u8 =
        match NaiveDate::from_ymd_opt(current_date.year(), current_date.month(), 1) {
            None => {
                println!("Invalid first of the month");
                return;
            }
            Some(date) => date.weekday().num_days_from_sunday() as u8,
        };
    let mut days_string = String::from("  ");

    // * Create the Header for the calendar
    println!(
        "{}
  Su Mo Tu We Th Fr Sa
  ====================",
        month.to_string()
    );

    // * Shift over the first week so that the 1st ends up on the correct day
    for _i in 0..first_day_of_the_week {
        days_string.push_str("   ");
    }
    // * Create a mutable reference (?) to the first day of the week
    let mut index = first_day_of_the_week;

    for i in 0..days_in_month {
        // * Append a 0 to the front of the date if it is less than 10 so that it is always represented as two digits
        if i < 9 {
            let appendix = format!("0{} ", i + 1);
            days_string.push_str(&appendix);
        } else {
            let appendix = format!("{} ", i + 1);
            days_string.push_str(&appendix);
        }
        // * Shift the week down if it is a Sunday
        if index >= 6 {
            days_string.push_str("\n  ");
            index = 0;
        } else {
            index = index + 1;
        }
    }
    println!("{}", days_string);

    //     loop {
    //         println!("Enter a command (or 'quit' to exit):");

    //         let mut input = String::new();
    //         io::stdin()
    //             .read_line(&mut input)
    //             .expect("Failed to read line");

    //         if input.trim() == "quit" || input.trim() == "q" {
    //             break;
    //         }
    //     }
}
