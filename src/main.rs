#[macro_use] extern crate rocket;

mod mongo_db;
use mongo_db::connect_db;

mod song;
use song::Song;

use rocket::serde::json::Json;


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