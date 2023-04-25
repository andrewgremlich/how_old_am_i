use chrono::prelude::*;
use how_old_am_i_lib::cli::{get_banana_env, get_detailed_env};
use regex::Regex;

mod age_in;

use age_in::BirthDateInfo;

pub fn get_birth_data(birth_date: String) {
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

    let birth_info = BirthDateInfo::new(naive_birth_date, naive_now_date);

    println!(
        "You are {} years, {} months and {} days old",
        birth_info.find_year_month_day().0,
        birth_info.find_year_month_day().1,
        birth_info.find_year_month_day().2
    );

    if get_detailed_env() {
        println!("That is {} months old", birth_info.age_in_months.floor());
        println!("That is {} days old", birth_info.age_in_days);
        println!("That is {} hours old", birth_info.age_in_hours);
        println!("That is {} minutes old", birth_info.age_in_minutes);
        println!("That is {} seconds old", birth_info.age_in_seconds);
    }

    if get_banana_env() {
        println!(
            "Your age is equivalent to the counter lifespan of {} bananas",
            birth_info.get_age_according_to_bananas().0
        );
    }
}
