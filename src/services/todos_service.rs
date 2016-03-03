extern crate diesel;

use persistence::connection::*;
use ::models::*;
use diesel::prelude::*;

pub fn fetch_todos() -> String {
    use schema::todos::dsl::*;

    let connection = establish_connection();
    let results = todos
    .load::<Todo>(&connection)
    .expect("Error loading todos");

    // TODO: find more idiomatic way of concatenating strings
    let mut s = "".to_string();

    for todo in results {
        s.push_str(&(todo.title + "\n" + &todo.body + "\n"));
    }

    s
}