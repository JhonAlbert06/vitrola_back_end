use mongodb::{Client, options::ClientOptions, bson::{doc, oid::ObjectId}, Collection};
use serde::{Serialize, Deserialize};


mod song;
use song::Song;

pub struct MongoDb {
    client: Client,
    db_name: String,
    coll_name: String,
}

impl MongoDb {

    pub async fn new() -> Self {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        MongoDb {
            client,
            db_name: "songs_db".to_string(),
            coll_name: "songs".to_string(),
        }
    }

    pub fn collection(&self) -> Collection<Song> {
        self.client.database(&self.db_name).collection(&self.coll_name)
    }

    pub async fn disconnect(&self) {
        //self.client.shutdown().await.unwrap();
    }
}






/* let songss: &[Song] = &[
        Song {
            name: "Bohemian Rhapsody".to_owned(),
            genre: "Rock".to_owned(),
            length: "5:55".to_owned(),
            artist: "Queen".to_owned(),
        },
        Song {
            name: "Hotel California".to_owned(),
            genre: "Rock".to_owned(),
            length: "6:30".to_owned(),
            artist: "Eagles".to_owned(),
        },
        Song {
            name: "Stairway to Heaven".to_owned(),
            genre: "Rock".to_owned(),
            length: "8:03".to_owned(),
            artist: "Led Zeppelin".to_owned(),
        },
        Song {
            name: "Thriller".to_owned(),
            genre: "Pop".to_owned(),
            length: "5:57".to_owned(),
            artist: "Michael Jackson".to_owned(),
        },
        Song {
            name: "Smells Like Teen Spirit".to_owned(),
            genre: "Grunge".to_owned(),
            length: "5:01".to_owned(),
            artist: "Nirvana".to_owned(),
        },
        Song {
            name: "Sweet Child o' Mine".to_owned(),
            genre: "Rock".to_owned(),
            length: "5:56".to_owned(),
            artist: "Guns N' Roses".to_owned(),
        },
        Song {
            name: "Billie Jean".to_owned(),
            genre: "Pop".to_owned(),
            length: "4:54".to_owned(),
            artist: "Michael Jackson".to_owned(),
        },
        Song {
            name: "November Rain".to_owned(),
            genre: "Rock".to_owned(),
            length: "8:57".to_owned(),
            artist: "Guns N' Roses".to_owned(),
        },
        Song {
            name: "Purple Haze".to_owned(),
            genre: "Rock".to_owned(),
            length: "2:50".to_owned(),
            artist: "Jimi Hendrix".to_owned(),
        },
        Song {
            name: "Nothing Else Matters".to_owned(),
            genre: "Metal".to_owned(),
            length: "6:28".to_owned(),
            artist: "Metallica".to_owned(),
        },
    ]; 
*/