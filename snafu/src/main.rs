use snafu::{ResultExt, Snafu};
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to open file ({:?})\n  Caused by : {}", path, source))]
    OpenFile {
        source: std::io::Error,
        path: PathBuf,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<(), Error> {
    let path = "test";
    let mut f = File::open(&path).context(OpenFile { path })?;
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
    dbg!(s);
    Ok(())
}
