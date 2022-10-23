use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub id: usize,
}

#[derive(Deserialize)]
pub struct CreatePlaylist {
    pub name: String,
}
