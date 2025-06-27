pub mod connections {
    use crate::models::models::{AppState, User};
    use rusqlite::{Connection, Error};
    use std::sync::{Arc, Mutex};

    pub async fn connect_to_db(path: &str) -> Result<Connection, &str> {
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

    pub async fn get_all_users(state: Arc<Mutex<AppState>>) -> Result<Vec<User>, Error> {
        let state = match state.lock() {
            Ok(r) => r,
            Err(_) => return Err(Error::UnwindingPanic),
        };
        let mut statement = match state.db_connection.prepare("SELECT * FROM users") {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        let users_iter = match statement.query_map([], |row| {
            Ok(User {
                id: match row.get(0) {
                    Ok(id) => Some(id),
                    Err(e) => return Err(e),
                },
                name: match row.get(1) {
                    Ok(name) => name,
                    Err(e) => return Err(e),
                },
            })
        }) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let mut users: Vec<User> = Vec::new();
        for user in users_iter {
            users.push(match user {
                Ok(u) => u,
                Err(e) => return Err(e),
            });
        }
        Ok(users)
    }

    pub async fn post_user(state: Arc<Mutex<AppState>>, user: User) -> Result<User, Error> {
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return Err(Error::UnwindingPanic),
        };
        match state
            .db_connection
            .execute("INSERT INTO users (name) VALUES (?1)", [user.name])
        {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        let mut statement = match state
            .db_connection
            .prepare("SELECT * FROM users ORDER BY ROWID DESC LIMIT 1")
        {
            Ok(s) => s,
            Err(e) => return Err(e),
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
            Err(e) => return Err(e),
        };
        let mut user: User = User {
            id: None,
            name: "".to_string(),
        };
        for u in users_iter {
            user = match u {
                Ok(u) => u,
                Err(e) => return Err(e),
            };
        }
        Ok(user)
    }
    pub async fn get_user_by_id(state: Arc<Mutex<AppState>>, user_id: u32) -> Result<User, Error> {
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return Err(Error::UnwindingPanic),
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
            return Ok(User {
                id: None,
                name: "".to_string(),
            });
        }
        Ok(User {
            id: users[0].id,
            name: users[0].name.clone(),
        })
    }
}
