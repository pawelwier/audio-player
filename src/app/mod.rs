use std::thread;
use std::fs::File; 
use std::io::BufReader;
use std::sync::Arc;
use std::sync::Mutex;

use eframe::egui::{CentralPanel, Context};
use eframe::{App, CreationContext, Frame};

use rodio::{Decoder, Sink};

use crate::file::file_system::read_file; 
use crate::ui::render_elements::{render_file_options, render_stream_buttons};

struct AudioStream {
    sink: Sink
}

pub struct AudioPlayer {
    stream: Arc<Mutex<AudioStream>>,
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

            // TODO: refactor and move logic out
            let (
                play_button, pause_button, stop_button
            ) = render_stream_buttons(ui);
            if play_button.clicked() {
                self.play_data(self.audio_path.to_owned());
            }
            if pause_button.clicked() {
                self.pause_data();
            }
            if stop_button.clicked() {
                self.stop_data();
            }

            render_file_options(ui, self);
        });
    }
}

impl AudioPlayer {
    pub fn new(_cc: &CreationContext<'_>, sink: Sink) -> Self {
        AudioPlayer { 
            stream: Arc::new(Mutex::new(AudioStream { sink })),
            audio_path: "".to_owned()
        }
    }

    fn get_local_stream(&mut self) -> Arc<Mutex<AudioStream>> {
        self.stream.clone()
    }

    pub fn play_data(&mut self, path: String) -> () {
        let data_result: Result<BufReader<File>, std::io::Error> = read_file(&path);

        if let Ok(file) = data_result {
            let source = Decoder::new(file).unwrap();
            let local_stream = self.get_local_stream();

            thread::spawn(move || {
                let _ = &local_stream.lock().unwrap().sink.append(source);
            });
        }
    }

    pub fn pause_data(&mut self) -> () {
        // TODO: fix
        // self.get_local_stream().lock().unwrap().sink.pause();
    }

    pub fn stop_data(&mut self) -> () {
        self.get_local_stream().lock().unwrap().sink.stop();
    }
}