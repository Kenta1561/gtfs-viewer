use crate::ui::{get_generic_block, UIBlock, App};
use tui::Frame;
use tui::layout::Rect;
use tui::backend::Backend;

pub fn build_trip<B>(app: &App, frame: &mut Frame<B>, area: Rect)
    where B: Backend
{
    let block = get_generic_block(app, UIBlock::TRIP, Some("Trip"));
    frame.render_widget(block, area);
}
