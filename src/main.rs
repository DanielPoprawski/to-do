fn main() {
    let month = "November";
    let first_day_of_the_week: u8 = 6;
    let mut days_string = String::from("  ");

    // * Create the Header for the calendar
    println!(
        "{}
  Su Mo Tu We Th Fr Sa
  ====================",
        month
    );

    // * Shift over the first week so that the 1st ends up on the correct day
    for i in 0..first_day_of_the_week {
        days_string.push_str("   ");
    }
    // * Create a mutable reference (?) to the first day of the week
    let mut index = first_day_of_the_week;

    for i in 1..31 {
        // * Append a 0 to the front of the date if it is less than 10 so that it is always represented as two digits
        if i < 10 {
            let appendix = format!("0{} ", i);
            days_string.push_str(&appendix);
        } else {
            let appendix = format!("{} ", i);
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
}
