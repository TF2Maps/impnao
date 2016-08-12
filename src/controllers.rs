use clockwork::Modules;
use clockwork::routes::{self, Routes, UriParams, BodyParams, RouteModel};
use clockwork_handlebars::ViewRenderer;
use models::{HomeModel, MapEntry};
use modules::Maps;

pub fn register(routes: &mut Routes) {
    routes.get("/", home_get);
    routes.post("/", routes::model_handler(home_post));
}

fn home_get(modules: &Modules, _: UriParams, _: BodyParams) -> Vec<u8> {
    render_home(modules)
}

fn home_post(modules: &Modules, model: PostModel) -> Vec<u8> {
    // Add the new map
    let maps: &Maps = modules.get().unwrap();
    maps.add(MapEntry { name: model.name, link: model.link });

    render_home(modules)
}

fn render_home(modules: &Modules) -> Vec<u8> {
    // TODO: This function should probably be done with a redirect to get once that's implemented
    let views: &ViewRenderer = modules.get().unwrap();
    let maps: &Maps = modules.get().unwrap();

    let maps = HomeModel {
        maps: maps.all()
    };

    views.render("home", &maps).into()
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
