// TODO: file search for postman collections
// TODO: change up the hotkeys and way of selecting request type

use std::{error::Error, fs, path::PathBuf};

use directories::ProjectDirs;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::api::Request;

// TODO: make it possible to select collections
#[derive(Debug, Default)]
pub struct CollectionStore {
    collections: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<Request>,
}

impl CollectionStore {
    fn get_config_path() -> Option<PathBuf> {
        let proj = ProjectDirs::from("com", "you", "reqtui")?;
        Some(proj.config_dir().join("collections"))
    }

    fn load_collection_to_history() -> Result<IndexSet<Request>, Box<dyn Error>> {
        let Some(file_path) = Self::get_config_path() else {
            return Ok(IndexSet::new());
        };
        // INFO: hardcoded default for now, after adding support to multiple collections rework
        let file = fs::read_to_string(file_path.join("default.json"))?;
        let collection: Collection = serde_json::from_str(&file)?;
        let history = collection.requests.into_iter().collect();
        Ok(history)
    }

    fn write_to_collection(history: IndexSet<Request>) -> Result<(), Box<dyn Error>> {
        let Some(file_path) = Self::get_config_path() else {
            return Err("could not find collection".into());
        };
        // INFO: hardcoded default for now, after adding support to multiple collections rework
        let full_path = file_path.join("default.json");
        let file = fs::read_to_string(&full_path)?;
        let mut collection: Collection = serde_json::from_str(&file)?;
        collection.requests = history.iter().cloned().collect();
        fs::write(full_path, serde_json::to_string_pretty(&collection)?)?;
        Ok(())
    }

    fn list_collections() -> Self {
        let Some(config_path) = Self::get_config_path() else {
            return Self {
                collections: vec![],
            };
        };

        if !config_path.exists() {
            if let Err(_e) = fs::create_dir_all(&config_path) {
                return Self {
                    collections: vec![],
                };
            }
        }

        let default_file = config_path.join("default.json");
        if !default_file.exists() {
            let default_file_content = r#"{
                "name": "Default",
                "requests": []
            }"#;

            let _ = fs::write(&default_file, default_file_content);
        }

        let mut collections = Vec::new();

        match fs::read_dir(&config_path) {
            Ok(entries) => {
                for entry in entries.into_iter().flatten() {
                    let path = entry.path();

                    if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                        if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                            collections.push(Collection {
                                name: name.to_string(),
                                requests: vec![],
                            });
                        }
                    }
                }
            }
            Err(_) => {}
        }

        Self { collections }
    }
}
