const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
  pub memory: [u8; 2048],
}

impl Display {
  pub fn new() -> Display {
    Display { memory: [0; 2048] }
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
    self.memory[x + y * WIDTH] = on as u8;
  }

  pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
    self.memory[x + y * WIDTH] == 1
  }

  pub fn cls(&mut self) {
    for x in 0..WIDTH {
      for y in 0..HEIGHT {
        self.set_pixel(x, y, false);
      }
    }
  }

  pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
    let sprite_rows = sprite.len();
    let mut collision = false;
    for row in 0..sprite_rows {
      let sprite_row = sprite[row] as usize;
      for sprite_column in 0..8 {
        let new_value = sprite_row >> (7 - sprite_column) & 0x01;
        if new_value == 1 {
          let sprite_x = (x + sprite_column) % WIDTH;
          let sprite_y = (y + sprite_row) % HEIGHT;
          let old_value = self.get_pixel(sprite_x, sprite_y);
          if old_value {
            collision = true;
          }

          self.set_pixel(sprite_column, sprite_row, (new_value == 1) ^ old_value);
        }
      }
    }
    return collision;
  }
}

pub static FONT_SET: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
