use eframe::egui::{Button, CursorIcon, Response, Ui};

use crate::app::AudioPlayer;
use crate::file::file_system::get_files_from_dir;

use super::layouts::vertical_align; 
use super::text_utils::render_big_text;

fn render_button_reg(ui: &mut Ui, text: &str) -> Response {
    let btn_text = render_big_text(text);
    ui
        .add_sized(
            [100., 30.],
            Button::new(btn_text)
        )
        .on_hover_cursor(CursorIcon::PointingHand)
}

pub fn render_stream_buttons(ui: &mut Ui) -> (Response, Response, Response) {
    (
        render_button_reg(ui, "Play!"),
        render_button_reg(ui, "Pause"),
        render_button_reg(ui, "Stop"),
    )
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