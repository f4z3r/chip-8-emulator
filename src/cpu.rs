//! CPU module

use interconnect::Interconnect;

/// A CHIP-8 CPU.
pub struct Cpu {
    // interconnect allowing access to peripherals
    interconnect: Interconnect,
    // program counter
    pc: u16,
    // function call stack
    stack: [u16; 16],
    // stack pointer
    sp: u8,
    // general purpose registers
    v: [u8; 16],
    // address register
    i: u16,
    // timer registers
    dt: u8,
}

impl Cpu {
    /// Constructor.
    ///
    /// # Arguments
    /// - `interconnect`: the interconnect that the CPU will use to communicate with memory and peripherals.
    pub fn new(interconnect: Interconnect) -> Cpu {
        Cpu {
            interconnect,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            v: [0; 16],
            i: 0,
            dt: 0
        }
    }

    /// Execute instructions from memory.
    pub fn run(&mut self) {
        unimplemented!();
    }
}
