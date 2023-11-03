use crate::reader::reader::{Reader, ReaderGeneral};
use crate::scanner::token::TOKEN;

// In case of multiple readers implementation, please use the next line to set the reader
// type Reader = Reader

pub struct Lexer {
    reader: Reader,
}

impl Lexer {
    // Create a new Lexer instance
    pub fn new(file_path: &str) -> Self {
        Self {
            reader: Reader::new(file_path),
        }
    }
    pub fn next_token(&mut self) -> TOKEN {
        let mut token = TOKEN::EOF;
        while !self.reader.is_eof() {
            let first_char = self.reader.consume().unwrap();
            match first_char {
                // Starts with '+'
                '+' => {
                    token = self.match_add();
                    break;
                }
                '-' => {
                    token = self.match_sub();
                    break;
                }

                '*' => {
                    token = self.match_mul();
                    break;
                }

                '/' => {
                    token = self.match_div();
                    break;
                }

                // If alphabetic, it is an identifier
                'A'..='Z' | 'a'..='z' => {
                    token = TOKEN::IDENTIFIER;
                    break;
                }

                ' ' => {
                    continue;
                }
                _ => {
                    token = TOKEN::INVALID;
                    break;
                }
            };
        }
        return token;
    } // Match the read character and assign appropriate type

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

    fn match_mul(&mut self) -> TOKEN {
        if !self.reader.is_eof() {
            let second_char = self.reader.consume().unwrap();
            match second_char {
                ' ' => TOKEN::MUL,
                '=' => TOKEN::MulAssign,
                '*' => TOKEN::EXP,
                _ => TOKEN::INVALID,
            }
        } else {
            TOKEN::MUL
        }
    }

    fn match_div(&mut self) -> TOKEN {
        if !self.reader.is_eof() {
            let second_char = self.reader.consume().unwrap();
            match second_char {
                ' ' => TOKEN::DIV,
                '=' => TOKEN::DivAssign,
                '/' => TOKEN::FloorDiv,
                '^' => TOKEN::CeilDiv,
                _ => TOKEN::INVALID,
            }
        } else {
            TOKEN::DIV
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
        let mut lxr = Lexer {
            reader: Reader::new_str("+"),
        };

        assert_eq!(lxr.next_token(), TOKEN::ADD);
        assert_eq!(lxr.next_token(), TOKEN::EOF);

        lxr.reader = Reader::new_str("+=");
        assert_eq!(lxr.next_token(), TOKEN::AddAssign);
    }

    #[test]
    fn test_sub_family() {
        let mut lxr = Lexer {
            reader: Reader::new_str("-"),
        };

        assert_eq!(lxr.next_token(), TOKEN::SUB);
        assert_eq!(lxr.next_token(), TOKEN::EOF);

        lxr.reader = Reader::new_str("-= ");
        assert_eq!(lxr.next_token(), TOKEN::SubAssign);
    }
    #[test]
    fn test_mul_family() {
        let mut lxr = Lexer {
            reader: Reader::new_str("*"),
        };

        assert_eq!(lxr.next_token(), TOKEN::MUL);
        assert_eq!(lxr.next_token(), TOKEN::EOF);

        lxr.reader = Reader::new_str("*=");
        assert_eq!(lxr.next_token(), TOKEN::MulAssign);

        lxr.reader = Reader::new_str("**");
        assert_eq!(lxr.next_token(), TOKEN::EXP);
    }
    #[test]
    fn test_div_family() {
        let mut lxr = Lexer {
            reader: Reader::new_str("/"),
        };

        assert_eq!(lxr.next_token(), TOKEN::DIV);
        assert_eq!(lxr.next_token(), TOKEN::EOF);

        lxr.reader = Reader::new_str("/= // /^");
        assert_eq!(lxr.next_token(), TOKEN::DivAssign);
        assert_eq!(lxr.next_token(), TOKEN::FloorDiv);
        assert_eq!(lxr.next_token(), TOKEN::CeilDiv);
    }

    #[test]
    fn test_whitespace() {
        let mut lxr = Lexer {
            reader: Reader::new_str(" -"),
        };

        assert_eq!(lxr.next_token(), TOKEN::SUB);
    }
}
