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

    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Bullet Journal"))
        .select(selected_tab)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black)
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
    
    let paragraph = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    
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

    let paragraph = Paragraph::new(messages)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("Messages"))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

pub fn render_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from("Bullet Journal - Help"),
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
        Line::from("Press ? to close help"),
    ];

    let paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .border_style(Style::default().fg(Color::Yellow))
        )
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

pub fn create_entry_list<'a>(entries: &'a [&'a crate::models::Entry], selected: Option<usize>) -> List<'a> {
    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let symbol = entry.symbol();
            let content = format!("{} {}", symbol, entry.content);
            
            let style = if entry.bullet_type == crate::models::BulletType::Task {
                match entry.status {
                    Some(crate::models::TaskStatus::Complete) => Style::default().fg(Color::Green),
                    Some(crate::models::TaskStatus::Migrated) => Style::default().fg(Color::Yellow),
                    Some(crate::models::TaskStatus::Irrelevant) => Style::default().fg(Color::DarkGray),
                    _ => Style::default().fg(Color::White),
                }
            } else if entry.bullet_type == crate::models::BulletType::Event {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Cyan)
            };

            let mut item = ListItem::new(content).style(style);
            
            if Some(i) == selected {
                item = item.style(style.bg(Color::DarkGray));
            }
            
            item
        })
        .collect();

    List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray))
}