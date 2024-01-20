mod book_mod {
    pub struct Book {
        name: String
    }

    impl Book {
        pub fn new(name: &str) -> Book {
            let book = Book {
                name: name.to_string()
            };
            return book;
        }
    }

    pub fn read_book(book: &Book) {
        println!("Reading book: {}", book.name);
    }
}

fn main() {
    let my_awesome_book = book_mod::Book::new("My Incredible Book");

    book_mod::read_book(&my_awesome_book);
}