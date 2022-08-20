use std::env;

use actix_cors::Cors;
use actix_web::{HttpServer, App, middleware::{Logger}, web, HttpRequest};

use crate::{infrastructure::database::PgPool, api::fetch_access_token::fetch_access_token};

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

  pub async fn run(&self, pool: PgPool) -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info,axtix_web=debug,actix_server=info");

    let server = HttpServer::new(move || {
      App::new()
        .wrap(
          Cors::default().allow_any_origin()
        )
        .wrap(Logger::default())
        .app_data(pool.clone())
        .route("/", web::get().to(index))
        .route("/signin", web::post().to(fetch_access_token))
    });

    server.bind(format!("{}:{}", "127.0.0.1", self.port))?.run().await
  }
}