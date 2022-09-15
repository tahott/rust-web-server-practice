use std::sync::Mutex;

use async_trait::async_trait;
use entity::user;
use entity::user::Entity as User;
use sea_orm::DatabaseConnection;
use sea_orm::{entity::*};

use crate::{domain::{user::entity::{UserId, UserEntity, UserName, UserLogin, UserAvatar}}, infrastructure::database::{Database}};

#[derive(Debug)]
pub enum InsertError {
  Conflict,
  Unknown,
}

pub enum UpdateError {
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
  ) -> Result<UserEntity, InsertError>;

  async fn update(
    &self,
    id: UserId,
    name: UserName,
    avatar_url: UserAvatar,
  ) -> Result<UserEntity, UpdateError>;

  async fn fetch_one(&self, id: UserId) -> Result<UserEntity, FetchOneError>;
}

pub struct InMemoryRepository {
  error: bool,
  users: Mutex<Vec<UserEntity>>,
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
  ) -> Result<UserEntity, InsertError> {
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

    let user = UserEntity::new(
      id,
      login,
      name,
      avatar_url,
    );
    lock.push(user.clone());

    Ok(user)
  }

  async fn fetch_one(&self, id: UserId) -> Result<UserEntity, FetchOneError> {
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

  async fn update(&self, id: UserId, name: UserName, avatar_url: UserAvatar) -> Result<UserEntity, UpdateError> {
    let mut lock = match self.users.lock() {
      Ok(lock) => lock,
      _ => todo!()
    };

    match lock.iter_mut().map(|user| {
      if user.id == i32::from(id) {
        user.name = String::from(name.clone());
        user.avatar_url = String::from(avatar_url.clone());
      };

      user
    }).find(|user| user.id == i32::from(id)) {
      Some(user) => Ok(user.clone()),
      None => Err(UpdateError::Unknown)
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
  ) -> Result<UserEntity, InsertError> {
    let conn = &self.conn;

    let user = UserEntity::new(id, login, name, avatar_url);

    let user_model = user::ActiveModel {
      id: Set(user.id),
      login: Set(user.login.clone()),
      name: Set(user.name.clone()),
      avatar_url: Set(user.avatar_url.clone()),
      email: Set(None),
      created_at: Set(user.created_at),
      updated_at: Set(user.updated_at),
    };

    let res = user_model.insert(conn).await;
    
    match res {
      Ok(_) => Ok(user),
      Err(e) => {
        println!("{:?}", e);
        Err(InsertError::Conflict)
      },
    }
  }

  async fn fetch_one(&self, user_id: UserId) -> Result<UserEntity, FetchOneError> {
    let conn = &self.conn;
  
    match User::find_by_id(i32::from(user_id)).one(conn).await {
      Ok(user) => match user {
        Some(user) => Ok(UserEntity {
          id: user.id,
          login: user.login,
          name: user.name,
          email: user.email,
          avatar_url: user.avatar_url,
          created_at: user.created_at,
          updated_at: user.updated_at,
        }),
        None => Err(FetchOneError::NotFound),
      },
      Err(_) => Err(FetchOneError::NotFound),
    }
  }

  async fn update(&self, id: UserId, name: UserName, avatar_url: UserAvatar) -> Result<UserEntity, UpdateError> {
    let conn = &self.conn;

    let user = user::ActiveModel {
      id: Set(i32::from(id)),
      name: Set(String::from(name)),
      avatar_url: Set(String::from(avatar_url)),
      ..Default::default()
    };

    let res = user.save(conn).await;

    match res {
      Ok(user) => Ok(UserEntity {
        id: user.id.unwrap(),
        login: user.login.unwrap(),
        name: user.name.unwrap(),
        avatar_url: user.avatar_url.unwrap(),
        email: user.email.unwrap(),
        created_at: user.created_at.unwrap(),
        updated_at: user.updated_at.unwrap(),
      }),
      Err(e) => {
        println!("{:?}", e);
        Err(UpdateError::Unknown)
      },
    }
  }
}