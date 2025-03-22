use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::{Error, OpenOptions, Path, Result};

pub struct FileBuffer {
    path: Path,
    file: File,
    index: usize,
    length: usize,
    data: Vec<u8>,
}

impl FileBuffer {
    pub fn open_read(path: impl Into<Path>) -> Result<FileBuffer> {
        let path = path.into();
        if path.try_canonicalize().exists() && !path.try_canonicalize().is_file() {
            return Err(Error::FileSystemError(format!("{} is not a file", path)));
        }
        let file = path.open(OpenOptions::new().read(true))?;
        Ok(FileBuffer {
            path,
            file,
            index: 0,
            length: 0,
            data: Vec::new(),
        })
    }

    pub fn open_write(path: impl Into<Path>) -> Result<FileBuffer> {
        let path = path.into();
        if path.try_canonicalize().exists() && !path.try_canonicalize().is_file() {
            return Err(Error::FileSystemError(format!("{} is not a file", path)));
        }
        let mut file = path.open(OpenOptions::new().read(true).write(true))?;
        let size = path.size()?.as_u64();
        let length = TryInto::<usize>::try_into(size).unwrap();
        let index = length;
        let mut data = Vec::with_capacity(length);
        data.resize(length, 0);
        file.read(&mut data)?;
        file.seek(std::io::SeekFrom::Start(size))?;

        Ok(FileBuffer {
            path,
            file,
            index,
            length,
            data,
        })
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn path(&self) -> Path {
        self.path.clone()
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl Read for FileBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let count = self.file.read(buf)?;
        buf.iter().for_each(|byte| self.data.push(*byte));
        self.index += count;
        self.length += count;
        self.data.resize(self.length, 0);
        Ok(count)
    }
}

impl Seek for FileBuffer {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        use std::io::SeekFrom::*;
        match pos {
            Start(index) => {
                self.index = match TryInto::<usize>::try_into(index) {
                    Ok(index) => index,
                    Err(e) =>
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::NotSeekable,
                            format!("{} > {}: {}", index, usize::MAX, e),
                        )),
                };
            },
            End(index) => {
                let length: i64 = match TryInto::<i64>::try_into(self.length) {
                    Ok(length) => length,
                    Err(e) =>
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::NotSeekable,
                            format!("{} to i64: {}", index, e),
                        )),
                };
                if index > length {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotSeekable,
                        format!("{} > {}", index, self.length),
                    ));
                }
                let index: i64 = if index < 0 { index * -1 } else { index };

                let delta = length - index;
                if delta < 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotSeekable,
                        format!("cannot seek to {} Δ {} < 0", index, delta),
                    ));
                }
                self.index = TryInto::<usize>::try_into(delta).unwrap();
            },
            Current(plus) => {
                let length: i64 = match TryInto::<i64>::try_into(self.length) {
                    Ok(length) => length,
                    Err(e) =>
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::NotSeekable,
                            format!("{} to i64: {}", self.length, e),
                        )),
                };
                let index: i64 = match TryInto::<i64>::try_into(self.index) {
                    Ok(index) => index,
                    Err(e) =>
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::NotSeekable,
                            format!("{} to i64: {}", self.index, e),
                        )),
                };
                let delta = index + plus;
                if delta < 0 || delta > length {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotSeekable,
                        format!("cannot seek to {} out-of-bounds", plus),
                    ));
                }
                self.index = TryInto::<usize>::try_into(delta).unwrap();
            },
        }
        self.file.seek(pos)?;
        Ok(TryInto::<u64>::try_into(self.index()).unwrap())
    }
}

impl Write for FileBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let count = self.file.write(buf)?;
        buf.iter().for_each(|byte| self.data.push(*byte));
        self.index += count;
        self.length += count;
        Ok(count)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()?;
        Ok(())
    }
}
