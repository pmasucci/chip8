use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn unchip(rom: &str) {
  println!("unchip {}", rom);
  let path = Path::new(rom);
  let mut f = File::open(path).expect("file not found!");

  let mut buffer = [0u8; 3584]; // 3584 is max filesize for chip8 games
  let bytes_read = if let Ok(bytes_read) = f.read(&mut buffer) {
    bytes_read
  } else {
    0
  };
  println!("bytes read: {}", bytes_read);
  let mut pc = 0;

  while pc < bytes_read {
    read_instruction(&buffer, pc);
    pc += 2;
  }
}

fn read_instruction(buffer: &[u8], pc: usize) {
  let code = [buffer[pc], buffer[pc + 1]];

  println!("{:0<4x} {:x} {:x}", pc, code[0], code[1]);
}
