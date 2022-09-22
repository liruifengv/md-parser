use tokenizer;
use tokenizer::Block;

pub fn lex(src: &String) -> Vec<Block> {
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
