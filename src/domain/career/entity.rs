use chrono::{NaiveDate};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct CareerEntity {
  pub user_id: i32,
  pub company_name: String,
  pub job: String,
  pub in_at: NaiveDate,
  pub out_at: Option<NaiveDate>,
}

impl CareerEntity {
  pub fn new(user_id: i32, company_name: String, job: String, in_at: NaiveDate, out_at: Option<NaiveDate>) -> Self {
    Self {
      user_id,
      company_name,
      job,
      in_at,
      out_at,
    }
  }
}