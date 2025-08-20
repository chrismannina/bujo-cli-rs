use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::ui::app::App;
use crate::ui::components::create_entry_list;

pub fn render_search_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    render_search_input(f, app, chunks[0]);
    render_search_results(f, app, chunks[1]);
}

fn render_search_input(f: &mut Frame, app: &App, area: Rect) {
    let query_display = if app.search_query.is_empty() {
        "Type / to search entries...".to_string()
    } else {
        app.search_query.clone()
    };

    let config = app.config.get_config();
    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();

    let paragraph = Paragraph::new(query_display)
        .style(Style::default().fg(colors.text()))
        .block(Block::default().borders(borders).title("Search Query"));
    
    f.render_widget(paragraph, area);
}

fn render_search_results(f: &mut Frame, app: &App, area: Rect) {
    let config = app.config.get_config();
    let colors = &config.theme.colors;
    let borders = config.layout.border_style.to_ratatui_border();
    
    if app.search_query.is_empty() {
        let help_msg = Paragraph::new("Press / to start searching\n\nSearch will match entry content and tags")
            .style(Style::default().fg(colors.muted()))
            .block(Block::default().borders(borders).title("Search Results"))
            .alignment(Alignment::Center);
        f.render_widget(help_msg, area);
        return;
    }

    let results = app.journal.search_entries(&app.search_query);
    
    if results.is_empty() {
        let no_results_msg = Paragraph::new(format!("No results found for '{}'", app.search_query))
            .style(Style::default().fg(colors.warning()))
            .block(Block::default().borders(borders).title("Search Results"))
            .alignment(Alignment::Center);
        f.render_widget(no_results_msg, area);
        return;
    }

    let list = create_entry_list(&results, app.selected_entry, app)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Search Results ({} found)", results.len()))
        );
    
    f.render_widget(list, area);
}