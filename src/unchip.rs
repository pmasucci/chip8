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
  let word = (buffer[pc] as u16) << 8 | buffer[pc + 1] as u16;
  let instruction = find_opcode(word);
  println!(
    "{:0>5x} {:02x} {:02x} {}",
    pc << 8,
    word >> 8,
    word & 0x00FF,
    instruction
  );
}

fn find_opcode(code: u16) -> String {
  let first_nibble = code >> 12;
  let instruction = match first_nibble {
    0x0 => match code & 0x00ff {
      0xE0 => "CLS".to_string(),
      0xEE => "RET".to_string(),
      _ => "UNIMPLEMENTED".to_string(),
    },
    0x1 => format!("{:<13} {:#x}", "JP", code & 0x0fff),
    0x2 => format!("{:<13} {:>#05x}", "CALL", code & 0x0fff),
    0x3 => format!(
      "{:<9} V{:X}, {:>#05x}",
      "SE",
      (code & 0x0f00) >> 8,
      code & 0x00ff
    ),
    0x4 => format!(
      "{:<9} V{:X}, {:>#05x}",
      "SNE",
      (code & 0x0f00) >> 8,
      code & 0x00ff
    ),
    0x5 => format!(
      "{:<9} V{:X}, V{:X}",
      "SE",
      (code & 0x0f00) >> 8,
      (code & 0x00f0) >> 4
    ),
    0x6 => format!(
      "{:<9} V{:X}, {:>#05x}",
      "LD",
      (code & 0x0f00) >> 8,
      code & 0x00ff
    ),
    0x7 => format! {"{:<9} V{:X}, {:>#05x}", "ADD", (code & 0x0f00) >> 8, code & 0x00ff},
    0x8 => match code & 0x000f {
      0x0 => format!(
        "{:<9} V{:X}, V{:X}",
        "LD",
        (code & 0x0f00) >> 8,
        (code & 0x00f0) >> 4
      ),
      0x1 => format!(
        "{:<9} V{:X}, V{:X}",
        "OR",
        (code & 0x0f00) >> 8,
        (code & 0x00f0) >> 4
      ),
      0x2 => format!(
        "{:<9} V{:X}, V{:X}",
        "AND",
        (code & 0x0f00) >> 8,
        (code & 0x00f0) >> 4
      ),
      0x3 => format!(
        "{:<9} V{:X}, V{:X}",
        "XOR",
        (code & 0x0f00) >> 8,
        (code & 0x00f0) >> 4
      ),
      0x4 => format!(
        "{:<9} V{:X}, V{:X}",
        "ADD.",
        (code & 0x0f00) >> 8,
        (code & 0x00f0) >> 4
      ),
      0x5 => format!(
        "{:<9} V{:X}, V{:X}",
        "SUB.",
        (code & 0x0f00) >> 8,
        (code & 0x00f0) >> 4
      ),
      0x6 => format!(
        "{:<9} V{:X}, V{:X}",
        "SHR.",
        (code & 0x0f00) >> 8,
        (code & 0x00f0) >> 4
      ),
      0x7 => format!(
        "{:<9} V{:X}, V{:X}",
        "SUBN.",
        (code & 0x0f00) >> 8,
        (code & 0x00f0) >> 4
      ),
      0xE => format!(
        "{:<9} V{:X}, V{:X}",
        "SHL.",
        (code & 0x0f00) >> 8,
        (code & 0x00f0) >> 4
      ),
      _ => "UNIMPLEMENTED".to_string(),
    },
    0x9 => format!(
      "{:<9} V{:X}, V{:X}",
      "SNE",
      (code & 0x0f00) >> 8,
      (code & 0x00f0) >> 4
    ),
    0xA => format!("{:<9}  I, {:>#05x}", "LD", code & 0x0fff),
    0xB => format!("{:<9} V0, {:>#05x}", "JP", code & 0x0fff),
    0xC => format!(
      "{:<9} V{:X}, {:>#05x}",
      "RND",
      (code & 0x0f00) >> 8,
      (code & 0x00ff)
    ),
    0xD => format!(
      "{:<5} V{:X}, V{:X}, {:>#05x}",
      "DRW",
      (code & 0x0f00) >> 8,
      (code & 0x00f0) >> 4,
      code & 0x000f
    ),
    0xE => match code & 0x00ff {
      0x9E => format!("{:<9} V{:X}", "SKP", (code & 0x0f00) >> 8),
      0xA1 => format!("{:<9} V{:X}", "SKNP", (code & 0x0f00) >> 8),
      _ => "UNIMPLEMENTED".to_string(),
    },
    0xF => match code & 0x00ff {
      0x07 => format!("{:<9} V{:X}, DT", "LD", (code & 0x0f00) >> 8),
      0x0A => format!("{:<9} V{:X}, K", "LD", (code & 0x0f00) >> 8),
      0x15 => format!("{:<9} DT, V{:X}", "LD", (code & 0x0f00) >> 8),
      0x18 => format!("{:<9} ST, V{:X}", "LD", (code & 0x0f00) >> 8),
      0x1E => format!("{:<9} I, V{:X}", "ADD", (code & 0x0f00) >> 8),
      0x29 => format!("{:<9} F, V{:X}", "LD", (code & 0x0f00) >> 8),
      0x33 => format!("{:<9} B, V{:X}", "LD", (code & 0x0f00) >> 8),
      0x55 => format!("{:<9} [I], V{:X}", "LD", (code & 0x0f00) >> 8),
      0x65 => format!("{:<9} V{:X}, [I]", "LD", (code & 0x0f00) >> 8),
      _ => "UNIMPLEMENTED".to_string(),
    },
    _ => "instruction not recognized".to_string(),
  };

  instruction.to_string()
}
