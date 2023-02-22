pub struct Songs {
    name: String,
    genre: String,
    length: String,
    artist: String
}

impl Songs {
    pub fn new(name: String, genre: String, length: String, artist: String) -> Songs {
        Songs { name, genre, length, artist }
    }
}