use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use luna_orm::prelude::*;
use luna_orm_axum::handler::PostHandler;
use luna_orm_axum::router::get_post_router;
use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
struct User {
    #[PrimaryKey]
    id: i64,
    username: String,
}
async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[tokio::test]
async fn test_router() {
    let config = SqliteLocalConfig::new("workspace", "test.db");
    let database = SqliteDatabase::build(config).await.unwrap();
    let router = get_post_router::<SqliteDatabase, User>(&database, "/user");
}
