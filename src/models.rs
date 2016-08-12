#[derive(Serialize)]
pub struct HomeModel {
    pub maps: Vec<MapEntry>
}

#[derive(Serialize, Clone)]
pub struct MapEntry {
    pub name: String,
    pub link: String,
}
