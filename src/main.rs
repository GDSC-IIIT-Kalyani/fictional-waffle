//// ANCHOR: app
use std::time::{Duration, Instant};

use color_eyre::eyre::{eyre, Result};
use futures::{FutureExt, StreamExt};
use itertools::Itertools;
use ratatui::widgets::block::Title;
use ratatui::{backend::CrosstermBackend as Backend, prelude::*, widgets::*};
use strum::EnumIs;
use tui_big_text::BigText;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

mod tui;

// async play audio (Wellerman)

fn play_audio() {
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let file = File::open("Wellerman_Nathan_Evans.mp3").unwrap();
  let source = Decoder::new(BufReader::new(file)).unwrap();
  let source = source.repeat_infinite();
  let sink = Sink::try_new(&stream_handle).unwrap();
  sink.append(source);
  // keep main thread alive while audio is playing
}

pub type Frame<'a> = ratatui::Frame<'a, Backend<std::io::Stderr>>;

#[tokio::main]
async fn main() -> Result<()> {
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let file = File::open("Wellerman_Nathan_Evans.mp3").unwrap();
  let source = Decoder::new(BufReader::new(file)).unwrap();
  let source = source.repeat_infinite();
  let sink = Sink::try_new(&stream_handle).unwrap();
  sink.append(source);
  println!("Hello, world!");
  let mut app = App::default();
  app.run().await
}

#[derive(Clone, Debug)]
pub enum Event {
  Error,
  Tick,
  Key(crossterm::event::KeyEvent),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIs)]
enum AppState {
  #[default]
  Stopped,
  Running,
  Quitting,
  Split,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
  Stop,
  Tick,
  Quit,
  NextPage,
  PrevPage,
  ToggleSplit,
  None,
}

#[derive(Debug, Clone, PartialEq)]
struct App {
  state: AppState,
  page_index: usize,
}

impl Default for App {
  fn default() -> Self {
    Self::new()
  }
}

impl App {
  fn new() -> Self {
    Self {
        state: AppState::Stopped,
        page_index: 0,  
    }
  }

  async fn run(&mut self) -> Result<()> {
    let mut tui = Tui::new()?;
    tui.enter()?;
    while !self.state.is_quitting() {
      tui.draw(|f| self.ui(f).expect("Unexpected error during drawing"))?;
      let event = tui.next().await.ok_or(eyre!("Unable to get event"))?; // blocks until next event
      let message = self.handle_event(event)?;
      self.update(message)?;
    }
    tui.exit()?;
    Ok(())
  }

  fn handle_event(&self, event: Event) -> Result<Message> {
    let msg = match event {
      Event::Key(key) => {
        match key.code {
          crossterm::event::KeyCode::Char('q') => Message::Quit,
          crossterm::event::KeyCode::Char(' ') => Message::ToggleSplit,
          crossterm::event::KeyCode::Char('l') | crossterm::event::KeyCode::Right => Message::NextPage,
          crossterm::event::KeyCode::Char('h') | crossterm::event::KeyCode::Left => Message::PrevPage,
          crossterm::event::KeyCode::Char('s') | crossterm::event::KeyCode::Enter => Message::Stop,
          _ => Message::Tick,
        }
      },
      _ => Message::Tick,
    };
    Ok(msg)
  }

  fn update(&mut self, message: Message) -> Result<()> {
    match message {
      Message::Stop => {},
      Message::Tick => {},
      Message::Quit => self.quit(),
      Message::ToggleSplit => {
        if self.state == AppState::Split {
            self.state = AppState::Stopped;
        } else {
            self.state = AppState::Split;
        }
      },
      Message::NextPage => self.page_index += 1,
      Message::PrevPage => self.page_index -= 1,
      Message::None => {}
    }
    Ok(())
  }

  fn quit(&mut self) {
    self.state = AppState::Quitting
  }

