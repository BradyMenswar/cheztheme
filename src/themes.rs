use anyhow::{Context, Result};
use colored::Colorize;
use hex_color::HexColor;
use std::{cmp::Ordering, env, fs, path::PathBuf};
use toml_edit::DocumentMut;

use crate::apply::Theme;

pub static PRESETS: include_dir::Dir = include_dir::include_dir!("presets");

#[derive(Eq, PartialEq)]
enum ThemeType {
    Preset(String),
    Custom(String),
}

impl Ord for ThemeType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_name().cmp(other.get_name())
    }
}

impl PartialOrd for ThemeType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ThemeType {
    fn get_name(&self) -> &str {
        match self {
            ThemeType::Preset(s) => s,
            ThemeType::Custom(s) => s,
        }
    }
}

pub fn list() -> Result<()> {
    let mut file_stems = Vec::new();

    for file in PRESETS.files() {
        let file_stem = file.path().file_stem().and_then(|e| e.to_str());
        if let Some(name) = file_stem {
            file_stems.push(ThemeType::Preset(name.to_string()));
        }
    }

    if let Ok(src) = env::var("HOME") {
        let themes_dir = PathBuf::from(src).join(".local/share/chezmoi/themes");
        if themes_dir.is_dir() {
            for entry in fs::read_dir(themes_dir).context("Unable to read themes directory")? {
                let path = entry?.path();
                let file_stem = path.file_stem().and_then(|e| e.to_str());
                let extension = path.extension().and_then(|e| e.to_str());
                if let Some(name) = file_stem {
                    if extension == Some("yaml") || extension == Some("yml") {
                        file_stems.push(ThemeType::Custom(name.to_string()));
                    }
                }
            }
        }
    }

    file_stems.sort();
    for stem in file_stems.iter() {
        match stem {
            ThemeType::Preset(name) => {
                println!("{} ", name.purple());
                print_palette(name)?;
            }
            ThemeType::Custom(name) => {
                println!("{} ", name);
                print_palette(name)?;
            }
        }
    }
    Ok(())
}

pub fn load(name: &str) -> Result<String> {
    if let Some(file) = PRESETS.get_file(format!("{}.yaml", name)) {
        return Ok(std::str::from_utf8(file.contents())?.to_string());
    }
    if let Ok(src) = env::var("HOME") {
        let path = PathBuf::from(src)
            .join(".local/share/chezmoi/themes")
            .join(format!("{}.yaml", name));
        return Ok(fs::read_to_string(path)?);
    }
    Ok("notfound".to_string())
}

pub fn current() -> Result<()> {
    let color_block = "   ";
    if let Ok(src) = env::var("HOME") {
        let config_location = PathBuf::from(src).join(".config/chezmoi/chezmoi.toml");
        let toml_str =
            fs::read_to_string(config_location.clone()).context("Chezmoi config file not found")?;
        let toml_config = toml_str
            .parse::<DocumentMut>()
            .context("Failed to parse TOML")?;
        let current_theme = &toml_config["data"]["cheztheme"]["themeName"];
        println!("Current theme:{}", current_theme);
        println!(
            "{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}\n",
            color_block.on_black(),
            color_block.on_red(),
            color_block.on_green(),
            color_block.on_yellow(),
            color_block.on_blue(),
            color_block.on_magenta(),
            color_block.on_cyan(),
            color_block.on_white(),
            color_block.on_bright_black(),
            color_block.on_bright_red(),
            color_block.on_bright_green(),
            color_block.on_bright_yellow(),
            color_block.on_bright_blue(),
            color_block.on_bright_magenta(),
            color_block.on_bright_cyan(),
            color_block.on_bright_white(),
        );
    }
    Ok(())
}

#[allow(non_snake_case)]
pub fn print_palette(theme_name: &str) -> Result<()> {
    let color_block = "   ";
    if let Ok(theme_contents) = load(theme_name) {
        let theme: Theme = serde_yaml::from_str(theme_contents.as_str())?;

        let base00_hex = HexColor::parse(theme.palette.base00.as_str())?;
        let base00 = color_block.on_truecolor(base00_hex.r, base00_hex.g, base00_hex.b);

        let base01_hex = HexColor::parse(theme.palette.base01.as_str())?;
        let base01 = color_block.on_truecolor(base01_hex.r, base01_hex.g, base01_hex.b);

        let base02_hex = HexColor::parse(theme.palette.base02.as_str())?;
        let base02 = color_block.on_truecolor(base02_hex.r, base02_hex.g, base02_hex.b);

        let base03_hex = HexColor::parse(theme.palette.base03.as_str())?;
        let base03 = color_block.on_truecolor(base03_hex.r, base03_hex.g, base03_hex.b);

        let base04_hex = HexColor::parse(theme.palette.base04.as_str())?;
        let base04 = color_block.on_truecolor(base04_hex.r, base04_hex.g, base04_hex.b);

        let base05_hex = HexColor::parse(theme.palette.base05.as_str())?;
        let base05 = color_block.on_truecolor(base05_hex.r, base05_hex.g, base05_hex.b);

        let base06_hex = HexColor::parse(theme.palette.base06.as_str())?;
        let base06 = color_block.on_truecolor(base06_hex.r, base06_hex.g, base06_hex.b);

        let base07_hex = HexColor::parse(theme.palette.base07.as_str())?;
        let base07 = color_block.on_truecolor(base07_hex.r, base07_hex.g, base07_hex.b);

        let base08_hex = HexColor::parse(theme.palette.base08.as_str())?;
        let base08 = color_block.on_truecolor(base08_hex.r, base08_hex.g, base08_hex.b);

        let base09_hex = HexColor::parse(theme.palette.base09.as_str())?;
        let base09 = color_block.on_truecolor(base09_hex.r, base09_hex.g, base09_hex.b);

        let base0A_hex = HexColor::parse(theme.palette.base0A.as_str())?;
        let base0A = color_block.on_truecolor(base0A_hex.r, base0A_hex.g, base0A_hex.b);

        let base0B_hex = HexColor::parse(theme.palette.base0B.as_str())?;
        let base0B = color_block.on_truecolor(base0B_hex.r, base0B_hex.g, base0B_hex.b);

        let base0C_hex = HexColor::parse(theme.palette.base0C.as_str())?;
        let base0C = color_block.on_truecolor(base0C_hex.r, base0C_hex.g, base0C_hex.b);

        let base0D_hex = HexColor::parse(theme.palette.base0D.as_str())?;
        let base0D = color_block.on_truecolor(base0D_hex.r, base0D_hex.g, base0D_hex.b);

        let base0E_hex = HexColor::parse(theme.palette.base0E.as_str())?;
        let base0E = color_block.on_truecolor(base0E_hex.r, base0E_hex.g, base0E_hex.b);

        let base0F_hex = HexColor::parse(theme.palette.base0F.as_str())?;
        let base0F = color_block.on_truecolor(base0F_hex.r, base0F_hex.g, base0F_hex.b);

        println!(
            "{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}\n",
            base00,
            base01,
            base02,
            base03,
            base04,
            base05,
            base06,
            base07,
            base08,
            base09,
            base0A,
            base0B,
            base0C,
            base0D,
            base0E,
            base0F
        );
    }
    Ok(())
}
