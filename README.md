# Bujo - Terminal Bullet Journal

A full-featured CLI bullet journal application built with Rust, featuring a text-based user interface.

## Features

- **Daily Log**: View and manage today's tasks, notes, and events
- **Monthly Log**: Calendar view with entries and navigation
- **Future Log**: Plan ahead with future entries
- **Collections**: Organize custom lists (feature in development)
- **Search**: Find entries across all logs by content and tags
- **Rapid Logging**: Quick entry with traditional bullet symbols
- **Data Persistence**: JSON file storage in user data directory

## Installation

```bash
# Clone and build
git clone <repository>
cd bujo
cargo build --release

# Run
cargo run
```

## Usage

### Navigation
- `Tab` / `Shift+Tab` - Switch between tabs
- `1-5` - Jump directly to Daily/Monthly/Future/Collections/Search
- `h/j/k/l` or Arrow Keys - Navigate entries and dates

### Entry Management
- `t` - Add new task
- `e` - Add new event  
- `n` - Add new note
- `Space` / `Enter` - Toggle task completion
- `Ctrl+d` - Delete selected entry

### Other Controls
- `/` - Start search
- `Ctrl+s` - Save journal
- `?` - Toggle help screen
- `q` - Quit application
- `Ctrl+c` - Force quit

## Bullet Symbols

- `•` - Incomplete task
- `✓` - Completed task
- `>` - Migrated task
- `<` - Scheduled task
- `✗` - Irrelevant task
- `○` - Event
- `-` - Note

## Data Storage

Journal data is stored as JSON in your system's data directory:
- **macOS**: `~/Library/Application Support/bujo/journal.json`
- **Linux**: `~/.local/share/bujo/journal.json`  
- **Windows**: `%APPDATA%/bujo/journal.json`

## Development

Built with:
- [Ratatui](https://ratatui.rs/) - Terminal UI framework
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal handling
- [Chrono](https://github.com/chronotope/chrono) - Date/time handling
- [Serde](https://serde.rs/) - JSON serialization

## License

MIT License