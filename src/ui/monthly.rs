use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::ui::app::App;
use crate::ui::components::create_entry_list;
use chrono::{NaiveDate, Datelike};

pub fn render_monthly_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(area);

    render_calendar(f, app, chunks[0]);
    render_monthly_entries(f, app, chunks[1]);
}

fn render_calendar(f: &mut Frame, app: &App, area: Rect) {
    let (year, month) = app.selected_month;
    let month_name = get_month_name(month);
    let config = app.config.get_config();
    let colors = &config.theme.colors;
    
    let title = format!("{} {}", month_name, year);
    let mut calendar_lines = vec![
        Line::from(title.clone()).style(Style::default().fg(colors.accent()).add_modifier(Modifier::BOLD)),
        Line::from(""),
        Line::from("Su Mo Tu We Th Fr Sa"),
    ];

    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let days_in_month = get_days_in_month(year, month);
    let first_weekday = first_day.weekday().num_days_from_sunday() as usize;

    let mut day = 1;
    
    for week in 0..6 {
        let mut week_str = String::new();
        
        for day_of_week in 0..7 {
            
            if (week == 0 && day_of_week < first_weekday) || day > days_in_month {
                week_str.push_str("   ");
            } else {
                let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                let has_entries = !app.journal.entries_for_date(date).is_empty();
                let is_today = date == chrono::Local::now().date_naive();
                
                let day_str = if has_entries {
                    format!("{:2}*", day)
                } else {
                    format!("{:2} ", day)
                };
                
                if is_today {
                    week_str.push_str(&format!("[{}]", &day_str[0..2]));
                } else {
                    week_str.push_str(&day_str);
                }
                
                day += 1;
            }
        }
        
        if !week_str.trim().is_empty() {
            calendar_lines.push(Line::from(week_str));
        }
        
        if day > days_in_month {
            break;
        }
    }

    calendar_lines.push(Line::from(""));
    calendar_lines.push(Line::from("* = has entries").style(Style::default().fg(colors.muted())));
    calendar_lines.push(Line::from("[] = today").style(Style::default().fg(colors.muted())));

    let borders = config.layout.border_style.to_ratatui_border();
    let paragraph = Paragraph::new(calendar_lines)
        .block(Block::default().borders(borders).title("Calendar"))
        .alignment(Alignment::Left);
    
    f.render_widget(paragraph, area);
}

fn render_monthly_entries(f: &mut Frame, app: &App, area: Rect) {
    let (year, month) = app.selected_month;
    let entries = app.journal.entries_for_month(year, month);
    let config = app.config.get_config();
    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();
    
    if entries.is_empty() {
        let empty_msg = Paragraph::new("No entries this month")
            .style(Style::default().fg(colors.muted()))
            .block(Block::default().borders(borders).title("Monthly Entries"))
            .alignment(Alignment::Center);
        f.render_widget(empty_msg, area);
        return;
    }

    let list = create_entry_list(&entries, app.selected_entry, app)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Monthly Entries ({})", entries.len()))
        );
    
    f.render_widget(list, area);
}

fn get_month_name(month: u32) -> &'static str {
    match month {
        1 => "January", 2 => "February", 3 => "March", 4 => "April",
        5 => "May", 6 => "June", 7 => "July", 8 => "August",
        9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "Unknown",
    }
}

fn get_days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 30,
    }
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}