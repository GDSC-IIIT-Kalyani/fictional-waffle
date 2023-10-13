use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // start :: enable raw mode
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // init :: terminal backend
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // define :: counter variable
    let mut counter = 0;

    // main :: application loop
    loop {
        // render :: ui
        terminal.draw(|f| {
            f.render_widget(
                Paragraph::new(format!("Counter: {}", counter)),
                f.size(),
            );
        })?;

        // handle :: input
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('j') => counter += 1,
                        crossterm::event::KeyCode::Char('k') => counter -= 1,
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    // stop :: disable raw mode
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
