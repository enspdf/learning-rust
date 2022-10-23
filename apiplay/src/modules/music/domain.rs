use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Song {
    pub name: String,
    pub author: String,
    pub duration_ms: u16,
}

#[derive(Debug, Serialize, Clone)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<Song>,
}
