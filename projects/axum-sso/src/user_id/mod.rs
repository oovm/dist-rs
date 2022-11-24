pub mod for_mem;
mod token;

use std::fmt::Debug;

use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub create_time: DateTime<Utc>,
    pub active_time: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        Self { id: Uuid::new_v4(), create_time: Utc::now(), active_time: Utc::now() }
    }
}

impl User {}

#[test]
fn test() {
    let user = User::default();

    println!("{:#?}", user);
}

pub struct Register {}
