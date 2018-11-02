//! Interconnect module

use sdl2;

use prelude::*;
use memory::Memory;
use graphics::{Display, TestDisplay};
use input::{Keyboard, TestKeyboard};

/// An interconnect allowing access to memory, peripherals, etc.
pub struct Interconnect<T, U> where T: Input, U: Graphics {
    /// Main memory
    pub memory: Memory,
    /// Grahpics
    pub graphics: U,
    /// Input
    pub input: T,
}

impl Interconnect<Keyboard, Display> {
    /// Constructor.
    pub fn new(rom: Vec<u8>) -> Interconnect<Keyboard, Display> {
        let context = sdl2::init().unwrap();
        let memory = Memory::new(rom);
        let graphics = Display::new(&context);
        let input = Keyboard::new(&context);

        Interconnect {
            memory,
            graphics,
            input
        }
    }
}

impl Interconnect<TestKeyboard, TestDisplay> {
    /// Constructor for a testing interconnect with fake keyboard and fake display.
    #[allow(dead_code)]
    pub fn new_test(rom: Vec<u8>) -> Interconnect<TestKeyboard, TestDisplay> {
        let memory = Memory::new(rom);
        let graphics = TestDisplay::new_test();
        let input = TestKeyboard::new_test();

        Interconnect {
            memory,
            graphics,
            input
        }
    }
}
