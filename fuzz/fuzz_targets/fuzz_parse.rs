use honggfuzz::fuzz;
use yul::parser::Parser;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let input_str: &str = std::str::from_utf8(data).unwrap();
            let parser = Parser::new(input_str);
            let _ = parser.finish();
        });
    }
}