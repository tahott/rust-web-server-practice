use std::sync::Arc;

use chrono::{NaiveDate};
use serde::Deserialize;

use crate::repositories::career::Repository;

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Request {
  pub user_id: i64,
  pub company: String,
  pub job: String,
  pub in_at: NaiveDate,
  pub out_at: Option<NaiveDate>,
}

pub struct Response {
  pub user_id: i64,
  pub company: String,
  pub job: String,
}

pub enum Error {
  Unknown,
}

pub async fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error>  {
  match repo.insert(req.user_id, req.company, req.job, req.in_at, req.out_at).await {
    Ok(res) => Ok(Response {
      user_id: res.user_id,
      company: res.company,
      job: res.job
    }),
    Err(_) => Err(Error::Unknown),
  }
}

#[cfg(test)]
mod tests {
  use crate::repositories::career::InMemoryRepository;
  use super::*;

  #[tokio::test]
  async fn it_should_be_return_ok() {
    let repo = Arc::new(InMemoryRepository::new());
    let req = Request::new(1, "PineApple".to_string(), "Server Engineer".to_string(), NaiveDate::from_ymd(2022, 1, 1), None);

    let res = execute(repo, req).await;

    match res {
      Ok(res) => {
        assert_eq!(res.company, "PineApple".to_string());
      },
      _ => unreachable!(),
    }
  }

  impl Request {
    fn new(user_id: i64, company: String, job: String, in_at: NaiveDate, out_at: Option<NaiveDate>) -> Self {
      Self {
        user_id,
        company,
        job,
        in_at,
        out_at,
      }
    }
  }
}