use std::fs::DirEntry;
use std::time::Duration;

use eframe::egui::{Button, CursorIcon, ProgressBar, Response, Ui, Vec2};

use crate::app::AudioPlayer;
use crate::audio::AudioState;
use crate::file::file_system::{get_file_duration, get_files_from_dir};

use super::layouts::vertical_align; 
use super::utils::{format_time_secs, render_big_text};

// TODO: break into separate files

fn render_button_reg(ui: &mut Ui, text: &str, disabled_states: Vec<AudioState>, state: &AudioState) -> Response {
    let disabled: bool = disabled_states.contains(&state);
    let btn_text = render_big_text(text);
    ui
        .add_enabled(
            !disabled,
            Button::new(btn_text)
                .selected(disabled)
                .min_size(Vec2::from([100., 30.])
            )
        )
        .on_hover_cursor(CursorIcon::PointingHand)
}

pub fn render_stream_buttons(ui: &mut Ui, state: &AudioState) -> (Response, Response, Response) {
    (
        // TODO: refactor
        render_button_reg(ui, "Play!", vec![AudioState::NotSelected, AudioState::Play], &state),
        render_button_reg(ui, "Pause", vec![AudioState::NotSelected, AudioState::Stop, AudioState::Pause], &state),
        render_button_reg(ui, "Stop", vec![AudioState::NotSelected, AudioState::Stop], &state),
    )
}

fn get_file_path(file_name: &str, base_path: &str) -> String {
    [base_path, &file_name].join("/")
}

fn get_file_display_info(file_name: String, file_path: &String) -> String {
    let duration_option = get_file_duration(file_path);

    match duration_option {
        Some(duration) => {
            let duration_formatted = format_time_secs(duration.as_secs());
            file_name + " (" + &duration_formatted + ")"
        },
        None => "Error reading file".to_owned()
    }
}

fn render_file_option(ui: &mut Ui, app: &mut AudioPlayer, file: DirEntry) -> Response {
    let file_name = file.file_name().to_str().unwrap().to_owned();
    let file_path = get_file_path(&file_name, &app.base_path);
    let display_value = get_file_display_info(file_name, &file_path);
    ui.selectable_value(&mut app.audio_path, file_path, display_value)
}

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

pub fn render_duration_progress_bar(ui: &mut Ui, app: &mut AudioPlayer) -> Response {
    // TODO: split logic
    let duration_option: Option<Duration> = get_file_duration(&app.audio_path);
    let stream = app.stream.lock().unwrap();
    let state: &AudioState = &stream.audio_state;
    let duration_full = duration_option.unwrap().as_millis() as f32;

    match state {
        &AudioState::NotSelected | &AudioState::Stop => {
            app.file_pos_milis = 0;
        },
        AudioState::Play | AudioState::Pause => {
            app.file_pos_milis = stream.sink.get_pos().as_millis() as u64;
        }
    }
    
    ui.add(ProgressBar::new(app.file_pos_milis as f32 / duration_full))
}