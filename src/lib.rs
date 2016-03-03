#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

pub mod schema;
pub mod models;
pub mod services;
pub mod persistence;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

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