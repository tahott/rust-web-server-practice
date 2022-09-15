use std::env;
use dotenv::dotenv;
use sea_orm::{DatabaseConnection, DbErr};

async fn init_pool(database_url: &str) -> Result<DatabaseConnection, DbErr> {
  sea_orm::Database::connect(database_url).await
}

fn database_url() -> String {
  dotenv().ok();

  match env::var("DATABASE_URL") {
    Ok(url) => url,
    Err(e) => panic!("{}", e),
  }
}

pub struct Database;

impl Database {
  pub async fn establish_connection() -> DatabaseConnection { // create connection pool
    init_pool(&database_url()).await.expect("Failed create pool")
  }
}
