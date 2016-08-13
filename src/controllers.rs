use clockwork::Modules;
use clockwork::routes::{self, Routes, UriParams, BodyParams, RouteModel};
use clockwork::routes::RouteResult::{self, Html, Redirect};
use clockwork_handlebars::ViewRenderer;
use models::{HomeModel, MapEntry};
use modules::Maps;

pub fn register(routes: &mut Routes) {
    routes.get("/impnao", home);
    routes.post("/impnao/add", routes::model_handler(add));
}

fn home(modules: &Modules, _: UriParams, _: BodyParams) -> RouteResult {
    let views: &ViewRenderer = modules.get().unwrap();
    let maps: &Maps = modules.get().unwrap();

    let maps = HomeModel {
        maps: maps.all()
    };

    Html(views.render("home", &maps))
}

fn add(modules: &Modules, model: PostModel) -> RouteResult {
    // Add the new map
    let maps: &Maps = modules.get().unwrap();
    maps.add(MapEntry { name: model.name, link: model.link });

    Redirect("/impnao".into())
}

pub struct PostModel {
    pub name: String,
    pub link: String,
}

impl RouteModel for PostModel {
    fn from(_url: UriParams, body: BodyParams) -> Self {
        let body = body.as_form();

        PostModel {
            name: body.get("name").unwrap(),
            link: body.get("link").unwrap(),
        }
    }
}
