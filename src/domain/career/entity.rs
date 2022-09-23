use chrono::{DateTime, Utc, FixedOffset};
use sea_orm::prelude::DateTimeWithTimeZone;

#[derive(Clone)]
pub struct CareerEntity {
  pub user_id: i32,
  pub company_name: String,
  pub job: String,
  pub in_at: DateTimeWithTimeZone,
  pub out_at: Option<DateTimeWithTimeZone>,
}

impl CareerEntity {
  pub fn new(user_id: i32, company_name: String, job: String, in_at: DateTime<FixedOffset>, out_at: Option<DateTime<FixedOffset>>) -> Self {
    Self {
      user_id,
      company_name,
      job,
      in_at,
      out_at,
    }
  }
}