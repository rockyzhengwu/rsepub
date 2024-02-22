use std::rc::Rc;

use markup5ever_rcdom::Node;

use crate::error::{EpubError, Result};
use crate::nav::{parse_title, NavItem, Navigation};
use crate::xml::{self, XMLDocument};

pub fn parse_toc(node: &Rc<Node>) -> Result<Vec<NavItem>> {
    let mut res = Vec::new();
    let points = xml::find_children(node, "navPoint");
    if points.is_empty() {
        return Ok(res);
    }
    for point in points {
        let item = parse_toc_item(&point)?;
        res.push(item)
    }
    Ok(res)
}

pub fn parse_toc_item(node: &Rc<Node>) -> Result<NavItem> {
    let label = xml::first_child(node, "navLabel")
        .ok_or(EpubError::ParseError("navLabel not exists".to_string()))?;

    let text = match xml::first_child(&label, "text") {
        Some(t) => xml::parse_text(&t),
        None => "".to_string(),
    };
    let content = xml::first_child(node, "content").ok_or(EpubError::ParseError(
        "content not exist in NavPoint".to_string(),
    ))?;
    let href = xml::parse_attribute_must_exist(&content, "src")?;
    let children = parse_toc(node)?;
    Ok(NavItem {
        href,
        text,
        children,
    })
}

pub fn parse(content: &[u8]) -> Result<Navigation> {
    let doc = XMLDocument::try_new(content)?;
    let title = parse_title(&doc);
    let mut toc = Vec::new();
    if let Some(root) = doc.find_tag("navMap") {
        toc = parse_toc(&root)?;
    }
    Ok(Navigation { title, toc })
}
