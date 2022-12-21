use std::{fmt::{Display, Formatter}};
use chrono::{FixedOffset, Utc};
use sea_orm::{FromQueryResult, prelude::DateTimeWithTimeZone};
use serde::Deserialize;

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

const USER_ID_MAX_LENGTH: i64 = 2_147_483_647;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct UserId(i64);

impl TryFrom<i64> for UserId {
  type Error = ();

  fn try_from(n: i64) -> Result<Self, Self::Error> {
    if n > USER_ID_MAX_LENGTH || n < 0 {
      Err(())
    } else {
      Ok(Self(n))
    }
  }
}

impl From<UserId> for i64 {
  fn from(n: UserId) -> Self {
    n.0
  }
}

#[cfg(test)]
impl UserId {
  pub fn one() -> Self {
    Self(i64::from(443))
  }

  pub fn two() -> Self {
    Self(i64::from(3000))
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserLogin(String);

impl TryFrom<String> for UserLogin {
  type Error = ();

  fn try_from(n: String) -> Result<Self, Self::Error> {
    if n.is_empty() {
      Err(())
    } else {
      Ok(Self(n))
    }
  }
}

impl From<UserLogin> for String {
  fn from(n: UserLogin) -> Self {
    n.0
  }
}

#[cfg(test)]
impl UserLogin {
  pub fn kent_back() -> Self {
    Self(String::from("kent-back"))
  }

  pub fn bad() -> Self {
    Self(String::from(""))
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

#[derive(Clone, PartialEq, Debug)]
pub struct UserAvatar(String);

impl TryFrom<String> for UserAvatar {
  type Error = ();

  fn try_from(n: String) -> Result<Self, Self::Error> {
    if n.is_empty() {
      Err(())
    } else {
      Ok(Self(n))
    }
  }
}

impl From<UserAvatar> for String {
  fn from(n: UserAvatar) -> Self {
    n.0
  }
}

#[cfg(test)]
impl UserAvatar {
  pub fn user() -> Self {
    Self(String::from("avatar_url"))
  }
}

#[derive(Clone, Debug, Deserialize, FromQueryResult)]
pub struct UserEntity {
  pub id: i64,
  pub login: String,
  pub name: String,
  pub avatar_url: String,
  pub email: Option<String>,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: DateTimeWithTimeZone,
}

impl UserEntity {
  pub fn new(id: UserId, login: UserLogin, name: UserName, avatar_url: UserAvatar) -> Self {
    let now = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));
    Self {
      id: i64::from(id),
      login: String::from(login),
      name: String::from(name),
      avatar_url: String::from(avatar_url),
      email: None,
      created_at: now,
      updated_at: now,
    }
  }
}