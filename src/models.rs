pub mod models {
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct User {
        pub id: Option<u32>,
        pub name: String,
    }

    pub struct AppState {
        pub db_connection: rusqlite::Connection,
    }
}
