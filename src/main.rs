#[macro_use] extern crate rocket;

mod songs;
use songs::Songs;
use rocket::serde::json::Json;



#[get("/Songs")]
fn get_songs() -> Json<Songs> {

    let song = Songs {
        name: "Acompáñame a Estar Solo".to_string(),
        genre: "Balada".to_string(),
        length: "4:43".to_string(),
        artist: "Ricardo Arjona".to_string()
    };

    Json(song);
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