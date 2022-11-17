mod enums;

struct Book {
    title: String,
    author: String,
    genre: enums::Genre,
    page_count: u64,
}

fn create_book(title: String, author: String, genre: enums::Genre, page_count: u64) -> Book {
    return Book {
        title: title,
        author: author,
        genre: genre,
        page_count: page_count,
    };
}

fn check_genre(genre: enums::Genre) {}

fn main() {
    println!("Hello, world!");
    let book1 = create_book(
        String::from("Titles"),
        String::from("Author"),
        enums::Genre::NonFiction,
        79,
    );
    println!("{:?}", book1.genre);

    if book1.genre == enums::Genre::NonFiction {
        println!("IT WORKS");
    }

    let genre_value = match book1 {
        NonFiction => String::from("NonFiction"),
        Fiction => String::from("Fiction"),
    };

    println!("{}", genre_value);
    // let some_num = Some(5);
    // let some_num = match some_num {
    //     Some(number) => number,
    //     None => 0,
    // };
    // let non_number: Option<String> = None;
    // println!("{:?}", some_num + 1);

    // you have to explicitly convert it to use the value nested in some/an enum with a value
}
