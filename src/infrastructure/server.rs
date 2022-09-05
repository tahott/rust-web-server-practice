use std::env;

use actix_cors::Cors;
use actix_web::{HttpServer, App, middleware::{Logger}, web, HttpRequest};
use sea_orm::DatabaseConnection;

use crate::{api::{fetch_access_token::fetch_access_token, authorization_code::{authorization_code}}};

pub struct Server {
  port: u16,
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
        .app_data(pool.clone())
        .route("/", web::get().to(index))
        .route("/signin", web::post().to(fetch_access_token))
        .route("/authorization/code", web::get().to(authorization_code))
    });

    server.bind(format!("{}:{}", "127.0.0.1", self.port))?.run().await
  }
}