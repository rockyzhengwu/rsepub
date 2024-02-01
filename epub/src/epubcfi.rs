/**
https://idpf.org/epub/linking/cfi/epub-cfi.html
**/
use crate::error::Result;

#[derive(Debug, Default)]
pub struct Epubcfi {
    target: String,
}

pub struct CfiPathItem {
    order: u32,
    id: Option<String>,
}

pub struct CfiPath {
    step: Vec<CfiPathItem>,
}

pub struct CfiRange {}

impl Epubcfi {
    pub fn parse(target: &str) -> Epubcfi {
        Epubcfi {
            target: target.to_string(),
        }
    }

    pub fn chapter(&self) -> Result<Vec<CfiPathItem>> {
        unimplemented!()
    }
}
