enum Event {
    Key(crossterm::event::KeyEvent)
}

struct EventHandler {
    rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    fn new() -> Self {
        let tick_rate = std::time::Duration::from_millis(250);
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::spawn(async move {
            loop {
                if crossterm::event::poll(tick_rate).unwrap() {
                    match crossterm::event::read().unwrap() {
                        CrosstermEvent::Key(e) => {
                            if key.kind == event::KeyEventKind::Press {
                                tx.send(Event::Key(e).unwrap())
                            }
                        },
                        _ => unimplemented!(), 
                    }
                }
            }
        })

        EventHandler { rx }
    }
}
