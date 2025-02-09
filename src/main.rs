mod audio;
mod file;
mod ui;
mod app;

use app::AudioPlayer;
use eframe::{
    egui::ViewportBuilder, 
    NativeOptions,
    run_native
};
use file::file_system::get_files_from_dir;

fn main() {
    get_files_from_dir("public");

    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([
            500.0, 500.0
        ]),
        centered: true,
        ..NativeOptions::default()
    };

    let _ = run_native(
        "Audio Player",
        options,
        Box::new(
            |cc| Ok(Box::new(AudioPlayer::new(cc)))
        )
    );
}
