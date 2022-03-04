use std::sync::Mutex;

use diesel::{QueryDsl, RunQueryDsl, insert_into };

use crate::{domain::{user::entity::{UserEmail, UserName, User}}, infrastructure::database::{PgPool, Database}};

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

    if lock.iter().any(|user| user.email == String::from(email.to_owned())) {
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

    match lock.iter().find(|user| user.email == String::from(email.to_owned())) {
      Some(user) => Ok(user.clone()),
      None => Err(FetchOneError::NotFound),
    }
  }
}

pub struct PgRepository {
  conn: PgPool,
}

impl PgRepository {
  pub fn try_new() -> Self {
    let pool = Database::establish_connection();
    
    Self {
      conn: pool,
    }
  }
}

impl Repository for PgRepository {
  fn insert(
    &self,
    uemail: UserEmail,
    uname: UserName,
  ) -> Result<User, InsertError> {
    use crate::domain::schema::users;

    let conn = match self.conn.get() {
      Ok(conn) => conn,
      _ => return Err(InsertError::Unknown),
    };

    let user = User {
      email: String::from(uemail),
      name: String::from(uname),
    };

    let res = insert_into(users::table)
      .values(&user)
      .returning(users::all_columns)
      .on_conflict(users::email)
      .do_nothing()
      .get_result::<User>(&conn);

    match res {
      Ok(user) => Ok(user),
      Err(_) => Err(InsertError::Conflict),
    }
  }

  fn fetch_one(&self, email: UserEmail) -> Result<User, FetchOneError> {
    use crate::domain::schema::users::dsl::*;
  
    let conn = match self.conn.get() {
      Ok(conn) => conn,
      _ => return Err(FetchOneError::Unknown),
    };
    
    let user = match users.find(email).first(&conn) {
      Ok(user) => user,
      Err(_) => return Err(FetchOneError::NotFound),
    };

    Ok(user)
  }
}