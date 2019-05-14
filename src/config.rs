use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use bincode::{deserialize_from, serialize_into};
use chrono::{DateTime, Utc};

use crate::data::{CommitLog, Data};

pub struct ConfigFile {
    data: Data,
    file: File,
}

impl ConfigFile {
    pub fn new(file: String) -> Self {
        let path = Path::new(&file);
        let open_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path);

        let file = match open_file {
            Ok(f) => f,
            Err(_) => {
                File::create(path).expect("Config file not creatable")
            }
        };

        let reader = BufReader::new(&file);

        let data = match deserialize_from(reader) {
            Err(_) => {
                Data::new()
            }
            Ok(e) => e
        };

        println!("Data is {:?}", &data);

        ConfigFile { file, data }
    }

    pub fn save(&mut self) {
        let mut f = BufWriter::new(&self.file);
        serialize_into(&mut f, &self.data).unwrap();
        f.flush().unwrap();
    }

    pub fn add_log(&mut self, log: CommitLog) {
        self.data.add(log);
        self.save();
    }

    pub fn print_logs(&self) {
        self.data.print();
    }

    pub fn print_next_commit(&self) {
        let commit = self.data.get_commit(Utc::now());

        match commit {
            Some(c) => println!("{} {}", c.get_commit(), c.get_time()),
            None => println!("There is nothing to be pushed.")
        }
    }

    pub fn get_commit(&self, time: DateTime<Utc>) -> Option<&CommitLog> {
        self.data.get_commit(time)
    }

    pub fn has_logs(&self) -> bool {
        self.data.has_logs()
    }

    pub fn clear_logs(&mut self, commit: &str) {
        self.data.clear_logs(commit);
    }
}
