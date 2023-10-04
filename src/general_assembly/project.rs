use std::fmt::Display;

use super::{DataHalfWord, DataWord, Endianness, WordSize};

#[derive(Debug)]
pub enum Error {
    InvalidAddress,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidAddress => {
                write!(f, "Invalid Address: Address not in program memory range.")
            }
        }
    }
}

#[derive(Debug)]
pub struct Project {
    program_memory: Box<[u8]>,
    start_addr: u64,
    end_addr: u64,
    word_size: WordSize,
    endianness: Endianness,
}

impl Project {
    /// Get a byte of data from program memory.
    pub fn get_byte(&self, address: u64) -> Result<u8, Error> {
        if address >= self.start_addr && address <= self.end_addr {
            Ok(self.program_memory[(self.start_addr - address) as usize])
        } else {
            Err(Error::InvalidAddress)
        }
    }

    /// Get a word from data memory
    pub fn get_word(&self, address: u64) -> Result<DataWord, Error> {
        let mem = self.program_memory.as_ref();
        Ok(match self.word_size {
            WordSize::Bit64 => {
                let mut data = [0; 8];
                if address >= self.start_addr && (address + 7) <= self.end_addr {
                    data.copy_from_slice(&mem[address as usize..(address + 8) as usize]);

                    DataWord::Word64(u64::from_le_bytes(data))
                } else {
                    return Err(Error::InvalidAddress);
                }
            }
            WordSize::Bit32 => {
                let mut data = [0; 4];
                if address >= self.start_addr && (address + 3) <= self.end_addr {
                    data.copy_from_slice(&mem[address as usize..(address + 4) as usize]);

                    DataWord::Word32(u32::from_le_bytes(data))
                } else {
                    return Err(Error::InvalidAddress);
                }
            }
            WordSize::Bit16 => {
                let mut data = [0; 2];
                if address >= self.start_addr && (address + 1) <= self.end_addr {
                    data.copy_from_slice(&mem[address as usize..(address + 2) as usize]);

                    DataWord::Word16(u16::from_le_bytes(data))
                } else {
                    return Err(Error::InvalidAddress);
                }
            }
            WordSize::Bit8 => DataWord::Word8(self.get_byte(address)?),
        })
    }

    pub fn get_half_word(&self, address: u64) -> Result<DataHalfWord, Error> {
        todo!()
    }
}
