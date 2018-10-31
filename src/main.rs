#[macro_use] extern crate clap;
extern crate sdl2;

use clap::App;

mod cpu;
mod interconnect;
mod vm;

fn main() {
    let yaml = load_yaml!("../static/cli.yml");
    let matches = App::from_yaml(yaml).version(env!("CARGO_PKG_VERSION")).get_matches();
    let rom = matches.value_of("ROM").expect("ROM should be supplied");

}
