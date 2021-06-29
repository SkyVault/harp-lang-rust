mod evaluator;
mod reader;

fn main() {
    let test_code = "
        (+ 1 2 3)
    ";

    let mut reader = reader::reader::Reader::new(test_code);
    let ast = reader.next_progn();
}
