-- Your SQL goes here
CREATE TABLE Book (
  book_id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  author TEXT NOT NULL,
  genru TEXT NOT NULL
);

CREATE TABLE User (
  user_id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  mail_address TEXT NOT NULL
);

CREATE TABLE Borrow (
    borrow_id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    borrow_date TEXT NOT NULL,
    refund_date TEXT NOT NULL,
    foreign key (user_id) references User(user_id),
    foreign key (book_id) references Book(book_id)
);
