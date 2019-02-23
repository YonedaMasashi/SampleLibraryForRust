table! {
    Book (book_id) {
        book_id -> Integer,
        name -> Text,
        author -> Text,
        genru -> Text,
    }
}

table! {
    Borrow (borrow_id) {
        borrow_id -> Integer,
        user_id -> Integer,
        book_id -> Integer,
        borrow_date -> Text,
        refund_date -> Text,
    }
}

table! {
    User (user_id) {
        user_id -> Integer,
        name -> Text,
        mail_address -> Text,
    }
}

joinable!(Borrow -> Book (book_id));
joinable!(Borrow -> User (user_id));

allow_tables_to_appear_in_same_query!(
    Book,
    Borrow,
    User,
);
