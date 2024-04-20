use color_eyre::Result;

use crate::app::App;

mod tui;
mod errors;
mod screen;
mod app;
mod events;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App::new().run(&mut terminal).await?;
    tui::restore()?;

    Ok(())
}
