use std::error;
use std::fmt;
use std::fmt::Display;
use std::path::Path;

use image::{Rgb, io::Reader as ImageReader, DynamicImage, RgbImage, GenericImageView};

#[derive(Debug)]
pub struct Game {
    pub width: u32,
    pub height: u32,
    pub rows: Vec<Vec<Clue>>,
    pub cols: Vec<Vec<Clue>>,
    img: RgbImage
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
    fn new(color: Rgb<u8>, count: u32) -> Self {
        Clue { color, count }
    }
}

pub struct Board {
    img: RgbImage
}

impl Board {
    pub fn get_pixel(&self, x: u32, y: u32) -> &Rgb<u8> {
        self.img.get_pixel(x, y)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pix: Rgb<u8>) {
        self.img.put_pixel(x, y, pix);
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "vvvvvvvvvv")?;
        for y in 0..self.img.height() {
            for x in 0..self.img.width() {
                let c = match self.get_pixel(x, y) {
                    &WHITE => " ",
                    _ => "X"
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "^^^^^^^^^^")
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

/// Result type for the picross game
type Result<T> = std::result::Result<T, GameError>;

pub const WHITE: Rgb<u8> = Rgb([0xFF, 0xFF, 0xFF]);

fn is_white(p: &Rgb<u8>) -> bool {
    return p[0] == WHITE[0] && p[1] == WHITE[1] && p[2] == WHITE[2];
}

struct Counter<'a> {
    image: &'a RgbImage,
    counter: u32,
    current_color: Rgb<u8>,
}

///
impl<'a> Counter<'a> {
    fn new(image: &'a RgbImage) -> Self {
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
        let imgbuffer = match img.as_rgb8() {
            Some(i) => i,
            None => return Err(GameError::UnsupportedFormatError(img))
        };

        let width = img.width();
        let height = img.height();

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
            img: imgbuffer.clone()
        })
    }

    pub fn new_board(&self) -> Board {
        let mut img = self.img.clone();
        for pix in img.pixels_mut() {
            *pix = WHITE;
        }
        Board { img }
    }

    pub fn is_finished(&self, board: &Board) -> bool {
        board.eq(self)
    }
}

impl PartialEq<Game> for Board {
    fn eq(&self, other: &Game) -> bool {
        for y in 0..self.img.width(){
            for x in 0..self.img.height() {
                if !self.get_pixel(x, y).eq(other.img.get_pixel(x, y)) {
                    return false;
                }
            }
        }
        return true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLACK: Rgb<u8> = Rgb([0x0, 0x0, 0x0]);

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
