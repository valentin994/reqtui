// TODO: file search for postman collections

use std::{error::Error, fmt, fs, path::PathBuf};

use directories::ProjectDirs;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::api::Request;

// TODO: make it possible to select collections
#[derive(Default, Debug)]
pub struct CollectionStore {
    pub collections: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<Request>,
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl CollectionStore {
    pub fn get_config_path() -> Option<PathBuf> {
        let proj = ProjectDirs::from("com", "you", "reqtui")?;
        Some(proj.config_dir().join("collections"))
    }

    pub fn load_collection_to_history(name: String) -> Result<IndexSet<Request>, Box<dyn Error>> {
        let Some(file_path) = Self::get_config_path() else {
            return Ok(IndexSet::new());
        };
        // INFO: hardcoded default for now, after adding support to multiple collections rework
        let file = fs::read_to_string(file_path.join(name))?;
        let collection: Collection = serde_json::from_str(&file)?;
        let history = collection.requests.into_iter().collect();
        Ok(history)
    }

    pub fn write_to_collection(history: IndexSet<Request>) -> Result<(), Box<dyn Error>> {
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

    pub fn add_collection(name: String) -> Result<(), Box<dyn Error>> {
        let Some(file_path) = Self::get_config_path() else {
            return Err("Could not access the base path".into());
        };
        let full_path = file_path.join(format!("{}.json", &name));
        let new_collection: Collection = Collection {
            name: name.to_string(),
            requests: vec![],
        };
        let collection_content = serde_json::to_string_pretty(&new_collection)?;
        fs::write(full_path, collection_content)?;
        Ok(())
    }

    pub fn delete_collection(name: String) -> Result<(), Box<dyn Error>> {
        let Some(file_path) = Self::get_config_path() else {
            return Err("Could not access the base path".into());
        };
        let full_path = file_path.join(format!("{}.json", &name));
        let _ = fs::remove_file(full_path);
        Ok(())
    }

    pub fn list_collections() -> Self {
        let Some(config_path) = Self::get_config_path() else {
            return Self {
                collections: vec![],
            };
        };

        if !config_path.exists()
            && let Err(_e) = fs::create_dir_all(&config_path)
        {
            return Self {
                collections: vec![],
            };
        }
        // TODO: maybe move this to startup, use serde instead of raw json
        let default_file = config_path.join("default.json");
        if !default_file.exists() {
            let default_file_content = r#"{
                "name": "Default",
                "requests": []
            }"#;

            let _ = fs::write(&default_file, default_file_content);
        }

        let mut collections = Vec::new();

        if let Ok(entries) = fs::read_dir(&config_path) {
            for entry in entries.into_iter().flatten() {
                let path = entry.path();

                if path.is_file()
                    && path.extension().is_some_and(|ext| ext == "json")
                    && let Some(name) = path.file_stem().and_then(|n| n.to_str())
                {
                    collections.push(Collection {
                        name: name.to_string(),
                        requests: vec![],
                    });
                }
            }
        }

        Self { collections }
    }
}
