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
    doc: XMLDocument,
}

impl Container {
    pub fn new(content: &[u8]) -> Result<Self> {
        let doc = XMLDocument::try_new(content)?;
        Ok(Container { doc })
    }

    pub fn full_path(&self) -> Result<String> {
        let rootfiles = self.root_files()?;
        Ok(rootfiles[0].full_path().to_string())
    }

    pub fn root_files(&self) -> Result<Vec<RootFile>> {
        let rootfiles = self.doc.find_all_tag("rootfile");
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

    #[allow(dead_code)]
    pub fn version(&self) -> Result<String> {
        let container = self
            .doc
            .find_tag("container")
            .ok_or(EpubError::ContainerError(
                "container not in container.xml file".to_string(),
            ))?;
        xml::parse_attribute_must_exist(&container, "version")
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> Result<String> {
        self.doc.to_string()
    }
}
