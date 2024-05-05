use std::fmt::Debug;

use enum_dispatch::enum_dispatch;
use ratatui::Frame;

use crate::events::Event;
use crate::screen::counter::CounterScreen;
use crate::screen::main::MainScreen;

#[derive(Debug, Clone)]
#[enum_dispatch(ScreenTrait)]
pub enum Screen {
    Main(MainScreen),
    Counter(CounterScreen),
}

#[enum_dispatch]
pub trait ScreenTrait {
    async fn handle_event(&mut self, _: Event) -> color_eyre::Result<()> {
        Ok(())
    }

    fn get_previous_screen(&mut self) -> Option<Box<Screen>> {
        None
    }

    fn render_widget(&self, frame: &mut Frame);
}