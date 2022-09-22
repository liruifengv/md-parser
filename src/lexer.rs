use tokenizer;
use tokenizer::Block;

pub fn lex(src: &str) -> Vec<Block> {
  let mut tokens = vec![];
  let lines: Vec<&str> = src.lines().collect();
  for line in lines {
    match tokenizer::heading(line) {
      Some(heading) => tokens.push(heading),
      None => println!("none"),
    };
  }
  tokens
}

#[cfg(test)]
mod test {
  use lexer::lex;
  use tokenizer::Block;
  use tokenizer::Heading;
  #[test]
  fn test_heading_level_1() {
    assert_eq!(
      lex("# hello world"),
      vec![Block::Heading(Heading {
        token_type: "heading",
        raw: "# hello world",
        text: "hello world",
        depth: 1,
      })]
    );
  }

  #[test]
  fn test_heading_level_2() {
    assert_eq!(
      lex("## hello world"),
      vec![Block::Heading(Heading {
        token_type: "heading",
        raw: "## hello world",
        text: "hello world",
        depth: 2,
      })]
    );
  }

  #[test]
  fn test_heading_level_3() {
    assert_eq!(
      lex("### hello world ##"),
      vec![Block::Heading(Heading {
        token_type: "heading",
        raw: "### hello world ##",
        text: "hello world",
        depth: 3,
      })]
    );
  }
}
