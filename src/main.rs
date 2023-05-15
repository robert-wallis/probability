mod flipper;

use clap::{Parser, Subcommand};
use flipper::{runner::RunnerLoop, stats::FinalStats};
use std::io;
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

    let total_count;
    let total_apps;

    let mut stdout = io::stdout();
    let mut command: Box<dyn RunnerLoop> = match args.command {
        Commands::Csv { count, apps } => {
            total_count = count;
            total_apps = apps;
            let csv = flipper::csv::Csv::new(&mut stdout);
            Box::new(csv)
        }
        Commands::Stdout { count, apps } => {
            total_count = count;
            total_apps = apps;
            let io = flipper::io::IO;
            Box::new(io)
        }
    };

    for _ in 0..total_apps {
        let (state, runners) = flipper::app(total_count);

        command.each_app(&state);
        for runner in runners {
            let final_stats = FinalStats::new(&runner.stats, &runner.account, state.total_count);

            if let Err(e) = command.each_run(&state, &runner, &final_stats) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}
