use crate::circular_buffer::CircularBuffer;
use std::fmt;

struct Indexes {
    i: usize,
    match_count: usize,
    min_match: usize,
}

pub struct Pattern<'a> {
    pattern: &'a [u8],
    lookback: CircularBuffer,
    idx: Indexes,
}

impl<'a> Pattern<'a> {
    pub fn new(pattern: &'a [u8]) -> Self {
        Pattern {
            pattern,
            lookback: CircularBuffer::new(0),
            idx: Indexes {
                i: 0,
                match_count: 0,
                min_match: 0,
            },
        }
    }

    pub fn push(&mut self, c: u8) -> Option<isize> {
        if self.pattern.is_empty() {
            return None;
        }

        let l = self.pattern.len();
        let i = &mut self.idx.i;

        let looking_back = !self.lookback.is_empty();

        //        println!(">>> {} ({:#?})", c as char, self.lookback);
        // no match, reset and return
        if c != self.pattern[*i] as u8 {
            if looking_back {
                for _ in 0..self.idx.match_count {
                    for p in self.pattern.iter() {
                        self.lookback.push(*p);
                    }
                }
                // push the half-backed pattern we were matching
                for j in 0..(*i) {
                    self.lookback.push(self.pattern[j]);
                }
            }

            self.idx.match_count = 0;

            if c != self.pattern[0] as u8 {
                *i = 0;
                self.lookback.push(c);
                return None;
            }
            *i = 0;
        }

        // matching, but still didn't consume pattern
        if (*i + 1) < (l) {
            *i += 1;
            return None;
        }

        *i = 0;
        // matched, we don't look for repeats
        if self.idx.min_match == 0 {
            return Some(l as isize);
        }

        // repeats handling
        self.idx.match_count += 1;

        // println!("MATCH COUNT: {}", self.idx.match_count);
        if self.idx.match_count < self.idx.min_match {
            return None;
        }

        Some((l * (self.idx.match_count)) as isize)
    }

    pub fn _ro_get(&'a self) -> &'a [u8] {
        let buf = &self.lookback;
        if !(0..(self.idx.i)).is_empty() {
            println!("WE ARE MISSING DATA ! you should use .get()");
        }
        &buf[0..buf.len()]
    }

    pub fn get(&'a mut self) -> &'a [u8] {
        let buf = &self.lookback;
        if buf.is_empty() {
            return &[];
        }

        for j in 0..(self.idx.i) {
            self.lookback.push(self.pattern[j]);
        }

        /*        println!(
            "GET self.buffer: {:#?}, self.pattern: {:#?}",
            self.buffer, self.pattern
        );*/

        self._ro_get()
    }

    pub fn lookback(mut self, p: usize) -> Self {
        self.lookback = CircularBuffer::new(p);
        self
    }

    pub fn repeats(mut self, r: usize) -> Self {
        self.idx.min_match = r;
        self
    }
}

fn to_str(a: &[u8]) -> String {
    a.iter().map(|c| format!("{:02x} ", c)).collect::<String>()
}

impl fmt::Debug for Pattern<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buffer = self._ro_get();
        if !buffer.is_empty() {
            f.write_str("[ ")?;
            let s = to_str(&buffer[0..buffer.len()]);
            f.write_str(&s)?;
            f.write_str("] ")?;
        }

        if self.idx.min_match > 0 {
            f.write_str("( ")?;
        }
        let s = to_str(self.pattern);
        f.write_str(&s)?;
        if self.idx.min_match > 0 {
            f.write_str(&format!(") * {} ", self.idx.match_count))?;
        };
        std::result::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut p = Pattern::new(&[]);
        assert_eq!(p.push(0), None);
    }

    #[test]
    fn oneinone() {
        let mut p = Pattern::new(b"a");
        assert_eq!(p.push(b'0'), None);
        assert_eq!(p.push(b'a'), Some(1));
        assert_eq!(p.get(), []);
    }

