use std::env;

use diesel::{r2d2::{Pool, PoolError, ConnectionManager}, PgConnection};
use dotenv::dotenv;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
  let manager = ConnectionManager::<PgConnection>::new(database_url);

  Pool::builder().build(manager)
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
  pub fn establish_connection() -> PgPool { // create connection pool
    init_pool(&database_url()).expect("Failed create pool")
  }
}
