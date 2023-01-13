use std::sync::Arc;

use serde::Deserialize;

use crate::repositories::user::Repository;
use crate::domain::user::entity::{UserId, UserName, UserAvatar};

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Request {
  pub id: i64,
  pub name: String,
  pub avatar_url: String,
}

pub async fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<(), ()> {
  match (
    UserId::try_from(req.id),
    UserName::try_from(req.name),
    UserAvatar::try_from(req.avatar_url)
  ) {
    (Ok(id), Ok(name), Ok(avatar_url)) => match repo.update(id, name, avatar_url).await {
      Ok(res) => Ok(()),
      Err(_) => todo!(),
    },
    _ =>  todo!()
  }
}