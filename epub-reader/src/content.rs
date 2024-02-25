use std::str::FromStr;

use anyhow::{anyhow, Result};
use epub::xml::{self, XMLDocument};
use markup5ever_rcdom::NodeData;
use url::Url;
use xml5ever::tendril::StrTendril;

pub fn relative_resources(doc: &XMLDocument, path: &str) -> Result<Vec<String>> {
    let images = doc.find_all_tag("img");
    let mut res = Vec::new();
    for image in images {
        if let Some(src) = xml::parse_attribute(&image, "src") {
            if src.starts_with("http") {
                continue;
            }
            let abs_src = get_url(src.as_str(), path)?;
            res.push(abs_src);
        }
    }

    let links = doc.find_all_tag("link");
    for link in links {
        if let Some(ty) = xml::parse_attribute(&link, "type") {
            if ty == "text/css" {
                let href = xml::parse_attribute(&link, "href").unwrap();
                if href.starts_with("http") {
                    continue;
                }
                let url = get_url(href.as_str(), path)?;
                res.push(url);
            }
        }
    }
    Ok(res)
}

fn get_url(href: &str, path: &str) -> Result<String> {
    let url = Url::parse(format!("http://localhost/{}", path).as_str())
        .map_err(|e| anyhow!(format!("url parse error:{:?}", e)))?;
    let url = url
        .join(href)
        .map_err(|e| anyhow!(format!("url join error:{:?}", e)))?;
    let mut s = url.path().to_string();
    if s.starts_with('/') {
        s = s.strip_prefix('/').unwrap().to_string()
    }
    Ok(s)
}

pub fn update_image_url(doc: &XMLDocument, src: &str, dest: &str) {
    let images = doc.find_all_tag("img");
    for node in images {
        if let NodeData::Element { ref attrs, .. } = node.data {
            for attr in attrs.borrow_mut().iter_mut() {
                if &*attr.name.local == "src" && src.contains(&*attr.value) {
                    attr.value = StrTendril::from_str(dest).unwrap();
                    return;
                }
            }
        }
    }
    let links = doc.find_all_tag("link");
    for link in links {
        if let NodeData::Element { ref attrs, .. } = link.data {
            for attr in attrs.borrow_mut().iter_mut() {
                if &*attr.name.local == "href" && src.contains(&*attr.value) {
                    attr.value = StrTendril::from_str(dest).unwrap();
                    return;
                }
            }
        }
    }
}
