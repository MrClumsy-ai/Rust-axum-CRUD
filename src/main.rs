use crate::{api::routes, database::connections, models::models::AppState};
use axum::{Router, routing::get};
use std::{
    panic,
    sync::{Arc, Mutex},
};
use tower_http::services::ServeDir;

mod api;
mod database;
mod models;

#[tokio::main]
async fn main() {
    const LISTENING: &str = "localhost:8080";
    let static_files = ServeDir::new("./static");
    let conn = match connections::connect_to_db("users.db").await {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
    let shared_state = Arc::new(Mutex::new(AppState {
        db_connection: conn,
    }));
    let app = Router::new()
        .route("/", get(routes::root))
        .route("/users", get(routes::get_users).post(routes::post_user))
        .route(
            "/users/{user_id}",
            get(routes::get_user_by_id)
                .put(routes::modify_user)
                .delete(routes::delete_user),
        )
        .with_state(shared_state)
        .nest_service("/static", static_files);
    let listener = tokio::net::TcpListener::bind(LISTENING).await.unwrap();
    println!("listening: {LISTENING}");
    axum::serve(listener, app).await.unwrap();
}
