use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::Path;
use anyhow::{Error, Result};

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

        ConfigFile { file, data }
    }

    pub fn save(&mut self) -> Result<(), Error>{
        let mut f = BufWriter::new(&self.file);
        f.seek(SeekFrom::Start(0))?;
        serialize_into(&mut f, &self.data).unwrap();
        f.flush().unwrap();
        Ok(())
    }

    pub fn add_log(&mut self, log: CommitLog) -> Result<(), Error> {
        self.data.add(log);
        self.save()?;
        Ok(())
    }

    pub fn print_logs(&self) {
        self.data.print();
    }

    pub fn print_next_commit(&self) {
        let commit = self.data.get_commit(Utc::now());

        match commit {
            Some(c) => println!("{}", c),
            None => println!("There is nothing to be pushed.")
        }
    }

    pub fn get_commit(&self, time: DateTime<Utc>) -> Option<String> {
        self.data.get_commit(time)
    }

    pub fn has_logs(&self) -> bool {
        self.data.has_logs()
    }

    pub fn clear_logs(&mut self, commit: &str) -> Result<(), Error>{
        self.data.clear_logs(commit);
        self.save()?;
        Ok(())
    }
}
