use std::env;
use std::sync::Arc;
use axum::{Router, routing};

mod db;
mod models;
mod visits;
mod mq;

#[derive(Clone)]
struct AppState{
    pub pool: sqlx::PgPool,
    pub mq_channel: Arc<lapin::Channel>
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error, no DATABASE_URL in .env");
    let amqp_url = env::var("AMQP_URL").unwrap_or_else(|_|"amqp://guest:guest@localhost:5672".to_string());

    let pool = db::create_pool(&database_url).await;
    let (_mq_connection,mq_channel) = mq::create_channel(&amqp_url).await;

    let sub_channel = mq_channel.clone();
    let pool_clone = pool.clone();
    tokio::spawn(async move{
        mq::subscribe_to_visits(sub_channel,pool_clone).await;
    });

    let mq_channel_shared = Arc::new(mq_channel);

    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

    let app_state = AppState{pool: pool, mq_channel: mq_channel_shared};

    let app = Router::new()
        .route("/visits", routing::get(visits::visits_repository::get_all))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3120").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
