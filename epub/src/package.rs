use std::collections::HashMap;
use std::rc::Rc;

use markup5ever_rcdom::{Node, NodeData};

use crate::error::{EpubError, Result};
use crate::xml::{self, XMLDocument};

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct MetaItem {
    ns: String,
    name: String,
    content: String,
    attrs: HashMap<String, String>,
}

#[derive(Debug, Default)]
pub struct MetaData {
    items: HashMap<String, MetaItem>,
}

impl MetaData {
    pub fn parse(xmldoc: &XMLDocument) -> Result<Self> {
        let mut metadata = MetaData::default();
        let metadata_node = xmldoc.find_tag("metadata").ok_or(EpubError::FormatError(
            "metadata is null in package format".to_string(),
        ))?;

        for child in metadata_node
            .children
            .borrow()
            .iter()
            .filter(|ch| matches!(ch.data, NodeData::Element { .. }))
        {
            let mut metaitem = MetaItem::default();
            if let NodeData::Element {
                ref name,
                ref attrs,
                ..
            } = child.data
            {
                let tagname = name.local.to_string();
                metaitem.name = tagname.clone();
                for attr in attrs.borrow().iter() {
                    let name = attr.name.local.to_string();
                    let value = attr.value.to_string();
                    metaitem.attrs.insert(name, value);
                }
                metaitem.content = xml::parse_text(child);
                metadata.items.insert(tagname, metaitem);
            }
        }

        Ok(metadata)
    }
    pub fn title(&self) -> &str {
        if let Some(item) = self.items.get("title") {
            return &item.content;
        }
        ""
    }
}

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct ManifestItem {
    id: String,
    href: String,
    media_type: String,
    fallback: Option<String>,
    properties: Option<String>,
    media_overlay: Option<String>,
}

impl ManifestItem {
    fn parse(node: &Rc<Node>) -> Result<Self> {
        let id = xml::parse_attribute_must_exist(node, "id")?;
        let href = xml::parse_attribute_must_exist(node, "href")?;
        let media_type = xml::parse_attribute_must_exist(node, "media-type")?;
        let fallback = xml::parse_attribute(node, "fallback");
        let properties = xml::parse_attribute(node, "properties");
        let media_overlay = xml::parse_attribute(node, "media-overlay");
        Ok(ManifestItem {
            id,
            href,
            media_type,
            fallback,
            properties,
            media_overlay,
        })
    }

    pub fn href(&self) -> &str {
        &self.href
    }

    pub fn media_type(&self) -> &str {
        &self.media_type
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ItemRef {
    idref: String,
    id: Option<String>,
    linear: Option<String>,
    properties: Option<String>,
}

impl ItemRef {
    fn parse(node: &Rc<Node>) -> Result<Self> {
        let idref = xml::parse_attribute_must_exist(node, "idref")?;
        let id = xml::parse_attribute(node, "id");
        let linear = xml::parse_attribute(node, "linear");
        let properties = xml::parse_attribute(node, "properties");
        Ok(ItemRef {
            idref,
            id,
            linear,
            properties,
        })
    }

    pub fn idref(&self) -> &str {
        &self.idref
    }
    pub fn id(&self) -> &Option<String> {
        &self.id
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Reference {
    t: String,
    title: Option<String>,
    href: Option<String>,
}

impl Reference {
    pub fn parse(node: &Rc<Node>) -> Result<Self> {
        let t = xml::parse_attribute_must_exist(node, "type")?;
        let title = xml::parse_attribute(node, "title");
        let href = xml::parse_attribute(node, "href");
        Ok(Reference { t, title, href })
    }
}

#[allow(dead_code)]
pub struct Package {
    path: String,
    doc: XMLDocument,
    metadata: MetaData,
    manifest: HashMap<String, ManifestItem>,
    guide: Vec<Reference>,
    spine: Vec<ItemRef>,
}
pub fn parse_guide(doc: &XMLDocument) -> Result<Vec<Reference>> {
    let mut guide = Vec::new();
    if let Some(gs) = doc.find_tag("guide") {
        for item in xml::find_children(&gs, "reference") {
            let r = Reference::parse(&item)?;
            guide.push(r);
        }
    }
    Ok(guide)
}

pub fn parse_spine(doc: &XMLDocument) -> Result<Vec<ItemRef>> {
    let mut spine = Vec::new();
    if let Some(spine_node) = doc.find_tag("spine") {
        for item in xml::find_children(&spine_node, "itemref") {
            let s = ItemRef::parse(&item)?;
            spine.push(s);
        }
    }
    Ok(spine)
}
pub fn parse_manifest(doc: &XMLDocument) -> Result<HashMap<String, ManifestItem>> {
    let mut manifest = HashMap::new();
    if let Some(mainfest_node) = doc.find_tag("manifest") {
        let items = xml::find_children(&mainfest_node, "item");
        for item in items {
            let manifest_item = ManifestItem::parse(&item)?;
            manifest.insert(manifest_item.id.clone(), manifest_item);
        }
    } else {
        return Err(EpubError::FormatError("manifest is null".to_string()));
    }
    Ok(manifest)
}

impl Package {
    pub fn new(path: &str, content: &[u8]) -> Result<Self> {
        let doc = XMLDocument::try_new(content)?;
        let metadata = MetaData::parse(&doc)?;
        let guide = parse_guide(&doc)?;
        let spine = parse_spine(&doc)?;
        let manifest = parse_manifest(&doc)?;
        Ok(Package {
            path: path.to_string(),
            metadata,
            manifest,
            spine,
            guide,
            doc,
        })
    }

    pub fn title(&self) -> &str {
        self.metadata.title()
    }

    pub fn spine(&self) -> &[ItemRef] {
        self.spine.as_slice()
    }
    pub fn metadata(&self) -> &MetaData {
        &self.metadata
    }

    pub fn get_manifest(&self, name: &str) -> Option<&ManifestItem> {
        self.manifest.get(name)
    }

    pub fn manifest(&self) -> &HashMap<String, ManifestItem> {
        &self.manifest
    }

    pub fn chapter(&self, n: u32) -> Option<ManifestItem> {
        unimplemented!()
    }
}
