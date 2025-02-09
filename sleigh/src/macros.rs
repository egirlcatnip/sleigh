#[macro_export]
macro_rules! main {
    () => {
        use clap::Parser;
        use std::fs;
        use std::path::Path;

        use anyhow::Result;
        use anyhow::anyhow;

        #[derive(Parser)]
        #[command(
            name = "AOC Client",
            version = "1.0",
            author = "@egirlcatnip",
            about = "Advent of Code Client"
        )]
        struct Cli {
            #[arg(
                                                short = 'p',
                                                long = "part",
                                                aliases = ["p", "part"],
                                                default_value = "both",
                                                value_parser = ["1", "2", "both", ""],
                                                help = "Sets the part to run: '1', '2', or 'both'"
                                            )]
            part: String,
        }

        fn main() ->  Result<()> {
            // Parse command-line arguments
            let cli = Cli::parse();

            // Extract year and day from the current file path using `file!()`
            let file_path = file!();
            println!("file_path: {}", file_path);
            let parts: Vec<&str> = file_path.split('/').collect();

            // Assume the directory structure is `/year/{year}/src/{day}.rs`
            let year = parts[1];
            let day = parts[4].strip_suffix(".rs").unwrap();

            // Construct input file path
            let input_path = format!("./year/{}/data/inputs/{:02}.txt", year, day);
            let input = fs::read_to_string(&input_path)
                .map_err(|e| anyhow::anyhow!("Failed to read input file {}: {}", input_path, e))?;

            // Call the `part_one` and `part_two` functions based on the `-p` argument
            match cli.part.as_str() {
                "1" => match part_one(&input) {
                    Ok(result) => println!("{:#?}", result),
                    Err(err) => eprintln!("Part 1 failed: {}", err),
                },
                "2" => match part_two(&input) {
                    Ok(result) => println!("{:#?}", result),
                    Err(err) => eprintln!("Part 2 failed: {}", err),
                },
                "both" | "" => {
                    match part_one(&input) {
                        Ok(result) => println!("Part 1: {:#?}", result),
                        Err(err) => eprintln!("Part 1 failed: {}", err),
                    }

                    match part_two(&input) {
                        Ok(result) => println!("Part 2: {:#?}", result),
                        Err(err) => eprintln!("Part 2 failed: {}", err),
                    }
                }
                _ => {
                    eprintln!("Invalid part: {}", cli.part);
                    std::process::exit(1);
                }
            }

            Ok(())
        }
    };
}
