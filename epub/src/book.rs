use std::path::Path;

use crate::container::Container;
use crate::content::Content;
use crate::error::Result;
use crate::nav::Navigation;
use crate::package::Package;
use crate::reader::Reader;

#[allow(dead_code)]
pub struct Book {
    reader: Reader,
    container: Container,
    package: Package,
    nav: Option<Navigation>,
    rootdir: String,
}

impl Book {
    fn new_book(mut reader: Reader) -> Result<Book> {
        let container = reader.read_meta_container()?;
        let fullpath = container.full_path()?;
        let package = reader.read_package(fullpath.as_str())?;
        let fp = Path::new(fullpath.as_str()).parent();
        let prefix = match fp {
            Some(f) => f.to_string_lossy().to_string(),
            None => "".to_string(),
        };

        let nav = match package.get_manifest("nav") {
            Some(nav) => {
                let path = Path::new(prefix.as_str()).join(nav.href());
                let p = path.to_str().unwrap();
                Some(reader.read_nav(p)?)
            }
            None => match package.get_manifest("ncx") {
                Some(ncx) => {
                    let path = Path::new(prefix.as_str()).join(ncx.href());
                    let p = path.to_str().unwrap();
                    Some(reader.read_ncx(p)?)
                }
                None => None,
            },
        };

        Ok(Book {
            reader,
            container,
            package,
            nav,
            rootdir: prefix.to_string(),
        })
    }

    pub fn open_from_file<T: AsRef<Path>>(path: T) -> Result<Book> {
        let reader = Reader::new_from_path(path.as_ref().into())?;
        Book::new_book(reader)
    }

    pub fn open_from_memory(buffer: Vec<u8>) -> Result<Book> {
        let reader = Reader::new_from_memory(buffer)?;
        Book::new_book(reader)
    }

    pub fn dump(&mut self) -> Result<String> {
        unimplemented!()
    }

    pub fn nav(&self) -> Option<&Navigation> {
        self.nav.as_ref()
    }

    pub fn package(&self) -> &Package {
        &self.package
    }

    pub fn title(&self) -> &str {
        self.package.title()
    }

    pub fn content(&mut self, name: &str) -> Result<Content> {
        let path = Path::new(self.rootdir.as_str()).join(name);
        let p = path.to_string_lossy().to_string();
        self.reader.read_content(p.as_str())
    }

    pub fn read_binary_file(&mut self, name: &str) -> Result<Vec<u8>> {
        self.reader.read_binary(name)
    }

    pub fn chapters(&mut self) -> PageIterator {
        PageIterator::new(self)
    }
}

pub struct PageIterator<'a> {
    book: &'a mut Book,
    current_chapter: u32,
}

impl<'a> PageIterator<'a> {
    pub fn new(book: &'a mut Book) -> Self {
        PageIterator {
            book,
            current_chapter: 0,
        }
    }
}

impl<'a> Iterator for PageIterator<'a> {
    type Item = Content;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.book.package.chapter(self.current_chapter) {
            let content = self.book.content(item.href()).unwrap();
            self.current_chapter += 1;
            Some(content)
        } else {
            None
        }
    }
}
