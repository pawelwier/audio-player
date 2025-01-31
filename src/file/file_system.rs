use std::{
    fs::File,
    io::BufReader
};
use rodio::{Decoder, OutputStream, source::Source};

fn get_file_from_path(path: &str) -> std::io::Result<File> {
    File::open(path)
}

fn read_file(path: &str) -> Result<BufReader<File>, std::io::Error> {
    let file_result: Result<File, std::io::Error> = get_file_from_path(path);

    if let Ok(file) = file_result {
        Ok(BufReader::new(file))
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Error reading file"))
    }
}

pub fn play_data(path: &str) -> () {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let data_result: Result<BufReader<File>, std::io::Error> = read_file(path);

    if let Ok(file) = data_result {
        let source = Decoder::new(file).unwrap();
        let _ = stream_handle.play_raw(source.convert_samples());
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}