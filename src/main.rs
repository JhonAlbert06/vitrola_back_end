#[macro_use] extern crate rocket;

mod mongo_db;
use mongo_db::MongoDb;

mod song;
use song::Song;

#[get("/Songs")]
async fn get_songs() -> &'static str {

    let db = MongoDb::new().await;

    let song = Song {
        id: None,
        name: "Bohemian Rhapsody".to_owned(),
        genre: "Rock".to_owned(),
        length: "5:55".to_owned(),
        artist: "Queen".to_owned(),
    };


    "Get Songs!"
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


/* 
    let mut songs: Vec<Song> = Vec::new();
    
    let aux = Song {
        name: "Bohemian Rhapsody".to_owned(),
        genre: "Rock".to_owned(),
        length: "5:55".to_owned(),
        artist: "Queen".to_owned(),
    };

    songs.push(aux);

    Json(songs.to_vec()) 
*/