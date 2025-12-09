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
        2 => day02::run(args.part),
        3 => day03::run(args.part),
        4 => day04::run(args.part),
        5 => day05::run(args.part),
        6 => day06::run(args.part),
        7 => day07::run(args.part),
        8 => day08::run(args.part),
        9 => day09::run(args.part),
        other => {
            eprintln!("Day {} is not implemented yet.", other);
            std::process::exit(1);
        }
    }
}
