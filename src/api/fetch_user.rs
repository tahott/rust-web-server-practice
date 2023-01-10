use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserBase {
  pub name: String
}

pub async fn fetch_user(_req: web::Json<UserBase>) -> HttpResponse {
  HttpResponse::Ok().finish()
}