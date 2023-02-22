#[macro_use] extern crate rocket;

mod songs;
use songs::Songs;

use rocket::serde::json::Json;


/* const SONGS: &[Songs] = &[
    Songs {
        name: "Acompáñame a Estar Solo".to_string(),
        genre: "Balada".to_owned(),
        length: "4:43".to_string(),
        artist: "Ricardo Arjona".to_owned()
    },
    Songs {
        name: "Bohemian Rhapsody".to_owned(),
        genre: "Rock".to_string(),
        length: "4:43".to_owned(),
        artist: "Queen".to_string()
    },
]; */

#[get("/Songs")]
fn get_songs() -> Json<Vec<Songs>> {
    let SONGS: &[Songs] = &[
        Songs {
            name: "Acompáñame a Estar Solo".to_string(),
            genre: "Balada".to_owned(),
            length: "4:43".to_string(),
            artist: "Ricardo Arjona".to_owned()
        },
        Songs {
            name: "Bohemian Rhapsody".to_owned(),
            genre: "Rock".to_string(),
            length: "4:43".to_owned(),
            artist: "Queen".to_string()
        },
    ];
    
    Json(SONGS.to_vec())
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