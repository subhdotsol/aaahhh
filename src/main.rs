mod audio;
mod cli;
mod core;

use clap::{Parser, Subcommand};
use cli::start::start;
use console::Term;
use core::utils::create_data_directory;
#[derive(Parser)]
#[clap(name = "Aaahhh", version = "0.1.2", author = "Subhajit chaudhury")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(name = "start")]
    Start {
        /// debug flag, to print debug information (-d, --debug)
        #[clap(short, long)]
        debug: bool,
    },
    #[clap(name = "stop")]
    Stop,
    #[clap(name = "daemon", hide = true)]
    Daemon {
        index: usize,
        #[clap(short, long)]
        debug: bool,
    },
}

fn main() {
    let _ = create_data_directory();
    let _ = Term::buffered_stdout();
    let args: CLI = CLI::parse();
    let _ = match args.command {
        Commands::Start { debug } => {
            if debug {
                tracing_subscriber::fmt::init();
            }
            let _ = start(debug);
        },
        Commands::Stop => {
            let _ = cli::stop::stop();
        },
        Commands::Daemon { index, debug } => {
            if debug {
                tracing_subscriber::fmt::init();
            }
            let _ = cli::daemon::daemon(index, debug);
        }
    };
}

// Optimization block for IO

// Params structure mapping
