use chrono::{DateTime, Local};

use crate::domain::user::User;
use crate::domain::book::Book;

pub struct Borrow<'a, 'b> {
    user: &'a User,
    book: &'b Book,
    borrow_date: DateTime<Local>,
    refund_date: DateTime<Local>
}

impl<'a, 'b> Borrow<'a, 'b> {
    pub fn new(user:&'a User, book:&'b Book, borrow_date:DateTime<Local>, refund_date:DateTime<Local>)
        -> Borrow<'a, 'b> {
        Borrow {
            user,
            book,
            borrow_date,
            refund_date
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.user.get_user_name(), self.book.get_book_name(), self.borrow_date.to_string(), self.refund_date.to_string())
    }
}
