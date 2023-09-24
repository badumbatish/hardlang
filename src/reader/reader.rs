use std::fs;

pub trait ReaderGeneral {
    fn new(file_path: &str) -> Self;
    fn new_str(str: &str) -> Self;
    fn change_file(&mut self, file_path: &str);
    fn get_row_counter(&self) -> usize;
    fn get_column_counter(&self) -> usize;
    fn peek(&self) -> Result<char, &str>;
    fn get_true_index(&self) -> usize;
    fn consume(&mut self) -> Result<char, &str>;
    fn is_eof(&self) -> bool;
}

pub struct Reader {
    file_content: String,
    true_index: usize,
    row_counter: usize,
    column_counter: usize,
}

impl Reader {
    pub fn read_file(file_path: &str) -> String {
        println!("In file {}", file_path);

        let error_msg = format!("Should have been able to read the {}", file_path);
        let contents = fs::read_to_string(file_path).expect(error_msg.as_str());

        contents
    }

    pub fn get_file_content<'a>(&self) -> &str {
        &(self.file_content)
    }
}

impl ReaderGeneral for Reader {
    fn new(file_path: &str) -> Self {
        Self {
            file_content: Self::read_file(file_path),
            row_counter: 0,
            column_counter: 0,
            true_index: 0,
        }
    }

    fn new_str(str: &str) -> Self {
        Self {
            file_content: str.to_string(),
            true_index: 0,
            row_counter: 0,
            column_counter: 0,
        }
    }
    fn change_file(&mut self, file_path: &str) {
        self.true_index = 0;
        self.row_counter = 0;
        self.column_counter = 0;
        self.file_content = Reader::read_file(file_path)
    }
    fn get_row_counter(&self) -> usize {
        self.row_counter * 1
    }
    fn get_column_counter(&self) -> usize {
        self.column_counter * 1
    }
    fn peek(&self) -> Result<char, &str> {
        let true_index = self.get_true_index();
        Ok(char::from(self.file_content.as_bytes()[true_index]))
    }
    fn get_true_index(&self) -> usize {
        self.true_index
    }
    fn consume(&mut self) -> Result<char, &str> {
        let peek_result = self.peek().expect("Failed in consume");
        if peek_result == '\n' {
            self.row_counter += 1;
            self.column_counter = 0;
            self.true_index += 1;
        } else {
            self.column_counter += 1;
            self.true_index += 1;
        }

        Ok(peek_result)
    }
    fn is_eof(&self) -> bool {
        self.true_index >= self.file_content.len()
    }
}

#[cfg(test)]
mod test_reader {
    use crate::reader::reader::{Reader, ReaderGeneral};

    const TEST_MATERIAL_DIRECTORY: &str = "src/reader/test_material";
    #[test]
    fn test_read_file() {
        let mut file_path = format!("{}/read_file/test_read_file.txt", TEST_MATERIAL_DIRECTORY);
        let mut result: String = String::from("I'm nobody! Who are you?");

        let test_1_msg: &str = "Reading single line from file";
        assert_eq!(Reader::read_file(&file_path), result, "{}", test_1_msg);

        file_path = format!(
            "{}/read_file/test_newline_twoline.txt",
            TEST_MATERIAL_DIRECTORY
        );
        result = String::from("I'm nobody! Who are you?\n");
        let test_2_msg: &str = "Reading two line from file, expected a newline char";
        assert_eq!(Reader::read_file(&file_path), result, "{}", test_2_msg);
    }

    #[test]
    fn test_constructor() {
        let file_path = format!("{}/read_file/test_read_file.txt", TEST_MATERIAL_DIRECTORY);
        let result: String = String::from("I'm nobody! Who are you?");

        let rdr: Reader = Reader::new(&file_path);

        assert_eq!(rdr.get_file_content(), result);
        assert_eq!(rdr.get_row_counter(), 0);
        assert_eq!(rdr.get_column_counter(), 0);
    }

    #[test]
    fn test_peek_and_consume() {
        let mut file_path = format!(
            "{}/peeking_and_consuming/basic_peek.txt",
            TEST_MATERIAL_DIRECTORY
        );
        let _result: String = String::from("I'm nobody! Who are you?");

        let mut rdr: Reader = Reader::new(&file_path);
        assert_eq!(rdr.peek().unwrap(), '1');

        file_path = format!(
            "{}/peeking_and_consuming/peek_line2.txt",
            TEST_MATERIAL_DIRECTORY
        );
        rdr.change_file(&file_path);
        assert_eq!(rdr.get_file_content().len(), 8);

        rdr.consume().expect("Consumed character '1'");
        assert_eq!(rdr.column_counter, 1);

        rdr.consume().expect("Consumed 1 character '2'");
        assert_eq!(rdr.column_counter, 2);

        rdr.consume().expect("Consumed 1 character '3'");
        assert_eq!(rdr.column_counter, 3);

        rdr.consume().expect("Consumed 1 character /\n");
        assert_eq!(rdr.column_counter, 0);
        assert_eq!(rdr.row_counter, 1);
        assert_eq!(rdr.true_index, 4);

        rdr.consume().expect("Consumed 1 character 'a'");
        assert_eq!(rdr.true_index, 5);
        assert_eq!(rdr.peek().expect("Peek 'b'"), 'b');
    }
}
