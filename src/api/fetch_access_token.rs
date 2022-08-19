use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Body {
  code: String,
}

pub async fn fetch_access_token(req: web::Json<Body>) -> HttpResponse {
  println!("{:?}", req.code);
  HttpResponse::Ok().json("success")
}