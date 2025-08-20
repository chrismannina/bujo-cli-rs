use crate::models::Journal;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct Storage {
    file_path: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .context("Could not find data directory")?
            .join("bujo");
        
        fs::create_dir_all(&data_dir)
            .context("Could not create data directory")?;
        
        let file_path = data_dir.join("journal.json");
        
        Ok(Self { file_path })
    }

    pub fn load_journal(&self) -> Result<Journal> {
        if !self.file_path.exists() {
            return Ok(Journal::default());
        }

        let content = fs::read_to_string(&self.file_path)
            .context("Could not read journal file")?;
        
        let journal: Journal = serde_json::from_str(&content)
            .context("Could not parse journal file")?;
        
        Ok(journal)
    }

    pub fn save_journal(&self, journal: &Journal) -> Result<()> {
        let content = serde_json::to_string_pretty(journal)
            .context("Could not serialize journal")?;
        
        fs::write(&self.file_path, content)
            .context("Could not write journal file")?;
        
        Ok(())
    }

    pub fn backup_journal(&self) -> Result<()> {
        if !self.file_path.exists() {
            return Ok(());
        }

        let backup_path = self.file_path.with_extension("json.backup");
        fs::copy(&self.file_path, backup_path)
            .context("Could not create backup")?;
        
        Ok(())
    }
}