//! Graphics module.

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const DISPLAY_SIZE: usize = WIDTH * HEIGHT;

pub trait Graphics {
    /// Constructor.
    fn new(context: &sdl2::Sdl) -> Self;

    /// Clears the display.
    fn cls(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set_pixel(x, y, false);
            }
        }
    }

    /// "Turns on" a pixel on the screen
    fn set_pixel(&mut self, x: usize, y: usize, on: bool);

    /// Checks if a pixel is "turned on"
    fn get_pixel(&self, x: usize, y: usize) -> bool;

    /// Draw a sprite at the given location.
    ///
    /// # Returns
    /// Returns `true` if the sprite collides with an existing sprite on the display.
    fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool;

}

pub struct Display {
    canvas: sdl2::render::WindowCanvas,
    memory: [u8; DISPLAY_SIZE],
}

impl Display {
    /// Draw the display state to the `WindowCanvas`.
    fn draw_display(&mut self) {
        // Clear canvas in black
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Draw the state to the display
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.get_pixel(x, y) {
                    let _ = self.canvas.draw_point(Point::new(x as i32, y as i32));
                }
            }
        }
        self.canvas.present();
    }
}

impl Graphics for Display {
    /// Constructor
    fn new(context: &sdl2::Sdl) -> Display {
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem.window("CHIP-8", (WIDTH * 10) as u32, (HEIGHT * 10) as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().software().build().unwrap();
        let _ = canvas.set_scale(10.0, 10.0);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display {
            canvas,
            memory: [0; DISPLAY_SIZE]
        }
    }

    /// "Turns on" a pixel on the screen
    #[inline(always)]
    fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.memory[x + y * WIDTH] = on as u8;
    }

    /// Checks if a pixel is "turned on"
    #[inline(always)]
    fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.memory[x + y * WIDTH] == 1
    }

    /// Draw a sprite at the given location.
    ///
    /// # Returns
    /// Returns `true` if the sprite collides with an existing sprite on the display.
    fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let rows = sprite.len();
        let mut collision = false;
        for j in 0..rows {
          let row = sprite[j];
          for i in 0..8 {
            let curr = row >> (7 - i) & 0x01;
            if curr == 1 {
              let xi = (x + i) % WIDTH;
              let yj = (y + j) % HEIGHT;
              let prev = self.get_pixel(xi, yj);
              if prev {
                collision = true;
              }
              self.set_pixel(xi, yj, (curr == 1) ^ prev);
            }
          }
        }
        self.draw_display();
        collision
    }
}

/// Display used for testing.
#[allow(dead_code)]
pub struct TestDisplay {
    memory: [u8; DISPLAY_SIZE],
}

#[allow(dead_code)]
impl TestDisplay {
    pub fn new_test() -> Self {
        Self { memory: [0; DISPLAY_SIZE] }
    }
}

impl Graphics for TestDisplay {
    /// Constructor.
    fn new(_context: &sdl2::Sdl) -> Self {
        panic!("No SDL context should be initialised for testing");
    }

    /// "Turns on" a pixel on the screen
    #[inline(always)]
    fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.memory[x + y * WIDTH] = on as u8;
    }

    /// Checks if a pixel is "turned on"
    #[inline(always)]
    fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.memory[x + y * WIDTH] == 1
    }

    /// Draw a sprite at the given location.
    ///
    /// # Returns
    /// Returns `true` if the sprite collides with an existing sprite on the display.
    fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let rows = sprite.len();
        let mut collision = false;
        for j in 0..rows {
          let row = sprite[j];
          for i in 0..8 {
            let curr = row >> (7 - i) & 0x01;
            if curr == 1 {
              let xi = (x + i) % WIDTH;
              let yj = (y + j) % HEIGHT;
              let prev = self.get_pixel(xi, yj);
              if prev {
                collision = true;
              }
              self.set_pixel(xi, yj, (curr == 1) ^ prev);
            }
          }
        }
        collision
    }
}


#[cfg(test)]
mod tests {
  use super::*;

    fn get_display() -> TestDisplay {
        TestDisplay::new_test()
    }

    #[test]
    fn set_pixel() {
        let mut display = get_display();
        display.set_pixel(1, 1, true);
        assert_eq!(true, display.get_pixel(1, 1));
    }

    #[test]
    fn cls() {
        let mut display = get_display();
        display.set_pixel(1, 1, true);
        display.cls();
        assert_eq!(false, display.get_pixel(1, 1));
    }

    #[test]
    fn draw() {
        let mut display = get_display();
        let sprite: [u8; 2] = [0b00110011, 0b11001010];

        display.draw(0, 0, &sprite);

        assert_eq!(false, display.get_pixel(0, 0));
        assert_eq!(false, display.get_pixel(1, 0));
        assert_eq!(true, display.get_pixel(2, 0));
        assert_eq!(true, display.get_pixel(3, 0));
        assert_eq!(false, display.get_pixel(4, 0));
        assert_eq!(false, display.get_pixel(5, 0));
        assert_eq!(true, display.get_pixel(6, 0));
        assert_eq!(true, display.get_pixel(7, 0));

        assert_eq!(true, display.get_pixel(0, 1));
        assert_eq!(true, display.get_pixel(1, 1));
        assert_eq!(false, display.get_pixel(2, 1));
        assert_eq!(false, display.get_pixel(3, 1));
        assert_eq!(true, display.get_pixel(4, 1));
        assert_eq!(false, display.get_pixel(5, 1));
        assert_eq!(true, display.get_pixel(6, 1));
        assert_eq!(false, display.get_pixel(7, 1));
    }

    #[test]
    fn draw_detects_collisions() {
        let mut display = get_display();

        let mut sprite: [u8; 1] = [0b00110000];
        let mut collision = display.draw(0, 0, &sprite);
        assert_eq!(false, collision);

        sprite = [0b00000011];
        collision = display.draw(0, 0, &sprite);
        assert_eq!(false, collision);

        sprite = [0b00000001];
        collision = display.draw(0, 0, &sprite);
        assert_eq!(true, collision);
    }
}
