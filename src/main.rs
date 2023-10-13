use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::*,
    // layout::*,
};
use std::io::{stderr, Result};

fn main() -> Result<()> {
    stderr().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            // small text centered in the middle of the screen
            frame.render_widget(
                Paragraph::new("Hello GDSC! (press 'q' to quit)")
                    .black()
                    .on_green()
                    .bold()
                    .alignment(Alignment::Center)
                    .block(Block::default()
                           .borders(Borders::ALL)
                           .border_type(BorderType::Thick)
                           .padding(Padding::new(0, 0, area.height / 2, 0))),
                area,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
