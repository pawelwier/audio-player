use eframe::egui::{Button, CursorIcon, Response, Ui};

use crate::app::AudioPlayer;
use crate::file::file_system::get_files_from_dir;

use super::layouts::vertical_align; 
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

// TODO: get path
// TODO: filter .mp3 files
pub fn render_file_options(ui: &mut Ui, app: &mut AudioPlayer) -> () {
    ui.with_layout(vertical_align(), |ui| {
        let base_path = "public";
        if let Ok(files) = get_files_from_dir(base_path) {
            for file in files {
                let file_name = file.file_name().to_str().unwrap().to_owned();
                let file_path = [base_path, &file_name.clone()].join("/");
                ui.selectable_value(&mut app.audio_path, file_path, file_name);
            }
        }
    });
}