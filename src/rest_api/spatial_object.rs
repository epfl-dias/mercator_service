use super::error_400;
use super::error_404;
use super::ok_200;
use super::AppState;
use super::StringOrStaticFileResult;
use crate::model::to_spatial_objects;

use actix_web::HttpRequest;
use actix_web::Path;

/*
pub fn post((_path, _state): (Path<String>, HttpRequest<AppState>)) -> StringOrStaticFileResult {
    info!("POST Triggered!");
    error_400()
}
*/

pub fn put(
    (core, id, state): (Path<String>, Path<String>, HttpRequest<AppState>),
) -> StringOrStaticFileResult {
    trace!("PUT Triggered!");
    error_400()
}

pub fn get(
    (path, state): (Path<(String, String)>, HttpRequest<AppState>),
) -> StringOrStaticFileResult {
    trace!("GET Triggered!");
    let (core, id) = path.into_inner();
    let core = core.to_string();
    let id = id.to_string();
    let db = state.state().shared.read().unwrap();

    match db.core(core) {
        Ok(core) => match core.get_by_id(&id, 0.0) {
            Ok(objects) => ok_200(&to_spatial_objects(&db, objects)),
            Err(_) => error_404(),
        },
        Err(_) => error_404(),
    }
}

pub fn patch(
    (core, id, state): (Path<String>, Path<String>, HttpRequest<AppState>),
) -> StringOrStaticFileResult {
    trace!("PATCH Triggered!");
    error_400()
}

pub fn delete(
    (core, id, state): (Path<String>, Path<String>, HttpRequest<AppState>),
) -> StringOrStaticFileResult {
    trace!("DELETE Triggered!");
    error_400()
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;

    const INSTANCE_EXISTS: &str = "/cores/42/spatial_objects/42";
    const INSTANCE_INVALID: &str = "/cores/42/spatial_objects/21";

    // FIXME: Add Body to request to see difference between (in)valid bodied requests

    #[test]
    fn put() {
        json::expect_200(http::Method::PUT, get_path(INSTANCE_EXISTS), "".to_string());
        json::expect_422(http::Method::PUT, get_path(INSTANCE_EXISTS), "".to_string());
        json::expect_200(
            http::Method::PUT,
            get_path(INSTANCE_INVALID),
            "".to_string(),
        );
    }

    #[test]
    fn patch() {
        json::expect_200(
            http::Method::PATCH,
            get_path(INSTANCE_EXISTS),
            "".to_string(),
        );
        json::expect_422(
            http::Method::PATCH,
            get_path(INSTANCE_EXISTS),
            "".to_string(),
        );
        expect_400(http::Method::PATCH, get_path(INSTANCE_INVALID));
    }

    #[test]
    fn get() {
        expect_200(http::Method::GET, get_path(INSTANCE_EXISTS));
        expect_404(http::Method::GET, get_path(INSTANCE_INVALID));
    }

    #[test]
    fn delete() {
        expect_200(http::Method::DELETE, get_path(INSTANCE_EXISTS));
        expect_404(http::Method::DELETE, get_path(INSTANCE_INVALID));
    }

    #[test]
    fn post() {
        expect_400(http::Method::POST, get_path(INSTANCE_EXISTS));
        expect_400(http::Method::POST, get_path(INSTANCE_INVALID));
    }
}
