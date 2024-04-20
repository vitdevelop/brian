use std::fmt::{Debug, Display, Formatter};

use crossterm::event::KeyEvent;
use ratatui::Frame;
use tokio::sync::mpsc::WeakUnboundedSender;

pub trait Screen: ScreenClone {
    fn new(screen_sender: WeakUnboundedSender<Box<dyn Screen>>,
           previous_screen: Option<Box<dyn Screen>>) -> impl Screen where Self: Sized;

    fn handle_key_event(&mut self, _: KeyEvent) -> color_eyre::Result<()> {
        Ok(())
    }

    fn get_previous_screen(&mut self) -> Option<Box<dyn Screen>> {
        None
    }

    fn render_widget(&self, frame: &mut Frame);
}

impl Debug for dyn Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for dyn Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Screen{{{}}}", self)
    }
}

pub trait ScreenClone {
    fn clone_box(&self) -> Box<dyn Screen>;
}

impl<T> ScreenClone for T
    where
        T: 'static + Screen + Clone,
{
    fn clone_box(&self) -> Box<dyn Screen> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Screen> {
    fn clone(&self) -> Box<dyn Screen> {
        self.clone_box()
    }
}
