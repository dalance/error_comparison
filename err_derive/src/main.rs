use err_derive::Error;
use std::{error::Error, fs::File, io::Read, path::PathBuf};

#[derive(Debug, Error)]
enum MyError {
    #[error(display = "Failed to open file ({:?})", path)]
    OpenFile {
        #[error(cause)]
        source: std::io::Error,
        path: PathBuf,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        let mut cause = e.source();
        while let Some(e) = cause {
            eprintln!("  Caused by: {}", e);
            cause = e.source();
        }
    }
}

fn run() -> Result<(), MyError> {
    let path = "test";
    let mut f = File::open(&path).map_err(|x| MyError::OpenFile {
        source: x,
        path: path.into(),
    })?;
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
    dbg!(s);
    Ok(())
}
