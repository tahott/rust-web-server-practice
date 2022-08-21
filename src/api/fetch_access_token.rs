use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::domain::auth::fetch_access_token::{execute, Request};

#[derive(Debug, Deserialize)]
pub struct SignInDto {
  pub provider: String,
  pub auth_code: String,
}

#[derive(Serialize)]
pub struct Res {
  pub data: String,
}

pub async fn fetch_access_token(req: web::Json<Request>) -> HttpResponse {
  match execute(req.0).await {
    Ok(res) => HttpResponse::Ok().json(Res {
      data: res,
    }),
    Err(e) => {
      HttpResponse::InternalServerError().json(Res {
        data: "internal server error".to_string()
      })
    }
  }
}