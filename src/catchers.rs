use rocket::{catchers, catch, Catcher, Request };
use rocket_dyn_templates::Template;
use serde_json::json;


pub fn catchers() -> Vec<Catcher> {
    catchers![not_found]
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        json! ({
            "uri": req.uri()
        }),
    )
}


pub fn api_catchers() -> Vec<Catcher> {
    catchers![api_not_found]
}

#[catch(404)]
pub fn api_not_found(_req: &Request<'_>) -> () {}
