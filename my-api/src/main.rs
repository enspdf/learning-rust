mod health;
mod repository;
mod user;
mod v1;

use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};
use tracing::{self as log};
use tracing_subscriber::EnvFilter;

use actix_web::{web, App, HttpServer};
use repository::PostgresRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let tracing = tracing_subscriber::fmt()
        // .with_span_events(
        //     tracing_subscriber::fmt::format::FmtSpan::ENTER
        //         | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        // )
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env());

    if cfg!(debug_assertions) {
        tracing.pretty().init();
    } else {
        tracing.json().init();
    }

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}", port);
    log::debug!("🚀 Server listening on: {}", address);

    let thread_counter = Arc::new(AtomicU16::new(1));
    let repo = PostgresRepository::from_env()
        .await
        .expect("Repository initialization error");
    let repo = web::Data::new(repo);

    HttpServer::new(move || {
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
        log::trace!("Starting thread {}", thread_index);

        App::new()
            .app_data(web::Data::new(thread_index))
            .app_data(repo.clone())
            .configure(v1::service::<PostgresRepository>)
            .configure(health::service)
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!(
            "🔥🔥🔥 Couldn't start the server  in port {}:{:?}",
            port, err
        )
    })
    .run()
    .await
}
