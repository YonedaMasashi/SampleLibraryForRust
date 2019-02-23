use crate::schema::{Book};

#[derive(Identifiable, Queryable, Insertable)]
#[primary_key(book_id)]
#[table_name = "Book"]
pub struct BookEntity {
//    pub id: i32,
    pub book_id: i32,
    pub name: String,
    pub author: String,
    pub genru: String
}