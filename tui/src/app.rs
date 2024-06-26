use color_eyre::eyre::WrapErr;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::events::{Event, Events};
use crate::screen::base::Screen;
use crate::screen::default::DefaultScreen;
use crate::tui;

#[derive(Debug)]
pub struct App {
    screen: Box<dyn Screen>,
    events: Events,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        let events = Events::new();
        App { screen: Box::new(DefaultScreen::new(events.get_screen_sender(), None)), events, exit: false }
    }

    pub async fn run(&mut self, terminal: &mut tui::Tui) -> color_eyre::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.screen.render_widget(frame))?;
            self.handle_events().await?;
        }

        Ok(())
    }

    async fn handle_events(&mut self) -> color_eyre::Result<()> {
        let event = self.events.read().await?;

        match event {
            Event::KEY(key_event) => {
                self.handle_key_event(key_event)
                    .wrap_err_with(|| {
                        format!("handling key event failed:\n{key_event:#?}")
                    })
            }
            Event::RENDER(screen) => {
                self.screen = screen;
                Ok(())
            }
            Event::PASS => { Ok(()) }
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            // Exit application on `q` or `Q`
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit(),
            // Return to previous screen on ESC
            KeyCode::Esc => self.back(),
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    self.exit()?;
                }
                Ok(())
            }
            _ => self.screen.handle_key_event(key_event)
        }
    }

    fn back(&mut self) -> color_eyre::Result<()> {
        match self.screen.get_previous_screen() {
            None => Ok(()),
            Some(previous_screen) => {
                match self.events.get_screen_sender().upgrade() {
                    None => {
                        // something goes wrong here
                        Ok(())
                    }
                    Some(sender) => {
                        let _ = sender.send(previous_screen);
                        return Ok(());
                    }
                }
            }
        }
    }

    fn exit(&mut self) -> color_eyre::Result<()> {
        self.exit = true;

        Ok(())
    }
}