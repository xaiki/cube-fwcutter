extern crate fwcutter;
use fwcutter::pattern::Pattern;

use memmap::Mmap;
use std::env;
use std::fs::File;

static UNIT: [&str; 5] = [" ", "k", "M", "G", "T"];

fn format_size(i: isize) -> String {
    let mut c = i;
    for u in UNIT.iter() {
        if (c / 1024) > 0 {
            c /= 1024;
        } else {
            return format!("{:.2}{}", c, u);
        }
    }
    "VALUE TOO BIG".to_string()
}

fn cut_fw(filename: &str) -> std::io::Result<()> {
    let file = File::open(filename)?;
    //    let mut reader = BufReader::with_capacity(1024 * 1024 * 1024, file);
    //     let mut buf = [0; 1];
    let mmap = unsafe { Mmap::map(&file)? };
    let mut reader = mmap.chunks(1);

    let mut patterns = [
        Pattern::new(&[0x5a, 0x4f, 0x00, 0x00]),
        Pattern::new(&[0xf7, 0x06, 0x00, 0x00]),
        Pattern::new(&[0x01, 0x08, 0x01]).lookback(20),
        //        Pattern::new(b"gr\\").lookback(20),
        Pattern::new(b"\0").lookback(20).repeats(15 * 16 + 7),
    ];
    let mut lasts = vec![0; patterns.len()];
    let mut s = vec![String::new(); patterns.len()];

    let mut l = 0;
    let mut last_addr = 0;

    loop {
        if let Some(d) = reader.next() {
            l += 1;

            for i in 0..patterns.len() {
                if let Some(n) = patterns[i].push(d[0]) {
                    let c = l - n;
                    let size = l - last_addr;
                    let format = || format!("{:#016x}: found {:#?}", c, patterns[i]);
                    if lasts[i] != c {
                        if lasts[i] == 0 {
                            s[i] = format()
                        }
                        println!("{:#08x} {:>5} {}", size, format_size(size), &s[i]);
                        last_addr = l;
                        lasts[i] = c;
                    }
                    s[i] = format();
                }
            }
        } else {
            return Ok(());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    cut_fw(&args[1]).unwrap();
}
