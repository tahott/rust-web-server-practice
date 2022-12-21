use chrono::{NaiveDate};

#[derive(Clone)]
pub struct CareerEntity {
  pub user_id: i64,
  pub company: String,
  pub job: String,
  pub in_at: NaiveDate,
  pub out_at: Option<NaiveDate>,
}

impl CareerEntity {
  pub fn new(user_id: i64, company: String, job: String, in_at: NaiveDate, out_at: Option<NaiveDate>) -> Self {
    Self {
      user_id,
      company,
      job,
      in_at,
      out_at,
    }
  }
}