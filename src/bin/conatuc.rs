use std::io;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    buf.trim_end();

    match &*buf.trim_end() {
        "私 は 歩く。" => {
            println!("I walk.");
        }
        "I walk." => {
            println!("私 は 歩く。");
        }
        _ => {
            panic!("{}: {}: error: unexpected input.", line!(), column!());
        }
    }
}
