use crate::util::key::Key;
use crossterm::event;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap crossterm input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    _tx: mpsc::Sender<Event<Key>>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        thread::spawn(move || loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let event::Event::Key(event) = event::read().unwrap() {
                    let key = Key::from(event);
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            tx.send(Event::Tick).unwrap();
        });
        Events { rx, _tx: event_tx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
