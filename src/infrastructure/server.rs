use std::env;

use actix_web::{HttpServer, App, middleware::{Logger, self}, web, HttpResponse, HttpRequest};

use crate::infrastructure::database::PgPool;

pub struct Server {
  port: u16,
}

async fn index(req: HttpRequest) -> &'static str {
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
        .wrap(Logger::default())
        .app_data(pool.clone())
        .route("/", web::get().to(index))
    });

    server.bind(format!("{}:{}", "127.0.0.1", self.port))?.run().await
  }
}