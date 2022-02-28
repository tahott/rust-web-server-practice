use crate::domain::user::entity::{ UserEmail, UserName };

pub struct Request {
  pub email: String,
  pub name: String,
}

pub struct Response {
  pub email: String,
  pub name: String,
}

pub enum Error {
  BadRequest,
}

pub fn execute(req: Request) -> Result<Response, Error> {
  match (
    UserEmail::try_from(req.email),
    UserName::try_from(req.name),
  ) {
    (Ok(email), Ok(name)) => Ok(Response {
      email: String::from(email),
      name: String::from(name),
    }),
    _ => Err(Error::BadRequest),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::domain::user::entity::{UserEmail, UserName};

  #[test]
  fn it_should_be_return_a_bad_request() {
    let req = Request::new(
    UserEmail::gmail(),
    UserName::bad(),
    );

    let res = execute(req);
    
    match res {
      Err(Error::BadRequest) => {},
      _ => unreachable!(),
    }
  }

  impl Request {
    fn new(email: UserEmail, name: UserName) -> Self {
      Self {
        email: String::from(email),
        name: String::from(name),
      }
    }
  }
}