#[macro_use]
extern crate diesel;

mod api;
mod infrastructure;
mod domain;
mod repositories;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let pool = infrastructure::database::Database::establish_connection();
  let server = infrastructure::Server::new(8082);

  server.run(pool).await
}
