use crate::ui::appstate::AppState;

mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    AppState::default().run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
