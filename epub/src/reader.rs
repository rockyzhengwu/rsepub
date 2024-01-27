use std::fs::File;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use zip::read::ZipArchive;

use crate::container::Container;
use crate::content::Content;
use crate::error::{EpubError, Result};
use crate::nav::Navigation;
use crate::package::Package;

pub trait EpubReader {
    fn container(&mut self) -> Result<Vec<u8>>;
    fn readfile(&mut self, path: &str) -> Result<Vec<u8>>;
}

pub struct ZipReader {
    inner: ZipArchive<Cursor<Vec<u8>>>,
}

impl ZipReader {
    pub fn new_from_path(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path.clone())
            .map_err(|e| EpubError::ReaderError(format!("open {:?} error;{:?}", path, e)))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| EpubError::ReaderError(format!("read {:?} error:{:?}", path, e)))?;
        ZipReader::new_from_memory(buffer)
    }

    pub fn new_from_memory(buffer: Vec<u8>) -> Result<Self> {
        let cursor = Cursor::new(buffer);
        let inner = ZipArchive::new(cursor)?;
        Ok(ZipReader { inner })
    }

    pub fn readfile(&mut self, filename: &str) -> Result<Vec<u8>> {
        let mut file = self
            .inner
            .by_name(filename)
            .map_err(|e| EpubError::ReaderError(format!("read{:?} error{:?}", filename, e)))?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)
            .map_err(|e| EpubError::ReaderError(format!("read {:?} error{:?}", filename, e)))?;
        Ok(content)
    }
}

impl EpubReader for ZipReader {
    fn container(&mut self) -> Result<Vec<u8>> {
        self.readfile("META-INF/container.xml")
    }

    fn readfile(&mut self, path: &str) -> Result<Vec<u8>> {
        self.readfile(path)
    }
}

pub struct DirReader {
    path: PathBuf,
}

impl DirReader {
    pub fn readfile(&mut self, path: PathBuf) -> Result<Vec<u8>> {
        let mut file = File::open(path.clone())
            .map_err(|e| EpubError::ReaderError(format!("open file {:?}, error:{:?}", path, e)))?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)
            .map_err(|e| EpubError::ReaderError(format!("read file:{:?} error:{:?}", path, e)))?;
        Ok(content)
    }
}

impl EpubReader for DirReader {
    fn container(&mut self) -> Result<Vec<u8>> {
        let fp = self.path.clone().join("META-INF/container.xml");
        if !fp.exists() {
            return Err(EpubError::ContainerError(format!(
                "META-INF/container.xml not in {:?}",
                self.path.display()
            )));
        }
        self.readfile(fp)
    }
    fn readfile(&mut self, path: &str) -> Result<Vec<u8>> {
        let fp = self.path.clone().join(path);
        if !fp.exists() {
            return Err(EpubError::ReaderError(format!(
                "{:?}not in {:?}",
                path,
                self.path.display()
            )));
        }
        self.readfile(fp)
    }
}

impl DirReader {
    pub fn try_new(path: PathBuf) -> Result<Self> {
        Ok(DirReader { path })
    }
}

pub struct Reader {
    inner: Box<dyn EpubReader>,
}

impl Reader {
    pub fn new_from_path(path: PathBuf) -> Result<Self> {
        let inner: Box<dyn EpubReader> = {
            if path.is_dir() {
                Box::new(DirReader::try_new(path)?)
            } else {
                Box::new(ZipReader::new_from_path(path)?)
            }
        };
        Ok(Reader { inner })
    }

    pub fn new_from_memory(buffer: Vec<u8>) -> Result<Self> {
        let inner = Box::new(ZipReader::new_from_memory(buffer)?);
        Ok(Reader { inner })
    }

    pub fn read_meta_container(&mut self) -> Result<Container> {
        let content = self.inner.container()?;
        Container::new(content.as_slice())
    }

    pub fn read_package(&mut self, path: &str) -> Result<Package> {
        let content = self.inner.readfile(path)?;
        Package::new(path, content.as_slice())
    }

    pub fn read_nav(&mut self, href: &str) -> Result<Navigation> {
        let content = self.inner.readfile(href)?;
        Navigation::new_from_nav(content.as_slice())
    }

    pub fn read_ncx(&mut self, href: &str) -> Result<Navigation> {
        let content = self.inner.readfile(href)?;
        Navigation::new_from_ncx(content.as_slice())
    }

    pub fn read_content(&mut self, href: &str) -> Result<Content> {
        let buf = self.inner.readfile(href)?;
        Content::new(href, buf.as_slice())
    }

    pub fn read_binary(&mut self, href: &str) -> Result<Vec<u8>> {
        self.inner.readfile(href)
    }
}
