use crate::config::ConfigManager;
use crate::models::{Journal, Entry, BulletType};
use crate::storage::Storage;
use anyhow::Result;
use chrono::{Local, NaiveDate, Datelike};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum AppTab {
    Daily,
    Monthly,
    Future,
    Collections,
    Search,
}

#[derive(Debug, Clone)]
pub enum AppMode {
    Normal,
    Insert,
    Command,
}

#[derive(Debug, Clone)]
pub enum InputMode {
    Task,
    Event,
    Note,
}

pub struct App {
    pub journal: Journal,
    pub storage: Storage,
    pub config: ConfigManager,
    pub current_tab: AppTab,
    pub mode: AppMode,
    pub current_date: NaiveDate,
    pub selected_month: (i32, u32),
    pub should_quit: bool,
    pub messages: VecDeque<String>,
    pub input_buffer: String,
    pub input_mode: Option<InputMode>,
    pub show_help: bool,
    pub search_query: String,
    pub selected_entry: Option<usize>,
}

impl App {
    pub fn new() -> Result<Self> {
        let storage = Storage::new()?;
        let journal = storage.load_journal()?;
        let config = ConfigManager::new()?;
        let today = Local::now().date_naive();
        
        Ok(Self {
            journal,
            storage,
            config,
            current_tab: AppTab::Daily,
            mode: AppMode::Normal,
            current_date: today,
            selected_month: (today.year(), today.month()),
            should_quit: false,
            messages: VecDeque::new(),
            input_buffer: String::new(),
            input_mode: None,
            show_help: false,
            search_query: String::new(),
            selected_entry: None,
        })
    }

