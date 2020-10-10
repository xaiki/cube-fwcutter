extern crate fwcutter;
use fwcutter::pattern::Pattern;

use memmap::Mmap;
use std::io::Write;

use std::env;
use std::fs;
use std::path::Path;

static EXTRACT_PATH: &str = "./extract";

fn get_filename(p: &mut Pattern) -> Option<String> {
    let b = &p._ro_get();
    let l = b.len();
    //                    println!("{:#?}", p);
    for i in 1..(l - 1) {
        if b[l - i - 1] == 0 {
            let s: String = b[(l - i)..]
                .iter()
                .map(|c| match c {
                    b'\\' => '/',
                    _ => *c as char,
                })
                .collect();
            return Some(s);
        }
    }
    None
}

fn cut_fw(filename: &str) -> std::io::Result<()> {
    let file = fs::File::open(filename)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let mut reader = mmap.chunks(1);
    let mut p = Pattern::new(b"\0").lookback(40).repeats(15 * 16);

    let mut buffer: Vec<u8> = Vec::with_capacity(1024 * 1024 * 1024);
    let mut read = 0;
    let mut last_p = 0;
    let mut last_n = 0;
    let mut current_file: Option<fs::File> = None;
    loop {
        if let Some(d) = reader.next() {
            read += 1;
            if let Some(n) = p.push(d[0]) {
                if read - n != last_p {
                    match get_filename(&mut p) {
                        Some(s) => {
                            if let Some(ref mut f) = current_file {
                                //        the_buffer   the_match  the_file  the_padding
                                let end = buffer.len() - last_n - s.len() - 1;
                                assert_ne!(buffer[0], b'\0');
                                f.write_all(&buffer[..end])?
                            }
                            buffer.clear();
                            let path = Path::new(EXTRACT_PATH).join(s);
                            let dir = path.parent().expect("path is malformed");
                            fs::create_dir_all(dir).expect("couldn't mkdir");

                            let file = fs::File::create(&path)?;
                            current_file = Some(file);

                            println!("filename: {:#?}", path);
                        }
                        None => println!("{:010x} NOT FOUND: {:#?}", last_p, p),
                    }
                    last_p = read - n;
                    last_n = n as usize;
                }
            } else {
                buffer.push(d[0]);
            }
        } else {
            if let Some(ref mut f) = current_file {
                f.write_all(&buffer)?
            }
            return Ok(());
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    cut_fw(&args[1])
}
