use std::rc::Rc;

use markup5ever_rcdom::Node;

use crate::error::Result;
use crate::nav::{parse_title, NavItem, Navigation};
use crate::xml::{self, XMLDocument};

fn parse_toc(node: &Rc<Node>) -> Vec<NavItem> {
    let mut items = Vec::new();
    let ols = xml::find_children(node, "ol");
    if ols.is_empty() {
        return items;
    }
    for child in ols {
        items.extend(parse_nav_ol(&child));
    }
    items
}

fn parse_nav_ol(node: &Rc<Node>) -> Vec<NavItem> {
    let mut res = Vec::new();
    let ls = xml::find_children(node, "li");
    if ls.is_empty() {
        return res;
    }
    for li in ls {
        if let Some(an) = xml::find_children(&li, "a").first() {
            let href = xml::parse_attribute(an, "href").unwrap_or_default();
            let text = xml::parse_text(an);
            let children = parse_toc(&li);
            let item = NavItem {
                href,
                text,
                children,
            };
            res.push(item)
        }
    }
    res
}

pub fn parse(content: &[u8]) -> Result<Navigation> {
    let doc = XMLDocument::try_new(content)?;
    let title = parse_title(&doc);
    let mut toc = Vec::new();
    for nav in doc.find_all_tag("nav") {
        if let Some(ty) = xml::parse_attribute(&nav, "type") {
            if ty.as_str() == "toc" {
                toc = parse_toc(&nav)
            }
        }
    }
    Ok(Navigation { title, toc })
}
