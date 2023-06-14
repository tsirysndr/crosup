use anyhow::Error;
use bat::PrettyPrinter;
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, List, ListItem},
    Terminal,
};

use crate::App;

pub fn display_history(mut app: App) -> Result<(), Error> {
    enable_raw_mode()?;
    let stdout = io::stdout();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    render_initial_ui(&mut terminal, &mut app)?;

    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up => {
                    if app.selected_index > 0 {
                        app.selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if app.selected_index < app.items.len() - 1 {
                        app.selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    let content = app.content.get(&app.selected_index).unwrap();
                    clear_screen(&mut terminal)?;
                    let language = match app.title.ends_with(".hcl") {
                        true => "hcl",
                        false => "toml",
                    };

                    PrettyPrinter::new()
                        .input_from_bytes(content.as_bytes())
                        .language(language)
                        .paging_mode(bat::PagingMode::Always)
                        .line_numbers(true)
                        .print()
                        .unwrap();
                    return Ok(());
                }
                _ => {}
            }
        }

        render_ui(&mut terminal, &mut app)?;
        thread::sleep(Duration::from_millis(10));
    }

    clear_screen(&mut terminal)?;

    Ok(())
}

pub fn clear_screen(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), Error> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    terminal.clear()?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

pub fn render_initial_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Error> {
    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);

        let items: Vec<ListItem> = app
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == app.selected_index {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Spans::from(Span::styled(item, style)))
            })
            .collect();

        let list = List::new(items).block(tui::widgets::Block::default());
        f.render_widget(list, chunks[1]);
    })?;
    Ok(())
}

pub fn render_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Error> {
    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);

        let items: Vec<ListItem> = app
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == app.selected_index {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Spans::from(Span::styled(item, style)))
            })
            .collect();

        let list = List::new(items).block(Block::default());
        f.render_widget(list, chunks[1]);
    })?;
    Ok(())
}
