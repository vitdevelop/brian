use std::default::Default;
use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::prelude::{Line, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::block::{Position, Title};
use tokio::sync::mpsc::WeakUnboundedSender;

use crate::events::Event;
use crate::events::Event::RENDER;
use crate::screen::base::Screen::Main;
use crate::screen::counter::CounterScreen;

use super::base::{Screen, ScreenTrait};

#[derive(Debug, Clone)]
pub struct MainScreen {
    event_sender: WeakUnboundedSender<Event>,
}

impl MainScreen {
    pub fn new(event_sender: WeakUnboundedSender<Event>,
           _: Option<Screen>) -> Screen {
        Main(MainScreen { event_sender })
    }
}

impl ScreenTrait for MainScreen {
    async fn handle_event(&mut self, event: Event) -> color_eyre::Result<()> {
        match event {
            Event::KEY(key) => {
                match key.code {
                    KeyCode::Char('d') => {
                        let current_screen = Box::new(Main(self.clone()));
                        self.send_screen_render_event(CounterScreen::new(self.event_sender.clone(), Some(current_screen)))?
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_previous_screen(&mut self) -> Option<Box<Screen>> {
        None
    }

    fn render_widget(&self, frame: &mut Frame) {
        let title = Title::from(Line::from(" Brian ").bold());
        let instructions = Title::from(Line::from(vec![
            " <q>".yellow().into(),
            " quit".into(),
            " |".into(),
            " <d>".yellow().into(),
            " counter ".into(),
        ]));
        let block = Block::default()
            .title(title
                .alignment(Alignment::Center))
            .title(instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom))
            .borders(Borders::ALL)
            .border_set(border::THICK);
        frame.render_widget(block, frame.size())
    }
}

impl MainScreen {
    fn send_screen_render_event(&self, screen: Screen) -> color_eyre::Result<()> {
        match self.event_sender.upgrade() {
            None => {}
            Some(sender) => {
                sender.send(RENDER(Box::new(screen)))
                    .expect("unable to create counter screen");
            }
        }
        Ok(())
    }
}