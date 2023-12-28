use crate::backend::database::schema::todos;
use chrono::prelude::Utc;
use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, Insertable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Default, Queryable, Selectable, Insertable)]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[allow(dead_code)]
impl Todo {
    pub fn new(title: String) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: Uuid::new_v4(),
            title: title,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn finish(&mut self) -> &mut Todo {
        self.completed = true;
        self.updated_at = Utc::now().naive_utc();
        self
    }

    pub fn update(&mut self, title: String) -> &mut Todo {
        self.title = title;
        self.updated_at = Utc::now().naive_utc();
        self
    }

    pub fn is_completed(&self) -> bool {
        self.completed == true
    }

    pub fn json(&self) {
        let todo = serde_json::to_string(&self).unwrap();
        println!("{}", todo);
    }
}
