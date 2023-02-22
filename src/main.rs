#[macro_use] extern crate rocket;

mod song;
use song::Song;

use rocket::serde::json::Json;

#[get("/Songs")]
fn get_songs() -> Json<Vec<Song>> {
    
    let songs: &[Song] = &[
        Song {
            name: "Acompáñame a Estar Solo".to_string(),
            genre: "Balada".to_owned(),
            length: "4:43".to_string(),
            artist: "Ricardo Arjona".to_owned()
        },
        Song {
            name: "Bohemian Rhapsody".to_owned(),
            genre: "Rock".to_owned(),
            length: "6:10".to_owned(),
            artist: "Queen".to_owned(),
        },
        Song {
            name: "Billie Jean".to_owned(),
            genre: "Pop".to_owned(),
            length: "4:09".to_owned(),
            artist: "Michael Jackson".to_owned(),
        },
        Song {
            name: "Stairway to Heaven".to_owned(),
            genre: "Rock".to_owned(),
            length: "8:03".to_owned(),
            artist: "Led Zeppelin".to_owned(),
        }
    ];
    
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