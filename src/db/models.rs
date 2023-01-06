use std::error::Error;
use std::io::{self, Write};

use super::schema::items;
use diesel::{prelude::*, sql_types::Integer};

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub content: String,
}

impl Item {
    // Print all items
    pub fn get_all_item(conn: &mut SqliteConnection) {
        use super::schema::items::dsl::*;

        let records: Vec<Item> = items
            .order(id.desc())
            .load::<Item>(conn)
            .expect("Error loading item");

        let stdout = io::stdout(); // get the global stdout entity
        let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer

        writeln!(handle, "Total {} items.", records.len()).unwrap();

        for r in records {
            writeln!(
                handle,
                "id: {}, title: {}, content: {}",
                r.id, r.title, r.content
            )
            .unwrap();
        }
        handle.flush().unwrap();
    }

    // Create todo item
    pub fn create_item(
        conn: &mut SqliteConnection,
        title: &str,
        content: &str,
    ) -> Result<(), Box<dyn Error>> {
        let new_item = NewItem { title, content };

        diesel::insert_into(items::table)
            .values(&new_item)
            .execute(conn)?;

        Ok(())
    }

    // Query all items count
    pub fn get_all_count(conn: &mut SqliteConnection) -> Result<i64, Box<dyn Error>> {
        use super::schema::items::dsl::*;
        use diesel::dsl::*;

        let r = items.select(count(id)).first::<i64>(conn)?;
        Ok(r)
    }

    // Delete item by id
    pub fn delete_item_by_id(
        conn: &mut SqliteConnection,
        target: i32,
    ) -> Result<usize, Box<dyn Error>> {
        use super::schema::items::dsl::*;

        let num_deleted = diesel::delete(items.filter(id.eq(target))).execute(conn)?;
        if num_deleted == 0 {
            return Err(super::DeleteNotExistsError.into());
        }
        Ok(num_deleted)
    }

    // Update item by id
    pub fn update_item(
        conn: &mut SqliteConnection,
        id: i32,
        new_title: &str,
        new_content: &str,
    ) -> Result<(), Box<dyn Error>> {
        use super::schema::items::dsl::{content, items, title};

        diesel::update(items.find(id))
            .set((title.eq(new_title), content.eq(new_content)))
            .execute(conn)?;

        Ok(())
    }
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
