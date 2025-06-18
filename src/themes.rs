use colored::Colorize;
use std::{cmp::Ordering, env, fs, path::PathBuf};

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

pub fn list() -> Result<(), Box<dyn std::error::Error>> {
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
            for entry in fs::read_dir(themes_dir)? {
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
            ThemeType::Preset(name) => println!("{}", name.purple()),
            ThemeType::Custom(name) => println!("{}", name),
        }
    }
    Ok(())
}

pub fn load(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(file) = PRESETS.get_file(format!("{}.yaml", name)) {
        return Ok(std::str::from_utf8(file.contents())?.to_string());
    }
    if let Ok(src) = env::var("HOME") {
        let path = PathBuf::from(src).join(".local/share/chezmoi/themes")
            .join(format!("{}.yaml", name));
        return Ok(fs::read_to_string(path)?);
    }
    Ok("notfound".to_string())
}

pub fn current() {
    let current_theme = "unimplemented";
    println!("Current theme: {}", current_theme)
}
