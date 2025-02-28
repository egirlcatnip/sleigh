mod macros;

use chrono::{Datelike, Utc};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aoc-cli")]
#[command(about = "A CLI for Advent of Code tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the solution for a specific day
    Run {
        /// Year of the challenge (defaults to current year, must be >= 2015)
        #[arg(short, long, default_value_t = Utc::now().year(), value_parser = valid_aoc_year)]
        year: i32,

        /// Day of the challenge (1-24, or "current", or "last_unsolved")
        #[arg(short, long, value_parser = valid_day)]
        day: String,

        /// Part of the challenge (defaults to both parts)
        #[arg(short, long, default_value = "both")]
        part: String,
    },
    /// Solve the challenge for a specific day
    Solve {
        /// Year of the challenge (defaults to current year, must be >= 2015)
        #[arg(short, long, default_value_t = Utc::now().year(), value_parser = valid_aoc_year)]
        year: i32,

        /// Day of the challenge (1-24, or "current", or "last_unsolved")
        #[arg(short, long, value_parser = valid_day)]
        day: String,

        /// Part of the challenge (defaults to both parts)
        #[arg(short, long, default_value = "both")]
        part: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { year, day, part } => {
            println!(
                "Running AoC for year: {}, day: {}, part: {}",
                year, day, part
            );
            // Call your function to run the challenge here
        }
        Commands::Solve { year, day, part } => {
            println!(
                "Solving AoC for year: {}, day: {}, part: {}",
                year, day, part
            );
            // Call your function to solve the challenge here
        }
    }
}

/// Validates that the year is >= 2015 and <= current year.
fn valid_aoc_year(year: &str) -> Result<i32, String> {
    let parsed_year: i32 = year
        .parse()
        .map_err(|_| format!("Year must be a valid integer: '{}'", year))?;
    let current_year = Utc::now().year();

    if parsed_year < 2015 || parsed_year > current_year {
        return Err(format!(
            "Year must be between 2015 and {}: '{}'",
            current_year, year
        ));
    }
    Ok(parsed_year)
}

/// Validates that the day is either "current", "last_unsolved", or an integer between 1 and 24.
fn valid_day(day: &str) -> Result<String, String> {
    if day == "current" || day == "last_unsolved" {
        return Ok(day.to_string());
    }

    let parsed_day: i32 = day.parse().map_err(|_| {
        format!(
            "Day must be a number between 1 and 24 or a valid keyword: '{}'",
            day
        )
    })?;

    if !(1..=24).contains(&parsed_day) {
        return Err(format!("Day must be between 1 and 24: '{}'", day));
    }

    Ok(day.to_string())
}
