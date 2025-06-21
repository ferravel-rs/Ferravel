use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CLI {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Test,
}

fn main() {
    let cli = CLI::parse();

    let res = cli.commands {
        Commands::Test {
        } => {

        }
    }
}
