use std::net::SocketAddr;

use anyhow::Error;
use axum::{
    extract::DefaultBodyLimit,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::get,
    Router,
};
use bcrypt::hash;
use config::Config;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_appender::rolling;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

use crate::router::api_router;

pub mod config;
pub mod documentation;
pub mod router;
pub mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub config: Config,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!(
        r#"

'########:'##::::'##:'##::::'##::'######:::::'###:::::::'########:'########::'######::'##::::'##:
... ##..:: ##:::: ##: ##:::: ##:'##... ##:::'## ##::::::... ##..:: ##.....::'##... ##: ##:::: ##:
::: ##:::: ##:::: ##: ##:::: ##: ##:::..:::'##:. ##:::::::: ##:::: ##::::::: ##:::..:: ##:::: ##:
::: ##:::: #########: ##:::: ##:. ######::'##:::. ##::::::: ##:::: ######::: ##::::::: #########:
::: ##:::: ##.... ##: ##:::: ##::..... ##: #########::::::: ##:::: ##...:::: ##::::::: ##.... ##:
::: ##:::: ##:::: ##: ##:::: ##:'##::: ##: ##.... ##::::::: ##:::: ##::::::: ##::: ##: ##:::: ##:
::: ##:::: ##:::: ##:. #######::. ######:: ##:::: ##::::::: ##:::: ########:. ######:: ##:::: ##:
:::..:::::..:::::..:::.......::::......:::..:::::..::::::::..:::::........:::......:::..:::::..::

    "#
    );

    let info_log_file = rolling::daily("./logs", "log");

    let info_log = tracing_subscriber::fmt::layer()
        .compact()
        .with_writer(info_log_file)
        .with_level(true)
        .with_target(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_ansi(false)
        .json()
        .with_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "mbc_cms_api=debug,tower_http=debug,sqlx=debug".into()),
        ));

    tracing_subscriber::registry()
        .with(info_log)
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_writer(std::io::stdout)
                .with_level(true)
                .with_thread_names(true)
                .with_ansi(true)
                .with_filter(tracing_subscriber::EnvFilter::new(
                    std::env::var("RUST_LOG")
                        .unwrap_or_else(|_| "mbc_cms_api=debug,tower_http=debug".into()),
                )),
        )
        .init();

    let config: Config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(32)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("✅ Connected to database.");
            pool
        }
        Err(error) => {
            tracing::error!("🔥 Failed to connect to database: {}", error);
            std::process::exit(1);
        }
    };

    match sqlx::migrate!().run(&pool).await {
        Ok(_) => {
            tracing::info!("✅ Database migrated.");
        }
        Err(error) => {
            tracing::error!("🔥 Failed to run database migrations: {}", error);
            std::process::exit(1);
        }
    };

    let admin_user: Option<_> = sqlx::query!(
        r#"
            SELECT *
            FROM users
            WHERE email = $1
        "#,
        config.admin_email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|error: sqlx::Error| {
        tracing::error!("🔥 Failed to query database: {}", error);
        error
    })?;

    match admin_user {
        Some(_) => {}
        None => {
            tracing::info!("🔒 Admin Password: {}", config.admin_password);

            let hashed_password = hash(&config.admin_password, 4).map_err(|error| {
                tracing::error!("🔥 Failed to hash password: {}", error);
                error
            })?;

            sqlx::query!(
                r#"
                    INSERT INTO users (email, password, role)
                    VALUES ($1, $2, 'System Admin')
                "#,
                config.admin_email,
                hashed_password
            )
            .execute(&pool)
            .await
            .map_err(|error| {
                tracing::error!("🔥 Failed to query database: {}", error);
                error
            })?;

            tracing::info!("✅ Created admin user.");
        }
    }

    let app_state: AppState = AppState { db: pool, config };

    let api_router: Router = api_router(app_state).await;

    let cors_layer: CorsLayer = CorsLayer::new()
        .allow_origin(["http://localhost:3000".parse::<HeaderValue>()?])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION, ACCEPT])
        .allow_credentials(true);

    let router: Router = Router::new()
        .nest("/api", api_router)
        .route("/", get(routes::index::get_index))
        .fallback(routes::fallback::get_fallback)
        .layer(DefaultBodyLimit::max(100_000_000))
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http());

    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 4000));

    let tcp_listener: TcpListener = TcpListener::bind(address).await?;

    tracing::info!("🚀 HTTP Server listening on: http://{}", address);

    axum::serve(tcp_listener, router.into_make_service()).await?;

    Ok(())
}
