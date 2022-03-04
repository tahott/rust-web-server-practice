use std::sync::Arc;

use crate::{domain::user::entity::{ UserEmail, UserName, User }, repositories::user::{Repository, InsertError}};

pub struct Request {
  pub email: String,
  pub name: String,
}

pub struct Response {
  pub email: String,
  pub name: String,
}

pub enum Error {
  BadRequest,
  Conflict,
  Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
  match (
    UserEmail::try_from(req.email),
    UserName::try_from(req.name),
  ) {
    (Ok(email), Ok(name)) => match repo.insert(email, name) {
      Ok(User {
        email,
        name
      }) => Ok(Response {
        email,
        name,
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
  use crate::{domain::user::entity::{UserEmail, UserName}, repositories::user::InMemoryRepository};

  #[test]
  fn it_should_be_return_a_bad_request() {
    let repo = Arc::new(InMemoryRepository::new());
    let req = Request::new(
    UserEmail::gmail(),
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
      UserEmail::gmail(),
      UserName::kent_back(),
    );

    let res = execute(repo, req);

    match res {
      Ok(res) => {
        assert_eq!(res.email, String::from(UserEmail::gmail()));
        assert_eq!(res.name, String::from(UserName::kent_back()));
      },
      _ => unreachable!(),
    }
  }

  impl Request {
    fn new(email: UserEmail, name: UserName) -> Self {
      Self {
        email: String::from(email),
        name: String::from(name),
      }
    }
  }
}