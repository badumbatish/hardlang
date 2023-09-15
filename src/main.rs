mod scanner;
mod parser;

fn main() {
    scanner::scanner::say_hello();

    let v = scanner::scanner::TOKEN::ADD;
}
