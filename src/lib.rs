extern crate byteorder;
use byteorder::{ NetworkEndian, ReadBytesExt, WriteBytesExt, ByteOrder };

use std::io;

pub trait SerDe {
    fn serialize(&self, &mut Vec<u8>);
    fn deserialize<T: io::Read>([u8;2], &mut T) -> io::Result<Self>
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
    fn serialize(&self, v: &mut Vec<u8>) {
        match self {
            MsgType::Verify => v.push(0),
            MsgType::Hash(hash) => {
                v.push(1);
                v.extend_from_slice(hash);
            },
            MsgType::Decline => v.push(2),
            MsgType::Accept => v.push(3),
            MsgType::Need(inner) => {
                v.push(4);
                inner.serialize(v);
            },
            MsgType::Status(inner) => {
                v.push(5);
                inner.serialize(v);
            },
            MsgType::Give(inner) => {
                v.push(6);
                inner.serialize(v);
            },
            MsgType::Mark(inner) => {
                v.push(7);
                inner.serialize(v);
            },
            MsgType::Marking(inner) => {
                v.push(8);
                inner.serialize(v);
            },
            MsgType::Done(inner) => {
                v.push(9);
                inner.serialize(v);
            },
        }
    }
    fn deserialize<T: io::Read>(type_field_: [u8;2], from: &mut T) -> io::Result<MsgType> {
        let type_field = NetworkEndian::read_u16(&type_field_);
        match type_field {
            0 => Ok(MsgType::Verify),
            1 => {
                let mut hash = [0u8;32];
                from.read(&mut hash)?;
                Ok(MsgType::Hash(hash))
            },
            2 => Ok(MsgType::Decline),
            3 => Ok(MsgType::Accept),
            4 => Ok(MsgType::Need(MsgNeed::deserialize(type_field_, from)?)),
            5 => Ok(MsgType::Status(MsgStatus::deserialize(type_field_, from)?)),
            6 => Ok(MsgType::Give(MsgGive::deserialize(type_field_, from)?)),
            7 => Ok(MsgType::Mark(MsgMark::deserialize(type_field_, from)?)),
            8 => Ok(MsgType::Marking(MsgMarking::deserialize(type_field_, from)?)),
            9 => Ok(MsgType::Done(MsgDone::deserialize(type_field_, from)?)),
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
    fn serialize(&self, v: &mut Vec<u8>) {
        match self {
            MarkResult::Fail => v.push(0),
            MarkResult::Success(s, ns) => {
                v.push(1);
                v.write_i32::<NetworkEndian>(*s);
                v.write_i32::<NetworkEndian>(*ns);
            },
            MarkResult::CE => v.push(2),
            MarkResult::RTE => v.push(3),
            MarkResult::TLE => v.push(4),
            MarkResult::Blocked(n) => {
                v.push(5);
                v.write_u64::<NetworkEndian>(*n);
            },
        }
    }
    fn deserialize<T: io::Read>(_: [u8;2], from: &mut T) -> io::Result<MarkResult> {
        Ok(match from.read_u8()? {
            0 => MarkResult::Fail,
            1 => {
                let s = from.read_i32::<NetworkEndian>()?;
                let ns = from.read_i32::<NetworkEndian>()?;
                MarkResult::Success(s, ns)
            }
            2 => MarkResult::CE,
            3 => MarkResult::RTE,
            4 => MarkResult::TLE,
            5 => {
                MarkResult::Blocked(from.read_u64::<NetworkEndian>()?)
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
    fn serialize(&self, v: &mut Vec<u8>) {
        v.write_u32::<NetworkEndian>(self.batch);
        v.write_u32::<NetworkEndian>(self.questions);
    }
    fn deserialize<T: io::Read>(_: [u8;2], from: &mut T) -> io::Result<MsgNeed> {
        let batch = from.read_u32::<NetworkEndian>()?;
        let questions = from.read_u32::<NetworkEndian>()?;
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
    fn serialize(&self, v: &mut Vec<u8>) {
        v.write_u8(self.ok as u8);
        v.write_u32::<NetworkEndian>(self.batch);
    }
    fn deserialize<T: io::Read>(_: [u8;2], from: &mut T) -> io::Result<MsgStatus> {
        let ok = from.read_u8()? > 0;
        let batch = from.read_u32::<NetworkEndian>()?;
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
    fn serialize(&self, v: &mut Vec<u8>) {
        v.write_u16::<NetworkEndian>(self.sequence);
        v.write_u32::<NetworkEndian>(self.batch);
        let mut s = self.text.clone().into_bytes();
        v.write_u64::<NetworkEndian>(s.len() as u64);
        v.append(&mut s);
    }
    fn deserialize<T: io::Read>(_: [u8;2], from: &mut T) -> io::Result<MsgGive> {
        let sequence = from.read_u16::<NetworkEndian>()?;
        let batch = from.read_u32::<NetworkEndian>()?;
        let len = from.read_u64::<NetworkEndian>()?;
        let mut v = vec![0u8; len as usize];
        from.read(&mut v);
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
    fn serialize(&self, v: &mut Vec<u8>) {
        v.write_u16::<NetworkEndian>(self.sequence);
        v.write_u32::<NetworkEndian>(self.batch);
        if let Some(time) = self.time {
            v.write_u8(1);
            v.write_u64::<NetworkEndian>(time);
        } else {
            v.write_u8(0);
        }
        let mut s = self.text.clone().into_bytes();
        v.write_u64::<NetworkEndian>(s.len() as u64);
        v.append(&mut s);
        let mut s = self.lang.clone().into_bytes();
        v.write_u64::<NetworkEndian>(s.len() as u64);
        v.append(&mut s);
    }
    fn deserialize<T: io::Read>(_: [u8;2], from: &mut T) -> io::Result<MsgMark> {
        let sequence = from.read_u16::<NetworkEndian>()?;
        let batch = from.read_u32::<NetworkEndian>()?;
        let time = if from.read_u8()? == 1 {
            Some(from.read_u64::<NetworkEndian>()?)
        } else { None };
        let len = from.read_u64::<NetworkEndian>()?;
        let mut v = vec![0u8; len as usize];
        from.read(&mut v);
        let s = match String::from_utf8(v) {
            Ok(s) => s,
            Err(err) => 
                return Err(io::Error::new(io::ErrorKind::InvalidData, err))
        };
        let len = from.read_u64::<NetworkEndian>()?;
        let mut v = vec![0u8; len as usize];
        from.read(&mut v);
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
    pub sequence:   u16,
    pub batch:      u32,
}
impl SerDe for MsgMarking {
    fn serialize(&self, v: &mut Vec<u8>) {
        v.write_u16::<NetworkEndian>(self.sequence);
        v.write_u32::<NetworkEndian>(self.batch);
    }
    fn deserialize<T: io::Read>(_: [u8;2], from: &mut T) -> io::Result<MsgMarking> {
        let sequence = from.read_u16::<NetworkEndian>()?;
        let batch = from.read_u32::<NetworkEndian>()?;
        Ok(MsgMarking {
            sequence:   sequence,
            batch:      batch
        })
    }
}
pub struct MsgDone {
    pub sequence:   u16,
    pub batch:      u32,
    pub test:       u64,
    pub result:     MarkResult,
}
impl SerDe for MsgDone {
    fn serialize(&self, v: &mut Vec<u8>) {
        v.write_u16::<NetworkEndian>(self.sequence);
        v.write_u32::<NetworkEndian>(self.batch);
        v.write_u64::<NetworkEndian>(self.test);
        self.result.serialize(v);
    }
    fn deserialize<T: io::Read>(pass: [u8;2], from: &mut T) -> io::Result<MsgDone> {
        let sequence = from.read_u16::<NetworkEndian>()?;
        let batch = from.read_u32::<NetworkEndian>()?;
        let test_num = from.read_u64::<NetworkEndian>()?;
        let result = MarkResult::deserialize(pass, from)?;
        Ok(MsgDone {
            sequence:   sequence,
            batch:      batch,
            test:       test_num,
            result:     result
        })
    }
}
