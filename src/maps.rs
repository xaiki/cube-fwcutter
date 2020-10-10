use memmap::Mmap;
use std::cmp;

pub struct ReadableMmap {
    m: Mmap,
    p: usize,
}

impl ReadableMmap {
    pub fn new(m: Mmap) -> Self {
        ReadableMmap { m, p: 0 }
    }
    pub fn seek(&mut self, s: isize) {
        self.p = (self.p as isize + s) as usize
    }
}

impl std::io::Read for ReadableMmap {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = cmp::min(self.m.len() - self.p, buf.len());
        buf.copy_from_slice(&self.m[self.p..(self.p + len)]);

        self.p += len;
        Ok(len)
    }
}
