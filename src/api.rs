pub mod routes {
    use std::sync::{Arc, Mutex};

    use axum::{
        Json,
        extract::{Path, State},
    };
    use rusqlite::Connection;
    use serde_json::{Value, json};

    use crate::{User, database::connections};

    pub struct AppState {
        pub db_connection: Connection,
    }

    pub async fn root() -> Json<Value> {
        Json(json!({"message": "this is the root route"}))
    }

    pub async fn users(State(state): State<Arc<Mutex<AppState>>>) -> Json<Value> {
        println!("GET /users");
        let users = match connections::get_all_users(state).await {
            Ok(r) => r,
            Err(e) => panic!("{e}"),
        };
        Json(json!({"users": users}))
    }

    pub async fn post_user(
        State(state): State<Arc<Mutex<AppState>>>,
        Json(data): Json<User>,
    ) -> Json<Value> {
        println!("POST /users");
        println!("{:?}", data);
        let user = match connections::post_user(state, data).await {
            Ok(u) => u,
            Err(e) => panic!("{e}"),
        };
        Json(json!({"user": user}))
    }

    pub async fn user(
        State(state): State<Arc<Mutex<AppState>>>,
        Path(user_id): Path<u32>,
    ) -> Json<Value> {
        println!("GET /users/{user_id}");
        let state = match state.lock() {
            Ok(s) => s,
            Err(_e) => return Json(json!({"code": 500, "message": "error locking state"})),
        };
        let mut statement = match state
            .db_connection
            .prepare("select * from users where id = (?1)")
        {
            Ok(s) => s,
            Err(_e) => panic!("{_e}"),
        };
        let users_iter = match statement.query_map([user_id], |r| {
            Ok(User {
                id: match r.get(0) {
                    Ok(i) => Some(i),
                    Err(_) => None,
                },
                name: match r.get(1) {
                    Ok(n) => n,
                    Err(_) => "".to_string(),
                },
            })
        }) {
            Ok(r) => r,
            Err(e) => panic!("{e}"),
        };
        let mut users: Vec<User> = Vec::new();
        for u in users_iter {
            users.push(match u {
                Ok(r) => r,
                Err(e) => panic!("{e}"),
            });
            break;
        }
        if users.len() == 0 {
            return Json(json!({"code": 404, "message": "user not found"}));
        }
        Json(json!({"user": users[0]}))
    }

    pub async fn put_user(
        State(_state): State<Arc<Mutex<AppState>>>,
        Path(user_id): Path<u32>,
        Json(data): Json<serde_json::Value>,
    ) -> Json<Value> {
        println!("GET /users/{user_id}");
        Json(json!({"id": user_id, "data": data}))
    }

    pub async fn delete_user(
        State(_state): State<Arc<Mutex<AppState>>>,
        Path(user_id): Path<u32>,
    ) -> Json<Value> {
        println!("GET /users/{user_id}");
        Json(json!({"id": user_id}))
    }
}
