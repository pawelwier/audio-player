use eframe::{
    egui::{
        CentralPanel, Context
    }, 
    App, CreationContext, Frame
};

use crate::{
    file::file_system::play_data, 
    ui::render_elements::{render_file_options, render_play_button}
};

pub struct AudioPlayer {
    pub audio_path: String
}

impl App for AudioPlayer {
    fn update(
        &mut self, 
        ctx: &Context,
        _frame: &mut Frame
    ) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Audio Player!");
            ui.add_space(10.);
            // TODO: move logic out
            let play_button = render_play_button(ui);
            if play_button.clicked() {
                play_data(self.audio_path.to_owned());
            }
            render_file_options(ui, self);
        });
    }
}

impl AudioPlayer {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        AudioPlayer {
            audio_path: "".to_owned()
        }
    }
}