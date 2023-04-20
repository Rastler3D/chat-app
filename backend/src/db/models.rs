use diesel::Queryable;
use diesel::Insertable;
use serde::Serialize;
use chrono::NaiveDateTime;
use super::schema::messages;

#[allow(unused)]
#[allow(clippy::all)]
#[derive(Queryable,Insertable, Debug, Serialize)]

pub struct Message {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub user_id: i32,
    pub user_name: String,
    pub text: String,
    pub creation_date: NaiveDateTime,
}
