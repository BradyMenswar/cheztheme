mod apply;
mod themes;

use clap::{Parser, Subcommand, command};

#[derive(Parser, Debug)]
#[command(version, about, name = "cheztheme")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    List {
        #[clap(short, long)]
        color: bool,
    },
    Current,
    Apply {
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List { color } => {
            let _ = themes::list(*color);
        }
        Commands::Current => {
            let _ = themes::current();
        }
        Commands::Apply { name } => {
            let _ = apply::run(name);
        }
    }
}
