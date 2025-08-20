use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::ui::app::{App, AppTab, AppMode};

pub fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let tab_titles = vec![
        "Daily (1)",
        "Monthly (2)", 
        "Future (3)",
        "Collections (4)",
        "Search (5)",
    ];
    
    let selected_tab = match app.current_tab {
        AppTab::Daily => 0,
        AppTab::Monthly => 1,
        AppTab::Future => 2,
        AppTab::Collections => 3,
        AppTab::Search => 4,
    };

    let config = app.config.get_config();
    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();

    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(borders).title("Bullet Journal"))
        .select(selected_tab)
        .style(Style::default().fg(colors.primary()))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(colors.background())
                .fg(colors.accent())
        );

    f.render_widget(tabs, area);
}

pub fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let mode_text = match app.mode {
        AppMode::Normal => "NORMAL",
        AppMode::Insert => "INSERT",
        AppMode::Command => "COMMAND",
    };

    let input_text = if app.input_mode.is_some() || app.current_tab == AppTab::Search {
        format!(" | Input: {}", app.input_buffer)
    } else {
        String::new()
    };

    let search_text = if app.current_tab == AppTab::Search && !app.search_query.is_empty() {
        format!(" | Search: {}", app.search_query)
    } else {
        String::new()
    };

    let status_text = format!("{}{}{}", mode_text, input_text, search_text);
    
    let config = app.config.get_config();
    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();
    
    let paragraph = Paragraph::new(status_text)
        .style(Style::default().fg(colors.accent()))
        .block(Block::default().borders(borders));
    
    f.render_widget(paragraph, area);
}

pub fn render_messages(f: &mut Frame, app: &App, area: Rect) {
    if app.messages.is_empty() {
        return;
    }

    let messages: Vec<Line> = app.messages
        .iter()
        .map(|msg| Line::from(msg.clone()))
        .collect();

    let config = app.config.get_config();
    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();

    let paragraph = Paragraph::new(messages)
        .style(Style::default().fg(colors.success()))
        .block(Block::default().borders(borders).title("Messages"))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

pub fn render_help(f: &mut Frame, app: &App, area: Rect) {
    let config = app.config.get_config();
    let help_text = vec![
        Line::from("🗒️ Bullet Journal - Help"),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  Tab/Shift+Tab - Switch tabs"),
        Line::from("  1-5 - Jump to specific tab"),
        Line::from("  h/j/k/l or arrows - Navigate"),
        Line::from(""),
        Line::from("Entry Creation:"),
        Line::from("  t - Add task"),
        Line::from("  e - Add event"),
        Line::from("  n - Add note"),
        Line::from(""),
        Line::from("Entry Management:"),
        Line::from("  Space/Enter - Toggle task completion"),
        Line::from("  Ctrl+d - Delete selected entry"),
        Line::from(""),
        Line::from("Other:"),
        Line::from("  / - Search"),
        Line::from("  Ctrl+s - Save"),
        Line::from("  ? - Toggle this help"),
        Line::from("  q - Quit"),
        Line::from(""),
        Line::from("Configuration:"),
        Line::from("  F1 - Cycle theme"),
        Line::from("  F2 - Toggle border style"),
        Line::from("  F3 - Toggle compact mode"),
        Line::from(""),
        Line::from("Current Settings:"),
        Line::from(format!("  Theme: {}", config.theme.name)),
        Line::from(format!("  Border: {:?}", config.layout.border_style)),
        Line::from(format!("  Compact mode: {}", config.layout.compact_mode)),
        Line::from(""),
        Line::from("Config file: ~/.config/bujo/config.toml"),
        Line::from(""),
        Line::from("Press ? to close help"),
    ];

    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();

    let paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(colors.text()))
        .block(
            Block::default()
                .borders(borders)
                .title("Help")
                .border_style(Style::default().fg(colors.accent()))
        )
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

pub fn create_entry_list<'a>(entries: &'a [&'a crate::models::Entry], selected: Option<usize>, app: &App) -> List<'a> {
    let config = app.config.get_config();
    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();

    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let symbol = entry.symbol();
            let content = format!("{} {}", symbol, entry.content);
            
            let style = if entry.bullet_type == crate::models::BulletType::Task {
                match entry.status {
                    Some(crate::models::TaskStatus::Complete) => Style::default().fg(colors.success()),
                    Some(crate::models::TaskStatus::Migrated) => Style::default().fg(colors.warning()),
                    Some(crate::models::TaskStatus::Irrelevant) => Style::default().fg(colors.muted()),
                    _ => Style::default().fg(colors.text()),
                }
            } else if entry.bullet_type == crate::models::BulletType::Event {
                Style::default().fg(colors.secondary())
            } else {
                Style::default().fg(colors.primary())
            };

            let mut item = ListItem::new(content).style(style);
            
            if Some(i) == selected {
                item = item.style(style.bg(colors.muted()));
            }
            
            item
        })
        .collect();

    List::new(items)
        .block(Block::default().borders(borders))
        .highlight_style(Style::default().bg(colors.muted()))
}