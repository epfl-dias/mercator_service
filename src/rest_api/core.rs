use std::sync::RwLock;

use super::error_400;
use super::error_404;
use super::ok_200;
use super::web;
use super::web::Data;
use super::web::Path;
use super::Core;
use super::HandlerResult;
use super::SharedState;

fn put(path: Path<String>) -> HandlerResult {
    trace!("PUT Triggered on {}", path);
    error_400()
}

fn get((core, state): (Path<String>, Data<RwLock<SharedState>>)) -> HandlerResult {
    trace!("GET '{:?}'", core);
    let core = core.to_string();
    let context = state
        .read()
        .unwrap_or_else(|e| panic!("Can't acquire read lock of the database: {}", e));

    match context.db().core(&core) {
        Ok(core) => ok_200(&Core::from(core)),
        Err(_) => error_404(),
    }
}

fn patch(path: Path<String>) -> HandlerResult {
    trace!("PATCH Triggered on {}", path);
    error_400()
}

fn delete(path: Path<String>) -> HandlerResult {
    trace!("DELETE Triggered on {}", path);
    error_400()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/cores/{name}")
            .route(web::get().to(get))
            .route(web::put().to(put))
            .route(web::patch().to(patch))
            .route(web::delete().to(delete)),
    );
}

#[cfg(test)]
mod routing {
    use super::super::tests_utils::*;

    const INSTANCE_EXISTS: &str = CORE;
    const INSTANCE_INVALID: &str = "/41-doesnotexists";

    // FIXME: Add Body to request to see difference between (in)valid bodied requests

    #[test]
    fn put() {
        json::expect_200(Method::PUT, &get_core(INSTANCE_EXISTS), "".to_string());
        json::expect_422(Method::PUT, &get_core(INSTANCE_EXISTS), "".to_string());
        json::expect_200(Method::PUT, &get_core(INSTANCE_INVALID), "".to_string());
    }

    #[test]
    fn patch() {
        json::expect_200(Method::PATCH, &get_core(INSTANCE_EXISTS), "".to_string());
        json::expect_422(Method::PATCH, &get_core(INSTANCE_EXISTS), "".to_string());
        expect_404(Method::PATCH, &get_core(INSTANCE_INVALID));
    }

    #[test]
    fn get() {
        expect_200(Method::GET, &get_core(INSTANCE_EXISTS));
        expect_404(Method::GET, &get_core(INSTANCE_INVALID));
    }

    #[test]
    fn delete() {
        expect_200(Method::DELETE, &get_core(INSTANCE_EXISTS));
        expect_404(Method::DELETE, &get_core(INSTANCE_INVALID));
    }

    #[test]
    fn post() {
        expect_405(Method::POST, &get_core(INSTANCE_EXISTS));
        expect_405(Method::POST, &get_core(INSTANCE_INVALID));
    }
}
