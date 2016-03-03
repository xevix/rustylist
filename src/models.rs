#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
}

use super::schema::todos;

#[insertable_into(todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub body: &'a str,
}