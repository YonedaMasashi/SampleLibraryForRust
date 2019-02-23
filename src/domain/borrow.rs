use chrono::{DateTime, Local};

pub struct Borrow {
    user_id: u32,
    book_id: u32,
    borrow_date: DateTime<Local>,
    refund_date: DateTime<Local>
}

impl Borrow {
    pub fn new(user_id:u32, book_id:u32, borrow_date:DateTime<Local>, refund_date:DateTime<Local>)
        -> Borrow {
        Borrow {
            user_id,
            book_id,
            borrow_date,
            refund_date
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.user_id, self.book_id, self.borrow_date.to_string(), self.refund_date.to_string())
    }
}
