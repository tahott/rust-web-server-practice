use std::sync::Arc;

use actix_web::{web, HttpResponse};

use crate::{domain::user::{ update_user::{execute, Request}, fetch_one_user::{Request as ReqFetchUser, execute as fetch_user_execute}}, repositories::user::PgRepository};

pub async fn update_user(req: web::Json<Request>) -> HttpResponse {
  let repo = Arc::new(PgRepository::try_new().await);
  match execute(repo, req.0).await {
    Ok(_) => {HttpResponse::Ok().finish()},
    Err(_) => todo!(),
  }
}

pub async fn fetch_user(req: web::Path<ReqFetchUser>) -> HttpResponse {
  let repo = Arc::new(PgRepository::try_new().await);
  match fetch_user_execute(repo, ReqFetchUser { id: req.id }).await {
    Ok(res) => HttpResponse::Ok().json(res),
    Err(_) => todo!(),
  }
}