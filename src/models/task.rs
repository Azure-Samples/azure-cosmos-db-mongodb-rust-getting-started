use bson::{oid::ObjectId, to_bson, Document, UtcDateTime};
use serde::{Deserialize, Serialize};
use std::convert::From;

/// Task Model to represent a document in  the database collection.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    #[serde(rename = "_id")]
    id: ObjectId,
    title: String,
    category: String,
    date_created: UtcDateTime,
    due_date: UtcDateTime,
    completed: bool,
}

/// Model to represent Task before it gets inserted into the collection,
/// since the id is automatically generated when the task is created in the collection.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableTask {
    title: String,
    category: String,
    date_created: UtcDateTime,
    due_date: UtcDateTime,
    completed: bool,
}

impl InsertableTask {
    pub fn new(
        title: String,
        category: String,
        date_created: UtcDateTime,
        due_date: UtcDateTime,
        completed: bool,
    ) -> Self {
        Self {
            title,
            category,
            date_created,
            due_date,
            completed,
        }
    }
}

/// Conversion functions to convert models to Document type.
impl From<InsertableTask> for Document {
    fn from(task: InsertableTask) -> Self {
        let bson_val = to_bson(&task).unwrap();
        let doc_val = bson_val.as_document().unwrap();
        doc_val.clone()
    }
}

impl From<Task> for InsertableTask {
    fn from(task: Task) -> Self {
        Self {
            title: task.title,
            category: task.category,
            date_created: task.date_created,
            due_date: task.due_date,
            completed: task.completed,
        }
    }
}
