use std::thread;
use std::fs::File; 
use std::io::BufReader;
use std::sync::Arc;
use std::sync::Mutex;

use eframe::egui::Response;
use eframe::egui::Sense;
use eframe::egui::{CentralPanel, Context};
use eframe::{App, CreationContext, Frame};

use rodio::{Decoder, Sink};

use crate::audio::AudioState;
use crate::file::file_system::read_file; 
use crate::ui::render_elements::{render_file_options, render_stream_buttons};

struct AudioStream {
    sink: Sink
}

pub struct AudioPlayer {
    stream: Arc<Mutex<AudioStream>>,
    // TODO: join?
    pub base_path: String,
    pub audio_path: String,
    pub audio_state: AudioState
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
            ) = render_stream_buttons(ui, &self.audio_state);
            // TODO: make btns active only when not selected
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
            audio_path: "".to_owned(),
            // TODO: get path
            base_path: "public".to_owned(),
            audio_state: AudioState::NotSelected
        }
    }

    fn get_local_stream(&mut self) -> Arc<Mutex<AudioStream>> {
        self.stream.clone()
    }

    pub fn set_audio_state(&mut self, state: AudioState) -> () {
        self.audio_state = state;
    }

    pub fn set_audio_state_on_option_change(&mut self, option: Response) -> () {
        let option_response = option.interact(Sense::click());
        if option_response.clicked() {
            // TODO: make 'stop' only initially; 
            // when audio playing, do not stop on select, stop and play on select -> play
            self.stop_data();
        }
    }

    pub fn play_data(&mut self, path: String) -> () {
        let data_result: Result<BufReader<File>, std::io::Error> = read_file(&path);

        if let Ok(file) = data_result {
            let source = Decoder::new(file).unwrap();
            let local_stream = self.get_local_stream();

            thread::spawn(move || {
                let _ = &local_stream.lock().unwrap().sink.append(source);
            });

            self.set_audio_state(AudioState::Play);
        }
    }

    pub fn pause_data(&mut self) -> () {
        self.get_local_stream().lock().unwrap().sink.pause();
        self.set_audio_state(AudioState::Pause);
    }
    
    pub fn stop_data(&mut self) -> () {
        self.get_local_stream().lock().unwrap().sink.stop();
        self.set_audio_state(AudioState::Stop);
    }
}