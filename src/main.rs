mod config;
mod models;
mod storage;
mod ui;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use ui::{
    app::{App, AppTab},
    components::{render_tabs, render_status_bar, render_messages, render_help},
    daily::render_daily_view,
    monthly::render_monthly_view,
    future::render_future_view,
    collections::render_collections_view,
    search::render_search_view,
};

#[derive(Parser)]
#[command(name = "bujo")]
#[command(about = "A terminal-based bullet journal application")]
struct Cli {
    #[arg(short, long, help = "Print version information")]
    version: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    if cli.version {
        println!("bujo {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut app = App::new()?;
    let result = run_app(&mut terminal, &mut app);
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    
    if let Err(e) = result {
        eprintln!("Application error: {}", e);
    }
    
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| draw_ui(f, app))?;
        
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                        break;
                    }
                    _ => {
                        app.handle_key(key)?;
                        if app.should_quit {
                            app.save()?;
                            break;
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn draw_ui(f: &mut ratatui::Frame, app: &App) {
    if app.show_help {
        render_help(f, app, f.area());
        return;
    }

    let main_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(3), // tabs
            ratatui::layout::Constraint::Min(0),    // main content
            ratatui::layout::Constraint::Length(3), // status bar
        ])
        .split(f.area());

    render_tabs(f, app, main_chunks[0]);
    
    let content_chunks = if !app.messages.is_empty() {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(5),
            ])
            .split(main_chunks[1]);
        render_messages(f, app, chunks[1]);
        chunks[0]
    } else {
        main_chunks[1]
    };
    
    match app.current_tab {
        AppTab::Daily => render_daily_view(f, app, content_chunks),
        AppTab::Monthly => render_monthly_view(f, app, content_chunks),
        AppTab::Future => render_future_view(f, app, content_chunks),
        AppTab::Collections => render_collections_view(f, app, content_chunks),
        AppTab::Search => render_search_view(f, app, content_chunks),
    }
    
    render_status_bar(f, app, main_chunks[2]);
}