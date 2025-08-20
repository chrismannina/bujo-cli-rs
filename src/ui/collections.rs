use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::ui::app::App;

pub fn render_collections_view(f: &mut Frame, app: &App, area: Rect) {
    let config = app.config.get_config();
    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();

    if app.journal.collections.is_empty() {
        let empty_msg = Paragraph::new("No collections yet.\n\nCollections feature coming soon!")
            .style(Style::default().fg(colors.muted()))
            .block(Block::default().borders(borders).title("Collections"))
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
                .borders(borders)
                .title(format!("Collections ({})", app.journal.collections.len()))
        )
        .highlight_style(Style::default().bg(colors.muted()));
    
    f.render_widget(list, area);
}