  fn ui(&mut self, f: &mut Frame) -> Result<()> {
    let layout = self.layout(f.size());
    let mut offset = 0;
    if self.state == AppState::Split {
        offset = 1;
        f.render_widget(Paragraph::new("Splits:").block(Block::default().borders(Borders::ALL)), layout[3]);
    }
    f.render_widget(Paragraph::new("fictional-waffle"), layout[0]);
    f.render_widget(self.fps_paragraph(), layout[1]);
    f.render_widget(self.timer_paragraph(), layout[2]);
    // f.render_widget(Paragraph::new("Splits:"), layout[3]);
    f.render_widget(self.page(), layout[4]);
    // f.render_widget(self.splits_paragraph(), layout[4]);
    f.render_widget(self.help_paragraph(), layout[5 + offset]);
    Ok(())
  }

  fn page(&mut self) -> Paragraph<'_> {
    // centred Paragraph with a border
    let text0 = "The command line, often referred to as the terminal or console, is a text-based interface for interacting with your computer's operating system. It allows you to perform various tasks, such as navigating your file system, managing files and directories, and running programs. This walkthrough will introduce you to the basics of the command line and essential commands to get you started.";
    let text1 = "The Basic Idea:\n\nTerminal: A terminal is like a magical talking box for computers. It's where you can give instructions to the computer by typing words. Imagine it's like a friendly robot you can talk to.

Shell: The shell is like the brain of the computer that listens to what you say in the terminal. It's the part that understands your words and makes the computer do what you want, like opening games or drawing pictures.

What they do:

Terminal: A terminal is a text-based interface for interacting with a computer's operating system. It provides a way to communicate with the computer using text commands. It's like a blank canvas where you can type in instructions, and the computer will respond accordingly.

Shell: The shell is a program within the terminal that interprets and executes the commands you enter. It's the intermediary between you and the computer's operating system. It takes your text commands, translates them into actions that the computer can understand, and then carries out those actions. Different shells, like Bash or PowerShell, may have their own features and capabilities, but they all serve as the bridge between your instructions and the computer's actions.";

    let text = match self.page_index {
      0 => text0,
      1 => text1,
      _ => text0,
    };
    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Green)).padding(Padding::new(1, 4, 1, 4)).title(Title::from("Learn The Command Line").alignment(Alignment::Center))).wrap({
      Wrap { trim: true }
    }).alignment(Alignment::Center);
    paragraph
  }

  fn fps_paragraph(&mut self) -> Paragraph<'_> {
    // let fps = format!("{:.2} fps", self.fps);
    Paragraph::new(format!("linux mastery v0.0.1 | page : {}", self.page_index))
      .dim()
      .alignment(Alignment::Right)
  }

  fn timer_paragraph(&mut self) -> BigText<'_> {
    let style = Style::new().green();
    // let style = if self.state.is_running() { Style::new().green() } else { Style::new().red() };
    // let elapsed = self.elapsed();
    // let duration = self.format_duration(elapsed);
    // let lines = vec![duration.into()];
    // tui_big_text::BigTextBuilder::default().lines(lines).style(style).build().unwrap()
    tui_big_text::BigTextBuilder::default().lines(vec!["GDSC IIITK".into()]).style(style).build().unwrap()
  }

  /// Renders the splits as a list of lines.
  ///
  /// ```text
  /// #01 -- 00:00.693 -- 00:00.693
  /// #02 -- 00:00.719 -- 00:01.413
  /// ```
  fn help_paragraph(&mut self) -> Paragraph<'_> {
    let space_action = "next page";
    let help_text =
      Line::from(vec!["space ".into(), space_action.dim(), " enter ".into(), "split".dim(), " q ".into(), "quit".dim()]);
    Paragraph::new(help_text).gray()
  }

  fn layout(&self, area: Rect) -> Vec<Rect> {
    // let layout = Layout::default()
    //   .direction(Direction::Vertical)
    //   .constraints(vec![
    //     Constraint::Length(2), // top bar
    //     Constraint::Length(8), // timer
    //     Constraint::Length(1), // splits header
    //     Constraint::Min(0),    // splits
    //     Constraint::Length(1), // help
    //   ])
    //   .split(area);
    
    // center splits and top bar
    let layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Length(2), // top bar
        Constraint::Length(8), // timer
        Constraint::Length(1), // splits header
        Constraint::Min(0),    // splits
        Constraint::Length(1), // help
      ])
      .split(area);

    let top_layout = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![
        Constraint::Length(20), // title
        Constraint::Min(0),     // fps counter
      ])
      .split(layout[0]);
    
    // center timer
    if AppState::Split == self.state {
        // break timer into 2 equal parts
        let timer_layout = Layout::default()
          .direction(Direction::Horizontal)
          .constraints(vec![
             Constraint::Percentage(50),
             Constraint::Percentage(50),
          ])
          .split(layout[3]);
        return vec![top_layout[0], top_layout[1], layout[1], timer_layout[0], timer_layout[1], layout[3], layout[4]];
    }

    // return a new vec with the top_layout rects and gdsc and then rest of layout
    vec![top_layout[0], top_layout[1], layout[1], layout[2], layout[3], layout[4]]

    // top_layout[..].iter().chain(layout[1..].iter()).copied().collect()
  }

  fn format_split<'a>(&self, index: usize, start: Instant, previous: Instant, current: Instant) -> Line<'a> {
    let split = self.format_duration(current - previous);
    let elapsed = self.format_duration(current - start);
    Line::from(vec![
      format!("#{:02} -- ", index + 1).into(),
      Span::styled(split, Style::new().yellow()),
      " -- ".into(),
      Span::styled(elapsed, Style::new()),
    ])
  }

  fn format_duration(&self, duration: Duration) -> String {
    format!("{:02}:{:02}.{:03}", duration.as_secs() / 60, duration.as_secs() % 60, duration.subsec_millis())
  }
}
//// ANCHOR_END: app

