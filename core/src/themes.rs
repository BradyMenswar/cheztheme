use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, env, fs, path::PathBuf, process::Command};

use crate::config;

pub static PRESETS: include_dir::Dir = include_dir::include_dir!("presets");

pub fn load_theme(theme_name: &str, themes_dir: &str) -> Result<Theme> {
    if let Some(file) = PRESETS.get_file(format!("{}.yaml", theme_name)) {
        let yaml = std::str::from_utf8(file.contents())?.to_string();
        let theme: Theme = serde_yaml::from_str(&yaml)?;
        return Ok(theme);
    }
    if let Ok(src) = env::var("HOME") {
        let path = PathBuf::from(src)
            .join(themes_dir)
            .join(format!("{}.yaml", theme_name));
        let yaml = fs::read_to_string(path).context("File not found")?;
        let theme: Theme = serde_yaml::from_str(&yaml).context("Unable to parse yaml")?;
        return Ok(theme);
    }

    bail!("Theme not found.")
}

pub fn all_theme_names(themes_dir: &str) -> Result<Vec<ThemeType>> {
    let mut theme_names = Vec::new();

    for file in PRESETS.files() {
        let file_stem = file.path().file_stem().and_then(|e| e.to_str());
        if let Some(name) = file_stem {
            theme_names.push(ThemeType::Preset(name.to_string()));
        }
    }

    if let Ok(src) = env::var("HOME") {
        let themes_dir = PathBuf::from(src).join(themes_dir);
        if themes_dir.is_dir() {
            for entry in fs::read_dir(themes_dir).context("Unable to read themes directory")? {
                let path = entry?.path();
                let file_stem = path.file_stem().and_then(|e| e.to_str());
                let extension = path.extension().and_then(|e| e.to_str());
                if let Some(name) = file_stem {
                    if extension == Some("yaml") || extension == Some("yml") {
                        theme_names.push(ThemeType::Custom(name.to_string()));
                    }
                }
            }
        }
    }
    theme_names.sort();
    Ok(theme_names)
}

pub fn current_theme_name() -> Result<String> {
    let (mut doc, _path) = config::load_config()?;
    let data = &mut doc["data"]["cheztheme"];
    let current_theme = data["themeName"].as_str().unwrap();
    Ok(current_theme.to_string())
}

pub fn apply_theme(theme_name: &str, theme: Theme) -> Result<()> {
    let (mut doc, path) = config::load_config()?;

    let data = &mut doc["data"]["cheztheme"];
    data["themeName"] = toml_edit::value(theme_name);
    data["base00"] = toml_edit::value(theme.palette.base00);
    data["base01"] = toml_edit::value(theme.palette.base01);
    data["base02"] = toml_edit::value(theme.palette.base02);
    data["base03"] = toml_edit::value(theme.palette.base03);
    data["base04"] = toml_edit::value(theme.palette.base04);
    data["base05"] = toml_edit::value(theme.palette.base05);
    data["base06"] = toml_edit::value(theme.palette.base06);
    data["base07"] = toml_edit::value(theme.palette.base07);
    data["base08"] = toml_edit::value(theme.palette.base08);
    data["base09"] = toml_edit::value(theme.palette.base09);
    data["base0A"] = toml_edit::value(theme.palette.base0A);
    data["base0B"] = toml_edit::value(theme.palette.base0B);
    data["base0C"] = toml_edit::value(theme.palette.base0C);
    data["base0D"] = toml_edit::value(theme.palette.base0D);
    data["base0E"] = toml_edit::value(theme.palette.base0E);
    data["base0F"] = toml_edit::value(theme.palette.base0F);

    config::save_config(&doc, &path)?;

    Command::new("chezmoi").arg("apply").output()?;

    // To extract to bash file loaded in config
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

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    pub palette: Palette,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Eq, PartialEq)]
pub enum ThemeType {
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
