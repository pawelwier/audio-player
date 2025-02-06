use std::{
    fs::{read_dir, DirEntry, File}, io::BufReader, thread, time::Duration
};
use rodio::{source::{SineWave, Source}, Decoder, OutputStream, Sink};

fn throw_io_error(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, message)
}

pub fn get_files_from_dir(path: &str) -> Result<Vec<DirEntry>, std::io::Error> {
    if let Ok (dir_result) = read_dir(path) {
        let dir_entries: Vec<DirEntry> =  dir_result.map(|el| el.unwrap()).collect();
        Ok(dir_entries)
    } else {
        Err(throw_io_error("Error reading files from directory"))
    }
}

fn get_file_from_path(path: &str) -> std::io::Result<File> {
    File::open(path)
}

fn read_file(path: &str) -> Result<BufReader<File>, std::io::Error> {
    let file_result: Result<File, std::io::Error> = get_file_from_path(path);

    if let Ok(file) = file_result {
        Ok(BufReader::new(file))
    } else {
        Err(throw_io_error("Error reading file"))
    }
}

pub fn play_data(path: String) -> () {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let data_result: Result<BufReader<File>, std::io::Error> = read_file(&path);

        if let Ok(file) = data_result {
            let source = Decoder::new(file).unwrap();
            sink.append(source);
            sink.sleep_until_end();
        }
    });
}