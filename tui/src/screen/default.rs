use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::prelude::{Line, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::block::{Position, Title};
use tokio::sync::mpsc::WeakUnboundedSender;

use crate::screen::counter::Counter;

use super::base::Screen;

#[derive(Debug, Clone)]
pub struct DefaultScreen {
    screen_sender: WeakUnboundedSender<Box<dyn Screen>>,
}

impl Screen for DefaultScreen {
    fn new(screen_sender: WeakUnboundedSender<Box<dyn Screen>>,
           _: Option<Box<dyn Screen>>) -> impl Screen {
        DefaultScreen { screen_sender }
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

    fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('d') => {
                let current_screen = Box::new(self.clone());
                self.send_screen_render_event(Box::new(Counter::new(self.screen_sender.clone(), Some(current_screen))))?
            }
            _ => {}
        }
        Ok(())
    }

    fn get_previous_screen(&mut self) -> Option<Box<dyn Screen>> {
        None
    }
}

impl DefaultScreen {
    fn send_screen_render_event(&self, screen: Box<impl Screen + 'static>) -> color_eyre::Result<()> {
        match self.screen_sender.upgrade() {
            None => {}
            Some(sender) => {
                sender.send(screen)
                    .expect("unable to create counter screen");
            }
        }
        Ok(())
    }
}