extern crate byteorder;
use byteorder::{ LittleEndian, ReadBytesExt, WriteBytesExt };

use std::io;

type Endian = LittleEndian;

pub trait SerDe {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()>;
    fn deserialize<T: io::Read>(&mut T) -> io::Result<Self>
        where Self: Sized;
}

pub enum MsgType {
    Verify,
    Accept,
    Decline,
    Key(String),
    Require(String),
    Satisfy(String),
    NoSatisfy(String),
    Give(String),
    Mark(MsgMark),
    Marked(MsgMarked),
}
impl SerDe for MsgType {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        use MsgType::*;
        match self {
            Verify => {
                to.write_u16::<Endian>(0)
            },
            Accept => {
                to.write_u16::<Endian>(1)
            },
            Decline => {
                to.write_u16::<Endian>(2)
            },
            Key(key) => {
                to.write_u16::<Endian>(3)?;
                key.serialize(to)
            },
            Require(hash) => {
                to.write_u16::<Endian>(4)?;
                hash.serialize(to)
            },
            Satisfy(hash) => {
                to.write_u16::<Endian>(5)?;
                hash.serialize(to)
            },
            NoSatisfy(hash) => {
                to.write_u16::<Endian>(6)?;
                hash.serialize(to)
            },
            Give(hash) => {
                to.write_u16::<Endian>(7)?;
                hash.serialize(to)
            },
            Mark(mark) => {
                to.write_u16::<Endian>(8)?;
                mark.serialize(to)
            },
            Marked(marked) => {
                to.write_u16::<Endian>(9)?;
                marked.serialize(to)
            },
        }
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<MsgType> {
        use MsgType::*;
        let type_field = from.read_u16::<Endian>()?;
        match type_field {
            0 => {
                Ok(Verify)
            },
            1 => {
                Ok(Accept)
            },
            2 => {
                Ok(Decline)
            },
            3 => {
                Ok(Key(String::deserialize(from)?))
            },
            4 => {
                Ok(Require(String::deserialize(from)?))
            },
            5 => {
                Ok(Satisfy(String::deserialize(from)?))
            },
            6 => {
                Ok(NoSatisfy(String::deserialize(from)?))
            },
            7 => {
                Ok(Give(String::deserialize(from)?))
            },
            8 => {
                Ok(Mark(MsgMark::deserialize(from)?))
            },
            9 => {
                Ok(Marked(MsgMarked::deserialize(from)?))
            },
            _ => {
                Err(io::Error::new(io::ErrorKind::InvalidData, "Couldn't read message!"))
            }
        }
    }
}

#[derive(Clone)]
pub enum MarkResult {
    Fail(String),
    Success(i32, i32),
    CE(String),
    RTE,
    TLE,
    Blocked(u64)
}
impl SerDe for MarkResult {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        match self {
            MarkResult::Fail(s) => {
                to.write_u8(0)?;
                s.serialize(to)?;
            },
            MarkResult::Success(s, ns) => {
                to.write_u8(1)?;
                to.write_i32::<Endian>(*s)?;
                to.write_i32::<Endian>(*ns)?;
            },
            MarkResult::CE(s) => {
                to.write_u8(2)?;
                s.serialize(to)?;
            },
            MarkResult::RTE => to.write_u8(3)?,
            MarkResult::TLE => to.write_u8(4)?,
            MarkResult::Blocked(n) => {
                to.write_u8(5)?;
                to.write_u64::<Endian>(*n)?;
            },
        }
        Ok(())
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<MarkResult> {
        Ok(match from.read_u8()? {
            0 => {
                MarkResult::Fail(String::deserialize(from)?)
            },
            1 => {
                let s = from.read_i32::<Endian>()?;
                let ns = from.read_i32::<Endian>()?;
                MarkResult::Success(s, ns)
            }
            2 => {
                MarkResult::CE(String::deserialize(from)?)
            }
            3 => MarkResult::RTE,
            4 => MarkResult::TLE,
            5 => {
                MarkResult::Blocked(from.read_u64::<Endian>()?)
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown result type"))
        })
    }
}

pub struct MsgMark {
    pub batch:  u32,
    pub hash:   String,
    pub lang:   String,
    pub answer: String
}

impl SerDe for MsgMark {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        to.write_u32::<Endian>(self.batch)?;
        self.hash.serialize(to)?;
        self.lang.serialize(to)?;
        self.answer.serialize(to)
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<Self> {
        let batch = from.read_u32::<Endian>()?;
        let hash = String::deserialize(from)?;
        let lang = String::deserialize(from)?;
        let answer = String::deserialize(from)?;
        Ok(MsgMark {
            batch,
            hash,
            lang,
            answer
        })
    }
}

pub struct MsgMarked {
    pub batch:  u32,
    pub case:   u32,
    pub result: MarkResult
}

impl SerDe for MsgMarked {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        to.write_u32::<Endian>(self.batch)?;
        to.write_u32::<Endian>(self.case)?;
        self.result.serialize(to)
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<Self> {
        let batch = from.read_u32::<Endian>()?;
        let case = from.read_u32::<Endian>()?;
        let result = MarkResult::deserialize(from)?;
        Ok(MsgMarked {
            batch,
            case,
            result
        })
    }
}

impl SerDe for String {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        let s = self.clone().into_bytes();
        to.write_u64::<Endian>(s.len() as u64)?;
        to.write(&s)?;
        Ok(())
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<String> {
        let len = from.read_u32::<Endian>()?;
        let mut v = vec![0u8; len as usize];
        from.read(&mut v)?;
        Ok(match String::from_utf8(v) {
            Ok(s) => s,
            Err(err) => 
                return Err(io::Error::new(io::ErrorKind::InvalidData, err))
        })
    }
}
