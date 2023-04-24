use chrono::prelude::*;
use how_old_am_i_lib::cli::{get_banana_env, get_detailed_env};

fn get_age_according_to_bananas(age_in_days: i64) -> (i64, i64) {
    let banana_counter_life = 7;
    let age_in_bananas = age_in_days / banana_counter_life;
    let bananas_left_in_counter_life = age_in_days % 7;

    return (age_in_bananas, bananas_left_in_counter_life);
}

fn find_year_month_day(age_in_days: f32) -> (u8, u8, u8) {
    let year = age_in_days / 365.25;
    let year_floor = year.floor();
    let year_decimal = year - year_floor;
    let month = year_decimal * 12.0;
    let month_floor = month.floor();
    let month_decimal = month - month_floor;
    let day = month_decimal * 30.4375;
    let day_floor = day.floor();

    return (year_floor as u8, month_floor as u8, day_floor as u8);
}

pub fn process_dates(naive_birth_date: NaiveDate, naive_now_date: NaiveDate) {
    let duration_since_birth = naive_now_date.signed_duration_since(naive_birth_date);
    let age_in_days = duration_since_birth.num_days() as f32;
    let age_in_months = age_in_days / 30.4375;
    let age_in_hours = duration_since_birth.num_hours();
    let age_in_minutes = duration_since_birth.num_minutes();
    let age_in_seconds = duration_since_birth.num_seconds();
    let year_month_day = find_year_month_day(age_in_days);

    println!(
        "You are {:?} years, {:?} months, and {:?} days old.",
        year_month_day.0, year_month_day.1, year_month_day.2
    );

    if get_banana_env() {
        let age_according_to_banana_lifespan = get_age_according_to_bananas(age_in_days as i64);

        println!(
            "{:?} Banana lifespans with {:?} days sitting on the counter.",
            age_according_to_banana_lifespan.0, age_according_to_banana_lifespan.1
        );
    }

    if get_detailed_env() {
        println!("You are {} months old", age_in_months.floor());
        println!("You are {} days old", age_in_days);
        println!("You are {} hours old", age_in_hours);
        println!("You are {} minutes old", age_in_minutes);
        println!("You are {} seconds old", age_in_seconds);
    }
}
