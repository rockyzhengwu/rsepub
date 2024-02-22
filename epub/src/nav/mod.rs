pub mod navigation;
pub mod ncx;

use crate::error::Result;
use crate::xml::{self, XMLDocument};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NavItem {
    href: String,
    text: String,
    children: Vec<NavItem>,
}
impl NavItem {
    pub fn href(&self) -> &str {
        &self.href
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn children(&self) -> &[NavItem] {
        self.children.as_slice()
    }
}

pub struct Navigation {
    title: String,
    toc: Vec<NavItem>,
}

impl Navigation {
    pub fn new_from_nav(content: &[u8]) -> Result<Self> {
        navigation::parse(content)
    }

    pub fn new_from_ncx(content: &[u8]) -> Result<Self> {
        ncx::parse(content)
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn toc(&self) -> &[NavItem] {
        self.toc.as_slice()
    }
}

fn parse_title(doc: &XMLDocument) -> String {
    match doc.find_tag("title") {
        Some(tl) => xml::parse_text(&tl),
        None => String::from(""),
    }
}
