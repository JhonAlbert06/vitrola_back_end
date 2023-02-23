use bson::doc;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

use crate::song::Song;

#[tokio::main]
pub async fn connect_db() -> Result<(), Box<dyn Error>> {

    Ok(())
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