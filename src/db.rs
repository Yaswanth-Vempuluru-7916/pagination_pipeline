use mongodb::{bson::doc, Client, Database};
use std::error::Error;
use tokio::sync::OnceCell;

// Global static OnceCell to hold the MongoDB client
static DB_CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn init_db() -> Result<Database, Box<dyn Error>> {
    // MongoDB connection string
    let uri = "mongodb://localhost:27017";

    // Initialize the client (runs only once)
    let client = DB_CLIENT
        .get_or_init(|| async {
            Client::with_uri_str(uri)
                .await
                .expect("Failed to connect to MongoDB")
        })
        .await;

    // Get the database handle
    let db = client.database("pagination_db");

    // Test the connection
    db.run_command(doc! {"ping": 1}).await?;
    println!("Connected to MongoDB successfully!");
    Ok(db)
}

pub fn get_db() -> Option<Database> {
    DB_CLIENT
        .get()
        .map(|client| client.database("pagination_db"))
}
