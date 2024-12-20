mod structs;

use chrono::{DateTime, Datelike, Local, NaiveDate};
use std::io;
use time::{self, Month};

fn main() {
    println!("Enter a command (or 'quit' to exit):");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "calendar" | "cal" | "c" => calendar(),
            "quit" | "q" => break,
            "help" | "h" => help(),
            _ => unknwn_cmd(input.trim()),
        }
    }
}

fn calendar() {
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
        "\x1B[34m\x1B[1m{}\x1B[0m
  \x1B[37mSu Mo Tu We Th Fr Sa\x1B[0m
  \x1B[30m====================\x1B[0m",
        month.to_string()
    );

    // * Shift over the first week so that the 1st ends up on the correct day
    for _i in 0..first_day_of_the_week {
        days_string.push_str("   ");
    }
    // * Create a mutable reference (?) to the first day of the week
    let mut index = first_day_of_the_week;

    for i in 0..days_in_month {
        let mut appendix: String;

        if i < 9 {
            appendix = format!("0{} ", i + 1);
            // * Append a 0 to the front of the date if it is less than 10 so that it is always represented as two digits
        } else {
            appendix = format!("{} ", i + 1);
        }
        if i as u32 == current_date.day0() {
            appendix = format!("\x1B[92m\x1B[1m{}\x1b[0m", appendix)
            // * Wrap the date with green text if it is today's date
        } else if index % 2 == 0 {
            appendix = format!("\x1B[30m{}\x1b[0m", appendix)
        }
        days_string.push_str(&appendix);

        // * Shift the week down if it is a Sunday
        if index >= 6 {
            days_string.push_str("\n  ");
            index = 0;
        } else {
            index = index + 1;
        }
    }
    println!("{}", days_string);
}

fn help() {
    println!(
        "--- Help page (1/1) ---
 - calendar (cal, c): Show this month's calendar
 - help (h): Displays this message
 - quit (q): Terminates the application"
    )
}

fn unknwn_cmd(cmd: &str) {
    println!("\x1B[31mError: Unknown command '{}'\x1B[0m", cmd);
    help();
}
