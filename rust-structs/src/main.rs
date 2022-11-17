struct Book {
    title: String,
    author: String,
    genre: String, // create enum
    page_count: u64,
}

fn create_book(title: String, author: String, genre: String, page_count: u64) -> Book {
    return Book {
        title: title,
        author: author,
        genre: genre,
        page_count: page_count,
    };
}

fn main() {
    // immutable instance
    let book1 = create_book(
        String::from("Holistic Management"),
        String::from("Allan Savory"),
        String::from("Non-Fiction"),
        615,
    );
    // print out immutable instance
    println!("{}", book1.title);
    // attempt to modify immutable instance
    // user1.active = false;

    // mutable instance. we have to mark the WHOLE struct as mutable, cannot mark individual fields as mutable

    let mut book2 = create_book(
        String::from(
            "Regenerative Agriculture: A Practical Whole Systems Guide to Making Small Farms Work",
        ),
        String::from("Richard Parkins"),
        String::from("Non-Fiction"),
        740,
    );
    // print out mutable instance
    println!("{}", book2.author);
    // attempt to modify mutable instance
    book2.author = String::from("Richard Perkins");
    println!("{}", book2.author);
}
