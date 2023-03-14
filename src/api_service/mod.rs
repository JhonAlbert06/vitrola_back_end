
use bson::oid::ObjectId;
// External imports
use bson::{doc, Document, Bson};
use mongodb::results::{DeleteResult, UpdateResult, InsertOneResult};
use mongodb::{error::Error, Collection};
use serde::{Deserialize, Serialize};
// External constructors
extern crate serde;
extern crate serde_json;

// Estructure data for DB
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub _id:  Option<Id>,
    pub name: String,
    genre: String,
    length: String,
    artist: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id {
    #[serde(rename = "$oid")]
    pub oid: String,
}

impl From<Id> for Bson {
    fn from(id: Id) -> Bson {
        Bson::ObjectId(ObjectId::with_string(&id.oid).unwrap())
    }
}


// Reference colection clone
#[derive(Clone)]
pub struct ApiServiceSong {
    collection: Collection,
}

#[derive(Clone)]
pub struct ApiServicePlayList {
    collection1: Collection,
}

// Transform data to mongo db document
fn data_to_document(data: &Data) -> Document {
    let Data {
        name,
        genre,
        length,
        artist,
        _id,
    } = data;

    match _id {
        Some(id) => doc! {
            "name": name,
            "genre": genre,
            "length": length,
            "artist": artist,
            "_id": id.clone(),
        },
        None => doc! {
            "name": name,
            "genre": genre,
            "length": length,
            "artist": artist,
        }
    }
}

// Functions with quieries to Mongo
impl ApiServiceSong {
    
    pub fn new(collection: Collection) -> ApiServiceSong {
        ApiServiceSong { collection }
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
    pub fn delete_song(&self, _id: &Option<Id>) -> Result<DeleteResult, Error> {
        let id_bson = _id.as_ref().map(|id| id.clone().into()).unwrap_or(Bson::Null);
        self.collection.delete_one(doc! { "_id": id_bson }, None)
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

// Functions with quieries to Mongo
impl ApiServicePlayList {
    
    pub fn new(collection1: Collection) -> ApiServicePlayList {
        ApiServicePlayList { collection1 }
    }

    // Insert data to Mongo DB
    pub fn create_song_list(&self, _data:&Data) -> Result<InsertOneResult, Error> {
        self.collection1.insert_one(data_to_document(_data), None)
    }


    pub fn delete_song_list(&self, _id: &Option<Id>) -> Result<DeleteResult, Error> {
        let id_bson = _id.as_ref().map(|id| id.clone().into()).unwrap_or(Bson::Null);
        self.collection1.delete_one(doc! { "_id": id_bson }, None)
    }
    

    // Get all documents
    pub fn get_all_songs_list(&self) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.collection1.find(None, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(docs)
    }

 
}