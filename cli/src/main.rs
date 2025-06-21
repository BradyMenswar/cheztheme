use anyhow::Result;
use cheztheme_core::{all_theme_names, apply_theme, current_theme_name, load_theme, themes::ThemeType, Theme};
use clap::{Parser, Subcommand, command};
use colored::Colorize;
use hex_color::HexColor;

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
        theme_name: String,
    },
}

fn main() -> Result<()>{
    let cli = Cli::parse();

    match &cli.command {
        Commands::List { color } => {
            list(*color)?;
        }
        Commands::Current => {
            current()?;
        }
        Commands::Apply { theme_name } => {
            apply(theme_name)?;
        }
    }
    Ok(())
}

fn list(color: bool) -> Result<()> {
    let theme_names = all_theme_names(".local/share/chezmoi/themes")?;
    let current_theme_name = current_theme_name()?;

    for theme_name in theme_names {
        match theme_name {
            ThemeType::Preset(name) => {
                if name == current_theme_name {
                    println!("{}", ("** ".to_string() + &name).yellow())
                } else {
                    println!("{}", name.purple())
                }
                if color {
                    let theme = load_theme(&name, ".local/share/chezmoi/themes")?;
                    print_palette(&theme)?;
                }
            }
            ThemeType::Custom(name) => {
                if name == current_theme_name {
                    println!("{}", ("** ".to_string() + &name).yellow())
                } else {
                    println!("{}", name)
                }
                if color {
                    let theme = load_theme(&name, ".local/share/chezmoi/themes")?;
                    print_palette(&theme)?;
                }
            }
        }
    }
    Ok(())
}

fn current() -> Result<()> {
    let current_theme_name = current_theme_name()?;
    let current_theme = load_theme(current_theme_name.as_str(), ".local/share/chezmoi/themes")?;
    println!("{}", current_theme_name);
    print_palette(&current_theme)?;
    Ok(())
}

fn apply(theme_name: &str) -> Result<()> {
    let new_theme = load_theme(theme_name, ".local/share/chezmoi/themes")?;
    apply_theme(theme_name, new_theme)?;
    println!("Theme applied.");
    Ok(())
}

fn print_palette(theme: &Theme) -> Result<()> {
    let block = "   ";
    for hex in [
        &theme.palette.base00,
        &theme.palette.base01,
        &theme.palette.base02,
        &theme.palette.base03,
        &theme.palette.base04,
        &theme.palette.base05,
        &theme.palette.base06,
        &theme.palette.base07,
        &theme.palette.base08,
        &theme.palette.base09,
        &theme.palette.base0A,
        &theme.palette.base0B,
        &theme.palette.base0C,
        &theme.palette.base0D,
        &theme.palette.base0E,
        &theme.palette.base0F,
    ] {
        let c = HexColor::parse(hex)?;
        print!("{}", block.on_truecolor(c.r, c.g, c.b));
    }
    println!();
    println!();
    Ok(())
}
