extern crate dotenv;
extern crate rustylist;
extern crate diesel;

use dotenv::dotenv;

use self::rustylist::*;
use std::io::{stdin, Read};

fn main() {
    dotenv().ok();

    let connection = persistence::connection::establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let todo = create_todo(&connection, title, &body);
    println!("\nSaved draft {} with id {}", title, todo.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";