use crate::schema::{Borrow};

use crate::entity::book::BookEntity;
use crate::entity::user::UserEntity;

#[derive(Identifiable, Queryable, Associations, Insertable)]
#[primary_key(borrow_id)] 
#[belongs_to(UserEntity, foreign_key="user_id")]
#[belongs_to(BookEntity, foreign_key="book_id")]
#[table_name = "Borrow"]
pub struct BorrowEntity {
//    pub id: i32,
    pub borrow_id: i32,
    pub user_id: i32,
    pub book_id: i32,
    pub borrow_date: String,
    pub refund_date: String,
}
