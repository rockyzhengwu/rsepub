use std::str::FromStr;

use markup5ever_rcdom::NodeData;
use url::Url;
use xml5ever::tendril::StrTendril;

use crate::error::{EpubError, Result};
use crate::xml::{self, XMLDocument};

pub struct Content {
    path: String,
    doc: XMLDocument,
}

impl Content {
    pub fn new(path: &str, content: &[u8]) -> Result<Self> {
        let doc = XMLDocument::try_new(content)?;
        Ok(Content {
            path: path.to_string(),
            doc,
        })
    }

    pub fn base_url(&self) -> Result<String> {
        let segments = self.path.split('/');
        let mut segments = segments.into_iter().collect::<Vec<&str>>();
        if !segments.is_empty() {
            segments.pop();
        }
        Ok(segments.join("/"))
    }

    pub fn ralative_sources(&self) -> Result<Vec<String>> {
        let images = self.doc.find_all_tag("img");
        let mut res = Vec::new();
        for image in images {
            if let Some(src) = xml::parse_attribute(&image, "src") {
                if src.starts_with("http") {
                    continue;
                }
                let abs_src = self.get_url(src.as_str())?;
                res.push(abs_src);
            }
        }

        let links = self.doc.find_all_tag("link");
        for link in links {
            if let Some(ty) = xml::parse_attribute(&link, "type") {
                if ty == "text/css" {
                    let href = xml::parse_attribute(&link, "href").unwrap();
                    if href.starts_with("http") {
                        continue;
                    }
                    let url = self.get_url(href.as_str())?;
                    res.push(url);
                }
            }
        }
        Ok(res)
    }

    pub fn get_url(&self, href: &str) -> Result<String> {
        let url = Url::parse(format!("http://localhost/{}", self.path).as_str())
            .map_err(|e| EpubError::UrlError(format!("url parse error:{:?}", e)))?;
        let url = url
            .join(href)
            .map_err(|e| EpubError::UrlError(format!("url join error:{:?}", e)))?;
        let mut s = url.path().to_string();
        if s.starts_with('/') {
            s = s.strip_prefix('/').unwrap().to_string()
        }
        Ok(s)
    }

    pub fn update_image_url(&mut self, src: &str, dest: &str) {
        let images = self.doc.find_all_tag("img");
        for node in images {
            if let NodeData::Element { ref attrs, .. } = node.data {
                for attr in attrs.borrow_mut().iter_mut() {
                    if &*attr.name.local == "src" && self.get_url(&attr.value).unwrap() == src {
                        attr.value = StrTendril::from_str(dest).unwrap();
                        return;
                    }
                }
            }
        }
        let links = self.doc.find_all_tag("link");
        for link in links {
            if let NodeData::Element { ref attrs, .. } = link.data {
                for attr in attrs.borrow_mut().iter_mut() {
                    if &*attr.name.local == "href" && self.get_url(&attr.value).unwrap() == src {
                        attr.value = StrTendril::from_str(dest).unwrap();
                        return;
                    }
                }
            }
        }
    }

    pub fn to_string(&self) -> Result<String> {
        self.doc.to_string()
    }

    pub fn doc(&self) -> &XMLDocument {
        &self.doc
    }
}
