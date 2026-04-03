struct Book {
    title: String,
    author: String,
    status: BookStatus,
}

struct Library {
    books: Vec<Book>,
}

#[derive(Debug, PartialEq)]
enum BookStatus {
    Available,
    Borrowed,
}

impl Book {
    fn new(title: &str, author: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            status: BookStatus::Available,
        }
    }

    fn borrow(&mut self) {
        self.status = BookStatus::Borrowed;
    }

    fn return_book(&mut self) {
        self.status = BookStatus::Available;
    }

    fn is_available(&self) -> bool {
        self.status == BookStatus::Available
    }
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn list_books(&self) {
        for (i, book) in self.books.iter().enumerate() {
            println!(
                "{} - title: {}, author: {}, status: {:?}",
                i + 1,
                book.title,
                book.author,
                book.status
            );
        }
    }

    fn available_books(&self) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.is_available())
            .collect()
    }

    fn find_by_author(&self, author: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.author == author)
            .collect()
    }
}

fn main() {
    let mut library = Library::new();

    let mut book1 = Book::new("The Hobbit", "Tolkien");
    let book2 = Book::new("The Silmarillion", "Tolkien");
    let book3 = Book::new("Rust Book", "Steve");

    book1.borrow();

    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);

    println!("---- ALL BOOKS ----");
    library.list_books();

    println!("\n---- AVAILABLE BOOKS ----");
    for book in library.available_books() {
        println!("{} by {}", book.title, book.author);
    }

    println!("\n---- BOOKS BY TOLKIEN ----");
    for book in library.find_by_author("Tolkien") {
        println!("{} ({:?})", book.title, book.status);
    }
}