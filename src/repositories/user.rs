use std::sync::Mutex;

use async_trait::async_trait;
use entity::users;
use entity::users::Entity as Users;
use sea_orm::DatabaseConnection;
use sea_orm::{entity::*};

use crate::{domain::{user::entity::{UserId, User, UserName, UserLogin, UserAvatar}}, infrastructure::database::{Database}};

#[derive(Debug)]
pub enum InsertError {
  Conflict,
  Unknown,
}

pub enum FetchOneError {
  NotFound,
  Unknown,
}

#[async_trait]
pub trait Repository: Send + Sync {
  async fn insert(
    &self,
    id: UserId,
    login: UserLogin,
    name: UserName,
    avatar_url: UserAvatar,
  ) -> Result<User, InsertError>;

  async fn fetch_one(&self, id: UserId) -> Result<User, FetchOneError>;
}

pub struct InMemoryRepository {
  error: bool,
  users: Mutex<Vec<User>>,
}

impl InMemoryRepository {
  pub fn _new() -> Self {
    let users = Mutex::new(vec![]);
    Self {
      error: false,
      users,
    }
  }

  #[cfg(test)]
  pub fn with_error(self) -> Self {
    Self {
      error: true,
      ..self
    }
  }
}

#[async_trait]
impl Repository for InMemoryRepository {
  async fn insert(
    &self,
    id: UserId,
    login: UserLogin,
    name: UserName,
    avatar_url: UserAvatar,
  ) -> Result<User, InsertError> {
    if self.error {
      return Err(InsertError::Unknown);
    }

    let mut lock = match self.users.lock() {
      Ok(lock) => lock,
      _ => return Err(InsertError::Unknown),
    };

    if lock.iter().any(|user| user.id == i32::from(id)) {
      return Err(InsertError::Conflict);
    }

    let user = User::new(
      id,
      login,
      name,
      avatar_url,
    );
    lock.push(user.clone());

    Ok(user)
  }

  async fn fetch_one(&self, id: UserId) -> Result<User, FetchOneError> {
    if self.error {
      return Err(FetchOneError::Unknown);
    }

    let lock = match self.users.lock() {
      Ok(lock) => lock,
      _ => return Err(FetchOneError::Unknown),
    };

    match lock.iter().find(|user| user.id == i32::from(id)) {
      Some(user) => Ok(user.clone()),
      None => Err(FetchOneError::NotFound),
    }
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
    id: UserId,
    login: UserLogin,
    name: UserName,
    avatar_url: UserAvatar,
  ) -> Result<User, InsertError> {
    let conn = &self.conn;

    let user = User::new(id, login, name, avatar_url);
    let res = users::ActiveModel {
      id: Set(user.id),
      login: Set(user.login.clone()),
      name: Set(user.name.clone()),
      avatar_url: Set(user.avatar_url.clone()),
      ..Default::default()
    }.save(conn).await;
    
    match res {
      Ok(_) => Ok(user),
      Err(e) => {
        println!("{:?}", e);
        Err(InsertError::Conflict)
      },
    }
  }

  async fn fetch_one(&self, user_id: UserId) -> Result<User, FetchOneError> {
    let conn = &self.conn;
  
    match Users::find_by_id(i32::from(user_id)).one(conn).await {
      Ok(user) => match user {
        Some(user) => Ok(User {
          id: user.id,
          login: user.login,
          name: user.name,
          email: user.email,
          avatar_url: user.avatar_url,
          created_at: Some(user.created_at),
          updated_at: Some(user.updated_at),
        }),
        None => Err(FetchOneError::NotFound),
      },
      Err(_) => Err(FetchOneError::NotFound),
    }
  }
}