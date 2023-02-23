#[macro_use] extern crate rocket;

mod song;
mod envs;
use song::Song;

use rocket::serde::json::Json;

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

#[get("/Songs")]
fn get_songs() -> Json<Vec<Song>> {

    let mut songs: Vec<Song> = Vec::new();
    
    let aux = Song {
        name: "Bohemian Rhapsody".to_owned(),
        genre: "Rock".to_owned(),
        length: "5:55".to_owned(),
        artist: "Queen".to_owned(),
    };

    songs.push(aux);

    Json(songs.to_vec())
}



#[post("/Songs")]
fn post_songs() -> &'static str {
    "Post Songs!"
}

#[put("/Songs")]
fn update_songs() -> &'static str {
    "Update Songs!"
}

#[delete("/Songs")]
fn delete_songs() -> &'static str {
    "Delete Songs!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_songs, post_songs, update_songs, delete_songs])
}