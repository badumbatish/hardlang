mod scanner;
mod parser;
mod reader;

fn main() {
    scanner::scanner::say_hello();

    let v = scanner::scanner::TOKEN::ADD;
}
