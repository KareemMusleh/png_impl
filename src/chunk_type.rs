#![allow(unused_variables)]
use std::fmt;
use std::str::FromStr;
use thiserror::Error;
#[derive (PartialEq, Eq, Debug, Clone)]
pub struct ChunkType {
    bytes: [u8;4]
}
impl ChunkType {
    const BIT_6: u8 = 0b0010_0000;
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes.clone()
    }
    pub fn is_valid(&self) -> bool {
        (self.bytes[2] & ChunkType::BIT_6) == 0
    }
    pub fn is_critical(&self) -> bool {
        println!("{} {} {}", std::str::from_utf8(&self.bytes).unwrap(), self.bytes[0], (self.bytes[0] & ChunkType::BIT_6));
        (self.bytes[0] & ChunkType::BIT_6) == 0
    } 
    pub fn is_public(&self) -> bool {
        (self.bytes[1] & ChunkType::BIT_6) == 0
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.bytes[2] & ChunkType::BIT_6) == 0
    }
    pub fn is_safe_to_copy(&self) -> bool {
        (self.bytes[3] & ChunkType::BIT_6) != 0
    }
}
#[derive(Error, Debug)]
pub enum ChunkTypeError {
    #[error("Bad Length {0}: expected 4")]
    NonAlpha(u8),
    #[error("Not Ascii Alphabetic {0}: ({0:b}")]
    BadLength(usize)
}

impl TryFrom<[u8;4]> for ChunkType {
    type Error = ChunkTypeError;
    fn try_from(bytes: [u8;4]) -> Result<Self, Self::Error> {
        if let Some(byte) = bytes.iter().find(|&x| !x.is_ascii_alphabetic()) {
            Err(ChunkTypeError::NonAlpha(*byte))
        } else {
            Ok(ChunkType{bytes})
        }
    }
}
impl FromStr for ChunkType {
    type Err = ChunkTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(ChunkTypeError::BadLength(s.len()))
        }
        if let Some(byte) = s.bytes().find(|c| !c.is_ascii_alphabetic()) {
            Err(ChunkTypeError::NonAlpha(byte))
        } else {
            let bytes: [u8;4] = s.as_bytes().try_into().unwrap();
            Ok(ChunkType{bytes})
        }
    }
}
impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.bytes).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
