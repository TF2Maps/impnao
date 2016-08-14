use std::sync::RwLock;
use clockwork::Module;
use models::MapEntry;

pub struct Maps {
    maps: RwLock<Vec<MapEntry>>
}

impl Module for Maps {}

impl Maps {
    pub fn new() -> Self {
        Maps {
            maps: RwLock::new(Vec::new())
        }
    }

    pub fn add(&self, entry: MapEntry) {
        // Don't add empty maps
        // TODO: Add form validation
        if entry.name == "" || entry.link == ""  {
            return;
        }

        let mut maps = self.maps.write().unwrap();
        maps.push(entry);
    }

    pub fn all(&self) -> Vec<MapEntry> {
        let maps = self.maps.read().unwrap();
        maps.clone()
    }

    pub fn remove(&self, name: &str, password: &str) {
        if password == "" {
            return; // Because there are lazy people
        }

        let mut maps = self.maps.write().unwrap();
        maps.retain(|m| !(m.name == name && m.password == password));
    }
}
