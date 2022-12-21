use std::sync::Mutex;

use async_trait::async_trait;
use chrono::{NaiveDate};
use entity::career;
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, QueryOrder};

use crate::{domain::career::entity::CareerEntity, infrastructure::database::Database};

pub enum InsertError {
  Conflict,
  Unknown
}

pub enum FetchError {
  NotFound,
  Unknown
}

#[async_trait]
pub trait Repository: Send + Sync {
  async fn insert(
    &self,
    user_id: i64,
    company: String,
    job: String,
    in_at: NaiveDate,
    out_at: Option<NaiveDate>
  ) -> Result<CareerEntity, InsertError>;

  async fn find_by_user_id(
    &self,
    user_id: i64
  ) -> Result<Vec<CareerEntity>, FetchError>;
}

pub struct InMemoryRepository {
  error: bool,
  careers: Mutex<Vec<CareerEntity>>,
}

impl InMemoryRepository {
  pub fn new() -> Self {
    let careers = Mutex::new(vec![]);
    Self {
      error: false,
      careers,
    }
  }
}

#[async_trait]
impl Repository for InMemoryRepository {
  async fn insert(
    &self,
    user_id: i64,
    company: String,
    job: String,
    in_at: NaiveDate,
    out_at: Option<NaiveDate>,
  ) -> Result<CareerEntity, InsertError> {
    let mut lock = match self.careers.lock() {
      Ok(lock) => lock,
      _ => return Err(InsertError::Unknown)
    };

    let career = CareerEntity::new(user_id, company, job, in_at, out_at);

    lock.push(career.clone());

    Ok(lock.last().unwrap().clone())
  }

  async fn find_by_user_id(
    &self,
    user_id: i64,
  ) -> Result<Vec<CareerEntity>, FetchError> {
    let lock = match self.careers.lock() {
      Ok(lock) => lock,
      _ => return Err(FetchError::Unknown)
    };

    let careers = lock.iter().filter(|c| c.user_id == user_id).cloned().collect();

    Ok(careers)
  }
}

pub struct PgRepository {
  conn: DatabaseConnection,
}

impl PgRepository {
  pub async fn try_new() -> Self {
    let pool = Database::establish_connection().await;

    Self {
      conn: pool,
    }
  }
}

#[async_trait]
impl Repository for PgRepository {
  async fn insert(
    &self,
    user_id: i64,
    company: String,
    job: String,
    in_at: NaiveDate,
    out_at: Option<NaiveDate>,
  ) -> Result<CareerEntity, InsertError> {
    let conn = &self.conn;

    let career = CareerEntity::new(user_id, company, job, in_at, out_at);

    let career_model = career::ActiveModel {
      user_id: Set(career.user_id),
      company: Set(career.company.clone()),
      job: Set(career.job.clone()),
      in_at: Set(career.in_at),
      out_at: Set(career.out_at),
      ..Default::default()
    };

    let res = career_model.insert(conn).await;

    match res {
      Ok(_) => Ok(career),
      Err(e) => {
        println!("{:?}", e);
        Err(InsertError::Conflict)
      }
    }
  }

  async fn find_by_user_id(&self, user_id: i64) -> Result<Vec<CareerEntity>, FetchError> {
    let conn = &self.conn;

    match career::Entity::find()
      .filter(career::Column::UserId.eq(user_id))
      .order_by_desc(career::Column::InAt)
      .all(conn)
      .await {
        Ok(careers) => Ok(careers.iter().map(|career| CareerEntity::new(career.user_id, career.company.clone(), career.job.clone(), career.in_at, career.out_at)).collect::<Vec<CareerEntity>>()),
        Err(_) => Err(FetchError::NotFound),
      }
  }
}