use std::default::Default;
use std::rc::Rc;

use html5ever::serialize;
use markup5ever_rcdom::{Handle, Node, NodeData, RcDom, SerializableHandle};
use xml5ever::driver::parse_document;
use xml5ever::tendril::TendrilSink;

use crate::error::{EpubError, Result};

pub struct XMLDocument {
    dom: RcDom,
}

impl XMLDocument {
    pub fn try_new(content: &[u8]) -> Result<Self> {
        let mut reader = std::io::BufReader::new(content);
        let dom: RcDom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut reader)
            .map_err(|e| EpubError::XmlError(format!("create xml failed {:?}", e)))?;
        Ok(XMLDocument { dom })
    }

    fn find_first_tag(prefix: &str, node: &Handle) -> Option<Rc<Node>> {
        match node.data {
            NodeData::Element { ref name, .. } => {
                if &*name.local == prefix {
                    return Some(node.clone());
                }
            }
            _ => {
                // donothing
            }
        }

        for child in node
            .children
            .borrow()
            .iter()
            .filter(|child| matches!(child.data, NodeData::Element { .. }))
        {
            if let Some(v) = XMLDocument::find_first_tag(prefix, child) {
                return Some(v);
            }
        }
        None
    }

    fn find_tags(prefix: &str, node: &Handle) -> Vec<Rc<Node>> {
        let mut res = Vec::new();
        match node.data {
            NodeData::Element { ref name, .. } => {
                if &*name.local == prefix {
                    res.push(node.clone());
                }
            }
            _ => {
                // donothing
            }
        }
        for child in node
            .children
            .borrow()
            .iter()
            .filter(|child| matches!(child.data, NodeData::Element { .. }))
        {
            let items = XMLDocument::find_tags(prefix, child);
            res.extend(items);
        }
        res
    }

    pub fn find_tag(&self, tag: &str) -> Option<Rc<Node>> {
        XMLDocument::find_first_tag(tag, &self.dom.document)
    }

    pub fn find_all_tag(&self, tag: &str) -> Vec<Rc<Node>> {
        XMLDocument::find_tags(tag, &self.dom.document)
    }
    pub fn to_string(&self) -> Result<String> {
        let document: SerializableHandle = self.dom.document.clone().into();
        let mut buffer = Vec::new();
        serialize(&mut buffer, &document, Default::default())
            .map_err(|e| EpubError::XmlError(format!("faild seraize to xml {:?}", e)))?;
        String::from_utf8(buffer)
            .map_err(|e| EpubError::XmlError(format!("faild converto to string:{:?}", e)))
    }
}

pub fn parse_attribute(node: &Rc<Node>, name: &str) -> Option<String> {
    match node.data {
        NodeData::Element { ref attrs, .. } => {
            for attr in attrs.borrow().iter() {
                if &*attr.name.local == name {
                    return Some(attr.value.to_string());
                }
            }
            None
        }
        _ => None,
    }
}

pub fn parse_attribute_must_exist(node: &Rc<Node>, name: &str) -> Result<String> {
    match node.data {
        NodeData::Element { ref attrs, .. } => {
            for attr in attrs.borrow().iter() {
                if &*attr.name.local == name {
                    return Ok(attr.value.to_string());
                }
            }
            Err(EpubError::XmlError(format!("{} not exist", name)))
        }
        _ => Err(EpubError::XmlError(format!("{} not exist", name))),
    }
}

pub fn parse_text(node: &Rc<Node>) -> String {
    let mut res = String::new();
    for child in node
        .children
        .borrow()
        .iter()
        .filter(|child| matches!(child.data, NodeData::Text { .. }))
    {
        if let NodeData::Text { ref contents } = child.data {
            res.push_str(contents.borrow().to_string().as_str());
        }
    }
    res
}

pub fn find_children(node: &Rc<Node>, tag: &str) -> Vec<Rc<Node>> {
    let mut res = Vec::new();
    for child in node
        .children
        .borrow()
        .iter()
        .filter(|child| match child.data {
            NodeData::Element { ref name, .. } => &*name.local == tag,
            _ => false,
        })
    {
        res.push(child.clone())
    }
    res
}

pub fn first_child(node: &Rc<Node>, tag: &str) -> Option<Rc<Node>> {
    node.children
        .borrow()
        .iter()
        .find(|child| match child.data {
            NodeData::Element { ref name, .. } => &*name.local == tag,
            _ => false,
        })
        .cloned()
}
