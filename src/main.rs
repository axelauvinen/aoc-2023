use clap::Parser;

mod day_1;
mod day_2;

#[derive(Parser, Debug)]
#[command(author = "Axel Auvinen", version = "1.0",  about = "Solves Advent of Code puzzles", long_about = None)]
struct Args {
    /// Day in the Advent of Code
    #[arg(short, long)]
    day: String,

    ///
    #[arg(short, long)]
    star: String,
}

fn main() {
    let args = Args::parse();

    let day = args.day.as_str();
    let star = args.star.as_str();

    match (day, star) {
        ("1", "1") => day_1::star_1::run(),
        ("1", "2") => day_1::star_2::run(),
        ("2", "1") => day_2::star_1::run(),
        ("2", "2") => day_2::star_2::run(),
        // Add other days and stars here
        _ => eprintln!("Invalid day or star"),
    }
}
