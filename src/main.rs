use chrono::prelude::*;
use clap::Parser;
use regex::Regex;

mod dates;

// a clap struct accepting a date cli parameter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, help = "Your birth date in YYYY-MM-DD format")]
    birth_date: String,

    #[clap(
        short,
        long,
        help = "Get a list of presidents' birthdays",
        default_value = "false"
    )]
    get_presidents: bool,
}

fn get_birth_age(args: Cli) {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    if !re.is_match(&args.birth_date) {
        println!("Invalid date format. Please use YYYY-MM-DD");
        std::process::exit(1);
    }

    let dt = NaiveDate::parse_from_str(&args.birth_date, "%Y-%m-%d").unwrap();
    let now = Utc::now().naive_utc().date();

    if dt > now {
        println!("You are not born yet");
        std::process::exit(1);
    }

    dates::age_in(dt, now);
}

fn main() {
    let args = Cli::parse();

    println!("{:?}", args);

    if args.get_presidents {
        println!("Presidents' birthdays:");
    }

    if args.birth_date != "" {
        get_birth_age(args);
    } else {
        println!("Please provide a birth date");
    }
}
