#![allow(unused_variables)]
// TODO 32nd bit in the length can't be a 1
use crc::{Crc,CRC_32_ISO_HDLC};
use std::fmt;
use crate::chunk_type::{ChunkType,ChunkTypeError};
use thiserror::Error;
pub struct Chunk {
    length : u32,
    ctype: ChunkType,
    data: Vec<u8>,
    crc: u32
}
#[derive(Error, Debug)]
pub enum ChunkError {
    #[error("The chunk is too short")]
    Short,
    #[error("Chunk header says the chunk is of length {0}, but it's {1}")]
    Length(u32, usize),
    #[error("The crc isn't equal to the crc of the chunk type + data")]
    Crc,
    #[error("Invalid chunk type: {0}")]
    ChunkType(#[from] ChunkTypeError),
}
impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            return Err(ChunkError::Short)
        }
        let (length_bytes, rest) = bytes.split_at(4);
        let length = u32::from_be_bytes(length_bytes.try_into().unwrap());
        let (ctype_bytes, rest) = rest.split_at(4);
        if length != rest.len() as u32 - 4 {
            return Err(ChunkError::Length(length, rest.len()));
        }
        let (data_bytes, crc_bytes) = rest.split_at(rest.len() - 4);

        let ctype: ChunkType = ChunkType::try_from(<[u8; 4]>::try_from(ctype_bytes).unwrap())?;
        let data: Vec<u8> = data_bytes.try_into().unwrap();
        let crc = u32::from_be_bytes(crc_bytes.try_into().unwrap());

        if crc != Chunk::CRC_32.checksum(&[&ctype.bytes(), data.as_slice()].concat()) {
            return Err(ChunkError::Crc);
        }
        return Ok(Chunk { length, ctype, data, crc})
    }
}
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk of length {} and ctype {}\nCRC = {}", self.length, self.ctype, self.crc)
    }
}
#[allow(dead_code)]
impl Chunk {
    const BIT_32: u32 = 0b1000_0000_0000_0000;
    const CRC_32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    // const CRC 
    fn new(ctype: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = Self::CRC_32.checksum(&[&ctype.bytes(), data.as_slice()].concat());
        return Chunk{length: data.len() as u32, ctype, data, crc}
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.ctype
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}