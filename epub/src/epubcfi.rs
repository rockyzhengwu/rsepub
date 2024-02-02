/**
https://idpf.org/epub/linking/cfi/epub-cfi.html
**/
use crate::error::{EpubError, Result};

#[derive(Debug, Default, Clone)]
pub struct Step {
    order: u32,
    asseration: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Offset {}

#[derive(Debug, Default, Clone)]
pub struct Path {
    steps: Vec<Step>,
    offset: Option<Offset>,
}

#[derive(Debug, Default, Clone)]
pub struct Range {
    start: Path,
    end: Path,
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

    fn get_char(&mut self) -> Option<&char> {
        let char = self.content.get(self.current);
        self.current += 1;
        char
    }

    fn is_finish(&self) -> bool {
        self.current >= self.content.len()
    }

    pub fn parse(&mut self) -> Result<Epubcfi> {
        let mut steps = Vec::new();
        let mut paths = Vec::new();
        while !self.is_finish() {
            let c = self.get_char().unwrap();
            match c {
                '/' => {
                    let item = self.parse_path_item();
                    steps.push(item);
                }
                '!' => {
                    if !steps.is_empty() {
                        let path = Path {
                            steps: steps.clone(),
                            offset: None,
                        };
                        steps.clear();
                        paths.push(path);
                    }
                }
                ',' | ':' | '~' | '@' => {
                    if !steps.is_empty() {
                        steps.clear();
                    }
                    // TODO parse range
                    println!("start parse range: {:?}", self.current);
                    break;
                }
                _ => {
                    println!("impossiable:{}", c);
                }
            }
        }
        Ok(Epubcfi { paths, range: None })
    }

    fn parse_path_item(&mut self) -> Step {
        let mut order: u32 = 0;
        let mut asseration = None;
        while !self.is_finish() {
            let c = self.get_char().unwrap();
            if c.is_ascii_digit() {
                order = order * 10 + c.to_digit(10).unwrap();
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
        Step { order, asseration }
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
}

#[cfg(test)]
mod tests {
    use crate::epubcfi::Epubcfi;

    #[test]
    fn test_parse() {
        let content = "/6/8!/4/4/20,/1:4,/1:21";
        let epubcfi = Epubcfi::try_new(content).unwrap();
        println!("{:?}", epubcfi.chapter());
    }
}
