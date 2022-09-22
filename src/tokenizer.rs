extern crate regex;
use self::regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Heading<'a> {
  pub token_type: &'a str,
  pub raw: &'a str,
  pub text: &'a str,
  pub depth: usize,
}

#[derive(Debug, PartialEq)]
pub enum Block<'a> {
  Heading(Heading<'a>),
}

pub fn heading(line: &str) -> Option<Block> {
  lazy_static! {
    static ref HEADING_RE: Regex = Regex::new(r"^(?P<depth>#{1,6})\s(?P<text>.*?)(?:\s#*)?$").unwrap();
  }
  if HEADING_RE.is_match(line) {
    let caps = HEADING_RE.captures(line).unwrap();
    return Some(Block::Heading(Heading {
      token_type: "heading",
      raw: line,
      text: caps.name("text").unwrap().as_str(),
      depth: caps.name("depth").unwrap().as_str().len(),
    }));
  }
  None
}

#[cfg(test)]
mod test {
  use tokenizer::{heading, Block, Heading};
  #[test]
  fn test_heading_level_1() {
    assert_eq!(
      heading("# hello world"),
      Some(Block::Heading(Heading {
        token_type: "heading",
        raw: "# hello world",
        text: "hello world",
        depth: 1,
      }))
    );
  }

  #[test]
  fn test_heading_level_2() {
    assert_eq!(
      heading("## hello world"),
      Some(Block::Heading(Heading {
        token_type: "heading",
        raw: "## hello world",
        text: "hello world",
        depth: 2,
      }))
    );
  }

  #[test]
  fn test_heading_level_3() {
    assert_eq!(
      heading("### hello world ##"),
      Some(Block::Heading(Heading {
        token_type: "heading",
        raw: "### hello world ##",
        text: "hello world",
        depth: 3,
      }))
    );
  }

  #[test]
  fn test_heading_level_7() {
    assert_eq!(heading("####### hello world"), None);
  }
}
