use clap::Parser;
use how_old_am_i_lib::cli::{set_env, Cli};

mod dates;

fn main() {
    let args = Cli::parse();

    set_env(&args);

    if args.get_presidents {
        println!("Presidents' birthdays:");
    }

    if args.birth_date != "" {
        dates::get_birth_age(args.birth_date);
    } else {
        println!("Please provide a birth date");
    }
}
