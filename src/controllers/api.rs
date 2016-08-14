use serde_json;
use clockwork::Modules;
use clockwork::routes::{Routes, UriParams, BodyParams};
use clockwork::routes::RouteResult::{self, Raw};
use models::{MapRemoveModel, MapEntry};
use modules::Maps;

pub fn register(routes: &mut Routes) {
    routes.get("/impnao/api/maps", get);
    routes.post("/impnao/api/maps", add);

    // TODO: These routes don't conform to REST
    routes.post("/impnao/api/maps/:name/remove", remove);
}

fn get(modules: &Modules, _: UriParams, _: BodyParams) -> RouteResult {
    let maps: &Maps = modules.get().unwrap();

    let mut data = maps.all();
    for map in &mut data {
        map.password = "".into(); // Don't expose this
    }

    Raw(serde_json::to_string(&data).unwrap().into())
}

fn add(modules: &Modules, _: UriParams, body: BodyParams) -> RouteResult {
    let maps: &Maps = modules.get().unwrap();

    let text = body.as_text().unwrap();
    let map: MapEntry = serde_json::from_str(text).unwrap();
    maps.add(map);

    Raw("".into())
}

fn remove(modules: &Modules, _: UriParams, body: BodyParams) -> RouteResult {
    let maps: &Maps = modules.get().unwrap();

    let text = body.as_text().unwrap();
    let remove_data: MapRemoveModel = serde_json::from_str(text).unwrap();
    maps.remove(&remove_data.name, &remove_data.password);

    Raw("".into())
}
