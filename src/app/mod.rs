use eframe::{
    egui::{
        CentralPanel, Context
    }, 
    App, CreationContext, Frame
};

use crate::{file::file_system::play_data, ui::render_elements::render_play_button};

#[derive(Default)]
pub struct AudioPlayer {}

impl App for AudioPlayer {
    fn update(
        &mut self, 
        ctx: &Context,
        frame: &mut Frame
    ) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Audio Player!");
            ui.add_space(10.);
            // TODO: move logic out
            let play_button = render_play_button(ui);
            if play_button.clicked() {
                play_data("public/beat_1.mp3");
            }
        });
    }
}

impl AudioPlayer {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

// TODO: add custom Default
// impl Default for AudioPlayer {
//     fn default() -> Self {
//         AudioPlayer {}
//     }
// }