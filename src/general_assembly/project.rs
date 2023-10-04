use std::fmt::Display;

use super::{DataHalfWord, DataWord, Endianness, WordSize};

#[derive(Debug)]
pub enum Error {
    InvalidAddress,
    UnavailableOperation,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidAddress => {
                write!(f, "Invalid Address: Address not in program memory range.")
            }
            Error::UnavailableOperation => {
                write!(f, "Operation not available for endianness or word size.")
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

    fn get_word_internal(&self, address: u64, width: WordSize) -> Result<DataWord, Error> {
        let mem = self.program_memory.as_ref();
        Ok(match width {
            WordSize::Bit64 => {
                let mut data = [0; 8];
                if address >= self.start_addr && (address + 7) <= self.end_addr {
                    data.copy_from_slice(&mem[address as usize..(address + 8) as usize]);

                    DataWord::Word64(match self.endianness {
                        Endianness::Little => u64::from_le_bytes(data),
                        Endianness::Big => u64::from_be_bytes(data),
                    })
                } else {
                    return Err(Error::InvalidAddress);
                }
            }
            WordSize::Bit32 => {
                let mut data = [0; 4];
                if address >= self.start_addr && (address + 3) <= self.end_addr {
                    data.copy_from_slice(&mem[address as usize..(address + 4) as usize]);

                    DataWord::Word32(match self.endianness {
                        Endianness::Little => u32::from_le_bytes(data),
                        Endianness::Big => u32::from_be_bytes(data),
                    })
                } else {
                    return Err(Error::InvalidAddress);
                }
            }
            WordSize::Bit16 => {
                let mut data = [0; 2];
                if address >= self.start_addr && (address + 1) <= self.end_addr {
                    data.copy_from_slice(&mem[address as usize..(address + 2) as usize]);

                    DataWord::Word16(match self.endianness {
                        Endianness::Little => u16::from_le_bytes(data),
                        Endianness::Big => u16::from_be_bytes(data),
                    })
                } else {
                    return Err(Error::InvalidAddress);
                }
            }
            WordSize::Bit8 => DataWord::Word8(self.get_byte(address)?),
        })
    }

    /// Get a word from data memory
    pub fn get_word(&self, address: u64) -> Result<DataWord, Error> {
        self.get_word_internal(address, self.word_size)
    }

    pub fn get_half_word(&self, address: u64) -> Result<DataHalfWord, Error> {
        Ok(match self.word_size {
            WordSize::Bit64 => match self.get_word_internal(address, WordSize::Bit32)? {
                DataWord::Word32(d) => DataHalfWord::HalfWord64(d),
                _ => panic!("Should never reach this part."),
            },
            WordSize::Bit32 => match self.get_word_internal(address, WordSize::Bit16)? {
                DataWord::Word16(d) => DataHalfWord::HalfWord32(d),
                _ => panic!("Should never reach this part."),
            },
            WordSize::Bit16 => match self.get_word_internal(address, WordSize::Bit8)? {
                DataWord::Word8(d) => DataHalfWord::HalfWord16(d),
                _ => panic!("Should never reach this part."),
            },
            WordSize::Bit8 => return Err(Error::UnavailableOperation),
        })
    }
}
