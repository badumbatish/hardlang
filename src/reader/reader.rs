use std::fs;

pub struct Reader {
    file_content : String,
    row_counter  : usize,
    column_counter : usize,
}

impl Reader {
    pub  fn new(file_path : &str) -> Self {
        Self {
            file_content : Self::read_file(file_path),
            row_counter : 0,
            column_counter : 0,
        }
    }

    pub fn change_file(&mut self, file_path :&str) {
        self.row_counter = 0;
        self.column_counter = 0;
        self.file_content = Reader::read_file(file_path)
    }

    pub fn read_file(file_path : &str) -> String {
        println!("In file {}", file_path);

        let error_msg = format!("Should have been able to read the {}", file_path);
        let contents = fs::read_to_string(file_path)
            .expect(error_msg.as_str());

        contents
    }


    pub fn get_file_content<'a>(&self) -> &str {
        &(self.file_content)
    }

    pub fn get_row_counter(&self) -> usize {
        self.row_counter * 1
    }
    pub fn get_column_counter(&self) -> usize {
        self.column_counter * 1
    }

    pub fn peek(&self) -> Result<char, &str> {
        // TODO : Fix this way of calculating the true index into the string in get_true_index
        let true_index = self.get_true_index();
        Ok(char::from(self.file_content.as_bytes()[true_index]))
    }

    fn get_true_index(&self) -> usize {
        self.get_row_counter()
    }

    pub fn consume(&mut self) -> Result<char, &str> {
        let peek_result = self.peek().expect("Failed in consume");
        if peek_result == '\n' {
            self.row_counter += 1;
            self.column_counter = 0;
        } else {
            self.column_counter += 1;
        }

        Ok(peek_result)
    }
}


#[cfg(test)]
mod test_reader {
    use crate::reader::reader::Reader;

    const TEST_MATERIAL_DIRECTORY: &str = "src/reader/test_material";
    #[test]
    fn test_read_file() {
        let mut file_path = format!("{}/read_file/test_read_file.txt", TEST_MATERIAL_DIRECTORY);
        let mut result: String = String::from("I'm nobody! Who are you?");

        let test_1_msg: &str = "Reading single line from file";
        assert_eq!(Reader::read_file(&file_path), result, "{}", test_1_msg);

        file_path = format!("{}/read_file/test_newline_twoline.txt", TEST_MATERIAL_DIRECTORY);
        result = String::from("I'm nobody! Who are you?\n");
        let test_2_msg: &str = "Reading two line from file, expected a newline char";
        assert_eq!(Reader::read_file(&file_path), result, "{}", test_2_msg);
    }

    #[test]
    fn test_constructor() {
        let mut file_path = format!("{}/read_file/test_read_file.txt", TEST_MATERIAL_DIRECTORY);
        let mut result: String = String::from("I'm nobody! Who are you?");


        let  rdr:Reader = Reader::new(&file_path);

        assert_eq!(rdr.get_file_content(), result);
        assert_eq!(rdr.get_row_counter(), 0);
        assert_eq!(rdr.get_column_counter(), 0);
    }

    #[test]
    fn test_peek_and_consume() {
        let mut file_path = format!("{}/peeking_and_consuming/basic_peek.txt", TEST_MATERIAL_DIRECTORY);
        let mut result: String = String::from("I'm nobody! Who are you?");


        let mut rdr : Reader = Reader::new(&file_path);
        assert_eq!(rdr.peek().unwrap(), '1');


        file_path = format!("{}/peeking_and_consuming/peek_line2.txt", TEST_MATERIAL_DIRECTORY);
        rdr.change_file(&file_path);
        assert_eq!(rdr.get_file_content().len(), 8);

        rdr.consume().expect("Consumed 1 character");
        assert_eq!(rdr.column_counter, 1);

    }

}