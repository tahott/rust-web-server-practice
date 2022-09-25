use std::sync::Arc;

use actix_web::{HttpResponse, web};
use serde::Serialize;

use crate::{domain::career::create_career::{execute, Request}, repositories::career::PgRepository};

#[derive(Serialize)]
pub struct Res {
  pub data: String,
}

pub async fn create_career(req: web::Json<Request>) -> HttpResponse {
  let repo = Arc::new(PgRepository::try_new().await);

  match execute(repo, req.0).await {
    Ok(res) => {
      HttpResponse::Ok().json(Res {
        data: res.job,
      })
    },
    Err(_) => {
      HttpResponse::InternalServerError().json(Res {
        data: "internal server error".to_string()
      })
    },
  }
}