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

fn ui(f: &mut Frame<'_>, app: &App) {
    f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
}

fn update(app: &mut App, event: Event) {
    match event {
        Event::Key(key) => {
            match key.code {
                Char('j') => app.counter += 1,
                Char('k') => app.counter -= 1,
                Char('q') => app.should_quit = true,
                _ => {},
            }
        },
        _ => {},
    };
}

async fn run() -> Result<()> {
    let mut tui = tui::Tui::new()?.tick_rate(1.0).frame_rate(30.0);
    tui.enter()?;
    let mut app = App { counter: 0, should_quit: false };

    loop {
        let event = tui.next().await?; // blocks until next event

        if let Event::Render = event.clone() {
            tui.draw(|f| {
                ui(f, &app);
            })?;
        }

        update(&mut app, event);

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let result = run().await;

    result?;

    Ok(())
}