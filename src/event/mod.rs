mod handler;

pub use handler::{handle_key_event, handle_mouse_event};

use color_eyre::Result;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

/// Application events
#[derive(Debug, Clone)]
pub enum Event {
    /// Terminal tick
    Tick,
    /// Keyboard input
    Key(KeyEvent),
    /// Mouse input
    Mouse(MouseEvent),
    /// Git repository change detected with changed file paths
    GitChange(Vec<String>),
    /// Terminal resize
    #[allow(dead_code)]
    Resize(u16, u16),
}

/// Event handler for the application
pub struct EventHandler {
    /// Event receiver
    receiver: mpsc::UnboundedReceiver<Event>,
    /// Event sender (for external events like git changes)
    sender: mpsc::UnboundedSender<Event>,
}

impl EventHandler {
    /// Create a new event handler with the specified tick rate in milliseconds
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();

        let event_sender = sender.clone();
        tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);

            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                    _ = tick_delay => {
                        if event_sender.send(Event::Tick).is_err() {
                            break;
                        }
                    }
                    Some(Ok(evt)) = crossterm_event => {
                        match evt {
                            CrosstermEvent::Key(key) => {
                                if event_sender.send(Event::Key(key)).is_err() {
                                    break;
                                }
                            }
                            CrosstermEvent::Mouse(mouse) => {
                                if event_sender.send(Event::Mouse(mouse)).is_err() {
                                    break;
                                }
                            }
                            CrosstermEvent::Resize(w, h) => {
                                if event_sender.send(Event::Resize(w, h)).is_err() {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Self { receiver, sender }
    }

    /// Get the event sender for external events
    pub fn sender(&self) -> mpsc::UnboundedSender<Event> {
        self.sender.clone()
    }

    /// Get the next event
    pub async fn next(&mut self) -> Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_else(|| color_eyre::eyre::eyre!("Event channel closed"))
    }
}
