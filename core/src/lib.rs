pub mod config;
pub mod themes;

pub use themes::{Theme, all_theme_names, apply_theme, current_theme_name, load_theme};
