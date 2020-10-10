use std::fmt;
use std::ops;

pub struct CircularBuffer {
    buffer: Vec<u8>,
    p: usize,
}

impl<'a> CircularBuffer {
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            buffer: vec![0; size * 2],
            p: 0,
        }
    }
    pub fn push(&mut self, c: u8) {
        self.set(0, c);
        self.p += 1;
    }

    pub fn set(&mut self, i: usize, c: u8) -> Option<u8> {
        if self.buffer.is_empty() {
            return None;
        }
        let l = self.buffer.len() / 2;
        let idx = (self.p + l - i) % l;
        self.buffer[idx] = c;
        self.buffer[idx + l] = c;
        Some(c)
    }

    pub fn len(&self) -> usize {
        self.buffer.len() / 2
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.len() == 0
    }
}

impl ops::Index<usize> for CircularBuffer {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        let l = self.buffer.len() / 2;
        &self.buffer[(self.p + i) % l]
    }
}

/*
impl ops::IndexMut<usize> for CircularBuffer {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        let l = self.buffer.len() / 2;
        //XXX(xaiki): we do not update the second part BUG FIXME !
        &mut self.buffer[(self.p + i) % l]
    }
}
 */

impl ops::Index<ops::Range<usize>> for CircularBuffer {
    type Output = [u8];

    fn index(&self, range: ops::Range<usize>) -> &Self::Output {
        let buffer_len = self.buffer.len() / 2;
        let buffer_offset = self.p % buffer_len;

        //        println!("{} {:#?} {:#?}", buffer_offset, self.buffer, range);
        &self.buffer[(buffer_offset + range.start)..(buffer_offset + range.end)]
    }
}

impl ops::Index<ops::RangeTo<usize>> for CircularBuffer {
    type Output = [u8];

    fn index(&self, range: ops::RangeTo<usize>) -> &Self::Output {
        &self.buffer[ops::Range {
            start: 0,
            end: range.end,
        }]
    }
}

fn to_str(a: &[u8]) -> String {
    a.iter()
        .map(|c| format!("{:02x}({}) ", c, *c as char))
        .collect::<String>()
}

impl fmt::Debug for CircularBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let l = self.buffer.len() / 2;

        match l {
            0 => f.write_str("[]"),
            _ => {
                let s = to_str(&self.buffer[(self.p % l)..((self.p % l) + l)]);
                f.write_str(&s)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(CircularBuffer::new(0).len(), 0);
    }

    #[test]
    fn oneelement() {
        let mut b = CircularBuffer::new(1);
        b.push(b'a');
        assert_eq!(b.len(), 1);
        assert_eq!(b[0], b'a');
    }

    #[test]
    fn twoelements() {
        let mut b = CircularBuffer::new(2);
        b.push(b'a');
        b.push(b'b');
        assert_eq!(b.len(), 2);
        assert_eq!(b[0], b'a');
        assert_eq!(b[1], b'b');
    }

    #[test]
    fn twoelementsoverflow() {
        let mut b = CircularBuffer::new(2);
        b.push(b'a');
        b.push(b'b');
        b.push(b'c');
        assert_eq!(b.len(), 2);
        assert_eq!(b[0], b'b');
        assert_eq!(b[1], b'c');
    }

    #[test]
    fn twoelementscompleteoverflow() {
        let mut b = CircularBuffer::new(2);
        b.push(b'a');
        b.push(b'b');
        b.push(b'c');
        b.push(b'd');
        b.push(b'e');
        assert_eq!(b.len(), 2);
        assert_eq!(b[0], b'd');
        assert_eq!(b[1], b'e');
    }

    #[test]
    fn sliceone() {
        let mut b = CircularBuffer::new(2);
        b.push(b'a');
        b.push(b'b');
        assert_eq!(b.len(), 2);
        assert_eq!(&b[0..2], b"ab");
    }

    #[test]
    fn sliceoverflow() {
        let mut b = CircularBuffer::new(2);
        b.push(b'a');
        b.push(b'b');
        b.push(b'c');
        assert_eq!(b.len(), 2);
        assert_eq!(&b[0..2], b"bc");
    }

    #[test]
    fn slicefulloverflow() {
        let mut b = CircularBuffer::new(2);
        b.push(b'a');
        b.push(b'b');
        b.push(b'c');
        b.push(b'd');
        b.push(b'e');
        assert_eq!(b.len(), 2);
        assert_eq!(&b[0..2], b"de");
    }
}
