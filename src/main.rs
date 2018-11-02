//! Implementation of CHIP-8 emulator. I implemented this to understand how emulators work and learn more about
//! computer architecture.
//!
//! Some of the code contained in this implementation is based on the following:
//! - https://github.com/ColinEberhardt/wasm-rust-chip8
//! - https://github.com/mikezaby/chip-8.rs
//! - https://github.com/Reshurum/notch

#[macro_use] extern crate clap;
extern crate sdl2;
extern crate rand;

use clap::App;

mod cpu;
mod interconnect;
mod vm;
mod memory;
mod input;
mod graphics;
mod prelude;


fn main() {
    let yaml = load_yaml!("../static/cli.yml");
    let matches = App::from_yaml(yaml).version(env!("CARGO_PKG_VERSION")).get_matches();
    let rom = matches.value_of("ROM").expect("ROM should be supplied");
    let rom_path = format!("{}/static/roms/{}", env!("CARGO_MANIFEST_DIR"), rom);
    let mut vm = vm::VirtualMachine::new(&rom_path);
    vm.run();
}
