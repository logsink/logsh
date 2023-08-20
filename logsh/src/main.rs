use clap::{Parser, Subcommand};
use logsh::connect;

#[derive(Parser, Debug)]
#[clap(name = "logsh", version = "0.1.0", author = "logship.llc")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short = 'v', action = clap::ArgAction::Count, global = true, help = "Set command verbosity. The more 'v's, the more verbose. -vvvv is the most verbose.")]
    verbose: u8,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(subcommand)]
    Connection(logsh::connect::ConnectCommand),
    Query(logsh::query::QueryCommand),
}

fn main() {
    let cli = Args::parse();

    let log_level = match cli.verbose {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    logsh::logger::install(log_level.to_level().unwrap());
    match cli.command {
        Some(Commands::Connection(command)) => {
            connect::execute_connect(command).unwrap();
        }
        Some(Commands::Query(command)) => {
            logsh::query::execute_query(command).unwrap();
        }
        None => println!("No subcommand was used. Try --help."),
    }
}
