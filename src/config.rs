use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

use bincode::{deserialize, serialize_into};

use crate::data::{CommitLog, Data};

pub struct ConfigFile {
    data: Data,
    file: File,
}

impl ConfigFile {
    pub fn new(file: String) -> Self {
        let path = Path::new(&file);
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => File::create(path).expect("Config file not creatable"),
        };

        file.sync_data().unwrap();

        let mut buf = vec![];
        file.read(&mut buf).unwrap();

        let data: Data = match buf.len() {
            0 => Data::new(),
            _ => deserialize(&buf).unwrap(),
        };

        ConfigFile { file, data }
    }

    pub fn save(&mut self) {
        let mut f = BufWriter::new(&self.file);
        serialize_into(&mut f, &self.data).unwrap();
        self.file.sync_data().unwrap();
    }

    pub fn add_log(&mut self, log: CommitLog) {
        self.data.add(log);
        self.save();
    }
}
