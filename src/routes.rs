use axum::{routing::get, Router, extract::Query, response::Json};
use serde::Deserialize;
use mongodb::bson::{doc, Document};
use crate::db::get_db;
use crate::models::User;
use futures:: StreamExt as _;
use mongodb::bson::from_document; // Add this import

#[derive(Debug, Deserialize)]
struct PaginationParams {
    page: Option<u32>,
    limit: Option<u32>,
    sort_by : Option<String>,
    sort_order : Option<String> //asc or desc
}


async fn get_users(Query(params): Query<PaginationParams>) -> Result<Json<Vec<User>>, String> {
    let db = get_db().ok_or("Database not initialized")?;
    let collection = db.collection::<User>("users");

    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(10).min(50).max(1);

    let skip = (page - 1) * limit;

    let sort_field = params.sort_by.unwrap_or("name".to_string());
    let sort_order = match params.sort_order.as_deref(){
        Some("desc")=>-1,
        _=>1,
    };


    let pipeline = vec![
        doc! { "$sort": { &sort_field: sort_order } },
        doc! {"$skip": skip as i64},
        doc! {"$limit": limit as i64},
    ];

    let mut cursor = collection
        .aggregate(pipeline) // Pass the pipeline 
        .await
        .map_err(|e| e.to_string())?;

    let mut users = Vec::new();

    while let Some(result) = cursor.next().await {
        let doc: Document = result.map_err(|e| e.to_string())?;
        let user: User = from_document(doc).map_err(|e| e.to_string())?; // Deserialize Document into User
        users.push(user);
    }

    Ok(Json(users))
}

// Build the router
pub fn app() -> Router {
    Router::new()
        .route("/users", get(get_users))
}