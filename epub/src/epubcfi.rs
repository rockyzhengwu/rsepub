/**
https://idpf.org/epub/linking/cfi/epub-cfi.html
**/
use crate::error::{EpubError, Result};

#[derive(Debug, Default, Clone)]
pub struct Step {
    index: u32,
    #[allow(dead_code)]
    asseration: Option<String>,
}
impl Step {
    pub fn index(&self) -> u32 {
        self.index / 2
    }
}

#[derive(Debug, Default, Clone)]
pub struct Offset {}

#[derive(Debug, Default, Clone)]
pub struct Path {
    steps: Vec<Step>,
}

impl Path {
    pub fn steps(&self) -> &[Step] {
        self.steps.as_slice()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RangeItem {
    offset: u32,
    n: u32,
}

impl RangeItem {
    fn new(offset: u32, n: u32) -> Self {
        RangeItem { offset, n }
    }

    pub fn is_element(&self) -> bool {
        self.offset % 2 == 0
    }

    pub fn offset(&self) -> u32 {
        //TODO check underflow
        if self.is_element() {
            self.offset / 2 - 1
        } else {
            (self.offset - 1) / 2
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Range {
    start: RangeItem,
    end: RangeItem,
}
impl Range {
    pub fn start_offset(&self) -> u32 {
        self.start.offset()
    }

    pub fn end_offset(&self) -> u32 {
        self.end.offset()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Epubcfi {
    paths: Vec<Path>,
    range: Option<Range>,
}

pub struct EpubCfiParser<'a> {
    content: &'a [char],
    current: usize,
}
impl<'a> EpubCfiParser<'a> {
    pub fn new(content: &'a [char]) -> Self {
        EpubCfiParser {
            content,
            current: 0,
        }
    }

    fn get_char(&mut self) -> Result<&char> {
        let char = self
            .content
            .get(self.current)
            .ok_or(EpubError::CfiError(format!(
                "read char error:{}",
                self.current()
            )));
        self.current += 1;
        char
    }

    fn is_finish(&self) -> bool {
        self.current >= self.content.len()
    }

    pub fn parse(&mut self) -> Result<Epubcfi> {
        let mut steps = Vec::new();
        let mut paths = Vec::new();
        let mut range = None;
        while !self.is_finish() {
            let c = self.get_char()?;
            match c {
                '/' => {
                    let item = self.parse_path_item();
                    steps.push(item);
                }
                '!' => {
                    if !steps.is_empty() {
                        let path = Path {
                            steps: steps.clone(),
                        };
                        steps.clear();
                        paths.push(path);
                    }
                }
                ',' | ':' | '~' | '@' => {
                    if !steps.is_empty() {
                        steps.clear();
                    }
                    range = Some(self.parse_range()?);
                    break;
                }
                _ => {
                    println!("impossiable:{}", c);
                }
            }
        }
        Ok(Epubcfi { paths, range })
    }

    fn current(&self) -> usize {
        self.current
    }

    fn read_number(&mut self) -> Result<u32> {
        let mut n: u32 = 0;

        loop {
            if self.is_finish() {
                break;
            }
            let cc = self.get_char()?;
            if !cc.is_ascii_digit() {
                self.current -= 1;
                break;
            }
            let v = cc.to_digit(10).ok_or(EpubError::CfiError(format!(
                "char not a number at :{:?}",
                self.current()
            )))?;
            n += n * 10 + v;
        }
        Ok(n)
    }

    fn parse_range(&mut self) -> Result<Range> {
        let c = self.get_char()?;
        if c != &'/' {
            return Err(EpubError::CfiError(
                "expected '/' when in range".to_string(),
            ));
        }
        let n = self.read_number()?;
        let c = self.get_char()?;
        if c != &':' {
            return Err(EpubError::CfiError("range need a offset".to_string()));
        }
        let offset = self.read_number()?;
        let start = RangeItem { offset, n };
        let c = self.get_char()?;
        if c != &',' {
            return Err(EpubError::CfiError(String::from(
                "expected ',' when in range",
            )));
        }
        let c = self.get_char()?;
        if c != &'/' {
            return Err(EpubError::CfiError(String::from(
                "expected '/' when in range",
            )));
        }
        let n = self.read_number()?;
        let c = self.get_char()?;
        if c != &':' {
            return Err(EpubError::CfiError("range need a offset".to_string()));
        }
        let offset = self.read_number()?;
        let end = RangeItem::new(offset, n);
        Ok(Range { start, end })
    }

    fn parse_path_item(&mut self) -> Step {
        let mut index: u32 = 0;
        let mut asseration = None;
        while !self.is_finish() {
            let c = self.get_char().unwrap();
            if c.is_ascii_digit() {
                index = index * 10 + c.to_digit(10).unwrap();
            } else if *c == '[' {
                let mut idvalue = String::new();
                let mut ch = self.get_char().unwrap();
                while *ch != ']' {
                    idvalue.push(ch.to_owned());
                    ch = self.get_char().unwrap();
                }
                asseration = Some(idvalue);
                self.current -= 1;
                break;
            } else {
                self.current -= 1;
                break;
            }
        }
        Step { index, asseration }
    }
}

impl Epubcfi {
    pub fn try_new(target: &str) -> Result<Self> {
        let chars: Vec<char> = target.chars().collect();
        let mut parser = EpubCfiParser::new(chars.as_slice());
        parser.parse()
    }

    pub fn chapter(&self) -> Option<&Path> {
        self.paths.first()
    }
    pub fn range(&self) -> Option<&Range> {
        self.range.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::epubcfi::{Epubcfi, RangeItem};

    #[test]
    fn test_parse() {
        let content = "/6/8!/4/4/20,/1:4,/1:21";
        let epubcfi = Epubcfi::try_new(content).unwrap();
        let chapter = epubcfi.chapter().unwrap();
        assert_eq!(chapter.steps.len(), 2);
        let range = epubcfi.range().unwrap();
        assert_eq!(range.start, RangeItem::new(4, 1));
    }
}
