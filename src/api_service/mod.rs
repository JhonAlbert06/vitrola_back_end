// External imports
use bson::{doc, Document};
use mongodb::results::{DeleteResult, UpdateResult, InsertOneResult};
use mongodb::{error::Error, Collection};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
// External constructors
extern crate serde;
extern crate serde_json;

// Estructure data for DB
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    genre: String,
    length: String,
    artist: String,
    image: String,
    music: String
}

pub static mut LIST_SONGS: Vec<Data> = Vec::new();


// Reference colection clone
#[derive(Clone)]
pub struct ApiService {
    collection: Collection,
}

// Transform data to mongo db document
fn data_to_document(data: &Data) -> Document {
    let Data {
        name,
        genre,
        length,
        artist,
        image,
        music
    } = data;
    doc! {
        "name": name,
        "genre": genre,
        "length": length,
        "artist": artist,
        "image": image,
        "music": music
    }
}

// Functions with quieries to Mongo
impl ApiService {
    
    pub fn new(collection: Collection) -> ApiService {
        ApiService { collection }
    }

    // Insert data to List
    pub fn create_song_to_list(&self, _data:&Data) -> Result<bool, Error> {

        let copy = Data {
            name: _data.name.to_string(),
            genre: _data.genre.to_string(),
            length:_data.length.to_string(),
            artist: _data.artist.to_string(),
            image: _data.image.to_string(),
            music: _data.music.to_string()
        };
        
        unsafe{LIST_SONGS.push(copy)}

        Ok( true )       
    }
    
    pub fn get_data_list_json() -> Result<String, serde_json::Error> {
        let data_list = unsafe { &LIST_SONGS };
        to_string(data_list)
    }

    // Insert data to Mongo DB
    pub fn create_song(&self, _data:&Data) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(data_to_document(_data), None)
    }

    // Update an existing document 
    pub fn update_song(&self, _data:&Data, _param: &String) -> Result<UpdateResult, Error> {
        let object_param = bson::oid::ObjectId::with_string(_param).unwrap();
        self.collection.update_one(doc! { "_id": object_param }, data_to_document(_data), None)
    }

    // Delete some document
    pub fn delete_song(&self, _title: &String) -> Result<DeleteResult, Error> {
        self.collection.delete_one(doc! { "title": _title }, None)
    }

    // Get all documents
    pub fn get_all_songs(&self) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.collection.find(None, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(docs)
    }

    // Get documents with quiery
    pub fn get_by_song(&self, param: &String) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.collection.find(doc! { "name": { "$regex": param } }, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        let _serialized = serde_json::to_string(&docs).unwrap();
        Ok(docs)
    }
}