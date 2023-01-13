use std::sync::Arc;

use actix_web::{web, HttpResponse};

use crate::{domain::user::{ update_user::{execute, Request}}, repositories::user::PgRepository};

pub async fn update_user(req: web::Json<Request>) -> HttpResponse {
  println!("{:?}", req);
  let repo = Arc::new(PgRepository::try_new().await);
  match execute(repo, req.0).await {
    Ok(res) => {HttpResponse::Ok().finish()},
    Err(_) => todo!(),
  }
}