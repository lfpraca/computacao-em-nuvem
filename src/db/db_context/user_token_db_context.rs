use diesel::{dsl::delete, insert_into, ExpressionMethods, JoinOnDsl, QueryDsl};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection, RunQueryDsl};
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::db_error::DbError;

#[derive(Clone)]
pub struct UserTokenDbContext {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl UserTokenDbContext {
    pub fn new(pool: Arc<Pool<AsyncPgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn register_token(&self, id_ins: &str, user_id_ins: Uuid) -> Result<usize, DbError> {
        use crate::schema::user_token::dsl::*;

        let mut conn = self.pool.get().await?;

        Ok(insert_into(user_token)
            .values((id.eq(id_ins), user_id.eq(user_id_ins)))
            .execute(&mut conn)
            .await
            .inspect_err(|e| error!(error = ?&e, "Error registering user token in database"))?)
    }

    pub async fn delete_token(&self, id_del: &str) -> Result<usize, DbError> {
        use crate::schema::user_token::dsl::*;

        let mut conn = self.pool.get().await?;

        Ok(delete(user_token.find(id_del))
            .execute(&mut conn)
            .await
            .inspect_err(|e| error!(error = ?&e, "Error deleting user from database"))?)
    }

    pub async fn fetch_user_token_details(&self, token_req: &str) -> Result<(Uuid, i16), DbError> {
        use crate::schema::user;
        use crate::schema::user_token::dsl::*;

        let mut conn = self.pool.get().await?;

        Ok(user_token
            .find(token_req)
            .inner_join(user::table.on(user::id.eq(user_id)))
            .select((user_id, user::role))
            .get_result(&mut conn)
            .await?)
    }
}
