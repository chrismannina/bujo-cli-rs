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

    let config = app.config.get_config();
    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();

    if future_entries.is_empty() {
        let empty_msg = Paragraph::new("No future entries.\n\nPress 't', 'e', or 'n' to add entries for future dates")
            .style(Style::default().fg(colors.muted()))
            .block(Block::default().borders(borders).title("Future Log"))
            .alignment(Alignment::Center);
        f.render_widget(empty_msg, area);
        return;
    }

    let list = create_entry_list(&future_entries, app.selected_entry, app)
        .block(
            Block::default()
                .borders(borders)
                .title(format!("Future Log ({} entries)", future_entries.len()))
        );
    
    f.render_widget(list, area);
}