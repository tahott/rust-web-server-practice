use std::sync::Arc;

use crate::repositories::user::Repository;
use crate::domain::user::entity::{UserId, UserName, UserAvatar};

pub struct Request {
  pub id: i32,
  pub name: String,
  pub avatar_url: String,
}

pub async fn execute(repo: Arc<dyn Repository>, req: Request) {
  match (
    UserId::try_from(req.id),
    UserName::try_from(req.name),
    UserAvatar::try_from(req.avatar_url)
  ) {
    (Ok(id), Ok(name), Ok(avatar_url)) => match repo.update(id, name, avatar_url).await {
      Ok(_) => {},
      Err(_) => todo!(),
    },
    _ =>  todo!()
  }
}