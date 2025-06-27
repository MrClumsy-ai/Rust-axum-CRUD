use crate::{api::routes, database::connections, models::models::AppState};
use axum::{Router, routing::get};
use std::{
    panic,
    sync::{Arc, Mutex},
};

mod api;
mod database;
mod models;

#[tokio::main]
async fn main() {
    let conn = match connections::connect_to_db("users.db").await {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
    let shared_state = Arc::new(Mutex::new(AppState {
        db_connection: conn,
    }));
    let app = Router::new()
        .route("/", get(routes::root))
        .route("/users", get(routes::users).post(routes::post_user))
        .route(
            "/users/{user_id}",
            get(routes::user)
                .put(routes::put_user)
                .delete(routes::delete_user),
        )
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
