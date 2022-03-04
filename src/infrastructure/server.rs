use std::env;

use actix_web::{HttpServer, App, middleware::Logger, web, HttpResponse};

use crate::infrastructure::database::PgPool;

pub struct Server {
  port: u16,
}

impl Server {
  pub fn new(port: u16) -> Self {
    Self { port }
  }

  pub async fn run(&self, pool: PgPool) -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info,axtix_web=debug,actix_server=info");

    let server = HttpServer::new(move || {
      App::new()
        .wrap(Logger::default())
        .data(pool.clone())
        .route("/", web::get().to(|| HttpResponse::Ok().body("body")))
    });

    server.bind(format!("{}:{}", "127.0.0.1", self.port))?.run().await
  }
}