struct Tui {
  pub terminal: Terminal<Backend<std::io::Stderr>>,
  pub task: tokio::task::JoinHandle<()>,
  pub cancellation_token: tokio_util::sync::CancellationToken,
  pub event_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
  pub event_tx: tokio::sync::mpsc::UnboundedSender<Event>,
}

impl Tui {
  fn new() -> Result<Tui> {
    let terminal = ratatui::Terminal::new(Backend::new(std::io::stderr()))?;
    let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
    let cancellation_token = tokio_util::sync::CancellationToken::new();
    let task = tokio::spawn(async {});
    Ok(Self { terminal, task, cancellation_token, event_rx, event_tx })
  }

  pub async fn next(&mut self) -> Option<Event> {
    self.event_rx.recv().await
  }

  pub fn enter(&mut self) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen, crossterm::cursor::Hide)?;
    self.start();
    Ok(())
  }

  pub fn exit(&self) -> Result<()> {
    self.stop()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen, crossterm::cursor::Show)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
  }

  pub fn cancel(&self) {
    self.cancellation_token.cancel();
  }

  pub fn stop(&self) -> Result<()> {
    self.cancel();
    let mut counter = 0;
    while !self.task.is_finished() {
      std::thread::sleep(Duration::from_millis(250));
      counter += 1;
      if counter > 5 {
        self.task.abort();
      }
      if counter > 10 {
        log::error!("Failed to abort task for unknown reason");
        return Err(eyre!("Unable to abort task"));
      }
    }
    Ok(())
  }

  pub fn start(&mut self) {
    let tick_rate = std::time::Duration::from_millis(60);
    self.cancel();
    self.cancellation_token = tokio_util::sync::CancellationToken::new();
    let _cancellation_token = self.cancellation_token.clone();
    let _event_tx = self.event_tx.clone();
    self.task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);
      loop {
        let delay = interval.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          _ = _cancellation_token.cancelled() => {
            break;
          }
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  crossterm::event::Event::Key(key) => {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                      _event_tx.send(Event::Key(key)).unwrap();
                    }
                  },
                  _ => {}
                }
              }
              Some(Err(_)) => {
                _event_tx.send(Event::Error).unwrap();
              }
              None => {},
            }
          },
          _ = delay => {
              _event_tx.send(Event::Tick).unwrap();
          },
        }
      }
    });
  }
}

impl std::ops::Deref for Tui {
  type Target = ratatui::Terminal<Backend<std::io::Stderr>>;

  fn deref(&self) -> &Self::Target {
    &self.terminal
  }
}

impl std::ops::DerefMut for Tui {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.terminal
  }
}

impl Drop for Tui {
  fn drop(&mut self) {
    self.exit().unwrap();
  }
}
