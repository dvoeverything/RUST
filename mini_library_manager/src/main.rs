use std::io::{self, Write};
use std::fmt::{self, Display, Formatter};
#[derive(Debug)]
enum BookStatus{
    Available,
    Borrowed,
    Lost
}

impl Display for BookStatus{

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        let(word, emoji) = match self {
            Self::Available => ("Available", "📗"),
            Self::Borrowed => ("Borrowed", "📕"),
            Self::Lost      => ("Lost",      "📙"),

        };
        write!(f, "{word} {emoji}")

    }
}

#[derive(Debug)]
struct Book{ 
    title: String,
    author: String,
    year: u16,
    status: BookStatus
}

impl Display for Book{
    fn fmt(&self , f:&mut Formatter<'_>)-> fmt::Result{
        write!(
            f, "Title: {:<30} | Author: {:<20} | {:4} | {}",
            self.title, self.author, self.year, self.status
        )
    }
}

#[derive(Default)]
struct Library{
    books: Vec<Book>
}

impl Library{
    fn new() -> Self{
        Self::default()
    }

    fn add_book(&mut self, b: Book){
        self.books.push(b);
        println!("✓ Book added!")
    }

    fn list(&self){
        if self.books.is_empty(){
            println!("No Books yet");
            return;
        }
        println!("\n===========Your Library=================\n");
        for book in &self.books{
            println!("{}",book);
        }
        println!("=============================================\n");
    }

    fn find_mut(&mut self, title_part: &str)-> Option<&mut Book>{
        let needle = title_part.to_lowercase();
        self.books.iter_mut()
        .find(|b| b.title.to_lowercase().contains(&needle))
    }

    fn borrow(&mut self, title_part: &str){
        match self.find_mut(title_part){
            Some(b) if matches!(b.status, BookStatus::Available) =>{
                b.status = BookStatus::Borrowed;
                println!("✓ Enjoy your reading!");
            }
            Some(_) => println!("⚠️  That book isn’t available."),
            None =>    println!("❓ No match for '{title_part}'."),
        }
    }

    fn give_back(&mut self, title_part: &str){
        match self.find_mut(title_part){
            Some(b) if matches!(b.status, BookStatus::Borrowed) => {
                b.status = BookStatus::Available;
                println!("✓ Thank you for returning it!");
            }
            Some(_) => println!("⚠️  That book wasn’t borrowed"),
            None => println!("❓ No match for '{title_part}'."),

        }
    }
}

fn main() {
    let mut lib = Library::new();

    loop {
        println!("\nMini-Library Manager
0 – List all books
1 – Add a new book
2 – Borrow a book
3 – Return a book
4 – Search by title
other – Quit");
        print!("Your choice: "); flush();

        match read_int() {
            0 => lib.list(),
            1 => {
                println!("Title? ");  let t = read_str();
                println!("Author? "); let a = read_str();
                println!("Year? ");   let y = read_int() as u16;
                lib.add_book(Book{ title:t, author:a, year:y,
                                   status:BookStatus::Available });
            }
            2 => { println!("Borrow which title? "); lib.borrow(&read_str()); }
            3 => { println!("Return which title? "); lib.give_back(&read_str()); }
            4 => {
                println!("Search keyword: "); let k = read_str();
                if let Some(b) = lib.find_mut(&k).map(|b| &*b) { // immutable view
                    println!("{b}");
                } else { println!("No match."); }
            }
            _ => break,
        }
    }
}

//tiny stdin helpers
fn flush() { let _ = std::io::stdout().flush(); }
fn read_str() -> String {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}
fn read_int() -> u32 { read_str().parse().unwrap_or(9999) }