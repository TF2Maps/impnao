use serde_json;
use clockwork::Modules;
use clockwork::routes::{Routes, UriParams, BodyParams};
use clockwork::routes::RouteResult::{self, Raw};
use models::MapRemoveModel;
use modules::Maps;

pub fn register(routes: &mut Routes) {
    // TODO: Turn this into a REST API
    routes.get("/impnao/api/maps", get);
    routes.post("/impnao/api/maps/remove", remove);
}

fn get(modules: &Modules, _: UriParams, _: BodyParams) -> RouteResult {
    let maps: &Maps = modules.get().unwrap();

    let data = maps.all();

    Raw(serde_json::to_string(&data).unwrap().into())
}

fn remove(modules: &Modules, _: UriParams, body: BodyParams) -> RouteResult {
    let maps: &Maps = modules.get().unwrap();

    let text = body.as_text().unwrap();
    let remove_data: MapRemoveModel = serde_json::from_str(text).unwrap();
    maps.remove(&remove_data.name, &remove_data.password);

    Raw("".into())
}
