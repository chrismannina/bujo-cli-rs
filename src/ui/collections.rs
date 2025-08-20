use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::ui::app::App;

pub fn render_collections_view(f: &mut Frame, app: &App, area: Rect) {
    if app.journal.collections.is_empty() {
        let empty_msg = Paragraph::new("No collections yet.\n\nCollections feature coming soon!")
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title("Collections"))
            .alignment(Alignment::Center);
        f.render_widget(empty_msg, area);
        return;
    }

    let collections: Vec<ListItem> = app.journal.collections
        .values()
        .map(|collection| {
            let content = format!("{} ({} entries)", 
                collection.name, 
                collection.entries.len()
            );
            ListItem::new(content)
        })
        .collect();

    let list = List::new(collections)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Collections ({})", app.journal.collections.len()))
        )
        .highlight_style(Style::default().bg(Color::DarkGray));
    
    f.render_widget(list, area);
}