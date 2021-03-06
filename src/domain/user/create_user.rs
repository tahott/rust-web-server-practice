use std::sync::Arc;

use crate::{domain::user::entity::{ UserId, UserName, UserLogin }, repositories::user::{Repository, InsertError}};

pub struct Request {
  pub id: i32,
  pub login: String,
  pub name: String,
}

#[derive(Debug)]
pub struct Response {
  pub name: String,
  pub email: Option<String>,
}

#[derive(Debug)]
pub enum Error {
  BadRequest,
  Conflict,
  Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
  match (
    UserId::try_from(req.id),
    UserLogin::try_from(req.login),
    UserName::try_from(req.name),
  ) {
    (Ok(id), Ok(login), Ok(name)) => match repo.insert(id, login, name) {
      Ok(user) => Ok(Response {
        name: user.name,
        email: user.email,
      }),
      Err(InsertError::Conflict) => Err(Error::Conflict),
      Err(InsertError::Unknown) => Err(Error::Unknown),
    }
    _ => Err(Error::BadRequest),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{domain::user::entity::{UserName}, repositories::user::InMemoryRepository};

  #[test]
  fn it_should_be_return_a_bad_request() {
    let repo = Arc::new(InMemoryRepository::new());
    let req = Request::new(
      443,
      UserLogin::kent_back(),
      UserName::bad(),
    );

    let res = execute(repo, req);
    
    match res {
      Err(Error::BadRequest) => {},
      _ => unreachable!(),
    }
  }

  #[test]
  fn it_should_be_return_a_user() {
    let repo = Arc::new(InMemoryRepository::new());
    let req = Request::new(
      443,
      UserLogin::kent_back(),
      UserName::kent_back(),
    );

    let res = execute(repo, req);

    match res {
      Ok(res) => {
        assert_eq!(res.name, String::from(UserName::kent_back()));
      },
      _ => unreachable!(),
    }
  }

  impl Request {
    fn new(id: i32, login: UserLogin, name: UserName) -> Self {
      Self {
        id,
        login: String::from(login),
        name: String::from(name),
      }
    }
  }
}