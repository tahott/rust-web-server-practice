mod infrastructure;
mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let server = infrastructure::Server::new(8082);
  println!("Hello, world!");

  server.run().await
}
