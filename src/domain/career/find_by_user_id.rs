use std::{sync::Arc, convert::TryFrom};

use chrono::NaiveDate;
use serde::Serialize;

use crate::repositories::career::Repository;

pub struct Request {
  pub user_id: i32,
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct FetchCareerDto {
  company: String,
  job: String,
  in_at: NaiveDate,
  out_at: Option<NaiveDate>
}

impl FetchCareerDto {
  fn new(company: String, job: String, in_at: NaiveDate, out_at: Option<NaiveDate>) -> Self {
    Self {
      company,
      job,
      in_at,
      out_at
    }
  }
}

pub struct Response {
  pub careers: Vec<FetchCareerDto>
}

pub enum Error {
  BadRequest,
  Unknown,
}

pub async fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
  match i32::try_from(req.user_id) {
    Ok(user_id) => match repo.find_by_user_id(user_id).await {
      Ok(res) => Ok(Response {
        careers: res.iter().map(|career| FetchCareerDto::new(career.company.clone(), career.job.clone(), career.in_at, career.out_at)).collect::<Vec<FetchCareerDto>>(),
      }),
      Err(_) => Err(Error::Unknown),
    },
    _ => Err(Error::BadRequest)
  }
}

#[cfg(test)]
mod tests {
  use chrono::{NaiveDate};
  use crate::repositories::career::InMemoryRepository;
  use super::*;

  #[tokio::test]
  async fn it_should_be_return_careers() {
    let repo = Arc::new(InMemoryRepository::new());

    let _ = repo.insert(
      1,
      "Micro Hard".to_string(),
      "Designer".to_string(),
      NaiveDate::from_ymd(2016, 5, 1),
      Some(NaiveDate::from_ymd(2018, 3, 31)),
    ).await;

    let _ =  repo.insert(
      2,
      "Micro Hard".to_string(),
      "Designer".to_string(),
      NaiveDate::from_ymd(2016, 7, 1),
      Some(NaiveDate::from_ymd(2018, 1, 31)),
    ).await;

    let _ = repo.insert(
      1,
      "Wercel".to_string(),
      "Server Engieneer".to_string(),
      NaiveDate::from_ymd(2018, 4, 1),
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