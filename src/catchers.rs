use rocket::{catchers, catch, Catcher, Request };
use rocket_dyn_templates::Template;
use serde_json::json;


pub fn catchers() -> Vec<Catcher> {
    catchers![not_found]
}

#[catch(400)]
pub fn e_400(req: &Request<'_>) {
}

#[catch(403)]
pub fn e_403(req: &Request<'_>) {
}

#[catch(500)]
pub fn e_500(req: &Request<'_>) {
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
    catchers![api_not_found, e_400, e_403, e_500]
}

#[catch(404)]
pub fn api_not_found(_req: &Request<'_>) -> () {}
