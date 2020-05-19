use super::config;
use bson::{doc, oid::ObjectId, UtcDateTime};
use chrono::Utc;
use mongodb::sync::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DrawResult {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: ObjectId,
    pub result: Vec<String>,
    pub result_date: UtcDateTime,
}

impl DrawResult {
    pub fn new() -> Self {
        let oid = ObjectId::new().expect("Object ID is not generated");
        DrawResult {
            id: oid,
            result: vec![],
            result_date: UtcDateTime(Utc::now()),
        }
    }
}

pub fn get_draw_results(
    client: Client,
    selected_date: chrono::DateTime<Utc>,
) -> Option<DrawResult> {
    let database = client.database(config::DB_NAME);
    let collection = database.collection(config::DRAW_RESULTS_COLLECTION);

    let result = match collection.find_one(doc! {"result_date":{"$eq":selected_date}}, None) {
        Ok(r) => r,
        Err(e) => panic!(e),
    };

    let docs = match result {
        Some(d) => d,
        None => return None,
    };

    let draw_result: DrawResult = match bson::from_bson(bson::Bson::Document(docs)) {
        Ok(d) => d,
        Err(e) => panic!(e),
    };

    return Some(draw_result);
}

pub fn insert_draw_result(client: Client, draw_result: DrawResult) -> () {
    let database = client.database(config::DB_NAME);
    let collection = database.collection(config::DRAW_RESULTS_COLLECTION);
    let bson = match bson::to_bson(&draw_result) {
        Ok(bson) => bson,
        Err(e) => panic!(e),
    }; // Serialize

    if let bson::Bson::Document(document) = bson {
        let _ = collection.insert_one(document, None);
        return;
    } else {
        panic!("Cannot insert ordered document into mongo DB");
    }
}
