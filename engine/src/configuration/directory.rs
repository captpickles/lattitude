use crate::configuration::{Configuration, ConfigurationLoader};
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;
use toml::Table;

pub struct DirectoryConfigurationLoader<P: AsRef<Path>> {
    base: P,
    watched: HashMap<String, SystemTime>,
}

impl<P: AsRef<Path>> DirectoryConfigurationLoader<P> {
    pub fn new(base: P) -> Self {
        Self {
            base,
            watched: Default::default(),
        }
    }
}

impl<P: AsRef<Path>> ConfigurationLoader for DirectoryConfigurationLoader<P> {
    async fn load(&mut self) -> Vec<Configuration> {
        let mut loaded = Vec::new();
        if let Ok(paths) = read_dir(&self.base) {
            'files: for path in paths.flatten() {
                if let Some(ext) = path.path().extension() {
                    if ext == ".toml" {
                        if let Some(key) = path.path().file_stem() {
                            let key = key.to_str().unwrap().to_string();
                            if let Ok(mut file) = File::open(path.path()) {
                                let file_mtime = file
                                    .metadata()
                                    .map(|inner| inner.modified().ok())
                                    .ok()
                                    .flatten();
                                let last_mtime = self.watched.get(&key).cloned();

                                match (file_mtime, last_mtime) {
                                    (Some(file), Some(last)) => {
                                        if file == last {
                                            continue 'files;
                                        }
                                    }
                                    _ => {
                                        todo!()
                                    }
                                }

                                let mut payload = String::new();
                                if file.read_to_string(&mut payload).is_ok() {
                                    match payload.parse::<Table>() {
                                        Ok(payload) => {
                                            loaded.push(Configuration {
                                                key: key.clone(),
                                                content: payload.into(),
                                            });
                                            self.watched.insert(key, file_mtime.unwrap());
                                        }
                                        Err(_) => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        loaded
    }
}
