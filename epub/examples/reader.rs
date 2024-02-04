use epub::book::Book;
fn main() {
    let filename = "/home/zhengwu/Documents/epub-example/surrender/The Surrender Experiment My Journey Into Lifes Perfection (Michael A. Singer) (Z-Library).epub";
    let filename =
        "/home/zhengwu/Documents/epub-example/the-economist/TheEconomist.2022.12.03.epub";
    let mut doc = Book::open_from_file(filename).unwrap();
    let content = doc.content("feed_0/article_2/index_u48.html").unwrap();
    //let content = doc
    //    .content("OEBPS/Sing_9780804141116_epub3_c13_r1.xhtml")
    //    .unwrap();

    println!("{:?}", content.ralative_sources());
}
