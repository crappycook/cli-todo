use super::schema::items;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub content: &'a str,
}
