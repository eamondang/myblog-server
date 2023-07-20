use axum::{routing, Router, Server};
use dotenv::dotenv;
use eamon_core::config::Config;
use tracing::info;

#[tokio::main]
async fn main() {
  let subscriber = tracing_subscriber::fmt::Subscriber::builder().with_max_level(tracing::Level::TRACE).finish();
  tracing::subscriber::set_global_default(subscriber).expect("Cannot setting subscriber global");

  info!("Loading Environment");
  dotenv().ok();

  let cfg = Config::from_env().map_err(|e| tracing::error!("Loading Environment Error: {}", e.to_string())).unwrap();
  info!("Server is running on port: {}", &cfg.web.addr);

  let app = Router::new().route("/", routing::get(|| async { "Hello World!!" }));

  Server::bind(&cfg.web.addr.parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}
