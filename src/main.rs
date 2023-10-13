use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};

// type Err = Box<dyn std::error::Error>;
// type Result<T> = std::result::Result<T, Err>;
use anyhow::Result;
pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;

struct App {
    counter: i64,
    should_quit: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    startup()?;
    let status = run();
    shutdown()?;
    status?;
    Ok(())
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &mut App, f: &mut Frame) -> Result<()> {
    f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
    Ok(())
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            // check if key.kind is a 'KeyEventKind::Press' for cross-platform compatibility
            if key.kind == crossterm::event::KeyEventKind::Press {
                match key.code {
                    Char('j') => app.counter += 1,
                    Char('k') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let mut app = App { counter: 0, should_quit: false };

    while !app.should_quit {
        terminal.draw(|f| {ui(&mut app, f);})?;
        update(&mut app)?;
    }

    Ok(())
}


