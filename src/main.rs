pub mod db;
pub mod models;
extern crate chrono;

use bson::UtcDateTime;
use bson::{doc, Document};
use chrono::prelude::*;
use db::*;
use models::task::*;
use mongodb::error::Error;

fn main() -> Result<(), Error> {
    let db_name = "rust-cosmos-demo";
    let collection_name = "tasks";
    let connection_string = "cosmos-connection-string";

    // Initialize mongo client using the given connection string
    let client = MongoClient::connect(&connection_string)?;

    let mongo_client = Mongo::new(db_name.to_string(), collection_name.to_string(), client);

    //************** Create a document in the collection **************//
    let task = InsertableTask::new(
        "Pay AmeX bill".to_string(),
        "Bill".to_string(),
        UtcDateTime(Utc::now()),
        UtcDateTime(Utc.ymd(2020, 04, 28).and_hms(12, 0, 9)),
        false,
    );

    let document: Document = task.into();

    let insert_result = mongo_client.create(document)?;

    println!("Inserted document id: {:?}", insert_result);

    //************** Read a document from the collection **************//
    let doc_filter = doc! {"title": "Pay AmeX bill"};

    let read_doc = mongo_client.read(doc_filter);

    if let Ok(doc) = read_doc {
        println!("Document retrieved: {:?}", doc.unwrap());
    }

    //************** Update a document int the collection **************//
    let update_filter = doc! {"title": "Pay AmeX bill"};

    let task_update = InsertableTask::new(
        "Pay AmeX bill".to_string(),
        "Bill".to_string(),
        UtcDateTime(Utc::now()),
        UtcDateTime(Utc.ymd(2020, 04, 28).and_hms(12, 0, 9)),
        true,
    );

    let update_doc: Document = task_update.into();

    let updated = mongo_client.update(update_filter, update_doc).unwrap();

    match updated {
        Some(result) => println!("Number of documents updated: {:?}", result),
        None => println!("Failed to update the document"),
    };

    //************** Delete a document from the collection **************//
    let delete_doc = doc! {"title" :"Pay AmeX bill"};

    let delete_result = mongo_client.delete(delete_doc).unwrap();

    println!("Number of documents deleted: {:?}", delete_result);

    Ok(())
}