    #[test]
    fn oneintwo() {
        let mut p = Pattern::new(b"b");
        assert_eq!(p.push(0), None);
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'b'), Some(1));
        assert_eq!(p.get(), []);
    }

    #[test]
    fn many() {
        let mut p = Pattern::new(b"a");
        assert_eq!(p.push(b'0'), None);
        assert_eq!(p.push(b'a'), Some(1));
        assert_eq!(p.push(b'a'), Some(1));
        assert_eq!(p.get(), []);
    }

    #[test]
    fn manytwo() {
        let mut p = Pattern::new(b"ab");
        assert_eq!(p.push(b'0'), None);
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'b'), Some(2));
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'b'), Some(2));
        assert_eq!(p.get(), []);
    }

    #[test]
    fn twoinfour() {
        let mut p = Pattern::new(b"bc");
        assert_eq!(p.push(0), None);
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'b'), None);
        assert_eq!(p.push(b'c'), Some(2));
        assert_eq!(p.get(), []);
    }

    #[test]
    fn threeinsix() {
        let mut p = Pattern::new(b"cde");
        assert_eq!(p.push(b'0'), None);
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'b'), None);
        assert_eq!(p.push(b'c'), None);
        assert_eq!(p.push(b'd'), None);
        assert_eq!(p.push(b'e'), Some(3));
        assert_eq!(p.get(), []);
    }

    #[test]
    fn empty_lookback() {
        let mut p = Pattern::new(&[]).lookback(0);
        assert_eq!(p.push(0), None)
    }

    #[test]
    fn oneinone_lookback() {
        let mut p = Pattern::new(b"a").lookback(1);
        assert_eq!(p.push(b'0'), None);
        assert_eq!(p.push(b'a'), Some(1));
        println!("DEBUG {:#?}", p);
        assert_eq!(p.get(), b"0");
    }

    #[test]
    fn oneintwo_lookback() {
        let mut p = Pattern::new(b"b").lookback(2);
        assert_eq!(p.push(0), None);
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'b'), Some(1));
    }

    #[test]
    fn twoinfour_lookback() {
        let mut p = Pattern::new(b"bc").lookback(2);
        assert_eq!(p.push(0), None);
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'b'), None);
        assert_eq!(p.push(b'c'), Some(2));
        assert_eq!(p.push(b'd'), None);
    }

    #[test]
    fn empty_repeats() {
        let mut p = Pattern::new(&[]).repeats(0);
        assert_eq!(p.push(0), None);
    }

    #[test]
    fn oneinone_repeats() {
        let mut p = Pattern::new(b"a").repeats(2);
        assert_eq!(p.push(b'0'), None);
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'a'), Some(2));
        assert_eq!(p.get(), []);
    }

    #[test]
    fn oneintwo_repeats() {
        let mut p = Pattern::new(b"b").repeats(2);
        assert_eq!(p.push(b'0'), None);
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'b'), None);
        assert_eq!(p.push(b'b'), Some(2));
        assert_eq!(p.get(), []);
    }

    #[test]
    fn twoinfour_repeats() {
        let mut p = Pattern::new(b"bc").repeats(2);
        assert_eq!(p.push(0), None);
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'b'), None);
        assert_eq!(p.push(b'c'), None);
        assert_eq!(p.push(b'b'), None);
        assert_eq!(p.push(b'c'), Some(4));
        assert_eq!(p.get(), []);
    }

    #[test]
    fn twoinfour_repeats_lookback() {
        let mut p = Pattern::new(b"bc").repeats(2).lookback(2);
        assert_eq!(p.push(b'0'), None);
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'b'), None);
        assert_eq!(p.push(b'c'), None);
        assert_eq!(p.push(b'b'), None);
        assert_eq!(p.push(b'c'), Some(4));
        assert_eq!(p.get(), b"0a");
    }

    #[test]
    fn twoinfour_repeats_lookback_overflow() {
        let mut p = Pattern::new(b"cd").repeats(2).lookback(2);
        assert_eq!(p.push(b'0'), None);
        assert_eq!(p.push(b'a'), None);
        assert_eq!(p.push(b'b'), None);
        assert_eq!(p.push(b'c'), None);
        assert_eq!(p.push(b'd'), None);
        assert_eq!(p.push(b'c'), None);
        assert_eq!(p.push(b'd'), Some(4));
        assert_eq!(p.get(), b"ab");
    }

    #[test]
    fn many_almost_many() {
        let mut p = Pattern::new(b"cd").repeats(2).lookback(2);
        let res = b"bcdecdcdf"
            .iter()
            .map(|c| p.push(*c))
            .collect::<Vec<Option<isize>>>();
        assert_eq!(p.get(), b"df");

        assert_eq!(
            res,
            //b     c     d     e     c     d     c     d     f
            [None, None, None, None, None, None, None, Some(4), None]
        );
    }

    #[test]
    fn many_many() {
        let mut p = Pattern::new(b"cd").repeats(2).lookback(2);
        let res = b"abccdcdefghcdcdcd"
            .iter()
            .map(|c| p.push(*c))
            .collect::<Vec<Option<isize>>>();

        assert_eq!(p.get(), b"gh");
        assert_eq!(
            res,
            [
                None,
                None,
                None,
                None,
                None,
                None,
                Some(4),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(4),
                None,
                Some(6)
            ]
        );
    }

    #[test]
    fn more_than_you_asked() {
        let mut p = Pattern::new(b"01").repeats(2).lookback(2);
        let res = b"abc0101010101010101010"
            .iter()
            .map(|c| p.push(*c))
            .collect::<Vec<Option<isize>>>();
        assert_eq!(p.get(), b"c0");
        assert_eq!(
            res,
            [
                None,
                None,
                None,
                None,
                None,
                None,
                Some(4),
                None,
                Some(6),
                None,
                Some(8),
                None,
                Some(10),
                None,
                Some(12),
                None,
                Some(14),
                None,
                Some(16),
                None,
                Some(18),
                None
            ]
        );
    }

    #[test]
    fn many_many_prefix() {
        let mut p = Pattern::new(b"cd").repeats(2).lookback(2);
        let res = b"abccdcd"
            .iter()
            .map(|c| p.push(*c))
            .collect::<Vec<Option<isize>>>();

        assert_eq!(p.get(), b"bc");
        assert_eq!(res, [None, None, None, None, None, None, Some(4)]);
    }

    #[test]
    fn many_in_many() {
        let mut p = Pattern::new(b"cd").repeats(2).lookback(2);
        let res = b"hello darkness my old friend cdcdcdcdcdcdcdcd, I've come to talk to you again ! cdcdcdcdcdcdcd".iter().map(|c| p.push(*c)).collect::<Vec<Option<isize>>>();
        assert_eq!(
            res,
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(4),
                None,
                Some(6),
                None,
                Some(8),
                None,
                Some(10),
                None,
                Some(12),
                None,
                Some(14),
                None,
                Some(16),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(4),
                None,
                Some(6),
                None,
                Some(8),
                None,
                Some(10),
                None,
                Some(12),
                None,
                Some(14)
            ]
        )
    }
}
