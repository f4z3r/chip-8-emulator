//! Virtual machine module. Contains a machine that is being emulated.

use std;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use cpu::Cpu;
use input::Keyboard;
use graphics::Display;
use interconnect::Interconnect;


/// A virtual machine emulating the CHIP-8.
pub struct VirtualMachine {
    cpu: Cpu<Keyboard, Display>
}

impl VirtualMachine {
    /// Constructor.
    pub fn new(rom: &str) -> VirtualMachine {
        let memory = VirtualMachine::get_bytes(rom);
        let interconnect: Interconnect<Keyboard, Display> = Interconnect::new(memory);
        let cpu = Cpu::new(interconnect);
        VirtualMachine { cpu }
    }

    /// Run the VM.
    pub fn run(&mut self) {
        self.cpu.run();
    }

    /// Get binary from storage
    fn get_bytes<P: AsRef<Path>>(path: P) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        let filename = format!("{}", path.as_ref().display());

        // Open and read the file if it exists.
        match File::open(path) {
            Ok(ref mut file)    => {
                file.read_to_end(&mut buffer).unwrap();
            },
            Err(why)            => {
                println!("Cannot open '{}': {}", filename, why);
                std::process::exit(1);
            },
        };

        buffer
    }
}
