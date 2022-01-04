use std::error;
use std::fmt;
use std::fmt::Display;
use std::path::Path;

use image::ImageBuffer;
use image::{io::Reader as ImageReader, DynamicImage, DynamicImage::*, GenericImageView};

#[derive(Debug)]
pub struct Game {
    pub width: u32,
    pub height: u32,
    pub rows: Vec<Vec<Clue>>,
    pub cols: Vec<Vec<Clue>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Clue {
    pub color: image::Rgb<u8>,
    pub count: u32,
}

impl Display for Clue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Clue ({:?}, {})", self.color, self.count)
    }
}

impl Clue {
    fn new(color: image::Rgb<u8>, count: u32) -> Self {
        Clue { color, count }
    }
}

#[derive(Debug)]
pub enum GameError {
    IoError(std::io::Error),
    ImageError(image::ImageError),
    UnsupportedFormatError(DynamicImage),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            GameError::IoError(e) => write!(f, "{}", e.to_string()),
            // The wrapped error contains additional information and is available
            // via the source() method.
            GameError::ImageError(e) => write!(f, "{}", e.to_string()),
            GameError::UnsupportedFormatError(img) => {
                write!(f, "Unsupported image format : {:?}", img)
            }
        }
    }
}

impl error::Error for GameError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &*self {
            GameError::IoError(ref e) => Some(e),
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            GameError::ImageError(ref e) => Some(e),
            GameError::UnsupportedFormatError(_) => None,
        }
    }
}

impl From<std::io::Error> for GameError {
    fn from(err: std::io::Error) -> GameError {
        GameError::IoError(err)
    }
}

impl From<image::ImageError> for GameError {
    fn from(err: image::ImageError) -> GameError {
        GameError::ImageError(err)
    }
}

type Result<T> = std::result::Result<T, GameError>;

const WHITE: image::Rgb<u8> = image::Rgb([0xFF, 0xFF, 0xFF]);

fn is_white(p: &image::Rgb<u8>) -> bool {
    return p[0] == WHITE[0] && p[1] == WHITE[1] && p[2] == WHITE[2];
}

struct Counter {
    image: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    counter: u32,
    current_color: image::Rgb<u8>,
}

///
impl Counter {
    fn new(image: ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Self {
        Counter {
            image,
            counter: 0,
            current_color: WHITE,
        }
    }
    fn reset(&mut self) {
        self.counter = 0;
        self.current_color = WHITE;
    }
    fn next(&mut self, x: u32, y: u32) -> Option<Clue> {
        let pix = self.image.get_pixel(x, y);
        if is_white(&pix) {
            if self.counter > 0 {
                let clue = self.clue();
                // println!("Counter::next({},{}) return {}", x, y, clue);
                self.reset();
                return Some(clue);
            }
        } else {
            if pix.eq(&self.current_color) {
                self.counter += 1;
                // println!("Counter::next({},{}) increment  {:?} : {}", x, y, self.current_color, self.counter);
            } else {
                self.current_color = *pix;
                self.counter = 1;
                // println!("Counter::next({},{}) initialize {:?} : {}", x, y, self.current_color, self.counter);
            }
        }
        None
    }

    fn clue(&self) -> Clue {
        Clue::new(self.current_color, self.counter)
    }

    fn end(&mut self) -> Option<Clue> {
        if self.counter > 0 {
            // println!("Counter::end() return {:?}", self.clue());
            Some(self.clue())
        }
        else {
            None
        }
    }
}

///
impl Game {
    pub fn from_image<P>(filename: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        println!("Game::from_image");
        let img = ImageReader::open(filename)?.decode()?;
        let width = img.width();
        let height = img.height();

        let imgbuffer = match img {
            ImageRgb8(i) => i,
            _ => {
                return Err(GameError::UnsupportedFormatError(img));
            }
        };

        let mut counter = Counter::new(imgbuffer);

        // Get the rows
        let mut rows = vec![];
        for y in 0..height {
            counter.reset();
            let mut v = vec![];
            for x in 0..width {
                if let Some(clue) = counter.next(x, y) {
                    // println!("===================================== pushing {:?}", clue);
                    v.push(clue);
                    counter.reset();
                }
            }
            if let Some(clue) = counter.end() {
                // println!("===================================== pushing {:?}", clue);
                v.push(clue);
            }
            rows.push(v);
        }

        // Get the cols
        let mut cols = vec![];
        for x in 0..width {
            counter.reset();
            let mut v = vec![];
            for y in 0..height {
                if let Some(clue) = counter.next(x, y) {
                    // println!("===================================== pushing {:?}", clue);
                    v.push(clue);
                    counter.reset();
                }
            }
            if let Some(clue) = counter.end() {
                // println!("===================================== pushing {:?}", clue);
                v.push(clue);
            }
            cols.push(v);
        }

        Ok(Game {
            width,
            height,
            rows,
            cols,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLACK: image::Rgb<u8> = image::Rgb([0x0, 0x0, 0x0]);

    #[test]
    fn it_creates_game_from_image() {
        let game_res = Game::from_image("test/4x4-c.png");
        assert!(&game_res.is_ok());
        let game = game_res.unwrap();

        assert_eq!(game.width, 4);
        assert_eq!(game.height, 4);
        assert_eq!(game.rows.len(), 4);
        assert_eq!(game.cols.len(), 4);

        let expected = vec![
            vec![Clue::new(BLACK, 4)],
            vec![Clue::new(BLACK, 1)],
            vec![Clue::new(BLACK, 1)],
            vec![Clue::new(BLACK, 4)],
        ];
        assert_eq!(&game.rows, &expected);

        let expected = vec![
            vec![Clue::new(BLACK, 4)],
            vec![Clue::new(BLACK, 1), Clue::new(BLACK, 1)],
            vec![Clue::new(BLACK, 1), Clue::new(BLACK, 1)],
            vec![Clue::new(BLACK, 1), Clue::new(BLACK, 1)],
        ];
        assert_eq!(&game.cols, &expected);
    }
}
