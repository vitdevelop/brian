use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::prelude::{Line, Stylize, Text};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::widgets::block::{Position, Title};
use tokio::sync::mpsc::WeakUnboundedSender;
use crate::screen::base::Screen;

#[derive(Debug, Default, Clone)]
pub struct Counter {
    counter: i64,
    previous_screen: Option<Box<dyn Screen>>,
}

impl Counter {
    fn increment_counter(&mut self) -> color_eyre::Result<()> {
        self.counter += 1;
        Ok(())
    }

    fn decrement_counter(&mut self) -> color_eyre::Result<()> {
        self.counter -= 1;
        Ok(())
    }
}

impl Screen for Counter {
    fn new(_: WeakUnboundedSender<Box<dyn Screen>>, previous_screen: Option<Box<dyn Screen>>) -> impl Screen {
        Counter { counter: 0, previous_screen }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Left => { self.decrement_counter()? }
            KeyCode::Right => { self.increment_counter()? }
            _ => {}
        };

        Ok(())
    }

    fn get_previous_screen(&mut self) -> Option<Box<dyn Screen>> {
        self.previous_screen.clone()
    }

    fn render_widget(&self, frame: &mut Frame) {
        let title = Title::from(Line::from(" Counter App ").bold());
        let instructions = Title::from(Line::from(vec![
            " <Left>".yellow().into(),
            " Decrement".into(),
            " <Right>".yellow().into(),
            " Increment".into(),
            " <q>".yellow().into(),
            " Quit".into(),
        ]));
        let block = Block::default()
            .title(title
                .alignment(Alignment::Center))
            .title(instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom))
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(Line::from(vec![
            "Counter: ".bold(),
            self.counter.to_string().yellow(),
        ]));

        let paragraph = Paragraph::new(counter_text)
            .centered()
            .block(block);

        frame.render_widget(paragraph, frame.size())
    }
}