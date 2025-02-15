mod audio;
mod file;
mod ui;
mod app;

use eframe::egui::ViewportBuilder; 
use eframe::{NativeOptions,run_native};
use rodio::{OutputStream, Sink};
use app::AudioPlayer;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

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
            |cc| Ok(Box::new(AudioPlayer::new(cc, sink)))
        )
    );
}
