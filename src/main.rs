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
    id: Option<u32>,
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
             name TEXT NOT NULL
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
    println!("GET /users");
    let state = match state.lock() {
        Ok(r) => r,
        Err(_e) => return Json(json!({"code": 500, "message": "error locking state"})),
    };
    let mut statement = match state.db_connection.prepare("SELECT * FROM users") {
        Ok(s) => s,
        Err(_e) => return Json(json!({"code": 500, "message": "error preparing db request"})),
    };
    let users_iter = match statement.query_map([], |row| {
        Ok(User {
            id: match row.get(0) {
                Ok(id) => Some(id),
                Err(_e) => None,
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

async fn post_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(data): Json<User>,
) -> Json<Value> {
    println!("POST /users");
    println!("{:?}", data);
    let state = match state.lock() {
        Ok(r) => r,
        Err(_e) => return Json(json!({"code": 500, "message": "error locking state"})),
    };
    println!("insert into users (name) values ({})", data.name);
    match state
        .db_connection
        .execute("INSERT INTO users (name) VALUES (?1)", [data.name])
    {
        Ok(_) => (),
        Err(_e) => {
            return Json(json!({"code": 500, "message": "error executing insert"}));
        }
    };
    let mut statement = match state
        .db_connection
        .prepare("SELECT * FROM users ORDER BY ROWID DESC LIMIT 1")
    {
        Ok(s) => s,
        Err(_e) => return Json(json!({"code": 500, "message": "error preparing db request"})),
    };
    let users_iter = match statement.query_map([], |row| {
        Ok(User {
            id: match row.get(0) {
                Ok(id) => Some(id),
                Err(_e) => None,
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
    let mut user: User = User {
        id: None,
        name: "".to_string(),
    };
    for u in users_iter {
        user = match u {
            Ok(u) => u,
            Err(_e) => {
                return Json(json!({"code": 500, "message": "error iterating through users_iter"}));
            }
        };
    }
    Json(json!({"user": user}))
}

async fn user(State(state): State<Arc<Mutex<AppState>>>, Path(user_id): Path<u32>) -> Json<Value> {
    println!("GET /users/{user_id}");
    let state = match state.lock() {
        Ok(s) => s,
        Err(_e) => return Json(json!({"code": 500, "message": "error locking state"})),
    };
    let id: Result<u32, rusqlite::Error> =
        state
            .db_connection
            .query_one("select * from users where id = (?1)", [user_id], |r| {
                r.get(0)
            });
    let name: Result<String, rusqlite::Error> =
        state
            .db_connection
            .query_one("select * from users where id = (?1)", [user_id], |r| {
                r.get(1)
            });
    let id = match id {
        Ok(r) => r,
        Err(e) => panic!("{e}"),
    };
    let name = match name {
        Ok(n) => n,
        Err(e) => panic!("{e}"),
    };
    let user = User {
        id: Some(id),
        name: name,
    };
    Json(json!({"user": user}))
}

async fn put_user(
    State(_state): State<Arc<Mutex<AppState>>>,
    Path(user_id): Path<u32>,
    Json(data): Json<serde_json::Value>,
) -> Json<Value> {
    println!("GET /users/{user_id}");
    Json(json!({"id": user_id, "data": data}))
}

async fn delete_user(
    State(_state): State<Arc<Mutex<AppState>>>,
    Path(user_id): Path<u32>,
) -> Json<Value> {
    println!("GET /users/{user_id}");
    Json(json!({"id": user_id}))
}
