use chrono::prelude::*;
use how_old_am_i_lib::cli::get_banana_env;
use how_old_am_i_lib::read_json;

fn get_age_according_to_bananas(age_in_days: i64) -> (i64, i64) {
    let banana_counter_life = 7;
    let age_in_bananas = age_in_days / banana_counter_life;
    let bananas_left_in_counter_life = age_in_days % 7;

    return (age_in_bananas, bananas_left_in_counter_life);
}

fn figure_out_months_and_days_left(days_left_in_year: f32) -> (u8, f32) {
    let json_data = read_json::days_to_months().expect("could not read days-to-months file");
    let mut days_left = days_left_in_year;
    let mut months_left_in_year: u8 = 0;
    let mut days_left_in_month: f32 = 0.0;

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

fn find_year_month_day(age_in_days: f32) -> (f32, u8, f32) {
    let days_in_a_year: f32 = 365.25;

    let age_in_years = age_in_days / days_in_a_year;
    let days_left_in_year = age_in_days % days_in_a_year;
    let months_and_days_left = figure_out_months_and_days_left(days_left_in_year);

    return (age_in_years, months_and_days_left.0, months_and_days_left.1);
}

pub fn process_dates(naive_birth_date: NaiveDate, naive_now_date: NaiveDate) {
    let duration_since_birth = naive_now_date.signed_duration_since(naive_birth_date);
    let age_in_days = duration_since_birth.num_days();
    let age_in_months = age_in_days as f32 / 30.4375;
    let age_in_hours = duration_since_birth.num_hours();
    let age_in_minutes = duration_since_birth.num_minutes();
    let age_in_seconds = duration_since_birth.num_seconds();

    if get_banana_env() {
        let age_according_to_banana_lifespan = get_age_according_to_bananas(age_in_days);

        println!(
            "{:?} Banana lifespans with {:?} days sitting on the counter.",
            age_according_to_banana_lifespan.0, age_according_to_banana_lifespan.1
        );
    }

    // println!(
    //     "year month day {:?}",
    //     find_year_month_day(age_in_days as f32)
    // );

    println!("You are {} months old", age_in_months);
    println!("You are {} days old", age_in_days);
    println!("You are {} hours old", age_in_hours);
    println!("You are {} minutes old", age_in_minutes);
    println!("You are {} seconds old", age_in_seconds);
}
