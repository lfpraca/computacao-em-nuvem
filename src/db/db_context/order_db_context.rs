use diesel::{insert_into, prelude::*, update};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection, RunQueryDsl};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::types::order::{OrderForUser, OrderInsert, OrderWithUser};
use crate::errors::db_error::DbError;

#[derive(Clone)]
pub struct OrderDbContext {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl OrderDbContext {
    pub fn new(pool: Arc<Pool<AsyncPgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn create_order(&self, order_req: OrderInsert<'_>) -> Result<usize, DbError> {
        use crate::schema::order::dsl::*;

        let mut conn = self.pool.get().await?;

        Ok(insert_into(order)
            .values(&order_req)
            .execute(&mut conn)
            .await
            .inspect_err(|e| error!(error = ?&e, "Error creating order in database"))?)
    }

    pub async fn list_recent_for_user(
        &self,
        user_req: Uuid,
        limit: i64,
    ) -> Result<Vec<OrderForUser>, DbError> {
        use crate::schema::order::dsl::*;

        let mut conn = self.pool.get().await?;

        Ok(order
            .filter(user_id.eq(user_req))
            .order(date.desc())
            .limit(limit)
            .select(OrderForUser::as_select())
            .get_results(&mut conn)
            .await
            .inspect_err(|e| error!(error = ?&e, "Error listing orders from database"))?)
    }

    pub async fn get_attachment_extension(&self, id_req: Uuid) -> Result<String, DbError> {
        use crate::schema::order::dsl::*;

        let mut conn = self.pool.get().await?;

        Ok(order
            .find(id_req)
            .select(attachment_extension)
            .get_result(&mut conn)
            .await
            .inspect_err(
                |e| error!(error = ?&e, "Error getting order attachment extension from database"),
            )?)
    }

    pub async fn mark_delivered(&self, id_req: Uuid) -> Result<usize, DbError> {
        use crate::schema::order::dsl::*;

        let mut conn = self.pool.get().await?;

        Ok(update(order.find(id_req))
            .set(state.eq(2))
            .execute(&mut conn)
            .await
            .inspect_err(|e| error!(error = ?&e, "Error updating order state in database"))?)
    }

    pub async fn list_undelivered(&self) -> Result<Vec<OrderWithUser>, DbError> {
        use crate::schema::order::dsl::*;
        use crate::schema::user;

        let mut conn = self.pool.get().await?;

        Ok(order
            .filter(state.ne(2))
            .inner_join(user::table.on(user::id.eq(user_id)))
            .select((
                id,
                date,
                amount,
                state,
                user::name,
                user::phone_number,
                user::address,
            ))
            .get_results(&mut conn)
            .await
            .inspect_err(|e| error!(error = ?&e, "Error listing orders from database"))?)
    }
}
