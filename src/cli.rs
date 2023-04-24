use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, help = "Your birth date in YYYY-MM-DD format")]
    pub birth_date: String,

    #[clap(
        short = 'n',
        long,
        help = "Get your age in banana counter-life metrics.",
        default_value = "false"
    )]
    pub banana_life_span: bool,

    #[clap(
        short,
        long,
        help = "Get your age detailed in months, days, hours, minutes, and seconds.",
        default_value = "false"
    )]
    pub detailed: bool,

    #[clap(
        short,
        long,
        help = "Get a list of presidents' birthdays",
        default_value = "false"
    )]
    pub get_presidents: bool,
}

pub fn set_env(args: &Cli) {
    env::set_var("BANANA_LIFE_SPAN", &args.banana_life_span.to_string());
    env::set_var("DETAILED", &args.detailed.to_string());
}

pub fn get_detailed_env() -> bool {
    let detailed = env::var("DETAILED").unwrap();

    if detailed == "true" {
        return true;
    } else {
        return false;
    }
}

pub fn get_banana_env() -> bool {
    let banana_life_span = env::var("BANANA_LIFE_SPAN").unwrap();

    if banana_life_span == "true" {
        return true;
    } else {
        return false;
    }
}
