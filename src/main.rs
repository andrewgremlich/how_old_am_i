use clap::Parser;
use how_old_am_i_lib::cli::Cli;
use how_old_am_i_lib::read_json::read_presidents;

mod dates;

fn main() {
    let args = Cli::parse();

    Cli::set_env(&args);

    if args.birth_date != "" {
        dates::get_birth_data(args.birth_date);
    } else {
        println!("Please provide a birth date");
    }

    if args.presidents {
        for president in read_presidents().unwrap() {
            println!("-------------------------");
            println!("{}", president.name,);
            dates::get_birth_data(president.birth);
        }
    }
}

// TODO: now something with Deno? create a netlify function and an API?
// Or just connect this with a netlify function?
