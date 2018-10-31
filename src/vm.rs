//! Virtual machine module. Contains a machine that is being emulated.

use std::path::Path;

use cpu::Cpu;
use interconnect::Interconnect;


/// A virtual machine emulating the CHIP-8.
pub struct VirtualMachine {
    cpu: Cpu
}

impl VirtualMachine {
    pub fn new(rom: &str) -> VirtualMachine {
        let memory = read_bytes(rom);
        let interconnect = Interconnect::new(memory);
        let cpu = Cpu::new(interconnect);
        VirtualMachine { cpu }
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
}


fn read_bytes<P>(path: P) -> Vec<u8> where P: AsRef<Path> {
    unimplemented!();
}
