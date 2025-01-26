use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    #[clap(about = "Parse a Cypher query")]
    Parse {
        #[clap(short, long, help = "Path to the file containing the Cypher query")]
        path: Option<PathBuf>,

        #[clap(short, long, help = "Parse a query string")]
        string: Option<String>,
    },
}

