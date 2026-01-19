use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crate::ui::appstate::AppState;

impl AppState {
    pub fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            _ => {}
        }
    }
}