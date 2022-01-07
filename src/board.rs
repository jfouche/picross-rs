use std::fmt::{Display, self};

use image::Rgb;

use crate::picross_image::WHITE;


#[derive(Clone, Eq, PartialEq, Copy)]
pub enum Pixel {
    Color(Rgb<u8>),
    Cross
}

pub struct Board {
    img: Vec<Pixel>,
    width:usize,
    height:usize
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Board {
            img: vec![Pixel::Color(WHITE); width*height],
            width,
            height
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &Pixel {
        self.img.get(x + y * self.width).unwrap()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pix: &Pixel) {
        match self.img.get_mut(x + y * self.width) {
            Some(p) => *p = *pix,
            None => {}
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=======================")?;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let c = match self.get_pixel(x, y) {
                    Pixel::Color(color) => {
                        match color {
                            &WHITE => " ",
                            _ => "█"

                        }
                    },
                    Pixel::Cross => {
                        "X"
                    }
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "=======================")
    }
}