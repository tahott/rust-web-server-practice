//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use chrono::{Utc, FixedOffset};
use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub login: String,
    pub name: String,
    pub avatar_url: String,
    pub email: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub channel: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::career::Entity")]
    Career,
}

impl Related<super::career::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Career.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
  fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
    let now = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));
    self.updated_at = Set(now);
    Ok(self)
  }
}
