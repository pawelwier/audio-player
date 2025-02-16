use eframe::egui::RichText;

fn render_text(
    text: impl Into<String>,
    size: f32
) -> RichText {
    RichText::new(text).size(size)
}

/*
    pub fn render_small_text(text: impl Into<String>) -> RichText {
        render_text(text, 15.0)
}
*/

pub fn render_big_text(text: impl Into<String>) -> RichText {
    render_text(text, 30.0)
}

fn get_unit_count(units: u64) -> String {
    let units_str = units.to_string();
    if units > 9 { units_str } else { "0".to_owned() + &units_str }
}

pub fn format_time_secs(secs: u64) -> String {
    let mins: u64 = secs / 60;
    let secs_left: u64 = secs % 60;
    get_unit_count(mins).to_string() + ":" + &get_unit_count(secs_left).to_string()
}