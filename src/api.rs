pub mod routes {
    use crate::database::connections;
    use crate::models::models::{AppState, User};
    use axum::{
        Json,
        extract::{Path, State},
    };
    use serde_json::{Value, json};
    use std::sync::{Arc, Mutex};

    pub async fn root() -> Json<Value> {
        println!("GET /");
        Json(json!({"message": "this is the root route"}))
    }

    pub async fn get_users(State(state): State<Arc<Mutex<AppState>>>) -> Json<Value> {
        println!("GET /users");
        let users = match connections::get_all_users(state).await {
            Ok(r) => r,
            Err(e) => panic!("{e}"),
        };
        Json(json!({"users": users}))
    }

    pub async fn get_user_by_id(
        State(state): State<Arc<Mutex<AppState>>>,
        Path(user_id): Path<u32>,
    ) -> Json<Value> {
        println!("GET /users/{user_id}");
        let user = match connections::get_user_by_id(state, user_id).await {
            Ok(u) => u,
            Err(e) => panic!("{e}"),
        };
        if user.id == None {
            return Json(json!({"code": 404, "message": "user not found"}));
        }
        Json(json!({"user": user}))
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

    pub async fn modify_user(
        State(_state): State<Arc<Mutex<AppState>>>,
        Path(user_id): Path<u32>,
        Json(data): Json<serde_json::Value>,
    ) -> Json<Value> {
        println!("GET /users/{user_id}");
        // TODO: database modify_user(state, user_id, data)
        Json(json!({"id": user_id, "data": data}))
    }

    pub async fn delete_user(
        State(_state): State<Arc<Mutex<AppState>>>,
        Path(user_id): Path<u32>,
    ) -> Json<Value> {
        println!("GET /users/{user_id}");
        // TODO: database delete_user(state, user_id)
        Json(json!({"id": user_id}))
    }
}
