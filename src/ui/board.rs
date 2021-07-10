use crate::ui::{App, UIBlock, get_generic_block};
use tui::Frame;
use tui::layout::Rect;
use tui::backend::Backend;

pub fn build_board<B>(app: &App, frame: &mut Frame<B>, area: Rect)
    where B: Backend
{
    let block = get_generic_block(app, UIBlock::BOARD, Some("Departures"));
    frame.render_widget(block, area);
}
