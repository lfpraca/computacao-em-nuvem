#[macro_use]
extern crate tracing;

use std::{env, error::Error, fs, sync::Arc, time::Duration};

use axum::{middleware::from_fn_with_state, Router};
use db::db_context::DbContext;
use diesel::Connection;
use diesel_async::{
    async_connection_wrapper::AsyncConnectionWrapper,
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use models::{
    app_state::AppState,
    config::{Config, ConfigBuilder},
};
use reqwest::Client;
use routes::open_routes;
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::{middleware::auth::auth, routes::protected_routes};

mod db;
mod errors;
mod middleware;
mod models;
mod routes;
mod schema;
mod util;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let config = Arc::new(load_config()?);

    {
        let mut conn: AsyncConnectionWrapper<AsyncPgConnection> =
            AsyncConnectionWrapper::establish(config.database_url())?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| e.to_string())?;
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let pool = Pool::builder()
                .build(AsyncDieselConnectionManager::<AsyncPgConnection>::new(
                    config.database_url(),
                ))
                .await?;

            let http_client = Client::builder().timeout(Duration::from_secs(15)).build()?;

            let app_state = AppState::new(DbContext::new(pool.into()), config.clone(), http_client);

            let app = Router::new()
                .merge(protected_routes())
                .layer(from_fn_with_state(app_state.clone(), auth))
                .merge(open_routes())
                .with_state(app_state);

            let bind_addr = config.bind_addr();

            let listener = TcpListener::bind(bind_addr).await?;

            info!(bind_addr, "Starting server");

            axum::serve(listener, Router::new().nest("/api", app)).await?;

            Ok(())
        })
}

fn load_config() -> Result<Config, Box<dyn Error>> {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();

    toml::from_str::<ConfigBuilder>(&fs::read_to_string(exe_dir.join("config.toml"))?)?.build()
}
