use oauth2::AuthorizationCode;

use super::entity::OAuthProvider;

pub struct Request {
  provider: String,
  auth_code: String,
}

pub struct Response {
  provider: OAuthProvider,
}

pub enum Error {
  BadRequest,
}

pub fn execute(req: Request) -> Result<Response, Error> {
  match (
    OAuthProvider::try_from(req.provider),
    AuthorizationCode::try_from(AuthorizationCode::new(req.auth_code)),
  ) {
    (Ok(provider), Ok(code)) => Ok(Response {
      provider,
    }),
    _ => Err(Error::BadRequest),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_be_return_a_bad_request_when_provider_valid() {
    let req = Request::new("google".to_string(), "test".to_string());

    let res = execute(req);

    match res {
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