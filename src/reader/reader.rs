use std::env;
use std::fs;

pub struct Reader {}

impl Reader {
    pub fn read_file(file_path : &str) -> String {
        println!("In file {}", file_path);

        let error_msg = format!("Should have been able to read the {}", file_path);
        let contents = fs::read_to_string(file_path)
            .expect(error_msg.as_str());

        contents
    }
}


#[cfg(test)]
mod test_reader {
    use crate::reader::reader::Reader;

    const TEST_MATERIAL_DIRECTORY: &str = "src/reader/test_material/read_file";
    #[test]
    fn test_read_file() {
        let mut file_path = format!("{}/test_read_file.txt", TEST_MATERIAL_DIRECTORY);
        let mut result: String = String::from("I'm nobody! Who are you?");

        let test_1_msg: &str = "Reading single line from file";
        assert_eq!(Reader::read_file(&file_path), result, "{}", test_1_msg);

        file_path = format!("{}/test_newline_twoline.txt", TEST_MATERIAL_DIRECTORY);
        result = String::from("I'm nobody! Who are you?\n");
        let test_2_msg: &str = "Reading two line from file, expected a newline char";
        assert_eq!(Reader::read_file(&file_path), result, "{}", test_2_msg);
    }
}