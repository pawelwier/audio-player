use std::fs::{read_dir, DirEntry, File};
use std::io::BufReader;
use std::time::Duration;

use rodio::{Decoder, Source};

fn throw_io_error(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, message)
}

fn is_mp3_file(file: &DirEntry) -> bool {
    match std::path::Path::new(&file.file_name()).extension()
    {
        Some(extension) => extension.to_str().unwrap() == "mp3",
        None => false
    }
}

pub fn get_files_from_dir(path: &str) -> Result<Vec<DirEntry>, std::io::Error> {
    if let Ok (dir_result) = read_dir(path) {
        let dir_entries: Vec<DirEntry> =  dir_result
            .map(|el| el.unwrap())
            .filter(|el| { is_mp3_file(el) })
            .collect();
        
        Ok(dir_entries)
    } else {
        Err(throw_io_error("Error reading files from directory"))
    }
}

fn get_file_from_path(path: &str) -> std::io::Result<File> {
    File::open(path)
}

pub fn read_file(path: &str) -> Result<BufReader<File>, std::io::Error> {
    let file_result: Result<File, std::io::Error> = get_file_from_path(path);

    if let Ok(file) = file_result {
        Ok(BufReader::new(file))
    } else {
        Err(throw_io_error("Error reading file"))
    }
}

pub fn get_file_duration(path: &String) -> Option<Duration> {
    if let Ok(source) = read_audio_file(path) {
        source.total_duration()
    } else {
        Some(Duration::ZERO)
    }
}

pub fn read_audio_file(path: &String) -> Result<Decoder<BufReader<File>>, std::io::Error> {
    let data_result: Result<BufReader<File>, std::io::Error> = read_file(&path);
    
    match data_result {
        Ok(file) => {
            if let Ok(source) = Decoder::new(file) {
                Ok(source)
            } else {
                Err(throw_io_error("Error decoding file, please check file format"))
            }  
        },
        Err(_) => { 
            Err(throw_io_error("Error reading file"))
         }
    }
}