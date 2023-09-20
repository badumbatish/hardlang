use crate::reader::reader::{Reader, ReaderGeneral};

// In case of multiple readers implementation, please use the next line to set the reader
// type Reader = Reader

pub struct Lexer {
    reader : Reader,
}

impl Lexer {
    // Create a new Lexer instance
    pub fn new(file_path : &str) -> Self {
        Self {
            reader : Reader::new(file_path)
        }
    }
    pub fn next_token() {

    }  // Match the read character and assign appropriate type

}


