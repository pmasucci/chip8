use std::io::{self, Write};
pub struct Screen {
  pub height: u8,
  pub width: u8,
}
pub struct DisplayDriver {
  screen: Screen,
  scale: u8,
  primary_color: [u8; 4],
}

impl DisplayDriver {
  pub fn new(screen: Screen, scale: u8, primary_color: [u8; 4]) -> DisplayDriver {
    DisplayDriver {
      screen,
      scale,
      primary_color,
    }
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
