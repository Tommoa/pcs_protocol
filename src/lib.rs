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
    Hash([u8;32]),
    Decline,
    Accept,
    Need(MsgNeed),
    Status(MsgStatus),
    Give(MsgGive),
    Mark(MsgMark),
    Marking(MsgMarking),
    Done(MsgDone)
}
impl SerDe for MsgType {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        match self {
            MsgType::Verify => to.write_u8(0)?,
            MsgType::Hash(hash) => {
                to.write_u8(1)?;
                to.write(hash)?;
            },
            MsgType::Decline => to.write_u8(2)?,
            MsgType::Accept => to.write_u8(3)?,
            MsgType::Need(inner) => {
                to.write_u8(4)?;
                inner.serialize(to)?;
            },
            MsgType::Status(inner) => {
                to.write_u8(5)?;
                inner.serialize(to)?;
            },
            MsgType::Give(inner) => {
                to.write_u8(6)?;
                inner.serialize(to)?;
            },
            MsgType::Mark(inner) => {
                to.write_u8(7)?;
                inner.serialize(to)?;
            },
            MsgType::Marking(inner) => {
                to.write_u8(8)?;
                inner.serialize(to)?;
            },
            MsgType::Done(inner) => {
                to.write_u8(9)?;
                inner.serialize(to)?;
            },
        }
        Ok(())
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<MsgType> {
        let type_field = from.read_u16::<Endian>()?;
        match type_field {
            0 => Ok(MsgType::Verify),
            1 => {
                let mut hash = [0u8;32];
                from.read(&mut hash)?;
                Ok(MsgType::Hash(hash))
            },
            2 => Ok(MsgType::Decline),
            3 => Ok(MsgType::Accept),
            4 => Ok(MsgType::Need(MsgNeed::deserialize(from)?)),
            5 => Ok(MsgType::Status(MsgStatus::deserialize(from)?)),
            6 => Ok(MsgType::Give(MsgGive::deserialize(from)?)),
            7 => Ok(MsgType::Mark(MsgMark::deserialize(from)?)),
            8 => Ok(MsgType::Marking(MsgMarking::deserialize(from)?)),
            9 => Ok(MsgType::Done(MsgDone::deserialize(from)?)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Encountered unknown type field"))
        }
    }
}

#[derive(Clone, Copy)]
pub enum MarkResult {
    Fail,
    Success(i32, i32),
    CE,
    RTE,
    TLE,
    Blocked(u64)
}
impl SerDe for MarkResult {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        match self {
            MarkResult::Fail => to.write_u8(0)?,
            MarkResult::Success(s, ns) => {
                to.write_u8(1)?;
                to.write_i32::<Endian>(*s)?;
                to.write_i32::<Endian>(*ns)?;
            },
            MarkResult::CE => to.write_u8(2)?,
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
            0 => MarkResult::Fail,
            1 => {
                let s = from.read_i32::<Endian>()?;
                let ns = from.read_i32::<Endian>()?;
                MarkResult::Success(s, ns)
            }
            2 => MarkResult::CE,
            3 => MarkResult::RTE,
            4 => MarkResult::TLE,
            5 => {
                MarkResult::Blocked(from.read_u64::<Endian>()?)
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown result type"))
        })
    }
}
pub struct MsgNeed {
    pub batch:      u32,
    pub questions:  u32
}
impl SerDe for MsgNeed {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        to.write_u32::<Endian>(self.batch)?;
        to.write_u32::<Endian>(self.questions)
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<MsgNeed> {
        let batch = from.read_u32::<Endian>()?;
        let questions = from.read_u32::<Endian>()?;
        Ok(MsgNeed {
            batch:      batch,
            questions:  questions
        })
    }
}
pub struct MsgStatus {
    pub ok:         bool,
    pub batch:      u32,
}
impl SerDe for MsgStatus {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        to.write_u8(self.ok as u8)?;
        to.write_u32::<Endian>(self.batch)
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<MsgStatus> {
        let ok = from.read_u8()? > 0;
        let batch = from.read_u32::<Endian>()?;
        Ok(MsgStatus {
            ok:     ok,
            batch:  batch
        })
    }
}
pub struct MsgGive {
    pub sequence:   u16,
    pub batch:      u32,
    pub text:       String,
}
impl SerDe for MsgGive {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        to.write_u16::<Endian>(self.sequence)?;
        to.write_u32::<Endian>(self.batch)?;
        let s = self.text.clone().into_bytes();
        to.write_u64::<Endian>(s.len() as u64)?;
        to.write(&s)?;
        Ok(())
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<MsgGive> {
        let sequence = from.read_u16::<Endian>()?;
        let batch = from.read_u32::<Endian>()?;
        let len = from.read_u64::<Endian>()?;
        let mut v = vec![0u8; len as usize];
        from.read(&mut v)?;
        let s = match String::from_utf8(v) {
            Ok(s) => s,
            Err(err) => 
                return Err(io::Error::new(io::ErrorKind::InvalidData, err))
        };
        Ok(MsgGive {
            sequence:   sequence,
            batch:      batch,
            text:       s
        })
    }
}
pub struct MsgMark {
    pub sequence:   u16,
    pub batch:      u32,
    pub time:       Option<u64>,
    pub text:       String,
    pub lang:       String,
}
impl SerDe for MsgMark {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        to.write_u16::<Endian>(self.sequence)?;
        to.write_u32::<Endian>(self.batch)?;
        if let Some(time) = self.time {
            to.write_u8(1)?;
            to.write_u64::<Endian>(time)?;
        } else {
            to.write_u8(0)?;
        }
        let s = self.text.clone().into_bytes();
        to.write_u64::<Endian>(s.len() as u64)?;
        to.write(&s)?;
        let s = self.lang.clone().into_bytes();
        to.write_u64::<Endian>(s.len() as u64)?;
        to.write(&s)?;
        Ok(())
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<MsgMark> {
        let sequence = from.read_u16::<Endian>()?;
        let batch = from.read_u32::<Endian>()?;
        let time = if from.read_u8()? == 1 {
            Some(from.read_u64::<Endian>()?)
        } else { None };
        let len = from.read_u64::<Endian>()?;
        let mut v = vec![0u8; len as usize];
        from.read(&mut v)?;
        let s = match String::from_utf8(v) {
            Ok(s) => s,
            Err(err) => 
                return Err(io::Error::new(io::ErrorKind::InvalidData, err))
        };
        let len = from.read_u64::<Endian>()?;
        let mut v = vec![0u8; len as usize];
        from.read(&mut v)?;
        let lang = match String::from_utf8(v) {
            Ok(s) => s,
            Err(err) => 
                return Err(io::Error::new(io::ErrorKind::InvalidData, err))
        };
        Ok(MsgMark {
            sequence:   sequence,
            batch:      batch,
            time:       time,
            text:       s,
            lang:       lang
        })
    }
}
pub struct MsgMarking {
    pub batch:      u32,
}
impl SerDe for MsgMarking {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        to.write_u32::<Endian>(self.batch)
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<MsgMarking> {
        let batch = from.read_u32::<Endian>()?;
        Ok(MsgMarking {
            batch:      batch
        })
    }
}
pub struct MsgDone {
    pub batch:      u32,
    pub test:       u64,
    pub result:     MarkResult,
}
impl SerDe for MsgDone {
    fn serialize<T: io::Write>(&self, to: &mut T) -> io::Result<()> {
        to.write_u32::<Endian>(self.batch)?;
        to.write_u64::<Endian>(self.test)?;
        self.result.serialize(to)
    }
    fn deserialize<T: io::Read>(from: &mut T) -> io::Result<MsgDone> {
        let batch = from.read_u32::<Endian>()?;
        let test_num = from.read_u64::<Endian>()?;
        let result = MarkResult::deserialize(from)?;
        Ok(MsgDone {
 batch:      batch,
            test:       test_num,
            result:     result
        })
    }
}
