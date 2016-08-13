use clockwork::routes::{UriParams, BodyParams, RouteModel};

#[derive(Serialize)]
pub struct HomeModel {
    pub maps: Vec<MapEntry>
}

#[derive(Serialize, Clone)]
pub struct MapEntry {
    pub name: String,
    pub link: String,
    pub password: String,
}

impl RouteModel for MapEntry {
    fn from(_url: UriParams, body: BodyParams) -> Self {
        let body = body.as_form();

        MapEntry {
            name: body.get("name").unwrap(),
            link: body.get("link").unwrap(),
            password: body.get("password").unwrap(),
        }
    }
}

pub struct MapRemoveModel {
    pub password: String,
}

impl RouteModel for MapRemoveModel {
    fn from(_url: UriParams, body: BodyParams) -> Self {
        let body = body.as_form();

        MapRemoveModel {
            password: body.get("password").unwrap(),
        }
    }
}
