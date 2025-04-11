use clap::Subcommand;
use clap::{Parser, command};

#[derive(Parser)]
#[command(name = "RustyPages")]
#[command(about = "A lightweight static site generator")]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a new RustyPages site
    Init,
    /// Build the site into static files
    Build,
    /// Clean the generated output
    Clean,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Init) => rustypages::commands::init(),
        Some(Commands::Build) => rustypages::commands::build(),
        Some(Commands::Clean) => rustypages::commands::clean(),
        None => println!("Unknown command. Please run 'rustypages help' to see available commands"),
    };
}
