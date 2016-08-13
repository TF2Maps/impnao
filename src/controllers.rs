use clockwork::Modules;
use clockwork::routes::{self, Routes, UriParams, BodyParams};
use clockwork::routes::RouteResult::{self, Html, Redirect};
use clockwork_handlebars::ViewRenderer;
use models::{HomeModel, MapEntry, MapRemoveModel};
use modules::Maps;

pub fn register(routes: &mut Routes) {
    routes.get("/impnao", home);
    routes.post("/impnao/add", routes::model_handler(add));
    routes.post("/impnao/remove", routes::model_handler(remove));
}

fn home(modules: &Modules, _: UriParams, _: BodyParams) -> RouteResult {
    let views: &ViewRenderer = modules.get().unwrap();
    let maps: &Maps = modules.get().unwrap();

    let maps = HomeModel {
        maps: maps.all()
    };

    Html(views.render("home", &maps))
}

fn add(modules: &Modules, model: MapEntry) -> RouteResult {
    let maps: &Maps = modules.get().unwrap();

    // Add the new map
    maps.add(model);

    Redirect("/impnao".into())
}

fn remove(modules: &Modules, model: MapRemoveModel) -> RouteResult {
    let maps: &Maps = modules.get().unwrap();

    // Remove the map
    maps.remove_with_password(&model.password);

    Redirect("/impnao".into())
}
