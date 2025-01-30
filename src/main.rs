mod audio;
mod file;
mod ui;

use file::file_system::play_data;

fn main() {
    play_data("public/beat_1.mp3");
}
