use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::user::entity::{UserId};
use crate::repositories::user::{Repository, FetchOneError};

#[derive(Deserialize)]
pub struct Request {
  pub id: i64,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Response {
  pub id: i64,
  pub login: String,
  pub name: String,
  pub avatar_url: String,
  pub email: Option<String>,
}

#[derive(Debug)]
pub enum Error {
  BadRequest,
  NotFound,
  Unknown,
}

pub async fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
  match UserId::try_from(req.id) {
    Ok(id) => match repo.fetch_one(id).await {
      Ok(user) => Ok(Response {
        id: user.id,
        login: user.login,
        name: user.name,
        avatar_url: user.avatar_url,
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
  use crate::{repositories::user::{InMemoryRepository}, domain::user::entity::{UserName, UserLogin, UserId, UserAvatar}};

  use super::*;

  #[tokio::test]
  async fn it_should_be_return_a_not_found_error_when_the_repo_does_not_contain_the_user() {
    let repo = Arc::new(InMemoryRepository::_new());
    let req = Request::new(UserId::two());

    let res = execute(repo, req).await;

    match res {
      Err(Error::NotFound) => {},
      _ => unreachable!(),
    }
  }

  #[tokio::test]
  async fn it_should_be_return_the_user_otherwise() {
    let repo = Arc::new(InMemoryRepository::_new());
    let _ = repo.insert(UserId::one(), UserLogin::kent_back(), UserName::kent_back(), UserAvatar::user()).await;

    let req = Request::new(UserId::one());

    let res = execute(repo, req).await;

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
        id: i64::from(id),
      }
    }
  }
}