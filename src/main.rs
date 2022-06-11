mod files;
mod api;

use rocket::{ get, launch, routes };

use crate::files::get_static;
use crate::api::routes_api;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/", &*routes_api)
        .mount("/", routes![get_static])
}
