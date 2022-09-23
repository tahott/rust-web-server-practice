use std::sync::Arc;

use chrono::{DateTime, FixedOffset};

use crate::repositories::career::Repository;

pub struct Request {
  pub user_id: i32,
  pub company_name: String,
  pub job: String,
  pub in_at: DateTime<FixedOffset>,
  pub out_at: Option<DateTime<FixedOffset>>,
}

pub struct Response {
  pub user_id: i32,
  pub company_name: String,
  pub job: String,
}

pub enum Error {
  BadRequest,
  Unknown,
}

pub async fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error>  {
  match repo.insert(req.user_id, req.company_name, req.job, req.in_at, req.out_at).await {
    Ok(res) => Ok(Response {
      user_id: res.user_id,
      company_name: res.company_name,
      job: res.job
    }),
    Err(_) => Err(Error::Unknown),
  }
}

#[cfg(test)]
mod tests {
  use chrono::Utc;
  use crate::repositories::career::InMemoryRepository;
  use super::*;

  #[tokio::test]
  async fn it_should_be_return_ok() {
    let repo = Arc::new(InMemoryRepository::new());
    let req = Request::new(1, "PineApple".to_string(), "Server Engineer".to_string(), Utc::now().with_timezone(&FixedOffset::east(9 * 3600)), None);

    let res = execute(repo, req).await;

    match res {
      Ok(res) => {
        assert_eq!(res.company_name, "PineApple".to_string());
      },
      _ => unreachable!(),
    }
  }

  impl Request {
    fn new(user_id: i32, company_name: String, job: String, in_at: DateTime<FixedOffset>, out_at: Option<DateTime<FixedOffset>>) -> Self {
      Self {
        user_id,
        company_name,
        job,
        in_at,
        out_at,
      }
    }
  }
}