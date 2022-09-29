use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};

use crate::{domain::career::{find_by_user_id::{execute, Request, FetchCareerDto, Error}}, repositories::career::PgRepository};

#[derive(Deserialize)]
pub struct Info {
  pub user_id: i32
}

#[derive(Serialize)]
pub struct Res {
  pub data: Vec<FetchCareerDto>
}

pub async fn fetch_career(req: web::Path<Info>) -> HttpResponse {
  let repo = Arc::new(PgRepository::try_new().await);

  match execute(repo, Request { user_id: req.user_id }).await {
    Ok(res) => HttpResponse::Ok().json(Res {
      data: res.careers,
    }),
    Err(e) => match e {
      Error::BadRequest => HttpResponse::BadRequest().finish(),
      Error::Unknown => HttpResponse::InternalServerError().finish(),
    } 
  }
}