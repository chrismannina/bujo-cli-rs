use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::ui::app::App;
use crate::ui::components::create_entry_list;
use chrono::Datelike;

pub fn render_daily_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    render_daily_header(f, app, chunks[0]);
    render_daily_entries(f, app, chunks[1]);
}

fn render_daily_header(f: &mut Frame, app: &App, area: Rect) {
    let date_str = format!(
        "{} - {}",
        app.current_date.format("%A, %B %d, %Y"),
        get_day_of_year_info(&app.current_date)
    );
    
    let config = app.config.get_config();
    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();
    
    let paragraph = Paragraph::new(date_str)
        .style(Style::default().fg(colors.accent()).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(borders).title("Daily Log"))
        .alignment(Alignment::Center);
    
    f.render_widget(paragraph, area);
}

fn render_daily_entries(f: &mut Frame, app: &App, area: Rect) {
    let entries = app.journal.entries_for_date(app.current_date);
    let config = app.config.get_config();
    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();
    
    if entries.is_empty() {
        let empty_msg = Paragraph::new("No entries for today.\n\nPress 't' for task, 'e' for event, 'n' for note")
            .style(Style::default().fg(colors.muted()))
            .block(Block::default().borders(borders).title("Entries"))
            .alignment(Alignment::Center);
        f.render_widget(empty_msg, area);
        return;
    }

    let list = create_entry_list(&entries, app.selected_entry, app)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Entries ({})", entries.len()))
        );
    
    f.render_widget(list, area);
}

fn get_day_of_year_info(date: &chrono::NaiveDate) -> String {
    let day_of_year = date.ordinal();
    let days_in_year = if date.leap_year() { 366 } else { 365 };
    format!("Day {} of {}", day_of_year, days_in_year)
}