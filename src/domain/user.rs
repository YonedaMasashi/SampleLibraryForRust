pub struct User {
    user_id: u32,
    name: String,
    mail_address: String
}

impl User {
    pub fn new(user_id: u32, name: String, mail_address: String) -> User {
        User {user_id, name, mail_address}
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{}", self.user_id, self.name, self.mail_address)
    }
}