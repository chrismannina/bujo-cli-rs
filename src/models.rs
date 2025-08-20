use chrono::{DateTime, Local, NaiveDate, Datelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BulletType {
    Task,
    Event,
    Note,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Incomplete,
    Complete,
    Migrated,
    Scheduled,
    Irrelevant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: Uuid,
    pub content: String,
    pub bullet_type: BulletType,
    pub status: Option<TaskStatus>,
    pub created_at: DateTime<Local>,
    pub date: NaiveDate,
    pub tags: Vec<String>,
    pub priority: Option<u8>,
}

impl Entry {
    pub fn new(content: String, bullet_type: BulletType, date: NaiveDate) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            bullet_type,
            status: match bullet_type {
                BulletType::Task => Some(TaskStatus::Incomplete),
                _ => None,
            },
            created_at: Local::now(),
            date,
            tags: Vec::new(),
            priority: None,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self.bullet_type {
            BulletType::Task => match self.status {
                Some(TaskStatus::Incomplete) => "•",
                Some(TaskStatus::Complete) => "✓",
                Some(TaskStatus::Migrated) => ">",
                Some(TaskStatus::Scheduled) => "<",
                Some(TaskStatus::Irrelevant) => "✗",
                None => "•",
            },
            BulletType::Event => "○",
            BulletType::Note => "-",
        }
    }

    pub fn toggle_complete(&mut self) {
        if self.bullet_type == BulletType::Task {
            self.status = match self.status {
                Some(TaskStatus::Incomplete) => Some(TaskStatus::Complete),
                Some(TaskStatus::Complete) => Some(TaskStatus::Incomplete),
                other => other,
            };
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub entries: Vec<Entry>,
    pub created_at: DateTime<Local>,
}

impl Collection {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            entries: Vec::new(),
            created_at: Local::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Journal {
    pub entries: Vec<Entry>,
    pub collections: HashMap<Uuid, Collection>,
    pub settings: JournalSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalSettings {
    pub week_starts_monday: bool,
    pub show_completed_tasks: bool,
    pub auto_migrate_tasks: bool,
}

impl Default for JournalSettings {
    fn default() -> Self {
        Self {
            week_starts_monday: true,
            show_completed_tasks: true,
            auto_migrate_tasks: false,
        }
    }
}

impl Default for Journal {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            collections: HashMap::new(),
            settings: JournalSettings::default(),
        }
    }
}

impl Journal {
    pub fn entries_for_date(&self, date: NaiveDate) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|entry| entry.date == date)
            .collect()
    }

    pub fn entries_for_month(&self, year: i32, month: u32) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|entry| entry.date.year() == year && entry.date.month() == month)
            .collect()
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
        self.entries.sort_by(|a, b| a.date.cmp(&b.date));
    }

    pub fn remove_entry(&mut self, id: Uuid) {
        self.entries.retain(|entry| entry.id != id);
    }

    pub fn get_entry_mut(&mut self, id: Uuid) -> Option<&mut Entry> {
        self.entries.iter_mut().find(|entry| entry.id == id)
    }

    pub fn search_entries(&self, query: &str) -> Vec<&Entry> {
        let query = query.to_lowercase();
        self.entries
            .iter()
            .filter(|entry| {
                entry.content.to_lowercase().contains(&query)
                    || entry.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .collect()
    }

    pub fn incomplete_tasks(&self) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|entry| {
                entry.bullet_type == BulletType::Task
                    && entry.status == Some(TaskStatus::Incomplete)
            })
            .collect()
    }

    pub fn migrate_incomplete_tasks(&mut self, from_date: NaiveDate, to_date: NaiveDate) {
        let mut new_entries = Vec::new();
        for entry in &mut self.entries {
            if entry.date == from_date
                && entry.bullet_type == BulletType::Task
                && entry.status == Some(TaskStatus::Incomplete)
            {
                entry.status = Some(TaskStatus::Migrated);
                let mut new_entry = entry.clone();
                new_entry.id = Uuid::new_v4();
                new_entry.date = to_date;
                new_entry.status = Some(TaskStatus::Incomplete);
                new_entries.push(new_entry);
            }
        }
        self.entries.extend(new_entries);
        self.entries.sort_by(|a, b| a.date.cmp(&b.date));
    }
}