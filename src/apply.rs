use anyhow::{Context, Result};
use std::{env, fs, path::PathBuf, process::Command};

use serde::Deserialize;
use toml_edit::{DocumentMut, value};

use crate::themes::load;

#[derive(Deserialize, Debug)]
pub struct Theme {
    pub palette: Palette,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Palette {
    pub base00: String,
    pub base01: String,
    pub base02: String,
    pub base03: String,
    pub base04: String,
    pub base05: String,
    pub base06: String,
    pub base07: String,
    pub base08: String,
    pub base09: String,
    pub base0A: String,
    pub base0B: String,
    pub base0C: String,
    pub base0D: String,
    pub base0E: String,
    pub base0F: String,
}

pub fn run(name: &str) -> Result<()> {
    if let Ok(theme_contents) = load(name) {
        let theme: Theme = serde_yaml::from_str(theme_contents.as_str())?;

        if let Ok(src) = env::var("HOME") {
            let config_location = PathBuf::from(src).join(".config/chezmoi/chezmoi.toml");
            let toml_str = fs::read_to_string(config_location.clone()).context("Chezmoi config file not found")?;
            let mut toml_config = toml_str
                .parse::<DocumentMut>()
                .context("Failed to parse TOML")?;

            toml_config["data"]["cheztheme"]["themeName"] = value(name);
            toml_config["data"]["cheztheme"]["base00"] = value(theme.palette.base00);
            toml_config["data"]["cheztheme"]["base01"] = value(theme.palette.base01);
            toml_config["data"]["cheztheme"]["base02"] = value(theme.palette.base02);
            toml_config["data"]["cheztheme"]["base03"] = value(theme.palette.base03);
            toml_config["data"]["cheztheme"]["base04"] = value(theme.palette.base04);
            toml_config["data"]["cheztheme"]["base05"] = value(theme.palette.base05);
            toml_config["data"]["cheztheme"]["base06"] = value(theme.palette.base06);
            toml_config["data"]["cheztheme"]["base07"] = value(theme.palette.base07);
            toml_config["data"]["cheztheme"]["base08"] = value(theme.palette.base08);
            toml_config["data"]["cheztheme"]["base09"] = value(theme.palette.base09);
            toml_config["data"]["cheztheme"]["base0A"] = value(theme.palette.base0A);
            toml_config["data"]["cheztheme"]["base0B"] = value(theme.palette.base0B);
            toml_config["data"]["cheztheme"]["base0C"] = value(theme.palette.base0C);
            toml_config["data"]["cheztheme"]["base0D"] = value(theme.palette.base0D);
            toml_config["data"]["cheztheme"]["base0E"] = value(theme.palette.base0E);
            toml_config["data"]["cheztheme"]["base0F"] = value(theme.palette.base0F);

            fs::write(config_location, toml_config.to_string())
                .context("Failed to write toml config")?
        }

        Command::new("chezmoi")
            .arg("apply")
            .output()
            .context("Chezmoi failed to apply.")?;

        // --- Kitty Reload ---- //
        let kitty_pid_output = Command::new("pgrep")
            .arg("-a")
            .arg("kitty")
            .output()
            .context("Pid of kitty not found")?;
        let kitty_stdout = String::from_utf8(kitty_pid_output.stdout)?;
        for line in kitty_stdout.lines() {
            let kitty_pid = line.trim();
            if kitty_pid.is_empty() {
                continue;
            }

            Command::new("kill")
                .arg("-SIGUSR1")
                .arg(kitty_pid.trim())
                .output()
                .context("Kitty failed to reload")?;
        }
        // ------------------- //

        println!("Theme applied.")
    } else {
        println!("Theme {} not found.", name);
    }
    Ok(())
}
