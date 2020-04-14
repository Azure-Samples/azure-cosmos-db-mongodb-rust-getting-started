---
page_type: sample
languages:
- rust
products:
- cosmos db
description: A sample to connect a Rust application with Azure Cosmos DB's API for MongoDB
urlFragment: azure-cosmos-db-mongodb-rust-getting-started
---

# Official Microsoft Sample

<!-- 
Guidelines on README format: https://review.docs.microsoft.com/help/onboard/admin/samples/concepts/readme-template?branch=master

Guidance on onboarding samples to docs.microsoft.com/samples: https://review.docs.microsoft.com/help/onboard/admin/samples/process/onboarding?branch=master

Taxonomies for products and languages: https://review.docs.microsoft.com/new-hope/information-architecture/metadata/taxonomies?branch=master
-->

This sample shows how to use the Azure Cosmos DB for MongoDB API and the Rust language to create, read, update, and delete a document in a collection.

## Contents

Outline the file contents of the repository. It helps users navigate the codebase, build configuration and any related assets.

| File/folder       | Description                                |
|-------------------|--------------------------------------------|
| `src`             | Sample source code.                        |
| `Cargo.toml`      | Define dependencies for the application.   |
| `.gitignore`      | Define what to ignore at commit time.      |
| `CHANGELOG.md`    | List of changes to the sample.             |
| `CONTRIBUTING.md` | Guidelines for contributing to the sample. |
| `README.md`       | This README file.                          |
| `LICENSE`         | The license for the sample.                |

## Prerequisites

*	[rust](https://www.rust-lang.org/tools/install) installed on your machine
*	[An Azure subscription](https://azure.microsoft.com/en-us/free/)
*	[Azure Cosmos DB account](https://docs.microsoft.com/en-us/azure/cosmos-db/create-mongodb-golang#create-a-database-account)
*	[VS Code](https://code.visualstudio.com/), [Sublime](https://www.sublimetext.com/), or your favorite code editor

## Setup

#### Clone the sample application

1.	In your terminal or git bash on windows, run the following command to clone the github repo.
  ```bash
  git clone https://github.com/Azure-Samples/azure-cosmos-db-mongodb-rust-getting-started
  ```
2. Navigate to the repo directory and build the code to download all the dependencies. 
  ```bash
  cargo build
  ```
  ***NOTE***: Do not run the application yet.
  
#### Update your connection string

Before you run the application, replace the connection string in `main.rs` file. 

1. To get the connection string for your Azure Cosmos DB account, navigate to Azure portal and go to your Cosmos DB account. 
    
2. Click on Connection String in the left navigation menu, and then copy one of the connection strings.

   **NOTE**: Double check if SSL is enabled (Cosmos DB rejects insecure incoming connections).
  	
3. In the sample application, edit the `main.rs` file in `src/` directory and update the connection string (from previous step), database name and collection name as shown in the following code snippet.
    ```rust
    fn main() -> Result<(), Error> {
        let db_name = "rust-cosmos-demo";
        let collection_name = "tasks";
        let connection_string = "cosmos-connection-string";
        ...
    }
    ```
4. Save the `main.rs` file.

## Running the sample

1.	Ensure that rust is installed and is invokable from the terminal/command prompt. Run `cargo build` to verify. 
2.	The code in `main.rs` file runs all CRUD operations one after the other. So to make sure the app successfully inserts a document,  comment out the code lines that deletes the document (lines 67-71). 
3.	In your terminal/command prompt, navigate to the root directory of the application where Cargo.toml file is and run the following command:
  
  ```bash
  cargo run
  ```
  
  If successful, the app will log result on the console.
    
4.	Review your documents in CosmosDB Data Explorer to ensure if the document was successfully inserted. 


## Key concepts

#### Connecting the rust application to Cosmos DB

The following code snippet (`src/db.rs`) connects the rust application to Azure Cosmos DB using the connection string provided. 

In the following code snippet, the struct `MongoClient` has a `connect` function that takes a connection string and connects to the mongo server using [ClientOptions](https://docs.rs/mongodb/0.9.2/mongodb/options/struct.ClientOptions.html
) and [Client](https://docs.rs/mongodb/0.9.2/mongodb/struct.Client.html) structs from mongodb rust driver. 

```rust
impl MongoClient {
    pub fn connect(connection_string: &str) -> Result<Self, Error> {
        /// parses the connection string and extract host information, token, tls configuration etc.
        let client_options = ClientOptions::parse(connection_string)?;
        /// Initialize a connection to Cosmos DB's mongo server
        let client = Client::with_options(client_options)?;

        Ok(Self { client })
    }
   …
}
```

#### Create a document

The following code snippets shows how to create a document in the rust application. 

In `main.rs` create an `InsertableTask` struct object with sample values.

```rust
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
```
***NOTE***: You can define your own custom struct object similar to `InsertableTask`. Refer to the example in `src/models/task.rs`.  

The code that actually inserts the document in the collection is defined in `src/db.rs`.

```rust
fn create(&self, doc: Document) -> Result<Bson, Error> {
        let collection = &self
            .client
            .get_db(&self.database)
            .get_collection(&self.collection);

        // inserts the given document in the collection
        match collection.insert_one(doc, None) {
            Ok(result) => Ok(result.inserted_id),
            Err(e) => Err(e),
        }
    }
```

#### Read a document

The following code snippet shows how to read a document from a collection.

In `main.rs`:

```rust
  /// document key to filter on
    let doc_filter = doc! {"title": "Pay AmeX bill"};

    let read_doc = mongo_client.read(doc_filter);

    if let Ok(doc) = read_doc {
        println!("{:?}", doc.unwrap());
    }
```

In `db.rs`:

```rust
  fn read(&self, doc: Document) -> Result<Option<Document>, Error> {
        let collection = &self
            .client
            .get_db(&self.database)
            .get_collection(&self.collection);

        collection.find_one(Some(doc), None)
}
``` 

#### Update a document

In `main.rs`:

```rust
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
``` 

In `db.rs`:

```rust
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
```

#### Delete a document

In `main.rs`:

```rust
  let delete_doc = doc! {"title" :"Pay AmeX bill"};

  let delete_result = mongo_client.delete(delete_doc).unwrap();

  println!("Docs deleted: {:?}", delete_result);
```

In `db.rs`:

```rust
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
```

## Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.
