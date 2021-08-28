use clap::Clap;
use lurien::cli::*;

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
