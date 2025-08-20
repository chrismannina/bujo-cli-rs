use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::ui::app::App;
use crate::ui::components::create_entry_list;
use chrono::Local;

pub fn render_future_view(f: &mut Frame, app: &App, area: Rect) {
    let today = Local::now().date_naive();
    let future_entries: Vec<_> = app.journal.entries.iter()
        .filter(|entry| entry.date > today)
        .collect();

    if future_entries.is_empty() {
        let empty_msg = Paragraph::new("No future entries.\n\nPress 't', 'e', or 'n' to add entries for future dates")
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title("Future Log"))
            .alignment(Alignment::Center);
        f.render_widget(empty_msg, area);
        return;
    }

    let list = create_entry_list(&future_entries, app.selected_entry)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Future Log ({} entries)", future_entries.len()))
        );
    
    f.render_widget(list, area);
}