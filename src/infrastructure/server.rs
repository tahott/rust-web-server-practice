use std::env;

use actix_cors::Cors;
use actix_web::{HttpServer, App, middleware::{Logger}, web, HttpRequest};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{api::{fetch_access_token::fetch_access_token, authorization_code::{authorization_code}, create_career::create_career, fetch_career::fetch_career, user::{update_user, fetch_user}}, middleware::auth_middleware::Authentication};

pub struct Server {
  port: u16,
}

#[derive(Debug, Deserialize)]
pub struct RequestName {
  pub name: String,
}

async fn index(_req: HttpRequest) -> &'static str {
  "body"
}

impl Server {
  pub fn new(port: u16) -> Self {
    Self { port }
  }

  pub async fn run(&self, pool: DatabaseConnection) -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info,axtix_web=debug,actix_server=info");

    let server = HttpServer::new(move || {
      App::new()
        .wrap(
          Cors::default().allow_any_origin().allow_any_method().allow_any_header()
        )
        .wrap(Logger::default())
        .wrap(Authentication)
        .app_data(pool.clone())
        .route("/", web::get().to(index))
        .route("/signin", web::post().to(fetch_access_token))
        .route("/authorization/code", web::get().to(authorization_code))
        .route("/career", web::post().to(create_career))
        .route("/career/{user_id}", web::get().to(fetch_career))
        .route("/user", web::patch().to(update_user))
        .route("/user/{id}", web::get().to(fetch_user))
    });

    server.bind(format!("{}:{}", "127.0.0.1", self.port))?.run().await
  }
}