pub const WIDTH: u16 = 64;
pub const HEIGHT: u16 = 32;
pub const AREA: usize = WIDTH as usize * HEIGHT as usize;

const WIDTH_MASK: usize = WIDTH as usize - 1;
const HEIGHT_MASK: usize = HEIGHT as usize - 1;

#[derive(Clone, Copy)]
pub enum Pixel {
  Off,
  On,
}

impl Pixel {
  pub fn invert(self) -> Pixel {
    match self {
      Pixel::Off => Pixel::On,
      Pixel::On => Pixel::Off,
    }
  }
}

pub struct Display {
  buffer: [[Pixel; WIDTH as usize]; HEIGHT as usize],
}

impl Display {
  pub fn new() -> Display {
    Display {
      buffer: [[Pixel::Off; WIDTH as usize]; HEIGHT as usize],
    }
  }

  pub fn pixel(&self, x: usize, y: usize) -> Pixel {
    self.buffer[y & HEIGHT_MASK][x & WIDTH_MASK]
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
    self.buffer[y & HEIGHT_MASK][x & WIDTH_MASK] = pixel
  }

  /// Handler for the `cls` instruction.
  pub fn cls(&mut self) {
    for y in 0..HEIGHT as usize {
      for x in 0..WIDTH as usize {
        self.set_pixel(x, y, Pixel::Off);
      }
    }
  }

  /// Handler for the `drw` instruction.
  pub fn drw(&mut self, x: usize, y: usize, sprite_data: &[(usize, usize)]) -> bool {
    let mut collision = false;

    for (row, tile) in sprite_data {
      for col in 0..7 {
        let pixel = (tile >> (7 - col)) & 1;
        if pixel == 1 {
          let previous = self.pixel(col + x, row + y);
          if let Pixel::On = previous {
            collision = true;
          }

          self.set_pixel(col + x, row + y, previous.invert())
        }
      }
    }

    collision
  }
}
