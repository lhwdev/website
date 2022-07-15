pub mod auth;
pub mod post;
pub mod utils;

use rocket::Route;

pub fn api_routes() -> Vec<Route> {
    [post::api_routes(), auth::api_routes()].concat()
}
