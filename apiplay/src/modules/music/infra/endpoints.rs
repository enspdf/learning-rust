use crate::state::State;

use super::super::domain::Playlist;
use super::dtos::{CreatePlaylist, Info};
use actix_web::{get, post, web, Responder};

#[get("/playlist")]
async fn playlist(data: web::Data<State>) -> impl Responder {
    let playlists = data.playlist.lock().expect("bad state");

    web::Json(playlists.clone())
}

#[get("/playlist/{id}")]
async fn get_playlist(info: web::Path<Info>, data: web::Data<State>) -> impl Responder {
    let playlists = data.playlist.lock().expect("bad state");

    let p1 = playlists[info.id].clone();

    web::Json(p1)
}

#[post("/playlist")]
async fn create_playlist(dto: web::Json<CreatePlaylist>, data: web::Data<State>) -> impl Responder {
    let mut playlists = data.playlist.lock().expect("bad state");

    let p1 = Playlist {
        name: dto.name.clone(),
        songs: vec![],
    };

    playlists.push(p1.clone());

    web::Json(p1)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(playlist);
    cfg.service(get_playlist);
    cfg.service(create_playlist);
}
