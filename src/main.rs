use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() -> std::io::Result<()> {
  println!("Hello, 这里是一个 Markdown 解析器!");

  let args: Vec<String> = env::args().collect();

  if args.len() > 1 {
    let target = &args[1];

    if Path::new(target).is_file() {
      let mut file = fs::File::open(target).unwrap();

      let mut contents = String::new();
      file.read_to_string(&mut contents).unwrap();
      println!("读取到的 Markdown 内容是 {}", contents);

      Ok(())
    } else {
      panic!("请选择 Markdown 文件")
    }
  } else {
    panic!("请选择 Markdown 文件")
  }
}
