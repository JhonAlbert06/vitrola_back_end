use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub name: String,
    pub genre: String,
    pub length: String,
    pub artist: String
}
