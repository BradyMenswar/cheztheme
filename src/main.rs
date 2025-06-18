mod themes;
mod apply;

use clap::{Parser, Subcommand, command};

#[derive(Parser, Debug)]
#[command(version, about, name = "cheztheme")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    List,
    Current,
    Apply { name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => {
            let _ = themes::list();
        }
        Commands::Current => {
            let _ = themes::current();
        }
        Commands::Apply { name } => {
            let _ = apply::run(name);
        }
    }

}
