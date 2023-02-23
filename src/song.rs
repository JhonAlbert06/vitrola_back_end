use bson::{oid::ObjectId, doc, Bson};
use serde::{Serialize, Deserialize};

use crate::mongo_db::MongoDb;

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    #[serde(rename = "_id")]
    id: Option<ObjectId>,
    name: String,
    genre: String,
    length: String,
    artist: String,
}

impl Song {

    pub async fn create(db: &MongoDb, song: Song) -> Option<Song> {
        let collection = db.collection();
        let result = collection.insert_one(song, None).await.unwrap();
        let id = result.inserted_id.as_object_id().unwrap();
        Self::find_by_id(db, &id).await
    }

    pub async fn find_all(db: &MongoDb) -> Vec<Song> {
        let collection = db.collection();
        collection.find(None, None).await.unwrap().map(|doc| bson::from_bson(Bson::Document(doc)).unwrap()).collect()
    }

    pub async fn find_by_id(db: &MongoDb, id: &ObjectId) -> Option<Song> {
        let collection = db.collection();
        let filter = doc! { "_id": id };
        collection.find_one(filter, None).await.unwrap().map(|doc| bson::from_bson(Bson::Document(doc)).unwrap())
    }

    pub async fn update(db: &MongoDb, song: Song) -> Option<Song> {
        let collection = db.collection();
        let filter = doc! { "_id": song.id.clone().unwrap() };
        let result = collection.replace_one(filter, song, None).await.unwrap();
        if result.modified_count == 0 {
            None
        } else {
            Self::find_by_id(db, &song.id.unwrap()).await
        }
    }

    pub async fn delete(db: &MongoDb, id: &ObjectId) -> bool {
        let collection = db.collection();
        let filter = doc! { "_id": id };
        let result = collection.delete_one(filter, None).await.unwrap();
        result.deleted_count > 0
    }
}

