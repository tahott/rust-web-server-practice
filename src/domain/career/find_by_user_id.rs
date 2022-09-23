use std::sync::Arc;

use crate::repositories::career::Repository;

use super::entity::CareerEntity;

pub struct Request {
  pub user_id: i32,
}

pub struct Response {
  pub careers: Vec<CareerEntity>
}

pub enum Error {
  BadRequest,
  Unknown,
}

pub async fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
  match repo.find_by_user_id(req.user_id).await {
    Ok(res) => Ok(Response {
      careers: res,
    }),
    Err(_) => Err(Error::Unknown),
  }
}

#[cfg(test)]
mod tests {
  use chrono::{Utc, TimeZone, FixedOffset};

use crate::repositories::career::InMemoryRepository;

use super::*;

  #[tokio::test]
  async fn it_should_be_return_careers() {
    let repo = Arc::new(InMemoryRepository::new());

    let _ = repo.insert(
      1,
      "Micro Hard".to_string(),
      "Designer".to_string(),
      Utc.ymd(2016, 5, 1).and_hms(0, 0, 0).with_timezone(&FixedOffset::east(9 * 3600)),
      Some(Utc.ymd(2018, 3, 31).and_hms(0, 0, 0).with_timezone(&FixedOffset::east(9 * 3600))),
    ).await;

    let _ =  repo.insert(
      2,
      "Micro Hard".to_string(),
      "Designer".to_string(),
      Utc.ymd(2016, 7, 1).and_hms(0, 0, 0).with_timezone(&FixedOffset::east(9 * 3600)),
      Some(Utc.ymd(2018, 1, 31).and_hms(0, 0, 0).with_timezone(&FixedOffset::east(9 * 3600))),
    ).await;

    let _ = repo.insert(
      1,
      "Wercel".to_string(),
      "Server Engieneer".to_string(),
      Utc.ymd(2018, 4, 1).and_hms(0, 0, 0).with_timezone(&FixedOffset::east(9 * 3600)),
      None,
    ).await;

    let req = Request::new(1);
    let res = execute(repo, req).await;

    match res {
      Ok(res) => {
        assert_eq!(res.careers.len(), 2)
      },
      Err(_) => unreachable!(),
    }
  }

  impl Request {
    fn new(user_id: i32) -> Self {
      Self {
        user_id,
      }
    }
  }
}