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
    //    let mut s = "".to_string();
    //
    //    for todo in results {
    //        s.push_str(&(todo.title + "\n" + &todo.body + "\n"));
    //    }
    //
    //    s

    // NOTE: more FP-style, but might be costly
    // results.iter().map(|todo| "".to_string() + &todo.title + "\n" + &todo.body + "\n").collect::<Vec<String>>().join("\n")

    // Most efficient way so far, somewhat FP
    results.iter().fold(String::new(), |out, todo| out + &todo.title + "\n" + &todo.body + "\n")
}