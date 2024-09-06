use diesel::{
    insert_into,
    prelude::*,
    result::{DatabaseErrorKind, Error as DieselError},
    QueryDsl,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection, RunQueryDsl};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::types::user::UserInsert;
use crate::errors::db_error::DbError;

#[derive(Clone)]
pub struct UserDbContext {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl UserDbContext {
    pub fn new(pool: Arc<Pool<AsyncPgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user_req: UserInsert<'_>) -> Result<Uuid, DbError> {
        use crate::schema::user::dsl::*;

        let mut conn = self.pool.get().await?;

        let res = insert_into(user)
            .values(&user_req)
            .returning(id)
            .get_result(&mut conn)
            .await;

        if let Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, ref e)) = res {
            if let Some("user_un_phone_number") = e.constraint_name() {
                return Err(DbError::ExpectedUniqueViolation {
                    constraint_name: "user_un_phone_number".into(),
                });
            }
        }

        Ok(res.inspect_err(|e| error!(error = ?&e, "Error creating user in database"))?)
    }

    pub async fn fetch_login_data(
        &self,
        phone_number_req: &str,
    ) -> Result<Option<(Uuid, String)>, DbError> {
        use crate::schema::user::dsl::*;

        let mut conn = self.pool.get().await?;

        Ok(user
            .filter(phone_number.eq(phone_number_req))
            .select((id, pass_hash))
            .get_result(&mut conn)
            .await
            .optional()
            .inspect_err(
                |e| error!(error = ?&e, "Error fetching user's pasword hash from database"),
            )?)
    }
}
