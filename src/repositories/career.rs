use std::sync::Mutex;

use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use sea_orm::prelude::DateTimeWithTimeZone;

use crate::domain::career::entity::CareerEntity;

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
    user_id: i32,
    company_name: String,
    job: String,
    in_at: DateTimeWithTimeZone,
    out_at: Option<DateTimeWithTimeZone>
  ) -> Result<CareerEntity, InsertError>;

  async fn find_by_user_id(
    &self,
    user_id: i32
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
    user_id: i32,
    company_name: String,
    job: String,
    in_at: DateTime<FixedOffset>,
    out_at: Option<DateTime<FixedOffset>>,
  ) -> Result<CareerEntity, InsertError> {
    let mut lock = match self.careers.lock() {
      Ok(lock) => lock,
      _ => return Err(InsertError::Unknown)
    };

    let career = CareerEntity::new(user_id, company_name, job, in_at, out_at);

    lock.push(career.clone());

    Ok(lock.last().unwrap().clone())
  }

  async fn find_by_user_id(
    &self,
    user_id: i32,
  ) -> Result<Vec<CareerEntity>, FetchError> {
    let lock = match self.careers.lock() {
      Ok(lock) => lock,
      _ => return Err(FetchError::Unknown)
    };

    let careers = lock.iter().filter(|c| c.user_id == user_id).cloned().collect();

    Ok(careers)
  }
}