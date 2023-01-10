use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

fn main() {
    struct Book {
        id: i32,
        title: String,
        author: String,
    }
    // create a connection to a db, if db does not exist it will be created
    let conn = Connection::open("books.db").unwrap();

    conn.execute(
        "CREATE TABLE library(id INTEGER PRIMARY KEY, title TEXT NOT NULL, author TEXT NOT NULL)",
        (),
    );
}
