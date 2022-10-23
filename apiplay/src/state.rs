use crate::modules::music::domain::Playlist;
use std::sync::{Arc, Mutex};

pub struct State {
    pub playlist: Arc<Mutex<Vec<Playlist>>>,
}
