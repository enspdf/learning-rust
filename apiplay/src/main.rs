use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use apiplay::config::read_config;
use apiplay::infra;
use apiplay::modules::music;
use apiplay::modules::music::domain::Playlist;
use apiplay::state::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = read_config();
    let stack: Vec<Playlist> = vec![];
    let playlists = Arc::new(Mutex::new(stack));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                playlist: playlists.clone(),
            }))
            .service(
                web::scope("/api")
                    .configure(infra::endpoints::config)
                    .configure(music::infra::endpoints::config),
            )
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
