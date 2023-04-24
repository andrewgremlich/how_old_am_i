use chrono::prelude::*;
use regex::Regex;

mod age_in;

pub fn get_birth_age(birth_date: String) {
    let test_valid_birth_date = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    if !test_valid_birth_date.is_match(&birth_date) {
        println!("Invalid date format. Please use YYYY-MM-DD");
        std::process::exit(1);
    }

    let naive_birth_date = NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d").unwrap();
    let naive_now_date = Utc::now().naive_utc().date();

    if naive_birth_date > naive_now_date {
        println!("You are not born yet");
        std::process::exit(1);
    }

    age_in::process_dates(naive_birth_date, naive_now_date);
}