    pub fn save(&self) -> Result<()> {
        self.storage.save_journal(&self.journal)
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push_back(message);
        if self.messages.len() > 5 {
            self.messages.pop_front();
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match self.mode {
            AppMode::Normal => self.handle_normal_key(key),
            AppMode::Insert => self.handle_insert_key(key),
            AppMode::Command => self.handle_command_key(key),
        }
    }

    fn handle_normal_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('?') => {
                self.show_help = !self.show_help;
            }
            KeyCode::Tab => {
                self.next_tab();
            }
            KeyCode::BackTab => {
                self.prev_tab();
            }
            KeyCode::Char('1') => self.current_tab = AppTab::Daily,
            KeyCode::Char('2') => self.current_tab = AppTab::Monthly,
            KeyCode::Char('3') => self.current_tab = AppTab::Future,
            KeyCode::Char('4') => self.current_tab = AppTab::Collections,
            KeyCode::Char('5') => self.current_tab = AppTab::Search,
            KeyCode::Char('t') => {
                self.mode = AppMode::Insert;
                self.input_mode = Some(InputMode::Task);
                self.input_buffer.clear();
            }
            KeyCode::Char('e') => {
                self.mode = AppMode::Insert;
                self.input_mode = Some(InputMode::Event);
                self.input_buffer.clear();
            }
            KeyCode::Char('n') => {
                self.mode = AppMode::Insert;
                self.input_mode = Some(InputMode::Note);
                self.input_buffer.clear();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.move_selection_down();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.move_selection_up();
            }
            KeyCode::Char('h') | KeyCode::Left => {
                self.handle_left();
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.handle_right();
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.toggle_selected_entry();
            }
            KeyCode::Char('d') => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.delete_selected_entry();
                }
            }
            KeyCode::Char('/') => {
                self.current_tab = AppTab::Search;
                self.mode = AppMode::Insert;
                self.search_query.clear();
            }
            KeyCode::Char('s') => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match self.save() {
                        Ok(_) => self.add_message("Journal saved".to_string()),
                        Err(e) => self.add_message(format!("Save failed: {}", e)),
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_insert_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = AppMode::Normal;
                self.input_mode = None;
            }
            KeyCode::Enter => {
                if self.current_tab == AppTab::Search {
                    self.search_query = self.input_buffer.clone();
                    self.input_buffer.clear();
                    self.mode = AppMode::Normal;
                } else if let Some(input_mode) = &self.input_mode {
                    self.create_entry(input_mode.clone())?;
                    self.input_buffer.clear();
                    self.mode = AppMode::Normal;
                    self.input_mode = None;
                }
            }
            KeyCode::Backspace => {
                if self.current_tab == AppTab::Search {
                    self.search_query.pop();
                } else {
                    self.input_buffer.pop();
                }
            }
            KeyCode::Char(c) => {
                if self.current_tab == AppTab::Search {
                    self.search_query.push(c);
                } else {
                    self.input_buffer.push(c);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_command_key(&mut self, _key: KeyEvent) -> Result<()> {
        Ok(())
    }

    fn create_entry(&mut self, input_mode: InputMode) -> Result<()> {
        if self.input_buffer.trim().is_empty() {
            return Ok(());
        }

        let bullet_type = match input_mode {
            InputMode::Task => BulletType::Task,
            InputMode::Event => BulletType::Event,
            InputMode::Note => BulletType::Note,
        };

        let date = match self.current_tab {
            AppTab::Daily => self.current_date,
            AppTab::Monthly => self.current_date,
            AppTab::Future => {
                Local::now().date_naive().succ_opt().unwrap_or(Local::now().date_naive())
            }
            _ => self.current_date,
        };

        let entry = Entry::new(self.input_buffer.clone(), bullet_type, date);
        self.journal.add_entry(entry);
        
        let type_name = match input_mode {
            InputMode::Task => "Task",
            InputMode::Event => "Event", 
            InputMode::Note => "Note",
        };
        self.add_message(format!("{} added", type_name));
        Ok(())
    }

    fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Daily => AppTab::Monthly,
            AppTab::Monthly => AppTab::Future,
            AppTab::Future => AppTab::Collections,
            AppTab::Collections => AppTab::Search,
            AppTab::Search => AppTab::Daily,
        };
    }

    fn prev_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Daily => AppTab::Search,
            AppTab::Monthly => AppTab::Daily,
            AppTab::Future => AppTab::Monthly,
            AppTab::Collections => AppTab::Future,
            AppTab::Search => AppTab::Collections,
        };
    }

    fn move_selection_down(&mut self) {
        let entries = self.get_current_entries();
        if let Some(selected) = self.selected_entry {
            if selected < entries.len().saturating_sub(1) {
                self.selected_entry = Some(selected + 1);
            }
        } else if !entries.is_empty() {
            self.selected_entry = Some(0);
        }
    }

    fn move_selection_up(&mut self) {
        if let Some(selected) = self.selected_entry {
            if selected > 0 {
                self.selected_entry = Some(selected - 1);
            }
        }
    }

    fn handle_left(&mut self) {
        match self.current_tab {
            AppTab::Daily => {
                self.current_date = self.current_date.pred_opt()
                    .unwrap_or(self.current_date);
                self.selected_entry = None;
            }
            AppTab::Monthly => {
                let (year, month) = self.selected_month;
                if month == 1 {
                    self.selected_month = (year - 1, 12);
                } else {
                    self.selected_month = (year, month - 1);
                }
                self.selected_entry = None;
            }
            _ => {}
        }
    }

    fn handle_right(&mut self) {
        match self.current_tab {
            AppTab::Daily => {
                self.current_date = self.current_date.succ_opt()
                    .unwrap_or(self.current_date);
                self.selected_entry = None;
            }
            AppTab::Monthly => {
                let (year, month) = self.selected_month;
                if month == 12 {
                    self.selected_month = (year + 1, 1);
                } else {
                    self.selected_month = (year, month + 1);
                }
                self.selected_entry = None;
            }
            _ => {}
        }
    }

    fn toggle_selected_entry(&mut self) {
        if let Some(selected) = self.selected_entry {
            let entries = self.get_current_entries();
            if let Some(entry) = entries.get(selected) {
                if let Some(entry_mut) = self.journal.get_entry_mut(entry.id) {
                    entry_mut.toggle_complete();
                    let status = if entry_mut.status.as_ref().unwrap_or(&crate::models::TaskStatus::Incomplete) 
                        == &crate::models::TaskStatus::Complete {
                        "completed"
                    } else {
                        "marked incomplete"
                    };
                    self.add_message(format!("Task {}", status));
                }
            }
        }
    }

    fn delete_selected_entry(&mut self) {
        if let Some(selected) = self.selected_entry {
            let entries = self.get_current_entries();
            if let Some(entry) = entries.get(selected) {
                self.journal.remove_entry(entry.id);
                self.add_message("Entry deleted".to_string());
                self.selected_entry = None;
            }
        }
    }

    fn get_current_entries(&self) -> Vec<&Entry> {
        match self.current_tab {
            AppTab::Daily => self.journal.entries_for_date(self.current_date),
            AppTab::Monthly => {
                let (year, month) = self.selected_month;
                self.journal.entries_for_month(year, month)
            }
            AppTab::Search => {
                if self.search_query.is_empty() {
                    Vec::new()
                } else {
                    self.journal.search_entries(&self.search_query)
                }
            }
            AppTab::Future => {
                let today = Local::now().date_naive();
                self.journal.entries.iter()
                    .filter(|entry| entry.date > today)
                    .collect()
            }
            AppTab::Collections => Vec::new(),
        }
    }

}