use std::sync::Arc;

use crate::domain::user::entity::{User, UserEmail};
use crate::repositories::user::{Repository, FetchOneError};

pub struct Request {
  pub email: String,
}

pub struct Response {
  pub email: String,
  pub name: String,
}

pub enum Error {
  BadRequest,
  NotFound,
  Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
  match UserEmail::try_from(req.email) {
    Ok(email) => match repo.fetch_one(email) {
      Ok(User {
        email,
        name
      }) => Ok(Response {
        email: String::from(email),
        name: String::from(name),
      }),
      Err(FetchOneError::NotFound) => Err(Error::NotFound),
      Err(FetchOneError::Unknown) => Err(Error::Unknown),
    },
    _ => Err(Error::BadRequest),
  }
}

#[cfg(test)]
mod tests {
  use crate::{repositories::user::InMemoryRepository, domain::user::entity::UserName};

  use super::*;

  #[test]
  fn it_should_be_return_a_not_found_error_when_the_repo_does_not_contain_the_user() {
    let repo = Arc::new(InMemoryRepository::new());
    let req = Request::new(UserEmail::gmail());

    let res = execute(repo, req);

    match res {
      Err(Error::NotFound) => {},
      _ => unreachable!(),
    }
  }

  #[test]
  fn it_should_be_return_the_user_otherwise() {
    let repo = Arc::new(InMemoryRepository::new());
    repo.insert(UserEmail::gmail(), UserName::kent_back()).ok();

    let req = Request::new(UserEmail::gmail());

    let res = execute(repo, req);

    match res {
      Ok(res) => {
        assert_eq!(res.email, String::from(UserEmail::gmail()));
        assert_eq!(res.name, String::from(UserName::kent_back()));
      },
      _ => unreachable!(),
    };
  }

  impl Request {
    fn new(email: UserEmail) -> Self {
      Self {
        email: String::from(email),
      }
    }
  }
}