struct Books{
    pages: i32,
    ratings: i32,
}

fn print_pages(book: &Books){
    println!("pages: {}", book.pages);
}

fn print_ratings(book: &Books){
    println!("ratings: {}", book.ratings);
}
fn main() {
let book = Books{
    pages: 100,
    ratings:10,
};
    print_pages(&book);
    print_ratings(&book);
}

