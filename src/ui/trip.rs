use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;

use crate::ui::{App, get_generic_block, UIBlock};

pub fn build_trip<B>(app: &App, frame: &mut Frame<B>, area: Rect)
    where B: Backend
{
    let block = get_generic_block(app, UIBlock::TRIP, Some("Trip"));
    frame.render_widget(block, area);
}
