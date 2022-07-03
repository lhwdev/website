pub mod post;
mod utils;

use rocket::Route;


pub fn api_routes() -> Vec<Route> {
    return post::api_routes();
}
