use std::fmt;
use std::error;
use std::path::Path;

use crate::{picross_image::{Image, ImageError}, board::Board};

pub struct Game {
    pub image : Image,
    pub board: Board
}

#[derive(Debug)]
pub enum GameError {
    ImageError(ImageError)
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            GameError::ImageError(e) => write!(f, "{}", e.to_string())
        }
    }
}

impl error::Error for GameError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &*self {
            GameError::ImageError(ref e) => Some(e)
        }
    }
}
impl From<ImageError> for GameError {
    fn from(err: ImageError) -> GameError {
        GameError::ImageError(err)
    }
}

type Result<T> = std::result::Result<T, GameError>;

impl Game {
    pub fn new<P>(filename: P) -> Result<Game>
    where P: AsRef<Path> {
        let image = Image::from_image(filename)?;
        let width = image.width as usize;
        let height = image.height as usize;
        Ok(Game {
            image,
            board: Board::new(width, height)
        })
    }

    pub fn is_finished(&self) -> bool {
        self.board.eq(&self.image)
    }
}