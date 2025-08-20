use anyhow::{Context, Result};
use ratatui::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: Theme,
    pub layout: Layout,
    pub journal: JournalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: ColorScheme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub text: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub muted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub border_style: BorderStyle,
    pub compact_mode: bool,
    pub show_line_numbers: bool,
    pub tab_width: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalConfig {
    pub week_starts_monday: bool,
    pub show_completed_tasks: bool,
    pub auto_migrate_tasks: bool,
    pub date_format: String,
    pub default_view: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BorderStyle {
    Rounded,
    Plain,
    Thick,
    Double,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            layout: Layout::default(),
            journal: JournalConfig::default(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            colors: ColorScheme::default(),
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            primary: "cyan".to_string(),
            secondary: "blue".to_string(),
            accent: "yellow".to_string(),
            background: "black".to_string(),
            text: "white".to_string(),
            success: "green".to_string(),
            warning: "yellow".to_string(),
            error: "red".to_string(),
            muted: "gray".to_string(),
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            border_style: BorderStyle::Rounded,
            compact_mode: false,
            show_line_numbers: false,
            tab_width: 20,
        }
    }
}

impl Default for JournalConfig {
    fn default() -> Self {
        Self {
            week_starts_monday: true,
            show_completed_tasks: true,
            auto_migrate_tasks: false,
            date_format: "%Y-%m-%d".to_string(),
            default_view: "daily".to_string(),
        }
    }
}

impl ColorScheme {
    pub fn get_color(&self, color_name: &str) -> Color {
        match color_name.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "gray" | "grey" => Color::Gray,
            "darkgray" | "darkgrey" => Color::DarkGray,
            "lightred" => Color::LightRed,
            "lightgreen" => Color::LightGreen,
            "lightyellow" => Color::LightYellow,
            "lightblue" => Color::LightBlue,
            "lightmagenta" => Color::LightMagenta,
            "lightcyan" => Color::LightCyan,
            "white" => Color::White,
            _ => Color::White,
        }
    }

    pub fn primary(&self) -> Color {
        self.get_color(&self.primary)
    }

    pub fn secondary(&self) -> Color {
        self.get_color(&self.secondary)
    }

    pub fn accent(&self) -> Color {
        self.get_color(&self.accent)
    }

    pub fn background(&self) -> Color {
        self.get_color(&self.background)
    }

    pub fn text(&self) -> Color {
        self.get_color(&self.text)
    }

    pub fn success(&self) -> Color {
        self.get_color(&self.success)
    }

    pub fn warning(&self) -> Color {
        self.get_color(&self.warning)
    }

    pub fn error(&self) -> Color {
        self.get_color(&self.error)
    }

    pub fn muted(&self) -> Color {
        self.get_color(&self.muted)
    }
}

impl BorderStyle {
    pub fn to_ratatui_border(&self) -> ratatui::widgets::Borders {
        match self {
            BorderStyle::Rounded => ratatui::widgets::Borders::ALL,
            BorderStyle::Plain => ratatui::widgets::Borders::ALL,
            BorderStyle::Thick => ratatui::widgets::Borders::ALL,
            BorderStyle::Double => ratatui::widgets::Borders::ALL,
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: Config,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("bujo");
        
        fs::create_dir_all(&config_dir)
            .context("Could not create config directory")?;
        
        let config_path = config_dir.join("config.toml");
        
        let config = if config_path.exists() {
            Self::load_config(&config_path)?
        } else {
            let default_config = Config::default();
            Self::save_config(&config_path, &default_config)?;
            default_config
        };
        
        Ok(Self {
            config_path,
            config,
        })
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn update_config<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut Config),
    {
        updater(&mut self.config);
        self.save()?;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        Self::save_config(&self.config_path, &self.config)
    }

    fn load_config(path: &PathBuf) -> Result<Config> {
        let content = fs::read_to_string(path)
            .context("Could not read config file")?;
        
        let config: Config = toml::from_str(&content)
            .context("Could not parse config file")?;
        
        Ok(config)
    }

    fn save_config(path: &PathBuf, config: &Config) -> Result<()> {
        let content = toml::to_string_pretty(config)
            .context("Could not serialize config")?;
        
        fs::write(path, content)
            .context("Could not write config file")?;
        
        Ok(())
    }

    pub fn reset_to_defaults(&mut self) -> Result<()> {
        self.config = Config::default();
        self.save()?;
        Ok(())
    }

    pub fn get_predefined_themes() -> Vec<(&'static str, ColorScheme)> {
        vec![
            ("default", ColorScheme::default()),
            ("dark", ColorScheme {
                primary: "lightcyan".to_string(),
                secondary: "lightblue".to_string(),
                accent: "lightyellow".to_string(),
                background: "black".to_string(),
                text: "white".to_string(),
                success: "lightgreen".to_string(),
                warning: "lightyellow".to_string(),
                error: "lightred".to_string(),
                muted: "darkgray".to_string(),
            }),
            ("light", ColorScheme {
                primary: "blue".to_string(),
                secondary: "darkgray".to_string(),
                accent: "magenta".to_string(),
                background: "white".to_string(),
                text: "black".to_string(),
                success: "green".to_string(),
                warning: "yellow".to_string(),
                error: "red".to_string(),
                muted: "gray".to_string(),
            }),
            ("nord", ColorScheme {
                primary: "lightcyan".to_string(),
                secondary: "lightblue".to_string(),
                accent: "yellow".to_string(),
                background: "black".to_string(),
                text: "white".to_string(),
                success: "green".to_string(),
                warning: "yellow".to_string(),
                error: "red".to_string(),
                muted: "gray".to_string(),
            }),
        ]
    }

    pub fn set_theme(&mut self, theme_name: &str) -> Result<()> {
        let themes = Self::get_predefined_themes();
        if let Some((_, color_scheme)) = themes.iter().find(|(name, _)| *name == theme_name) {
            self.config.theme.name = theme_name.to_string();
            self.config.theme.colors = color_scheme.clone();
            self.save()?;
        }
        Ok(())
    }
}