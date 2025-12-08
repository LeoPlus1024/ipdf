use crate::error::Result;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub trait Sequence {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn seek(&mut self, pos: u64) -> Result<u64>;
    fn size(&self) -> Result<u64>;
}

pub struct RandomAccessFile {
    file: File,
}

impl RandomAccessFile {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl Sequence for RandomAccessFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.file.read(buf)?;
        Ok(n)
    }

    fn seek(&mut self, pos: u64) -> Result<u64> {
        let n = self.file.seek(SeekFrom::Start(pos))?;
        Ok(n)
    }

    fn size(&self) -> Result<u64> {
        let n = self.file.metadata()?.len();
        Ok(n)
    }
}
