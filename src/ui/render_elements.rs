use eframe::egui::{Button, CursorIcon, Response, Ui};
use super::text_utils::render_big_text;

pub fn render_play_button(ui: &mut Ui) -> Response {
    let text = render_big_text("Play!");
    ui
        .add_sized(
            [80., 30.],
            Button::new(text)
        )
        .on_hover_cursor(CursorIcon::PointingHand)
}