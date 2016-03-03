#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

pub mod schema;
pub mod models;
pub mod services;
pub mod persistence;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use self::models::{Todo, NewTodo};

pub fn create_todo<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Todo {
    use schema::todos;

    let new_todo = NewTodo {
        title: title,
        body: body,
    };

    diesel::insert(&new_todo).into(todos::table)
    .get_result(conn)
    .expect("Error saving new todo")
}