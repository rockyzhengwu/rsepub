use crate::error::EpubError;
use crate::error::Result;
use crate::xml::{self, XMLDocument};

#[derive(Debug)]
#[allow(dead_code)]
pub struct RootFile {
    full_path: String,
    media_type: String,
}

impl RootFile {
    pub fn full_path(&self) -> &str {
        &self.full_path
    }
}

pub(crate) struct Container {
    version: String,
    root_files: Vec<RootFile>,
}

fn parse_root_files(doc: &XMLDocument) -> Result<Vec<RootFile>> {
    let rootfiles = doc.find_all_tag("rootfile");
    let mut files = Vec::new();
    for rootfile in rootfiles {
        let full_path = xml::parse_attribute_must_exist(&rootfile, "full-path")?;
        let media_type = xml::parse_attribute_must_exist(&rootfile, "media-type")?;
        if media_type != "application/oebps-package+xml" {
            return Err(EpubError::FormatError(
                "media_type must be applicatioin/oebps-package+xml".to_string(),
            ));
        }
        files.push(RootFile {
            full_path,
            media_type,
        })
    }
    Ok(files)
}

fn parse_version(doc: &XMLDocument) -> Result<String> {
    let container = doc.find_tag("container").ok_or(EpubError::ContainerError(
        "container not in container.xml file".to_string(),
    ))?;
    xml::parse_attribute_must_exist(&container, "version")
}

impl Container {
    pub fn new(content: &[u8]) -> Result<Self> {
        let doc = XMLDocument::try_new(content)?;
        let version = parse_version(&doc)?;
        let root_files = parse_root_files(&doc)?;
        Ok(Container {
            version,
            root_files,
        })
    }

    pub fn full_path(&self) -> Option<String> {
        if self.root_files.is_empty() {
            return None;
        }
        Some(self.root_files().first().unwrap().full_path().to_string())
    }

    pub fn root_files(&self) -> &[RootFile] {
        self.root_files.as_slice()
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
