//! Input module

use std::thread::sleep;
use std::time::Duration;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Wait for the duration it takes for an instruction to execute.
const INPUT_WAIT_DELAY: u64 = 2;

/// Trait implemented by all input devices
pub trait Input {
    /// Constructor.
    fn new(context: &sdl2::Sdl) -> Self;

    /// Input handling.
    fn handle_inputs(&mut self);

    /// Wait for an input event.
    fn wait_input(&mut self) -> u8;

    /// Checks if a key is pressed.
    fn is_key_down(&self, key: u8) -> bool;

    /// Checks if a close was requested.
    fn close_requested(&self) -> bool;
}

/// A keyboard
pub struct Keyboard {
    event_pump: sdl2::EventPump,
    state: [bool; 16],
    last_input: u8,
    input_dirty: bool,
    close_requested: bool
}

impl Keyboard {
    /// Set an input.
    fn set_input(&mut self, key: u8, value: bool) {
        self.state[key as usize] = value;
        self.last_input = key;
        self.input_dirty = true;
    }
}

impl Input for Keyboard {
    /// Constructor
    fn new(context: &sdl2::Sdl) -> Self {
        let event_pump = context.event_pump().unwrap();

        Self {
            event_pump,
            state: [false; 16],
            last_input: 0,
            input_dirty: false,
            close_requested: false
        }
    }

    /// Handles inputs
    fn handle_inputs(&mut self) {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in events {
            match event {
                Event::Quit {..}                                    => self.close_requested = true,
                Event::KeyDown { keycode: Some(Keycode::Num0), .. } => self.set_input(0x0, true ),
                Event::KeyUp   { keycode: Some(Keycode::Num0), .. } => self.set_input(0x0, false),
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => self.set_input(0x1, true ),
                Event::KeyUp   { keycode: Some(Keycode::Num1), .. } => self.set_input(0x1, false),
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => self.set_input(0x2, true ),
                Event::KeyUp   { keycode: Some(Keycode::Num2), .. } => self.set_input(0x2, false),
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => self.set_input(0x3, true ),
                Event::KeyUp   { keycode: Some(Keycode::Num3), .. } => self.set_input(0x3, false),
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => self.set_input(0x4, true ),
                Event::KeyUp   { keycode: Some(Keycode::Num4), .. } => self.set_input(0x4, false),
                Event::KeyDown { keycode: Some(Keycode::Num5), .. } => self.set_input(0x5, true ),
                Event::KeyUp   { keycode: Some(Keycode::Num5), .. } => self.set_input(0x5, false),
                Event::KeyDown { keycode: Some(Keycode::Num6), .. } => self.set_input(0x6, true ),
                Event::KeyUp   { keycode: Some(Keycode::Num6), .. } => self.set_input(0x6, false),
                Event::KeyDown { keycode: Some(Keycode::Num7), .. } => self.set_input(0x7, true ),
                Event::KeyUp   { keycode: Some(Keycode::Num7), .. } => self.set_input(0x7, false),
                Event::KeyDown { keycode: Some(Keycode::Num8), .. } => self.set_input(0x8, true ),
                Event::KeyUp   { keycode: Some(Keycode::Num8), .. } => self.set_input(0x8, false),
                Event::KeyDown { keycode: Some(Keycode::Num9), .. } => self.set_input(0x9, true ),
                Event::KeyUp   { keycode: Some(Keycode::Num9), .. } => self.set_input(0x9, false),
                Event::KeyDown { keycode: Some(Keycode::A),    .. } => self.set_input(0xa, true ),
                Event::KeyUp   { keycode: Some(Keycode::A),    .. } => self.set_input(0xa, false),
                Event::KeyDown { keycode: Some(Keycode::B),    .. } => self.set_input(0xb, true ),
                Event::KeyUp   { keycode: Some(Keycode::B),    .. } => self.set_input(0xb, false),
                Event::KeyDown { keycode: Some(Keycode::C),    .. } => self.set_input(0xc, true ),
                Event::KeyUp   { keycode: Some(Keycode::C),    .. } => self.set_input(0xc, false),
                Event::KeyDown { keycode: Some(Keycode::D),    .. } => self.set_input(0xd, true ),
                Event::KeyUp   { keycode: Some(Keycode::D),    .. } => self.set_input(0xd, false),
                Event::KeyDown { keycode: Some(Keycode::E),    .. } => self.set_input(0xe, true ),
                Event::KeyUp   { keycode: Some(Keycode::E),    .. } => self.set_input(0xe, false),
                Event::KeyDown { keycode: Some(Keycode::F),    .. } => self.set_input(0xf, true ),
                Event::KeyUp   { keycode: Some(Keycode::F),    .. } => self.set_input(0xf, false),
                _                                                   => {}
            }
        }
    }

    /// Wait until an input event comes through and return the key for that input event.
    fn wait_input(&mut self) -> u8 {
        self.input_dirty = false;
        loop {
            self.handle_inputs();
            // return last key pressed if key was pressed
            if self.input_dirty {
                break;
            }
            sleep(Duration::from_millis(INPUT_WAIT_DELAY));
        }

        self.last_input
    }

    /// Checks if a key is pressed.
    #[inline(always)]
    fn is_key_down(&self, key: u8) -> bool {
        self.state[key as usize]
    }

    /// Checks if a close was requested.
    #[inline(always)]
    fn close_requested(&self) -> bool {
        self.close_requested
    }
}


/// Keyboard used for testing.
#[allow(dead_code)]
pub struct TestKeyboard {
    state: [bool; 16],
    close_requested: bool,
}

#[allow(dead_code)]
impl TestKeyboard {
    /// Build new testing keyboard.
    pub fn new_test() -> Self {
        Self { state: [false; 16], close_requested: false }
    }

    /// Simulate key press event.
    pub fn press_key(&mut self, key: u8) {
        self.state[key as usize] = true;
    }

    pub fn close(&mut self) {
        self.close_requested = true;
    }
}

impl Input for TestKeyboard {
    /// Constructor.
    fn new(_context: &sdl2::Sdl) -> Self {
        panic!("No SDL context should be initialised for testing");
    }

    /// Input handling.
    fn handle_inputs(&mut self) {
        // input handling is not tested hence this function does nothing
    }

    /// Wait for an input event.
    fn wait_input(&mut self) -> u8 {
        // input handling is not tested hence this fucntion does nothing
        0
    }

    /// Checks if a key is pressed.
    #[inline(always)]
    fn is_key_down(&self, key: u8) -> bool {
        self.state[key as usize]
    }

    /// Checks if a close was requested.
    #[inline(always)]
    fn close_requested(&self) -> bool {
        self.close_requested
    }
}
