use std::io::Write;
use std::process::{Command, Stdio};

// "f"orce unwrap
macro_rules! f {
    ($expr:expr) => {{
        match $expr {
            Ok(v) => v,
            Err(e) => panic!("{}: {}: {}", line!(), column!(), e),
        }
    }};
}

fn debug_bin() -> Command {
    let mut c = Command::new("./target/debug/natuconv");
    c.stdin(Stdio::piped()).stdout(Stdio::piped());
    c
}

fn assert_eq(expect: &str, actual: &str) -> bool {
    let mut child = f!(debug_bin().spawn());
    {
        let i = child.stdin.as_mut();
        let i = f!(i.ok_or("get stdin failed."));
        f!(i.write_all(actual.as_bytes()));
    }
    let o = f!(child.wait_with_output());
    let actual = String::from_utf8_lossy(&o.stdout);
    if expect == actual {
        true
    } else {
        println!("test failed. expect: {:?}, actual: {:?}", expect, actual);
        false
    }
}

fn main() {
    let mut ok = true;

    ok = assert_eq("I walk.\n", "私 は 歩く。\n") && ok;
    ok = assert_eq("私 は 歩く。\n", "I walk.\n") && ok;

    if ok {
        println!("OK");
        return;
    }

    panic!("{}: {}: test failed.", line!(), column!());
}
