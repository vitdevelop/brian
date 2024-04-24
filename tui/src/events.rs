use crossterm::event::{EventStream, KeyEvent, KeyEventKind};
use futures::FutureExt;
use futures::StreamExt;
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender, WeakUnboundedSender};

use crate::events::Event::{KEY, PASS};
use crate::screen::base::Screen;

#[derive(Debug)]
pub struct Events {
    event_sender: UnboundedSender<Event>,
    event_receiver: UnboundedReceiver<Event>,

    key_events: EventStream,
}

#[derive(Debug)]
pub enum Event {
    RENDER(Box<Screen>),
    KEY(KeyEvent),
    STATE,
    PASS,
}

impl Events {
    pub fn new() -> Events {
        let (sender, receiver) =
            unbounded_channel::<Event>();
        let event_stream = EventStream::new();

        Events { event_sender: sender, event_receiver: receiver, key_events: event_stream }
    }

    pub fn get_event_sender(&self) -> WeakUnboundedSender<Event> {
        self.event_sender.downgrade()
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

            event = self.event_receiver.recv() => {
                match event {
                    None => Ok(PASS),
                    Some(event) => Ok(event),
                }
            }
        }
    }
}