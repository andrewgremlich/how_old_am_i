use chrono::prelude::*;
use read_json;

fn get_age_according_to_bananas(age_in_days: i64) -> (i64, i64) {
    let banana_counter_life = age_in_days / 7;
    let bananas_left_in_counter_life = age_in_days % 7;

    return (banana_counter_life, bananas_left_in_counter_life);
}

fn figure_out_months_and_days_left(
    days_left_in_year: i64,
    json_data: Vec<read_json::DaysToMonth>,
) -> (i64, i64) {
    let mut days_left = days_left_in_year;
    let mut months_left_in_year = 0;
    let mut days_left_in_month = 0;

    for month in json_data.iter() {
        if days_left >= month.days {
            days_left -= month.days;
            months_left_in_year += 1;
        } else {
            days_left_in_month = days_left;
            break;
        }
    }

    return (months_left_in_year, days_left_in_month);
}

fn find_year_month_day(age_in_days: i64) -> (i64, i64, i64) {
    let json_data = read_json::days_to_months().expect("could not read file");
    println!("{:?}", json_data);

    let age_in_years = age_in_days / 365;
    let days_left_in_year = age_in_days % 365;

    println!(
        "{:?}",
        figure_out_months_and_days_left(days_left_in_year, json_data)
    );

    let months_left_in_year = days_left_in_year / 30;
    let days_left_in_month = months_left_in_year % 30;

    return (age_in_years, months_left_in_year, days_left_in_month);
}

pub fn age_in(dt: NaiveDate, now: NaiveDate) {
    let age_in_months = now.signed_duration_since(dt).num_days() / 30;
    let age_in_days = now.signed_duration_since(dt).num_days();
    let age_in_hours = now.signed_duration_since(dt).num_hours();
    let age_in_minutes = now.signed_duration_since(dt).num_minutes();
    let age_in_seconds = now.signed_duration_since(dt).num_seconds();

    println!("year month day {:?}", find_year_month_day(age_in_days));
    println!("bananas {:?}", get_age_according_to_bananas(age_in_days));

    println!("You are {} months old", age_in_months);
    println!("You are {} days old", age_in_days);
    println!("You are {} hours old", age_in_hours);
    println!("You are {} minutes old", age_in_minutes);
    println!("You are {} seconds old", age_in_seconds);
}
