use crate::maps;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use chrono::NaiveDateTime;
use std::fmt;

pub struct Version {
    major: u8,
    minor: u8,
}

pub struct Header {
    sign: [u8; 4],
    pub ver: Version,
    pub entries: u16,
}

impl Header {
    pub fn new(reader: &mut maps::ReadableMmap) -> std::io::Result<Self> {
        Ok(Header {
            sign: [
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
                reader.read_u8()?,
            ],
            ver: Version {
                major: reader.read_u8()?,
                minor: reader.read_u8()?,
            },
            entries: reader.read_u16::<BigEndian>()?,
        })
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(" Header\n")?;
        f.write_str(&format!(
            " version: {}.{}\n",
            self.ver.major, self.ver.minor
        ))?;
        f.write_str(&format!(" entries: {}\n", self.entries))?;
        std::result::Result::Ok(())
    }
}

pub struct FileHeader {
    filename: [u8; 4],
    start: u32,
    size: u32,
    timestamp: u32,
    microtime: u32,
    flags: u16,
}

impl FileHeader {
    pub fn new(reader: &mut maps::ReadableMmap) -> std::io::Result<Self> {
        let buf = [
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        ];

        Ok(FileHeader {
            filename: [buf[3], buf[2], buf[1], buf[0]],
            start: reader.read_u32::<BigEndian>()?,
            size: reader.read_u32::<BigEndian>()?,
            timestamp: reader.read_u32::<BigEndian>()?,
            microtime: reader.read_u32::<BigEndian>()?,
            flags: reader.read_u16::<BigEndian>()?,
        })
    }
}

impl fmt::Debug for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MPFS File Header\n")?;
        f.write_str(&format!(
            " filename: {}\n",
            std::str::from_utf8(&self.filename).unwrap()
        ))?;
        f.write_str(&format!(
            " start: {:#02x}\tsize: {}\n",
            self.start, self.size
        ))?;
        f.write_str(&format!(
            " time: {}\n",
            NaiveDateTime::from_timestamp(self.timestamp as i64, self.microtime / 1e3 as u32)
        ))?;
        f.write_str(&format!(" flags: {:0b}\n", self.flags))?;
        std::result::Result::Ok(())
    }
}
