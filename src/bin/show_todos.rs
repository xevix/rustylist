extern crate rustylist;
extern crate diesel;

use self::rustylist::*;
use self::rustylist::models::*;
use self::diesel::prelude::*;

fn main() {
    use rustylist::schema::todos::dsl::*;

    let connection = establish_connection();
    let results = todos
    .limit(5)
    .load::<Todo>(&connection)
    .expect("Error loading todos");

    println!("Displaying {} todos", results.len());
    for todo in results {
        println!("{}", todo.title);
        println!("----------\n");
        println!("{}", todo.body);
    }
}