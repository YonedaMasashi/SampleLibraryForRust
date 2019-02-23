pub struct Book {
    book_id: u32,
    name: String,
    author: String,
    genru: String
}

impl Book {
    pub fn new (book_id: u32, name: String, author: String, genru: String) 
        -> Book {
        Book { book_id, name, author, genru }
    }

    pub fn get_book_name(&self) -> String {
        self.name.to_string()
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.book_id, self.name, self.author, self.genru)
    }
}