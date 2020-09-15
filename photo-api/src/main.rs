#[cfg(target_os = "linux")]
extern crate openssl;

#[macro_use]
extern crate gotham_derive;

#[macro_use]
extern crate log;

mod auth;
mod conduit;
mod connection;
mod handlers;
mod middlewares;
mod utils;

use crate::auth::google::GoogleRedirectExtractor;
use crate::auth::{get_secret, AuthUser};
use crate::handlers::auth::{google_authorize_handler, google_redirect_handler};
use crate::handlers::utils::empty_handler;
use crate::middlewares::cors::CorsMiddleware;
use dotenv::dotenv;
use gotham::middleware::logger::RequestLogger;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::set::{finalize_pipeline_set, new_pipeline_set};
use gotham::router::builder::*;
use gotham::router::Router;
use gotham_middleware_diesel::{self, DieselMiddleware};
use gotham_middleware_jwt::JWTMiddleware;
use hyper::Method;

fn main() {
    #[cfg(target_os = "linux")]
    openssl_probe::init_ssl_cert_env_vars();

    dotenv().ok();
    pretty_env_logger::init();

    let port = std::env::var("PORT").unwrap_or(String::from("7878"));
    let addr = format!("127.0.0.1:{}", port);

    info!("Listening for requests at http://{}", addr);

    gotham::start(addr, router());
}

fn router() -> Router {
    let repo = connection::repo();

    let pipelines = new_pipeline_set();
    let (pipelines, default) = pipelines.add(
        new_pipeline()
            .add(DieselMiddleware::new(repo))
            .add(RequestLogger::new(log::Level::Info))
            .add(CorsMiddleware::default())
            .build(),
    );

    let (pipelines, _authenticated) = pipelines.add(
        new_pipeline()
            .add(JWTMiddleware::<AuthUser>::new(get_secret()).scheme("Bearer"))
            .build(),
    );
    let (pipelines, cors) = pipelines.add(
        new_pipeline()
            .add(CorsMiddleware::default())
            .add(RequestLogger::new(log::Level::Info))
            .build(),
    );

    let pipeline_set = finalize_pipeline_set(pipelines);
    let default_chain = (default, ());
    let cors_preflight_chain = (cors, ());
    // let auth_chain = (authenticated, default_chain);

    build_router(default_chain, pipeline_set, |route| {
        route.get_or_head("/").to(empty_handler);

        route.get("/google/authorize").to(google_authorize_handler);
        route
            .get("/google/redirect")
            .with_query_string_extractor::<GoogleRedirectExtractor>()
            .to(google_redirect_handler);

        route.scope("/api", |route| {
            route.with_pipeline_chain(cors_preflight_chain, |route| {
                route
                    .request(vec![Method::OPTIONS, Method::HEAD], "/me")
                    .to(empty_handler);
            });

            // route.with_pipeline_chain(auth_chain, |route| {
            //     route.get("/me").to(handlers::users::me);
            // });
        })
    })
}
