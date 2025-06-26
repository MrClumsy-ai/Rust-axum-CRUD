use axum::{
    Router,
    extract::{Path, State},
    response::Json,
    routing::get,
};
use rusqlite::Connection;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct AppState {
    db_connection: Connection,
}

#[tokio::main]
async fn main() {
    let conn = match connect_to_db("users.db").await {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
    let shared_state = Arc::new(Mutex::new(AppState {
        db_connection: conn,
    }));
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(users).post(post_user))
        .route(
            "/users/{user_id}",
            get(user).put(put_user).delete(delete_user),
        )
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn connect_to_db(path: &str) -> Result<Connection, &str> {
    let conn = match Connection::open(path) {
        Ok(c) => c,
        Err(_e) => return Err("error opening path"),
    };
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL UNIQUE
         )",
        [],
    ) {
        Ok(_) => (),
        Err(_e) => return Err("error creating table users"),
    };
    Ok(conn)
}

async fn root() -> Json<Value> {
    Json(json!({"message": "this is the root route"}))
}

async fn users(State(state): State<Arc<Mutex<AppState>>>) -> Json<Value> {
    println!("{:?}", state);
    let app_state = match Arc::try_unwrap(state) {
        Ok(m) => match m.into_inner() {
            Ok(s) => s,
            Err(_e) => return Json(json!({"code": 500, "message": "error unwrapping mutex"})),
        },
        Err(_e) => return Json(json!({"code": 500, "message": "error unwrapping arc"})),
    };
    let mut statement = match app_state.db_connection.prepare("SELECT * FROM users") {
        Ok(s) => s,
        Err(_e) => return Json(json!({"code": 500, "message": "error preparing db request"})),
    };
    let users_iter = match statement.query_map([], |row| {
        Ok(User {
            id: match row.get(0) {
                Ok(id) => id,
                Err(_e) => 0,
            },
            name: match row.get(1) {
                Ok(name) => name,
                Err(_e) => "".to_string(),
            },
        })
    }) {
        Ok(r) => r,
        Err(_e) => return Json(json!({"code": 500, "message": "error creating users_iter"})),
    };
    let mut users: Vec<User> = Vec::new();
    for user in users_iter {
        users.push(match user {
            Ok(u) => u,
            Err(_e) => {
                return Json(json!({"code": 500, "message": "error iterating through users_iter"}));
            }
        });
    }
    Json(json!({"users": users}))
}

async fn post_user(Json(data): Json<serde_json::Value>) -> Json<Value> {
    Json(json!({"data": data}))
}

async fn user(Path(user_id): Path<u32>) -> Json<Value> {
    Json(json!({"id": user_id}))
}

async fn put_user(Path(user_id): Path<u32>, Json(data): Json<serde_json::Value>) -> Json<Value> {
    Json(json!({"id": user_id, "data": data}))
}

async fn delete_user(Path(user_id): Path<u32>) -> Json<Value> {
    Json(json!({"id": user_id}))
}
