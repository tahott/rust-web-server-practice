use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
          .create_table(
            Table::create()
              .table(User::Table)
              .if_not_exists()
              .col(
                ColumnDef::new(User::Id)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
              )
              .col(ColumnDef::new(User::Login).string().not_null())
              .col(ColumnDef::new(User::Name).string().not_null())
              .col(ColumnDef::new(User::AvatarUrl).string().not_null())
              .col(ColumnDef::new(User::Email).string().null())
              .col(ColumnDef::new(User::Channel).json().null())
              .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().not_null())
              .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().not_null())
              .to_owned()
          ).await?;

        manager
            .create_table(
              Table::create()
                .table(Career::Table)
                .if_not_exists()
                .col(
                  ColumnDef::new(Career::Id)
                    .big_integer().not_null().auto_increment().primary_key()
                )
                .col(ColumnDef::new(Career::UserId).big_integer().not_null())
                .col(ColumnDef::new(Career::Company).string().not_null())
                .col(ColumnDef::new(Career::Job).string().not_null())
                .col(ColumnDef::new(Career::InAt).date().not_null())
                .col(ColumnDef::new(Career::OutAt).date())
                .to_owned()
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
  Table,
  Id,
  Login,
  Name,
  AvatarUrl,
  Email,
  CreatedAt,
  UpdatedAt,
  Channel,  
}

#[derive(Iden)]
enum Career {
  Table,
  Id,
  UserId,
  Company,
  InAt,
  OutAt,
  Job,
}
