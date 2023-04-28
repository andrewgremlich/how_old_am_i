use chrono::prelude::*;

pub struct Fart {
    pub minimum_in_life: i64,
    pub maximum_in_life: i64,
    pub volume_maximum_in_life: f32,
    pub volume_minimum_in_life: f32,
}

#[derive(Debug)]
pub struct BirthDateInfo {
    pub age_in_days: i64,
    pub age_in_months: f32,
    pub age_in_hours: i64,
    pub age_in_minutes: i64,
    pub age_in_seconds: i64,
}

impl BirthDateInfo {
    pub fn new(naive_birth_date: NaiveDate, naive_now_date: NaiveDate) -> BirthDateInfo {
        let duration_since_birth = naive_now_date.signed_duration_since(naive_birth_date);
        let age_in_days = duration_since_birth.num_days();
        let age_in_months = age_in_days as f32 / 30.4375;
        let age_in_hours = duration_since_birth.num_hours();
        let age_in_minutes = duration_since_birth.num_minutes();
        let age_in_seconds = duration_since_birth.num_seconds();

        BirthDateInfo {
            age_in_days: age_in_days,
            age_in_months: age_in_months,
            age_in_hours: age_in_hours,
            age_in_minutes: age_in_minutes,
            age_in_seconds: age_in_seconds,
        }
    }

    pub fn get_age_according_to_bananas(&self) -> (i64, i64) {
        let banana_counter_life = 7;
        let age_in_bananas = self.age_in_days / banana_counter_life;
        let bananas_left_in_counter_life = self.age_in_days % 7;

        return (age_in_bananas, bananas_left_in_counter_life);
    }

    pub fn find_year_month_day(&self) -> (u8, u8, u8) {
        let year = self.age_in_days as f32 / 365.25;
        let year_floor = year.floor();
        let year_decimal = year - year_floor;
        let month = year_decimal * 12.0;
        let month_floor = month.floor();
        let month_decimal = month - month_floor;
        let day = month_decimal * 30.4375;
        let day_floor = day.floor();

        return (year_floor as u8, month_floor as u8, day_floor as u8);
    }

    // https://www.vox.com/2014/12/4/7332411/fart-flatulence
    pub fn get_farts_in_life(&self) -> Fart {
        let minimum_farts_per_day = 10;
        let maximum_farts_per_day = 20;

        let minimum_volumetric_farts_per_day = 0.5;
        let maximum_volumetric_farts_per_day = 1.5;

        let minimum_in_life = minimum_farts_per_day * self.age_in_days as i64;
        let maximum_in_life = maximum_farts_per_day * self.age_in_days as i64;

        let volume_minimum_in_life = minimum_volumetric_farts_per_day * self.age_in_days as f32;
        let volume_maximum_in_life = maximum_volumetric_farts_per_day * self.age_in_days as f32;

        return Fart {
            minimum_in_life: minimum_in_life,
            maximum_in_life: maximum_in_life,
            volume_maximum_in_life: volume_maximum_in_life,
            volume_minimum_in_life: volume_minimum_in_life,
        };
    }
}
