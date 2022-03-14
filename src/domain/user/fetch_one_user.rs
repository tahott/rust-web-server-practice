use std::sync::Arc;

use crate::domain::user::entity::{UserId};
use crate::repositories::user::{Repository, FetchOneError};

pub struct Request {
  pub id: i32,
}

#[derive(Debug)]
pub struct Response {
  pub name: String,
  pub email: Option<String>,
}

#[derive(Debug)]
pub enum Error {
  BadRequest,
  NotFound,
  Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
  match UserId::try_from(req.id) {
    Ok(id) => match repo.fetch_one(id) {
      Ok(user) => Ok(Response {
        name: user.name,
        email: user.email,
      }),
      Err(FetchOneError::NotFound) => Err(Error::NotFound),
      Err(FetchOneError::Unknown) => Err(Error::Unknown),
    },
    _ => Err(Error::BadRequest),
  }
}

#[cfg(test)]
mod tests {
  use crate::{repositories::user::{InMemoryRepository}, domain::user::entity::{UserName, UserLogin, UserId}};

  use super::*;

  #[test]
  fn it_should_be_return_a_not_found_error_when_the_repo_does_not_contain_the_user() {
    let repo = Arc::new(InMemoryRepository::new());
    let req = Request::new(UserId::two());

    let res = execute(repo, req);

    match res {
      Err(Error::NotFound) => {},
      _ => unreachable!(),
    }
  }

  #[test]
  fn it_should_be_return_the_user_otherwise() {
    let repo = Arc::new(InMemoryRepository::new());
    let r = repo.insert(UserId::one(), UserLogin::kent_back(), UserName::kent_back());

    let req = Request::new(UserId::one());

    let res = execute(repo, req);

    match res {
      Ok(res) => {
        assert_eq!(res.name, String::from(UserName::kent_back()));
      },
      _ => unreachable!(),
    };
  }

  impl Request {
    fn new(id: UserId) -> Self {
      Self {
        id: i32::from(id),
      }
    }
  }
}