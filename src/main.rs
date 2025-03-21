mod data;
mod db;
mod models;
mod routes;
use data::populate_data;
use routes::app;

#[tokio::main]
async fn main() {
    // init_db().await.expect("Failed to init DB");

    // match get_db() {
    //     Some(db) => println!("Got database : {}", db.name()),
    //     None => println!("No database found"),
    // }

    // let user = User {
    //     id: None,
    //     name: "Alicia Bernel".to_string(),
    //     age: 25,
    // };

    // println!("User : {:?}", user);
    // let json = serde_json::to_string(&user).expect("failed to serialise");
    // println!("JSON: {}", json);

    // match populate_data().await {
    //     Ok(()) => println!("Data population completed!"),
    //     Err(e) => eprintln!("Error: {}", e),
    // }
    populate_data().await.expect("Failed to populate data");
    println!("Server running at http://localhost:3030");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await.unwrap();
    axum::serve(listener, app()).await.unwrap();

}
