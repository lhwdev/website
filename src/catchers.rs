use rocket::{catchers, catch, Catcher, Request, response::{Responder, self, content::RawJson}, Response, http::Status };
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

type SimpleResponse = (Status, RawJson<String>);
struct ApiErrorCacheKey(Option<SimpleResponse>);

pub fn api_error_cache(request: &Request, response: SimpleResponse) {
    request.local_cache(|| ApiErrorCacheKey(Some(response)));
}

fn get_api_error_cache<'a>(request: &'a Request) -> &'a Option<SimpleResponse> {
    &request.local_cache(|| ApiErrorCacheKey(None)).0
}

pub struct RawResponser<'r>(Response<'r>);

impl <'r, 'o : 'r> Responder<'r, 'o> for RawResponser<'o> {
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'o> {
        Ok(self.0)
    }
}


#[catch(default)]
pub fn api_not_found<'a>(request: &'a Request<'_>) -> Result<RawResponser<'a>, Status> {
    let cache = get_api_error_cache(request);
    let result = if let Some(error) = cache {
        error.clone().respond_to(request)
    } else {
        Ok(Response::new())
    };
    Ok(RawResponser(result?))
}
