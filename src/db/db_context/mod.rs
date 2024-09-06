use std::sync::Arc;

use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

use self::order_db_context::OrderDbContext;
use self::user_db_context::UserDbContext;
use self::user_token_db_context::UserTokenDbContext;

mod order_db_context;
mod user_db_context;
mod user_token_db_context;

#[derive(Clone)]
pub struct DbContext {
    user_db_context: UserDbContext,
    user_token_db_context: UserTokenDbContext,
    order_db_context: OrderDbContext,
}

impl DbContext {
    pub fn new(pool: Arc<Pool<AsyncPgConnection>>) -> Self {
        Self {
            user_db_context: UserDbContext::new(pool.clone()),
            user_token_db_context: UserTokenDbContext::new(pool.clone()),
            order_db_context: OrderDbContext::new(pool),
        }
    }

    pub fn user(&self) -> &UserDbContext {
        &self.user_db_context
    }

    pub fn user_token(&self) -> &UserTokenDbContext {
        &self.user_token_db_context
    }

    pub fn order(&self) -> &OrderDbContext {
        &self.order_db_context
    }
}
