use crate::schema::{User};

#[derive(Identifiable, Queryable, Insertable)]
#[primary_key(user_id)] 
#[table_name = "User"]
pub struct UserEntity {
//    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub mail_address: String,
}
