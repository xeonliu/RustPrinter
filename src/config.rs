use std::{
    error::Error,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use reqwest::{cookie, header::HeaderValue};

const APP_NAME: &str = "RupmPrinter";

const TEMP_FOLDER: &str = "temp";

pub fn ensure_directory(dir: &str) {
    let metadata = fs::metadata(dir);
    if metadata.is_err() {
        _ = fs::create_dir_all(dir);
    }
}

fn portable() -> bool {
    let exe_dir = std::env::current_exe()
        .map(|path| path.parent().unwrap().to_owned())
        .ok();

    if let Some(dir) = exe_dir {
        let portable_file = dir.join(".config/PORTABLE");
        portable_file.exists()
    } else {
        false
    }
}

// Create one if not exists
pub fn config_dir() -> PathBuf {
    match portable() {
        true => {
            let exe_dir = std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_owned();
            exe_dir.join(".config")
        }
        _ => {
            // Create config dir if not exists
            let conf_dir = dirs::config_dir().unwrap().join(APP_NAME);
            ensure_directory(conf_dir.to_str().unwrap());
            conf_dir
        }
    }
}

pub fn temp_dir() -> PathBuf {
    let temp_dir = config_dir().join(TEMP_FOLDER);
    ensure_directory(temp_dir.to_str().unwrap());
    temp_dir
}

fn cookie_path() -> PathBuf {
    config_dir().join("cookie.txt")
}

pub fn save_cookie(cookie: HeaderValue) {
    let mut file = File::create(cookie_path()).expect("Unable to open file");
    file.write_all(cookie.as_bytes())
        .expect("Unable to write data");
    println!("{:?}", cookie);
}

pub fn load_cookie() -> Option<String> {
    if let Ok(mut file) = File::open(cookie_path()) {
        let mut cookie_string = String::new();
        file.read_to_string(&mut cookie_string)
            .expect("Unable to read data");
        return Some(cookie_string);
    }

    File::create(cookie_path()).expect("Unable to create file");
    None
}