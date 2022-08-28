use oauth2::AuthorizationCode;
use serde::Deserialize;

use crate::domain::auth::entity::Authentication;

use super::entity::OAuthProvider;

#[derive(Debug, Deserialize)]
pub struct Request {
  pub provider: String,
  pub auth_code: String,
}

#[derive(Debug)]
pub enum Error {
  BadRequest,
}

pub async fn execute(req: Request) -> Result<String, Error> {
  match (
    OAuthProvider::try_from(req.provider),
    AuthorizationCode::try_from(AuthorizationCode::new(req.auth_code)),
  ) {
    (Ok(provider), Ok(auth_code)) => {
      let auth = Authentication::new(provider, auth_code);

      match auth.get_access_token().await {
        Ok(access_token) => {
          match auth.create_jwt(access_token).await {
            Ok(jwt) => Ok(jwt),
            Err(_) => Err(Error::BadRequest),
          }
        },
        Err(_) => todo!(),
      }
    },
    _ => Err(Error::BadRequest),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn it_should_be_return_a_bad_request_when_provider_valid() {
    let req = Request::new("google".to_string(), "test".to_string());

    let res = execute(req);

    match res.await {
      Err(Error::BadRequest) => {},
      _ => unreachable!(),
    }
  }

  impl Request {
    fn new(provider: String, auth_code: String) -> Self {
      Self {
        provider,
        auth_code,
      }
    }
  }
}