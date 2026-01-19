use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph, Widget};
use crate::ui::appstate::AppState;

impl Widget for &AppState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();

        Paragraph::new(" ")
            .block(block)
            .render(area, buf);
    }
}