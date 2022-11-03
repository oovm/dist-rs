use std::fmt::{Debug, Formatter};

use chrono::{NaiveDateTime, TimeZone, Utc};
use uuid::Uuid;

pub struct User {
    id: Uuid,
    create_time: i64,
    active_time: i64,
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("create_time", &NaiveDateTime::from_timestamp(self.create_time, 0))
            .field("active_time", &NaiveDateTime::from_timestamp(self.active_time, 0))
            .finish()
    }
}

impl User {
    pub fn create_time(&self) -> NaiveDateTime {
        let sec = self.id.get_timestamp().map(|f| f.to_unix().0).unwrap_or_default();
        NaiveDateTime::from_timestamp(sec as i64, 0)
    }
    pub fn active_time(&self) {}
}

#[test]
fn test() {
    let now = Utc::now().naive_utc();
    let user = User { id: Uuid::new_v4(), create_time: now.timestamp(), active_time: now.timestamp() };

    println!("{:#?}", user);
    println!("{:#?}", user.create_time());
}

pub struct Register {}
