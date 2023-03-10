use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

fn main() {
    #[derive(Debug)]
    struct Book {
        id: i32,
        title: String,
        author: String,
    }
    // create a connection to a db, if db does not exist it will be created
    let conn = Connection::open("books.db").unwrap();

    let _ = conn.execute(
        "CREATE TABLE library(id INTEGER PRIMARY KEY, title TEXT NOT NULL, author TEXT NOT NULL)",
        (),
    );

    let cocaine_nights = Book {
        id: 1,
        title: "Cocaine Nights ".to_string(),
        author: "J. G. Ballard".to_string(),
    };

    let _ = conn.execute(
        "INSERT INTO library (id, title, author) VALUES (?1, ?2, ?3)",
        (
            &cocaine_nights.id,
            &cocaine_nights.title,
            &cocaine_nights.author,
        ),
    );

    let mut data = conn
        .prepare("SELECT id, title, author FROM library")
        .unwrap();

    let book_iter = data
        .query_map([], |row| {
            Ok(Book {
                id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                author: row.get(2).unwrap(),
            })
        })
        .unwrap();

    for book in book_iter {
        println!("{:?}", book.unwrap());
    }
}
