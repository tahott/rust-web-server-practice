use std::future::{ready, Ready};

use actix_web::{
  body::EitherBody,
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
  http::{Method},
  Error, HttpResponse
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{Validation, DecodingKey};
use serde::Serialize;

use crate::domain::auth::entity::Claims;

const IGNORE_ROUTES: [&str; 2] = ["/authorization/code", "/signin"];

#[derive(Serialize)]
pub struct AuthRes {
  message: String
}

pub struct Authentication;
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type InitError = ();
  type Transform = AuthenticationMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
      ready(Ok(AuthenticationMiddleware { service }))
  }
}

pub struct AuthenticationMiddleware<S> {
  service: S
}
impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let mut authenticate_pass = false;

    if Method::OPTIONS == *req.method() {
      authenticate_pass = true;
    } else {
      for ignore_route in IGNORE_ROUTES.iter() {
        if req.path().starts_with(ignore_route) {
          authenticate_pass = true;
          break;
        }
      }

      if !authenticate_pass {
        if let Some(token) = req.headers().get("Authorization") {
          let jwt = jsonwebtoken::decode::<Claims>(token.to_str().unwrap(), &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).unwrap();
          println!("{:?}", jwt);
          authenticate_pass = true;
        }
      }
    }
    println!("Hi from start. You requested: {}", req.path());

    if authenticate_pass {
      let fut = self.service.call(req);
      Box::pin(async move {
        Ok(fut.await?.map_into_left_body())
      })
    } else {
      let (request, _pl) = req.into_parts();
      let response = HttpResponse::Unauthorized()
        .finish()
        .map_into_right_body();
      Box::pin(async { Ok(ServiceResponse::new(request, response)) })
    }
  }
}