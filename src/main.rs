extern crate getopts;
mod chip;
mod cpu;
mod display;
mod unchip;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("u", "unchip", "dissassemble chip8 program");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(error) => panic!(error.to_string()),
    };

    let rom = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    if matches.opt_present("u") {
        unchip::unchip(&rom);
    } else {
        chip::emulate(&rom);
    }
}
