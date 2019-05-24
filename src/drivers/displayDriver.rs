use std::io::{self, Write};
pub struct DisplayDriver {}

impl DisplayDriver {
  pub fn new() -> DisplayDriver {
    DisplayDriver {}
  }
  pub fn display(&mut self, screen: [u8; 2048]) {
    println!("------------------------------------------------");
    for i in 0..32 {
      for j in 0..64 {
        let pixel = if screen[j + i * 64] == 1 { "*" } else { " " };
        print!("{}", pixel);
      }
      print!("\n");
      io::stdout().flush().unwrap();
    }
  }
}
