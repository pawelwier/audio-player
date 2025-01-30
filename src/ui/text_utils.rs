use eframe::egui::RichText;

fn render_text(
    text: impl Into<String>,
    size: f32
) -> RichText {
    RichText::new(text).size(size)
}

pub fn render_small_text(text: impl Into<String>) -> RichText {
    render_text(text, 15.0)
}

pub fn render_big_text(text: impl Into<String>) -> RichText {
    render_text(text, 30.0)
}