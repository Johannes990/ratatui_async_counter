mod tui;

use color_eyre::eyre::Result;
use crossterm::event::KeyCode::Char;
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph};
use tui::Event;

pub type Frame<'a> = ratatui::Frame<'a>;


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
    let mut events = Tui::new();
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