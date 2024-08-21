struct Book {
    title: String,
    author: String,
    stock: i32,
}

struct Inventory {
    books: Vec<Book>,
}

impl Inventory {
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn number_of_books_in_stock(&self, title: &str) {
        for book in &self.books {
            if book.title == title {
                println!("{} has {} copies in stock", title, book.stock);
            }
        }
    }

    fn sell_book(&mut self, title: &str) {
        for book in &mut self.books {
            if book.title == title {
                if book.stock > 0 {
                    book.stock -= 1;
                    println!("{} has been sold", title);
                } else {
                    println!("{} is out of stock", title);
                }
            }
        }
    }

    fn books_in_inventory(&self) {
        for book in &self.books {
            println!("Title: {}, Author: {}, Stock: {}", book.title, book.author, book.stock);
        }
    }
}


fn main() {
    let mut inventory = Inventory {
        books: Vec::new(),
    };

    let book1 = Book {
        title: String::from("Rust Programming"),
        author: String::from("John Doe"),
        stock: 10,
    };

    let book2 = Book {
        title: String::from("Python Programming"),
        author: String::from("Jane Doe"),
        stock: 5
    };

    inventory.add_book(book1);
    inventory.add_book( book2);

    println!();

    inventory.number_of_books_in_stock( "Rust Programming");

    println!();

    inventory.books_in_inventory();

    println!();

    inventory.sell_book("Rust Programming");

    println!();

    inventory.number_of_books_in_stock( "Rust Programming");
}
