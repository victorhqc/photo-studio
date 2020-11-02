#[cfg(target_os = "linux")]
extern crate openssl;

#[cfg(target_os = "linux")]
extern crate openssl_probe;

#[macro_use]
extern crate gotham_derive;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate snafu_derive;

mod auth;
mod aws;
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
use photo_core::connection::{connect, db_migrate, get_database_url};
use photo_core::custom_migrations::apply_custom_migrations;

lazy_static! {
    pub static ref OPTIONS_OR_HEAD: Vec<Method> = {
        let mut v = Vec::new();

        v.push(Method::OPTIONS);
        v.push(Method::HEAD);

        v
    };
}

fn main() {
    #[cfg(target_os = "linux")]
    openssl_probe::init_ssl_cert_env_vars();

    dotenv().ok();
    pretty_env_logger::init();

    let url = get_database_url(None);
    let conn = connect(Some(url.clone())).unwrap();
    info!("Running migrations");
    db_migrate(&conn).unwrap();

    info!("Running custom migrations");
    apply_custom_migrations(Some(url)).unwrap();

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

    let (pipelines, authenticated) = pipelines.add(
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
    let auth_chain = (authenticated, default_chain);

    build_router(default_chain, pipeline_set, |route| {
        route.get_or_head("/").to(empty_handler);

        route.get("/google/authorize").to(google_authorize_handler);
        route
            .get("/google/redirect")
            .with_query_string_extractor::<GoogleRedirectExtractor>()
            .to_async(google_redirect_handler);

        route.scope("/api", |route| {
            route
                .get("/public/album")
                .with_query_string_extractor::<handlers::albums::WithIdExtractor>()
                .to_async(handlers::albums::get_main_public);

            route
                .get("/public/album/:name")
                .with_query_string_extractor::<handlers::albums::WithIdExtractor>()
                .with_path_extractor::<handlers::albums::WithNameExtractor>()
                .to_async(handlers::albums::get_album_by_name);

            route.with_pipeline_chain(auth_chain, |route| {
                route.get("/me").to_async(handlers::users::me);

                route.get("/albums").to_async(handlers::albums::all_albums);

                route.scope("/album", |route| {
                    route.post("/").to_async(handlers::albums::new_album);

                    route
                        .put("/:id")
                        .with_path_extractor::<handlers::albums::AlbumPathExtractor>()
                        .to_async(handlers::albums::update_album);

                    route
                        .delete("/:id")
                        .with_path_extractor::<handlers::albums::AlbumPathExtractor>()
                        .to_async(handlers::albums::delete_album);

                    route
                        .post("/:id/photo")
                        .with_path_extractor::<handlers::photos::AlbumPathExtractor>()
                        .to_async(handlers::photos::new_photo);

                    route
                        .get("/:id/photos")
                        .with_path_extractor::<handlers::albums::AlbumPathExtractor>()
                        .to_async(handlers::albums::album_photos);
                });

                route.scope("/photo", |route| {
                    route
                        .put("/:id")
                        .with_path_extractor::<handlers::photos::PhotoPathExtractor>()
                        .to_async(handlers::photos::update_photo);

                    route
                        .delete("/:id")
                        .with_path_extractor::<handlers::photos::PhotoPathExtractor>()
                        .to_async(handlers::photos::delete_photo);

                    route
                        .post("/upload")
                        .to_async(handlers::photos::upload_photo);
                });

                route.scope("/book_me", |route| {
                    route.get("/").to_async(handlers::book_me::find_by_user);

                    route.put("/").to_async(handlers::book_me::update);
                });
            });

            // CORS, need to investigate a better way to do this without repeating routes.
            route.with_pipeline_chain(cors_preflight_chain, |route| {
                route
                    .request(OPTIONS_OR_HEAD.clone(), "/me")
                    .to(empty_handler);

                route
                    .request(OPTIONS_OR_HEAD.clone(), "/public/album")
                    .to(empty_handler);

                route
                    .request(OPTIONS_OR_HEAD.clone(), "/public/album/:name")
                    .to(empty_handler);

                route
                    .request(OPTIONS_OR_HEAD.clone(), "/albums")
                    .to(empty_handler);

                route.scope("/album", |route| {
                    route
                        .request(OPTIONS_OR_HEAD.clone(), "/")
                        .to(empty_handler);

                    route
                        .request(OPTIONS_OR_HEAD.clone(), "/:id")
                        .to(empty_handler);

                    route
                        .request(OPTIONS_OR_HEAD.clone(), "/:id/photos")
                        .to(empty_handler);

                    route
                        .request(OPTIONS_OR_HEAD.clone(), "/:id/photo")
                        .to(empty_handler);
                });

                route.scope("/photo", |route| {
                    route
                        .request(OPTIONS_OR_HEAD.clone(), "/:id")
                        .to(empty_handler);

                    route
                        .request(OPTIONS_OR_HEAD.clone(), "/upload")
                        .to(empty_handler);
                });

                route.scope("/book_me", |route| {
                    route
                        .request(OPTIONS_OR_HEAD.clone(), "/")
                        .to(empty_handler);
                });
            });
        })
    })
}
