//! Memory module.

/// Font set of the CHIP-8
static FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,   // 0
    0x20, 0x60, 0x20, 0x20, 0x70,   // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0,   // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0,   // 3
    0x90, 0x90, 0xF0, 0x10, 0x10,   // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0,   // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0,   // 6
    0xF0, 0x10, 0x20, 0x40, 0x40,   // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0,   // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0,   // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90,   // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0,   // B
    0xF0, 0x80, 0x80, 0x80, 0xF0,   // C
    0xE0, 0x90, 0x90, 0x90, 0xE0,   // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0,   // E
    0xF0, 0x80, 0xF0, 0x80, 0x80    // F
];


/// Initial offset of program memory
pub const END_RESERVED: usize = 0x200;

/// Memory of the program
pub struct Memory {
    ram: [u8; 4096]
}

#[allow(dead_code)]
impl Memory {
    /// Constructor
    pub fn new(rom: Vec<u8>) -> Memory {
        let mut memory = [0; 4096];
        Memory::dump_fontset(&mut memory);
        Memory::dump_program(&mut memory, rom);
        Memory { ram: memory }
    }

    /// Read from memory at address `addr`
    #[inline(always)]
    pub fn read(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    /// Write to memory at address `addr`
    #[inline(always)]
    pub fn write(&mut self, addr: usize, byte: u8) {
        self.ram[addr] = byte;
    }

    /// Read a word from memory
    #[inline(always)]
    pub fn read_word(&self, addr: usize) -> u16 {
        (self.ram[addr] as u16) << 8 | (self.ram[addr + 1] as u16)
    }

    /// Reads a slice from memory.
    #[inline(always)]
    pub fn get_slice(&self, addr: usize, length: u8) -> &[u8] {
        &self.ram[addr..(addr + length as usize)]
    }

    /// Gets a mutable reference to a slice from memory.
    #[inline(always)]
    pub fn get_slice_mut(&mut self, addr: usize, length: u8) -> &mut [u8] {
        &mut self.ram[addr..(addr + length as usize)]
    }

    /// Loads the program into memory
    fn dump_program(memory: &mut [u8], rom: Vec<u8>) {
        for idx in 0..rom.len() {
            memory[idx + END_RESERVED] = rom[idx];
        }
    }

    /// Loads the fontset into memory
    fn dump_fontset(memory: &mut [u8]) {
        for idx in 0..FONTSET.len() {
            memory[idx] = FONTSET[idx];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_mem() -> Memory {
        let rom = vec![1, 2, 3, 4, 5, 6, 7];
        let memory = Memory::new(rom);
        memory
    }

    #[test]
    fn read_write() {
        let mut memory = get_mem();
        assert_eq!(memory.read(20), 0x90, "first byte of the 5th font is returned");
        memory.write(0x200, 8);
        assert_eq!(memory.read(0x201), 2, "second byte of program code is returned");
        assert_eq!(memory.read(0x200), 8, "first overwriten byte of program code is returned");
    }

    #[test]
    fn read_slice() {
        let mut memory = get_mem();
        memory.write(0x207, 8);
        memory.write(0x208, 9);
        memory.write(0x209, 10);
        memory.write(0x20a, 11);
        assert_eq!(memory.get_slice(0x200, 11), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    }

    #[test]
    fn write_slice() {
        let mut memory = get_mem();
        memory.get_slice_mut(0x200, 8)[7] = 8;
        assert_eq!(memory.read(0x207), 8);
    }

    #[test]
    fn read_word() {
        let memory = get_mem();
        assert_eq!(memory.read_word(0x200), (1 as u16) << 8 | (2 as u16));
    }
}
