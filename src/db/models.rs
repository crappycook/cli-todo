use super::schema::todo_items;

#[derive(Queryable)]
pub struct TodoItem {
    pub id: i64,
    pub title: String,
    pub content: String,
}
