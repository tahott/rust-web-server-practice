use std::fmt::{Display, Formatter};

// use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
  EmptyString,
  MissingSeparator,
  LocalPartTooLong,
  DomainTooLong,
}

impl std::error::Error for Error {}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::EmptyString => write!(f, "Empty String"),
      Error::MissingSeparator => write!(f, "Missing separator character '{}'.", '@'),
      Error::LocalPartTooLong => write!(
        f,
        "Local part is too long. Length limit: {}",
        LOCAL_PART_MAX_LENGTH,
      ),
      Error::DomainTooLong => write!(
        f,
        "Domain is too long. Length limit: {}",
        DOMAIN_MAX_LENGTH,
      ),
    }
  }
}

impl<T> From<Error> for std::result::Result<T, Error> {
  fn from(err: Error) -> Self {
      Err(err)
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct UserEmail(String);

// https://www.rfc-editor.org/errata/eid1690
const LOCAL_PART_MAX_LENGTH: usize = 64;
const DOMAIN_MAX_LENGTH: usize = 254;

impl TryFrom<String> for UserEmail {
  type Error = Error;

  fn try_from(n: String) -> Result<Self, Self::Error> {
    if n.is_empty() {
      Err(Error::EmptyString)
    } else {
      parse_address(&n)
    }
  }
}

impl From<UserEmail> for String {
  fn from(n: UserEmail) -> Self {
    n.0
  }
}

#[cfg(test)]
impl UserEmail {
  pub fn gmail() -> Self {
    Self(String::from("test@gmail.com"))
  }

  pub fn bad() -> Self {
    Self(String::from(""))
  }
}

fn parse_address(address: &str) -> Result<UserEmail, Error> {
  let (local, domain) = split_at(address)?;
  parse_local_part(local)?;
  parse_domain(domain)?;
  Ok(UserEmail(address.to_owned()))
}

fn split_at(address: &str) -> Result<(&str, &str), Error> {
  match address.rsplit_once('@') {
    None => Error::MissingSeparator.into(),
    Some(left_right) => Ok(left_right),
  }
}

fn parse_local_part(part: &str) -> Result<(), Error> {
  if part.is_empty() {
    Error::EmptyString.into()
  } else if part.len() > LOCAL_PART_MAX_LENGTH {
    Error::LocalPartTooLong.into()
  } else {
    Ok(())
  }
}

fn parse_domain(part: &str) -> Result<(), Error> {
  if part.is_empty() {
    Error::EmptyString.into()
  } else if part.len() > DOMAIN_MAX_LENGTH {
    Error::DomainTooLong.into()
  } else {
    Ok(())
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct UserName(String);

impl TryFrom<String> for UserName {
  type Error = ();

  fn try_from(n: String) -> Result<Self, Self::Error> {
    if n.is_empty() {
      Err(())
    } else {
      Ok(Self(n))
    }
  }
}

impl From<UserName> for String {
  fn from(n: UserName) -> Self {
    n.0
  }
}

#[cfg(test)]
impl UserName {
  pub fn kent_back() -> Self {
    Self(String::from("kent back"))
  }

  pub fn bad() -> Self {
    Self(String::from(""))
  }
}

#[derive(Clone)]
pub struct User {
  pub email: UserEmail,
  pub name: UserName,
}

impl User {
  pub fn new(email: UserEmail, name: UserName) -> Self {
    Self {
      email,
      name,
    }
  }
}

