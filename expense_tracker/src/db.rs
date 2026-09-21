use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "expense.db".to_string());

    SqliteConnection::establish(&database_url)
        .expect("Error connecting to database")
}