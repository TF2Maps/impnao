use serde_json;
use clockwork::Modules;
use clockwork::routes::{Routes, UriParams, BodyParams};
use clockwork::routes::RouteResult::{self, Raw};
use models::MapEntry;
use modules::Maps;

pub fn register(routes: &mut Routes) {
    routes.get("/impnao/api/maps", get);
    routes.post("/impnao/api/maps", add);
    routes.delete("/impnao/api/maps/:name/remove", remove);
}

fn get(modules: &Modules, _: UriParams, _: BodyParams) -> RouteResult {
    let maps: &Maps = modules.get().unwrap();

    let data = maps.all(); // Keep in mind, passwords are exposed, they're not meant to be secure

    Raw(serde_json::to_string(&data).unwrap().into())
}

fn add(modules: &Modules, _: UriParams, body: BodyParams) -> RouteResult {
    let maps: &Maps = modules.get().unwrap();

    let text = body.as_text().unwrap();
    let map: MapEntry = serde_json::from_str(text).unwrap();
    maps.add(map);

    Raw("".into())
}

fn remove(modules: &Modules, uri: UriParams, _: BodyParams) -> RouteResult {
    let maps: &Maps = modules.get().unwrap();

    maps.remove(&uri.get("name").unwrap());

    Raw("".into())
}
