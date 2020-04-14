use std::sync::Arc;

use {
    bson::{Bson, Document},
    mongodb::{error::Error, options::ClientOptions, Client, Collection, Database},
};

/// `Crud` defines the interface for data operations
/// supported by Mongo database.
pub trait Crud {
    fn create(&self, doc: Document) -> Result<Bson, Error>;
    fn read(&self, doc: Document) -> Result<Option<Document>, Error>;
    fn update(&self, filter: Document, update: Document) -> Result<Option<i64>, Error>;
    fn delete(&self, doc: Document) -> Result<i64, Error>;
}

/// Represents a Mongo database and metadata needed to
/// perform CRUD operations.
pub struct Mongo {
    /// Name of the database to connect to.
    database: String,

    /// Name of the collection to connect to.
    collection: String,

    /// Handle to Mongo Client instance.
    client: Arc<MongoClient>,
}

impl Mongo {
    pub fn new(database: String, collection: String, client: MongoClient) -> Self {
        Self {
            database,
            collection,
            client: Arc::new(client),
        }
    }
}

impl Crud for Mongo {
    /// Creates a new document in the collection.
    /// Returns document id (ObjectId) if successfully inserted.
    fn create(&self, doc: Document) -> Result<Bson, Error> {
        let collection = &self
            .client
            .get_db(&self.database)
            .get_collection(&self.collection);

        match collection.insert_one(doc, None) {
            Ok(result) => Ok(result.inserted_id),
            Err(e) => Err(e),
        }
    }

    /// Reads a document from the collection.
    /// Returns the document if it exists, else returns `None`.
    fn read(&self, doc: Document) -> Result<Option<Document>, Error> {
        let collection = &self
            .client
            .get_db(&self.database)
            .get_collection(&self.collection);

        collection.find_one(Some(doc), None)
    }

    /// Updates a document in the collection.
    /// Returns number of documents updated.
    fn update(&self, filter: Document, update: Document) -> Result<Option<i64>, Error> {
        let collection = &self
            .client
            .get_db(&self.database)
            .get_collection(&self.collection);

        match collection.update_one(filter, update, None) {
            Ok(result) => Ok(Some(result.modified_count)),
            Err(e) => Err(e),
        }
    }

    /// Deletes a document from the collection.
    /// Returns number of documents deleted.
    fn delete(&self, doc: Document) -> Result<i64, Error> {
        let collection = &self
            .client
            .get_db(&self.database)
            .get_collection(&self.collection);

        match collection.delete_one(doc, None) {
            Ok(result) => Ok(result.deleted_count),
            Err(e) => Err(e),
        }
    }
}

pub struct MongoClient {
    client: Client,
}

impl MongoClient {
    /// Establishes a connection to the monogo server with the given connection string.
    pub fn connect(connection_string: &str) -> Result<Self, Error> {
        // parses the connection string and extract host information, token, tls configuration etc.
        let client_options = ClientOptions::parse(connection_string)?;
        // Initialize a connection to Cosmos DB's mongo server
        let client = Client::with_options(client_options)?;

        Ok(Self { client })
    }

    /// Gets handle to a database.
    /// Note: Creates a new database if it doesn't exist.
    fn get_db(&self, db_name: &str) -> MongoDatabase {
        let db = self.client.database(db_name);
        MongoDatabase::new(db)
    }
}

pub struct MongoDatabase {
    db: Database,
}

impl MongoDatabase {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Gets handle to a collection in the database.
    /// Note: Creates a new collection if it doesn't exist.
    pub fn get_collection(&self, collection: &str) -> Collection {
        self.db.collection(collection)
    }
}
