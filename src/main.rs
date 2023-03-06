// External imports
use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use api_service::{ApiServiceSong, ApiServicePlayList};

// External modules reference
mod api_router;
mod api_service;

// Api Service constructor
pub struct ServiceManagerSong {
    api: ApiServiceSong,
}

pub struct ServiceManagerPlayList {
    api: ApiServicePlayList,
}


// Api Servie Implementation
impl ServiceManagerSong {
    pub fn new(api: ApiServiceSong) -> Self {
        ServiceManagerSong { api }
    }
}

impl ServiceManagerPlayList {
    pub fn new(api: ApiServicePlayList) -> Self {
        ServiceManagerPlayList { api }
    }
}


// Service Manager constructor
pub struct AppStateSong {
    service_manager_song: ServiceManagerSong,
}

pub struct AppStatePlayList {
    service_manager_play_list: ServiceManagerPlayList,
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // init env
    dotenv().ok();

    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // Parse a connection string into an options struct.
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL is not in .env file");
    let client_options = ClientOptions::parse(&database_url).unwrap();

    // Get the reference to Mongo DB
    let client = Client::with_options(client_options).unwrap();

    // get the reference to the Data Base
    let database_name = env::var("DATABASE_NAME").expect("DATABASE NAME is not in .env file");
    let db = client.database(&database_name);

    // get the reference to the Collection
    let collection_name = env::var("USER_COLLECTION_SONGS").expect("COLLECTION NAME is not in .env file");
    let collection_song = db.collection(&collection_name);

    let collection_name1 = env::var("USER_COLLECTION_PLAYLIST").expect("COLLECTION NAME is not in .env file");
    let collection_play_list = db.collection(&collection_name1);

    // Gte the server URL
    let server_url = env::var("SERVER_URL").expect("SERVER URL is not in .env file");

    // Start the server
    HttpServer::new(move || {

        let service_worker_song = ApiServiceSong::new(collection_song.clone());
        let service_manager_song = ServiceManagerSong::new(service_worker_song);

        let service_worker_play_list = ApiServicePlayList::new(collection_play_list.clone());
        let service_manager_play_list = ServiceManagerPlayList::new(service_worker_play_list);

        // cors
        let cors_middleware = Cors::new()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
            .finish();

        // Init http server
        App::new()
            .wrap(cors_middleware)
            .wrap(middleware::Logger::default())
            .data(AppStateSong { service_manager_song })
            .data(AppStatePlayList { service_manager_play_list })
            .configure(api_router::init)
    })
    .bind(server_url)?
    .run()
    .await
}