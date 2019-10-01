use failure::{Error, ResultExt};
use std::{fs::File, io::Read};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        for e in e.as_fail().iter_causes() {
            eprintln!("  Caused by: {}", e);
        }
    }
}

fn run() -> Result<(), Error> {
    let path = "test";
    let mut f = File::open(&path).context(format!("Failed to open file ({:?})", path))?;
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
    dbg!(s);
    Ok(())
}
