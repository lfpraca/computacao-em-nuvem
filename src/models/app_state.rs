use std::sync::Arc;

use reqwest::Client;

use crate::db::db_context::DbContext;
use crate::models::config::Config;

#[derive(Clone)]
pub struct AppState {
    db_context: DbContext,
    config: Arc<Config>,
    http_client: Client,
}

impl AppState {
    pub fn new(db_context: DbContext, config: Arc<Config>, http_client: Client) -> Self {
        Self {
            db_context,
            config,
            http_client,
        }
    }

    pub fn db_context(&self) -> &DbContext {
        &self.db_context
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn http_client(&self) -> &Client {
        &self.http_client
    }
}
