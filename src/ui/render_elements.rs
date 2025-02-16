use std::fs::DirEntry;

use eframe::egui::{Button, CursorIcon, Response, Ui};

use crate::app::AudioPlayer;
use crate::audio::AudioState;
use crate::file::file_system::get_files_from_dir;

use super::layouts::vertical_align; 
use super::text_utils::render_big_text;

fn render_button_reg(ui: &mut Ui, text: &str, disabled_states: Vec<AudioState>, state: &AudioState) -> Response {
    let selected: bool = disabled_states.contains(&state);
    let btn_text = render_big_text(text);
    ui
        .add_sized(
            [100., 30.],
            Button::new(btn_text).selected(selected)
        )
        .on_hover_cursor(CursorIcon::PointingHand)
}

pub fn render_stream_buttons(ui: &mut Ui, state: &AudioState) -> (Response, Response, Response) {
    println!("{:?}", &state);
    (
        // TODO: refactor
        render_button_reg(ui, "Play!", vec![AudioState::NotSelected, AudioState::Play], &state),
        render_button_reg(ui, "Pause", vec![AudioState::NotSelected, AudioState::Pause], &state),
        render_button_reg(ui, "Stop", vec![AudioState::NotSelected, AudioState::Stop], &state),
    )
}

fn get_file_path(file_name: &str, base_path: &str) -> String {
    [base_path, &file_name].join("/")
}

fn render_file_option(ui: &mut Ui, app: &mut AudioPlayer, file: DirEntry) -> Response {
    let file_name = file.file_name().to_str().unwrap().to_owned();
    let file_path = get_file_path(&file_name, &app.base_path);
    ui.selectable_value(&mut app.audio_path, file_path, file_name)
}

// TODO: filter .mp3 files
pub fn render_file_options(ui: &mut Ui, app: &mut AudioPlayer) -> () {
    ui.with_layout(vertical_align(), |ui| {
        if let Ok(files) = get_files_from_dir(&app.base_path) {
            for file in files {
                let option = render_file_option(ui, app, file);
                app.set_audio_state_on_option_change(option);
            }
        }
    });
}