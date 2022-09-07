use chrono::{DateTime, Utc};

pub enum Job {
  BackEnd,
  FrontEnd,
  Infra,
  DevOps,
  Data,
}

pub struct Career {
  pub user_id: i32,
  pub company_name: String,
  pub job: Job,
  pub in_at: DateTime<Utc>,
  pub out_at: Option<DateTime<Utc>>,
}

impl Career {
  pub fn new(user_id: i32, company_name: String, job: Job, in_at: DateTime<Utc>, out_at: Option<DateTime<Utc>>) -> Self {
    Self {
      user_id,
      company_name,
      job,
      in_at,
      out_at,
    }
  }
}