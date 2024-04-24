use color_eyre::Result;
use tokio::task::LocalSet;

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

    let local = LocalSet::new();
    local.run_until(App::new().run(&mut terminal)).await?;

    tui::restore()?;

    Ok(())
}
