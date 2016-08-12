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
        let mut maps = self.maps.write().unwrap();
        maps.push(entry);
    }

    pub fn all(&self) -> Vec<MapEntry> {
        let maps = self.maps.read().unwrap();
        maps.clone()
    }
}
