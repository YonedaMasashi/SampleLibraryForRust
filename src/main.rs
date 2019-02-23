#[macro_use]extern crate diesel;

extern crate dotenv;

pub mod schema;
pub mod domain;
pub mod entity;
pub mod service;

use diesel::prelude::*;

use crate::service::db_connection::establish_connection;

use chrono::{Local};

fn main() {
    domain_object();
    db_process();
}

fn domain_object() {
    println!("{}", "----- Console -----");
    let user = domain::user::User::new(1, "Masashi".to_string(), "hoge@hoge.com".to_string());
    let book = domain::book::Book::new(1, "ProgInRust".to_string(), "Jim Blandy".to_string(), "programming".to_string());
    let borrow = domain::borrow::Borrow::new(&user, &book, Local::now(), Local::now());

    println!("{}", user.to_string());
    println!("{}", book.to_string());
    println!("{}", borrow.to_string());
}

fn db_process() {
    println!("{}", "----- DB -----");
    let connection = establish_connection();

    use schema::User::dsl::*;
    use schema::Book::dsl::*;
    use schema::Borrow::dsl::*;

    // user の追加
    let userEntity = entity::user::UserEntity {
        user_id: 1,
        name: "Yoneda".to_string(),
        mail_address: "hogehoge@gmail.com".to_string(),
    };
    diesel::
        insert_into(User)
        .values(&userEntity)
        .execute(&connection)
        .expect("Error User");

    // book の追加
    let bookEntity = entity::book::BookEntity {
        book_id: 1,
        name: "ProgInRust".to_string(),
        author: "hoge".to_string(),
        genru: "programming".to_string(),
    };
    diesel::
        insert_into(Book)
        .values(&bookEntity)
        .execute(&connection)
        .expect("Error Book");

    // Borrow の追加
    let borrowEntity = entity::borrow::BorrowEntity {
        borrow_id:1,
        user_id:1,
        book_id:1,
        borrow_date: "today".to_string(),
        refund_date: "2weeks".to_string(),
    };
    diesel::
        insert_into(Borrow)
        .values(&borrowEntity)
        .execute(&connection)
        .expect("Error Borrow");

    // DB のデータを取得してみる
    {
        let userGet = User.find(1)
            .first::<entity::user::UserEntity>(&connection)
            .expect("Error loading user");
        println!("Get User {}", userGet.name);

        let borrowGet = entity::borrow::BorrowEntity::belonging_to(&userGet)
            .first::<entity::borrow::BorrowEntity>(&connection)
            .expect("Error loading Borrow");
        println!("Get Borrow {}", borrowGet.borrow_date)
    }

}