mod flipper;

use clap::{Parser, Subcommand};
use std::process;

#[derive(Subcommand, Debug)]
enum Commands {
    Stdout {
        // flips in a set
        #[arg(long, default_value_t = 1000)]
        count: u32,
        // number of sets to run
        #[arg(long, default_value_t = 1000)]
        apps: u32,
    },
    Csv {
        // flips in a set
        #[arg(long, default_value_t = 1000)]
        count: u32,
        // number of sets to run
        #[arg(long, default_value_t = 1000)]
        apps: u32,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args: Args = Args::parse();

    match args.command {
        Commands::Csv { count, apps } => {
            if let Err(e) = flipper::multi_csv(count, apps) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Stdout { count, apps } => {
            for _ in 0..apps {
                let (state, runners) = flipper::app(count);
                flipper::io::print(&state, &runners);
            }
        }
    }
}
