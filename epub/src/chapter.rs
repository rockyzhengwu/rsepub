use crate::content::Content;

pub struct Chapter {
    index: usize,
    href: String,
    content: Content,
}

impl Chapter {
    pub fn new(index: usize, href: &str, content: Content) -> Self {
        Chapter {
            index,
            href: href.to_string(),
            content,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn href(&self) -> &str {
        self.href.as_str()
    }

    pub fn content(&self) -> &Content {
        &self.content
    }
}
