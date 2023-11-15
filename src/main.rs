use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};

mod tui;

use crate::tui::Event;
use crate::tui::EventHandler;


pub type Frame<'a> = ratatui::Frame<'a>;

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

struct App {
    counter: i64,
    should_quit: bool,
}

fn ui(app: &App, f: &mut Frame<'_>) {
    f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
}

fn update(app: &mut App, event: Event) -> Result<()> {
    if let Event::Key(key) = event {
        match key.code {
            Char('j') => app.counter += 1,
            Char('k') => app.counter -= 1,
            Char('q') => app.should_quit = true,
            _ => {},
        }
    }
    Ok(())
}

async fn run() -> Result<()> {
    let mut events = EventHandler::new();
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let mut app = App { counter: 0, should_quit: false };

    loop {
        let event = events.next().await?;

        let _ = update(&mut app, event);
        t.draw(|f| {
            ui(&app, f);
        })?;

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    startup()?;

    let result = run().await;

    shutdown()?;

    result?;

    Ok(())
}