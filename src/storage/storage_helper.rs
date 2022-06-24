use std::fs;
use std::io::{Read, Write};

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::storage::r#struct::storage::Storage;

const STORAGE_PATH: &str = "./files/store.json";

pub fn storage_read() -> Option<Storage> {
    match file_read::<Storage>(STORAGE_PATH) {
        Some(data) => {
            info!("Read data form store!");
            Some(data)
        }
        None => {
            error!("Storage data could not be loaded!");
            None
        }
    }
}

pub fn storage_write(data: Storage) {
    match file_write(STORAGE_PATH, data) {
        Ok(_) => { info!("Written data into store!"); }
        Err(_) => { error!("Unable to write into store!"); }
    }
}


fn file_read<Storeable: DeserializeOwned>(path: &str) -> Option<Storeable> {
    let mut file = fs::File::open(path)
        .expect("file should open read only");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    match serde_json::from_str::<Storeable>(&data) {
        Ok(d) => { Some(d) }
        Err(_) => {
            error!("Unable to parse file");
            None
        }
    }
}

fn file_write<Storeable: Serialize>(path: &str, data: Storeable) -> std::io::Result<()> {
    let mut file = fs::File::create(path).expect("Unable to open file!");
    file.write_all(serde_json::to_string_pretty(&data).unwrap().as_bytes())
}