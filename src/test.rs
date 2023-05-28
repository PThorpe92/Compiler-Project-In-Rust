use std::io::read_to_string;

fn main() {
    let file = read_to_string("/home/dvektor/Projects/compiler/src/test.p");
    match file {
        Ok(file) => tokenizer(String::from(file)),
        Err(err) => println!("{}", err),
    }
}
