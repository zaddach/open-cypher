mod args;

use clap::Parser;
use open_cypher::parse_cypher;


fn main() {
    let args = args::Args::parse();

    match &args.command {
        args::Command::Parse { path, string } => {
            if let Some(string) = string {
                match parse_cypher(string) {
                    Ok(parsed_query) => println!("{:?}", parsed_query),
                    Err(e) => {
                        eprintln!("{}", e);
                        eprintln!("{:?}", e);
                    },
                }
                return;
            }

            if let Some(path) = path {
                let query = std::fs::read_to_string(path).unwrap();
                match parse_cypher(&query) {
                    Ok(parsed_query) => println!("{:?}", parsed_query),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }

    }
}
