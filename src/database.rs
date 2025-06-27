pub mod connections {
    use crate::models::models::{AppState, User};
    use rusqlite::{Connection, Error, params};
    use std::sync::{Arc, Mutex};

    pub async fn connect_to_db(path: &str) -> Result<Connection, &str> {
        let conn = match Connection::open(path) {
            Ok(c) => c,
            Err(_) => return Err("error opening path"),
        };
        return match conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL
         )",
            [],
        ) {
            Ok(_) => Ok(conn),
            Err(_) => Err("error creating table users"),
        };
    }

    pub async fn get_all_users(state: Arc<Mutex<AppState>>) -> Result<Vec<User>, Error> {
        let state = match state.lock() {
            Ok(r) => r,
            Err(_) => return Err(Error::UnwindingPanic),
        };
        let mut statement = state.db_connection.prepare("SELECT * FROM users")?;
        let users_iter = statement.query_map([], |row| {
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
        })?;
        let mut users: Vec<User> = Vec::new();
        for user in users_iter {
            users.push(user?);
        }
        Ok(users)
    }

    pub async fn post_user(state: Arc<Mutex<AppState>>, user: User) -> Result<User, Error> {
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return Err(Error::UnwindingPanic),
        };
        state
            .db_connection
            .execute("INSERT INTO users (name) VALUES (?1)", [user.name])?;
        let mut statement = state
            .db_connection
            .prepare("SELECT * FROM users ORDER BY ROWID DESC LIMIT 1")?;
        let users_iter = statement.query_map([], |row| {
            Ok(User {
                id: match row.get(0) {
                    Ok(id) => Some(id),
                    Err(_) => None,
                },
                name: match row.get(1) {
                    Ok(name) => name,
                    Err(_) => "".to_string(),
                },
            })
        })?;
        let mut user: User = User {
            id: None,
            name: "".to_string(),
        };
        for u in users_iter {
            user = u?;
        }
        Ok(user)
    }

    pub async fn get_user_by_id(state: Arc<Mutex<AppState>>, user_id: u32) -> Result<User, Error> {
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return Err(Error::UnwindingPanic),
        };
        let mut statement = state
            .db_connection
            .prepare("SELECT * FROM users WHERE id=(?1)")?;
        let users_iter = statement.query_map([user_id], |r| {
            Ok(User {
                id: match r.get(0) {
                    Ok(id) => Some(id),
                    Err(_) => None,
                },
                name: match r.get(1) {
                    Ok(name) => name,
                    Err(_) => "".to_string(),
                },
            })
        })?;
        let mut users: Vec<User> = Vec::new();
        for u in users_iter {
            users.push(u?);
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

    pub async fn modify_user(
        state: Arc<Mutex<AppState>>,
        user_id: u32,
        user: User,
    ) -> Result<User, Error> {
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return Err(Error::UnwindingPanic),
        };
        state.db_connection.execute(
            "update users set name = (?1) where id=(?2)",
            params![user.name, user_id],
        )?;
        let mut statement = state
            .db_connection
            .prepare("SELECT * FROM users WHERE id=(?1)")?;
        let users_iter = statement.query_map([user_id], |row| {
            Ok(User {
                id: match row.get(0) {
                    Ok(id) => Some(id),
                    Err(_) => None,
                },
                name: match row.get(1) {
                    Ok(name) => name,
                    Err(_) => "".to_string(),
                },
            })
        })?;
        let mut user: User = User {
            id: None,
            name: "".to_string(),
        };
        for u in users_iter {
            user = u?;
        }
        Ok(user)
    }

    pub async fn delete_user(state: Arc<Mutex<AppState>>, user_id: u32) -> Result<User, Error> {
        let user = get_user_by_id(state.clone(), user_id).await?;
        let state = match state.lock() {
            Ok(s) => s,
            Err(_) => return Err(Error::UnwindingPanic),
        };
        return match state
            .db_connection
            .execute("DELETE FROM users WHERE id=(?1)", [user_id])
        {
            Ok(_) => Ok(user),
            Err(e) => Err(e),
        };
    }
}
