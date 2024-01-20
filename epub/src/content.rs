use std::str::FromStr;

use markup5ever_rcdom::NodeData;
use xml5ever::tendril::StrTendril;

use crate::error::Result;
use crate::xml::{self, XMLDocument};

pub struct Content {
    doc: XMLDocument,
}

impl Content {
    pub fn new(content: &[u8]) -> Result<Self> {
        let doc = XMLDocument::try_new(content)?;
        Ok(Content { doc })
    }

    pub fn ralative_sources(&self) -> Vec<String> {
        let images = self.doc.find_all_tag("img");
        let mut res = Vec::new();
        for image in images {
            if let Some(src) = xml::parse_attribute(&image, "src") {
                if src.starts_with("http") {
                    continue;
                }
                res.push(src);
            }
        }
        res
    }

    pub fn update_image_url(&mut self, src: &str, dest: &str) {
        let images = self.doc.find_all_tag("img");
        for node in images {
            if let NodeData::Element { ref attrs, .. } = node.data {
                for attr in attrs.borrow_mut().iter_mut() {
                    if &*attr.name.local == "src" || &*attr.value == src {
                        attr.value = StrTendril::from_str(dest).unwrap();
                    }
                }
            }
        }
    }

    pub fn to_string(&self) -> Result<String> {
        self.doc.to_string()
    }
}
