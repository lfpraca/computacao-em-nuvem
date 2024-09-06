use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::order)]
pub struct OrderInsert<'a> {
    pub id: Uuid,
    pub amount: i16,
    pub user_id: Uuid,
    pub attachment_extension: &'a str,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::order)]
pub struct OrderForUser {
    pub id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub date: OffsetDateTime,
    pub amount: i16,
    pub state: i16,
}

#[derive(Queryable, Serialize)]
pub struct OrderWithUser {
    pub id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub date: OffsetDateTime,
    pub amount: i16,
    pub state: i16,
    pub user_name: String,
    pub user_phone_number: String,
    pub user_address: String,
}
