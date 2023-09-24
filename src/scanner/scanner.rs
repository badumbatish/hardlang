use crate::reader::reader::{Reader, ReaderGeneral};
use crate::scanner::token::TOKEN;

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
    pub fn next_token(&mut self) -> TOKEN {
        let mut token = TOKEN::INVALID;
        while !self.reader.is_eof() {
            let first_char = self.reader.consume().unwrap();
            token = match first_char {
                // Starts with '+'
                 '+' => {
                     self.match_add()
                 }
                '-' => {
                    self.match_sub()
                }
                 _ => TOKEN::INVALID
            };

        }
        return token;
    }  // Match the read character and assign appropriate type

    fn match_add(&mut self) -> TOKEN {
        if !self.reader.is_eof() {
            let second_char = self.reader.consume().unwrap();
            match second_char {
                ' ' => TOKEN::ADD,
                '=' => TOKEN::AddAssign,
                '+' => TOKEN::AddIncr,
                _ => TOKEN::INVALID,
            }
        } else {
            TOKEN::ADD
        }
    }

    fn match_sub(&mut self) -> TOKEN {
        if !self.reader.is_eof() {
            let second_char = self.reader.consume().unwrap();
            match second_char {
                ' ' => TOKEN::SUB,
                '=' => TOKEN::SubAssign,
                '-' => TOKEN::SubDecr,
                _ => TOKEN::INVALID,
            }
        } else {
            TOKEN::SUB
        }
    }


}

#[cfg(test)]
mod test_next_token {
    use crate::reader::reader::{Reader, ReaderGeneral};
    use crate::scanner::scanner::Lexer;
    use crate::scanner::token::TOKEN;

    #[test]
    fn test_add_family() {
        let mut lxr = Lexer{
            reader : Reader::new_str("+")
        };


        assert_eq!(lxr.next_token(), TOKEN::ADD);
        assert_eq!(lxr.next_token(), TOKEN::INVALID);

        lxr.reader = Reader::new_str("+=");
        assert_eq!(lxr.next_token(), TOKEN::AddAssign);

    }

    #[test]
    fn test_sub_family() {
        let mut lxr = Lexer{
            reader : Reader::new_str("-")
        };


        assert_eq!(lxr.next_token(), TOKEN::SUB);
        assert_eq!(lxr.next_token(), TOKEN::INVALID);

        lxr.reader = Reader::new_str("-=");
        assert_eq!(lxr.next_token(), TOKEN::SubAssign);

    }
}




