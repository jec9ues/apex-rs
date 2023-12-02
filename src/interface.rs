use mem_struct::data_struct::apex::*;
use mem_struct::data_struct::apex::structs::EntityList;
use memprocfs::{FLAG_NOCACHE, FLAG_NOPAGING, FLAG_ZEROPAD_ON_FAIL, VmmProcess};

pub trait Interface {
    fn read_direct(&self, addr: u64, size: usize) -> Option<Vec<u8>>;
    fn write_direct(&self, addr: u64, data: Vec<u8>) -> Result<(), anyhow::Error>;
}
impl Interface for VmmProcess<'_> {
    fn read_direct(&self, addr: u64, size: usize) -> Option<Vec<u8>> {
        match self.mem_read_ex(addr, size, FLAG_NOCACHE | FLAG_ZEROPAD_ON_FAIL | FLAG_NOPAGING) {
            Err(e) => { println!("{}:{} -> read fail [{}]", e, addr, size); None },
            Ok(data) => { Some(data) },
        }
    }

    fn write_direct(&self, addr: u64, data: Vec<u8>) -> Result<(), anyhow::Error> {
        match self.mem_write(addr, &data) {
            Err(e) => { println!("{}:{} -> write fail [{}]", e, addr, data.len());Err(e) }
            Ok(_) => { Ok(()) }
        }
    }
}
pub trait Readable {
    fn read<T>(&self, addr: u64) -> Option<T>
        where
            T: std::fmt::Debug + Default + FromBytes;

    fn read_op<T>(&self, addr: u64, size: usize) -> Option<T>
        where
            T: std::fmt::Debug + Default + FromBytes;
}

impl Readable for VmmProcess<'_> {
    fn read<T>(&self, addr: u64) -> Option<T>
        where
            T: std::fmt::Debug + Default + FromBytes,
    {
        let data = match self.read_direct(addr, std::mem::size_of::<T>()) {
            Some(data) => data,
            None => return None,
        };
        T::from_bytes(&data)
    }

    fn read_op<T>(&self, addr: u64, size: usize) -> Option<T>
        where
            T: std::fmt::Debug + Default + FromBytes,
    {
        let data = match self.read_direct(addr, size) {
            Some(data) => data,
            None => return None,
        };
        T::from_bytes(&data)
    }
}

pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
        where
            Self: Sized;
}

impl FromBytes for u8 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= std::mem::size_of::<u8>() {
            Some(bytes[0])
        } else {
            None
        }
    }
}

impl FromBytes for u16 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= std::mem::size_of::<u16>() {
            Some(u16::from_le_bytes(bytes[..2].try_into().unwrap()))
        } else {
            None
        }
    }
}

impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= std::mem::size_of::<u32>() {
            Some(u32::from_le_bytes(bytes[..4].try_into().unwrap()))
        } else {
            None
        }
    }
}

impl FromBytes for i32 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= std::mem::size_of::<i32>() {
            Some(i32::from_le_bytes(bytes[..4].try_into().unwrap()))
        } else {
            None
        }
    }
}

impl FromBytes for f32 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= std::mem::size_of::<f32>() {
            Some(f32::from_le_bytes(bytes[..4].try_into().unwrap()))
        } else {
            None
        }
    }
}

// 实现 FromBytes trait for u64
impl FromBytes for u64 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= std::mem::size_of::<u64>() {
            Some(u64::from_le_bytes(bytes[..8].try_into().unwrap()))
        } else {
            None
        }
    }
}

// 实现 FromBytes trait for String
impl FromBytes for String {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let trimmed_bytes = bytes.iter().rev().position(|&x| x != 0).map(|pos| &bytes[..bytes.len() - pos]);
        String::from_utf8(trimmed_bytes.unwrap_or(bytes).to_vec()).ok()
    }
}

impl FromBytes for EntityList {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= (60 << 5) {
            let data = bytes
                .chunks_exact(0x20)
                .enumerate()
                .map(|(index, chunk)| {
                    let chunk_u64 = u64::from_le_bytes(chunk[..8].try_into().unwrap());
                    (index as u64, if chunk_u64 != 0 { Some(chunk_u64) } else { None })
                })
                .collect();
            Some(EntityList { data })
        } else {
            None
        }
    }
}