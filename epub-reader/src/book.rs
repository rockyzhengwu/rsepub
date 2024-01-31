use epub::book::Book;
use epub::nav::NavItem;

use log::info;

use crate::resources::Resources;

#[derive(Default)]
#[allow(dead_code)]
pub struct ReadingBook {
    book: Option<Book>,
    file_name: String,
    file_type: String,
    resources: Resources,
}

impl ReadingBook {
    pub fn new(file_name: String, file_type: String, buffer: Vec<u8>) -> Self {
        let book = Book::open_from_memory(buffer).unwrap();
        ReadingBook {
            book: Some(book),
            file_name,
            file_type,
            resources: Resources::default(),
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.book.is_some()
    }

    pub fn title(&self) -> &str {
        if self.book.is_some() {
            self.book.as_ref().unwrap().title()
        } else {
            ""
        }
    }

    pub fn toc(&self) -> &[NavItem] {
        self.book.as_ref().unwrap().nav().unwrap().toc()
    }

    pub fn read_content(&mut self, name: &str) -> String {
        self.resources.clear();
        let mut content = self.book.as_mut().unwrap().content(name).unwrap();

        let res = content.ralative_sources().unwrap();
        for url in res {
            let image_path = url;

            let data = self
                .book
                .as_mut()
                .unwrap()
                .read_binary_file(image_path.as_str())
                .unwrap();

            let dest_url = self
                .resources
                .add_resource(image_path.as_str(), data.as_slice());
            info!("{:?},{:?}", image_path, dest_url);
            content.update_image_url(image_path.as_str(), dest_url.as_str());
        }
        content.to_string().unwrap()
    }

    pub fn create_resources(&mut self) {}
}
