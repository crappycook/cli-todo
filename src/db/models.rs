use super::schema::items;
use diesel::{prelude::*, sql_types::Integer};

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Queryable, QueryableByName, Debug)]
pub struct QueryTableCount {
    #[diesel(sql_type = Integer)]
    pub cnt: i32,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub content: &'a str,
}
#[derive(Queryable, Debug)]
pub struct SqlMaster {
    pub name: String,
    pub sql: String,
}
