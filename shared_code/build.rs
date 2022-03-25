// build.rs
use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("Heheje");
    return;
    let output = Command::new("../deps/flatbuffers/tools/flatbuffers/flatc.exe").args(["--rust", "--gen-object-api", "-o", "./src", "./schema.fbs"]).output().expect("why hello");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}