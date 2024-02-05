// hanve a epub file , and audio , this probgrame crate audio book

use std::path::PathBuf;

use clap::Parser;

use epub::book::Book;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(short, long, value_name = "epub_file")]
    epub_file: PathBuf,
    #[arg(short, long, value_name = "audio_dir")]
    audio_dir: PathBuf,
    #[arg(short, long, value_name = "out_dir")]
    out_dir: PathBuf,
}

// TODO 1. split html content to sentences 2. match html content and audio times. create new epub file


pub fn command(conf: &Config) {
    let mut book = Book::open_from_file(conf.epub_file.as_path()).unwrap();
    for chapter in book.chapters() {
        if chapter.index() == 67 {
            println!("chapter:{:?},{:?}", chapter.index(), chapter.href());
            println!("{:?}", chapter.content().to_string());
        }
    }
}
