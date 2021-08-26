use lurien::{app::App, populate::populate, save::save, watch::watch};

use clap::Clap;

fn main() {
    let input = App::parse();

    let result = match input {
        App::Populate(args) => populate(&args),
        App::Save(args) => save(&args),
        App::Watch(args) => watch(&args),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    } else {
        println!("exiting lurien")
    }
}
