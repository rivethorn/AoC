pub mod days;
use clap::Parser;
use days::*;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
    #[arg(short, long)]
    part: Option<u8>,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day01::run(args.part),
        other => {
            eprintln!("Day {} is not implemented yet.", other);
            std::process::exit(1);
        }
    }
}
