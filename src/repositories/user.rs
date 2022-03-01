use std::sync::Mutex;

use crate::domain::user::entity::{UserEmail, UserName, User};

pub enum InsertError {
  Conflict,
  Unknown,
}

pub enum FetchOneError {
  NotFound,
  Unknown,
}

pub trait Repository: Send + Sync {
  fn insert(
    &self,
    email: UserEmail,
    name: UserName,
  ) -> Result<User, InsertError>;

  fn fetch_one(&self, email: UserEmail) -> Result<User, FetchOneError>;
}

pub struct InMemoryRepository {
  error: bool,
  users: Mutex<Vec<User>>,
}

impl InMemoryRepository {
  pub fn new() -> Self {
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

impl Repository for InMemoryRepository {
  fn insert(
    &self,
    email: UserEmail,
    name: UserName,
  ) -> Result<User, InsertError> {
    if self.error {
      return Err(InsertError::Unknown);
    }

    let mut lock = match self.users.lock() {
      Ok(lock) => lock,
      _ => return Err(InsertError::Unknown),
    };

    if lock.iter().any(|user| user.email == email) {
      return Err(InsertError::Conflict);
    }

    let user = User::new(email, name);
    lock.push(user.clone());

    Ok(user)
  }

  fn fetch_one(&self, email: UserEmail) -> Result<User, FetchOneError> {
    if self.error {
      return Err(FetchOneError::Unknown);
    }

    let lock = match self.users.lock() {
      Ok(lock) => lock,
      _ => return Err(FetchOneError::Unknown),
    };

    match lock.iter().find(|user| user.email == email) {
      Some(user) => Ok(user.clone()),
      None => Err(FetchOneError::NotFound),
    }
  }
}