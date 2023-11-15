use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};
use color_eyre::eyre::Result;
use crossterm::{
    cursor,
    event::{
        DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture, Event as CrosstermEvent,
        KeyEvent, KeyEventKind, MouseEvent,
    },
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend as Backend;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

pub type Frame<'a> = ratatui::Frame<'a>;

#[derive(Clone, Debug)]
pub enum Event {
    Init,
    Quit,
    Error,
    Closed,
    Tick,
    Render,
    FocusGained,
    FocusLost,
    Paste(String),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct Tui {
    pub terminal: ratatui::Terminal<Backend<std::io::Stderr>>,
    pub task: JoinHandle<()>,
    pub cancellation_token: CancellationToken,
    pub event_rx: UnboundedReceiver<Event>,
    pub event_tx: UnboundedSender<Event>,
    pub frame_rate: f64,
    pub tick_rate: f64,
    pub mouse: bool,
    pub paste: bool,
}

impl Tui {
    pub fn new() -> Self {
        let tick_rate = std::time::Duration::from_millis(250);
        let (tx, rx) = mpsc::unbounded_channel();
        let _tx = tx.clone();

        let task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut interval = tokio::time::interval(tick_rate);

            loop {
                let delay = interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(evt)) => {
                                match evt {
                                    crossterm::event::Event::Key(key) => {
                                        if key.kind == crossterm::event::KeyEventKind::Press {
                                            tx.send(Event::Key(key)).unwrap();
                                        }
                                    },
                                    _ => {},
                                }
                            }
                            Some(Err(_)) => {
                                tx.send(Event::Error).unwrap();
                            }
                            None => {},
                        }
                    },
                    _ = delay => {
                        tx.send(Event::Tick).unwrap();
                    },
                }
            }
        });

        Self { _tx, rx, task: Some(task) }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.rx.recv().await.ok_or(color_eyre::eyre::eyre!("Unable to get event"))
    }
}
