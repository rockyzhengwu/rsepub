pub struct Chapter {
    index: usize,
    href: String,
    content: String,
}

impl Chapter {
    pub fn new(index: usize, href: &str, content: String) -> Self {
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

    pub fn content(&self) -> &str {
        self.content.as_str()
    }
}
