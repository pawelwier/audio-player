use std::thread;
use std::fs::File; 
use std::io::BufReader;
use std::sync::Arc;
use std::sync::Mutex;

use eframe::egui::{CentralPanel, Context};
use eframe::{App, CreationContext, Frame};

use rodio::{Decoder, Sink};

use crate::file::file_system::read_file; 
use crate::ui::render_elements::{render_file_options, render_play_button};

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
            // TODO: move logic out
            let play_button = render_play_button(ui);
            if play_button.clicked() {
                self.play_data(self.audio_path.to_owned());
            }
            render_file_options(ui, self);
        });
    }
}

impl AudioPlayer {
    pub fn new(cc: &CreationContext<'_>, sink: Sink) -> Self {
        AudioPlayer { 
            stream: Arc::new(Mutex::new(AudioStream { sink })),
            audio_path: "".to_owned()
        }
    }

    pub fn play_data(&mut self, path: String) -> () {
        let data_result: Result<BufReader<File>, std::io::Error> = read_file(&path);

        if let Ok(file) = data_result {
            let source = Decoder::new(file).unwrap();

            let local_self = self.stream.clone();

            thread::spawn(move || {
                let sink = &local_self.lock().unwrap().sink;
                sink.append(source);
                sink.sleep_until_end();
            });
        }
    }
}