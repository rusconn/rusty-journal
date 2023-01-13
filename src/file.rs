use std::fs::File;
use std::io::{Result, Seek, SeekFrom};

pub fn with_rewind<F, T>(f: F, mut file: &File) -> Result<T>
where
    F: FnOnce(&File) -> Result<T>,
{
    file.seek(SeekFrom::Start(0))?;

    let output = f(file)?;

    file.seek(SeekFrom::Start(0))?;

    Ok(output)
}
