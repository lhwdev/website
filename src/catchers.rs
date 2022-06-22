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
