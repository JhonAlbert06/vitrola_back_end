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

    // Create a new Song
    let song = Song::create(&db, song).await.unwrap();
    println!("Created Song: {:?}", song);

    // Find all Songs
    let songs = Song::find_all(&db).await;
    println!("All Songs: {:?}", songs);

    // Find a Song by ID
    let song_id = song.id.unwrap();
    let song = Song::find_by_id(&db, &song_id).await.unwrap();
    println!("Found Song: {:?}", song);

    // Update a Song
    let updated_song = Song {
        id: Some(song_id.clone()),
        name: "Another One Bites the Dust".to_string(),
        genre: "Rock".to_string(),
        length: "3:36".to_string(),
        artist: "Queen".to_string(),
    };
    
    let song = Song::update(&db, &updated_song).await.unwrap();
    println!("Updated Song: {:?}", song);

    // Delete a Song
    let deleted = Song::delete(&db, &song_id).await;
    println!("Song deleted: {}", deleted);

    db.disconnect().await;


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