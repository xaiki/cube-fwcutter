extern crate fwcutter;
use fwcutter::maps;
use fwcutter::mpfs;

use memmap::Mmap;
use std::env;
use std::fs;

fn read_mpfs(filename: &str) -> std::io::Result<()> {
    let file = fs::File::open(filename)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let mut reader = maps::ReadableMmap::new(mmap);

    let header = mpfs::Header::new(&mut reader)?;
    println!("{:#?}", header);
    reader.seek((header.entries * 2) as isize);
    let file_header = mpfs::FileHeader::new(&mut reader)?;
    println!("{:#?}", file_header);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    read_mpfs(&args[1]).unwrap();
}
