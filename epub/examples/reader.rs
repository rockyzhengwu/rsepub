use epub::book::Book;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].to_string();
    let mut doc = Book::open_from_file(filename).unwrap();
    for chapter in doc.chapters() {
        println!("chapter: {}, {}", chapter.index(), chapter.href());
    }
    let path = doc.resolve_path("feed_20/article_0/index_u6.html");
    println!("{:?}", path);
}
