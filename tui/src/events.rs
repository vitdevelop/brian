use crossterm::event::{EventStream, KeyEvent, KeyEventKind};
use futures::StreamExt;
use futures::FutureExt;
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender, WeakUnboundedSender};
use crate::events::Event::{KEY, PASS, RENDER};

use crate::screen::base::Screen;

#[derive(Debug)]
pub struct Events {
    screen_sender: UnboundedSender<Box<dyn Screen>>,
    screen_receiver: UnboundedReceiver<Box<dyn Screen>>,
    key_events: EventStream,
}

#[derive(Debug)]
pub enum Event {
    RENDER(Box<dyn Screen>),
    KEY(KeyEvent),
    PASS,
}

impl Events {
    pub fn new() -> Events {
        let (sender, receiver) = unbounded_channel::<Box<dyn Screen>>();
        let event_stream = EventStream::new();

        Events { screen_sender: sender, screen_receiver: receiver, key_events: event_stream }
    }

    pub fn get_screen_sender(&self) -> WeakUnboundedSender<Box<dyn Screen>> {
        self.screen_sender.downgrade()
    }

    pub async fn read(&mut self) -> color_eyre::Result<Event> {
        let event = self.key_events.next().fuse();
        select! {
            event = event => {
                let event = match event {
                    None => { return Ok(PASS); }
                    Some(event) => { event? }
                };

                match event {
                    crossterm::event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        Ok(KEY(key_event))
                    }
                    _ => Ok(PASS)
                }
            }

            event = self.screen_receiver.recv() => {
                match event {
                    None => Ok(PASS),
                    Some(screen) => Ok(RENDER(screen)),
                }
            }
        }
    }
}
