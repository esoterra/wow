#[allow(warnings)]
mod bindings;

use std::{env::args, fs::File, io::{Read, Write}, path::PathBuf};


fn main() -> anyhow::Result<()> {
    // Skip the first argument (binary name)
    // read the rest of the arguments as paths
    let paths: Vec<PathBuf> = args().skip(1).map(|p| PathBuf::from(p)).collect();

    let mut buf = [0u8; 256];
    let mut stdout = std::io::stdout();

    for path in paths {
        let mut file = File::open(path)?;
        loop {
            let n = file.read(&mut buf)?;
            if n == 0 {
                break;
            }
            stdout.write_all(&buf[..n])?;
        }
    }

    stdout.flush()?;

    Ok(())
}
