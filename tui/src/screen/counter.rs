use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::prelude::{Line, Stylize, Text};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::widgets::block::{Position, Title};
use tokio::sync::mpsc::WeakUnboundedSender;

use crate::events::Event;
use crate::events::Event::STATE;
use crate::screen::base::{Screen, ScreenTrait};

#[derive(Debug, Clone)]
struct State {
    counter: i64,
}

#[derive(Debug, Clone)]
pub struct CounterScreen {
    event_sender: WeakUnboundedSender<Event>,
    previous_screen: Option<Box<Screen>>,
    state: State,
}

impl CounterScreen {
    pub fn new(event_sender: WeakUnboundedSender<Event>, previous_screen: Option<Box<Screen>>) -> Screen {
        Screen::Counter(CounterScreen { event_sender, previous_screen, state: State { counter: 0 } })
    }

    fn increment_counter(&mut self) -> color_eyre::Result<()> {
        self.state.counter += 1;
        Ok(())
    }

    fn decrement_counter(&mut self) -> color_eyre::Result<()> {
        self.state.counter -= 1;
        Ok(())
    }

    fn increment_async(&self) -> color_eyre::Result<()> {
        let mut state = self.state.clone();
        let event_sender = self.event_sender.clone();
        tokio::task::spawn_local(async move {
            state.counter += 2;
            match event_sender.upgrade() {
                None => {}
                Some(sender) => { let _ = sender.send(STATE); }
            }
        });
        Ok(())
    }
}

impl ScreenTrait for CounterScreen {
    fn handle_event(&mut self, event: Event) -> color_eyre::Result<()> {
        match event {
            Event::KEY(key) => {
                match key.code {
                    KeyCode::Left => { self.decrement_counter()? }
                    KeyCode::Right => { self.increment_counter()? }
                    KeyCode::Char('a') => { self.increment_async()? }
                    _ => {}
                };
            }
            _ => {}
        };

        Ok(())
    }

    fn get_previous_screen(&mut self) -> Option<Box<Screen>> {
        self.previous_screen.clone()
    }

    fn render_widget(&self, frame: &mut Frame) {
        let title = Title::from(Line::from(" Counter App ").bold());
        let instructions = Title::from(Line::from(vec![
            " <Left>".yellow().into(),
            " Decrement".into(),
            " <Right>".yellow().into(),
            " Increment".into(),
            " <a>".yellow().into(),
            " Async Increment".into(),
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
            self.state.counter.to_string().yellow(),
        ]));

        let paragraph = Paragraph::new(counter_text)
            .centered()
            .block(block);

        frame.render_widget(paragraph, frame.size())
    }
}