use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(|| async { "this is the users route" }));
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "this is the root"
}
