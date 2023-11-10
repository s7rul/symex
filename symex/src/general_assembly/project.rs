use std::{collections::HashMap, fmt::Debug, fs};

use armv6_m_instruction_parser::parse;
use gimli::{DebugAbbrev, DebugInfo, DebugPubNames};
use object::{Object, ObjectSection, ObjectSymbol};
use tracing::debug;

use crate::{general_assembly::translator::Translatable, memory::MemoryError};

use super::{instruction::Instruction, DataHalfWord, DataWord, Endianness, RawDataWord, WordSize};

mod dwarf_helper;
use dwarf_helper::*;

type Result<T> = std::result::Result<T, ProjectError>;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ProjectError {
    #[error("Unable to parse elf file: {0}")]
    UnableToParseElf(String),

    #[error("Program memmory error")]
    ProgramMemmoryError(#[from] MemoryError),

    #[error("Unavalable operation")]
    UnabvalableOperation,
}

#[derive(Debug, Clone, Copy)]
pub enum PCHook {
    Continue,
    EndSuccess,
    EndFaliure,
    Suppress,
}

pub type PCHooks = HashMap<u64, PCHook>;

/// Holds all data read from the ELF file.
// Add all read only memmory here later to handle global constants.
pub struct Project {
    program_memory: Vec<u8>,
    start_addr: u64,
    end_addr: u64,
    word_size: WordSize,
    endianness: Endianness,
    architecture: object::Architecture,
    symtab: HashMap<String, u64>,
    pc_hooks: PCHooks,
}

impl Project {
    pub fn manual_project(
        program_memory: Vec<u8>,
        start_addr: u64,
        end_addr: u64,
        word_size: WordSize,
        endianness: Endianness,
        architecture: object::Architecture,
        symtab: HashMap<String, u64>,
        pc_hooks: PCHooks,
    ) -> Project {
        Project {
            program_memory,
            start_addr,
            end_addr,
            word_size,
            endianness,
            architecture,
            symtab,
            pc_hooks,
        }
    }
    pub fn from_path(path: &str, pc_hooks: Vec<(&str, PCHook)>) -> Result<Self> {
        debug!("Parsing elf file: {}", path);
        let file = fs::read(path).expect("Unable to open file.");
        let obj_file = match object::File::parse(&*file) {
            Ok(x) => x,
            Err(e) => {
                debug!("Error: {}", e);
                return Err(ProjectError::UnableToParseElf(path.to_owned()));
            }
        };

        let text_section = match obj_file.section_by_name(".text") {
            Some(section) => section,
            None => {
                return Err(ProjectError::UnableToParseElf(
                    ".text section not found.".to_owned(),
                ))
            }
        };

        let text_start = text_section.address();
        let text_data = match text_section.data() {
            Ok(data) => data.to_owned(),
            Err(_) => {
                return Err(ProjectError::UnableToParseElf(
                    "Unable to read .text section.".to_owned(),
                ))
            }
        };
        let text_end = text_start + text_section.size();

        let endianness = if obj_file.is_little_endian() {
            Endianness::Little
        } else {
            Endianness::Big
        };

        let architecture = obj_file.architecture();

        // Do not catch 16 or 8 bit architectures but will do for now.
        let word_size = if obj_file.is_64() {
            WordSize::Bit64
        } else {
            WordSize::Bit32
        };

        let mut symtab = HashMap::new();
        for symbol in obj_file.symbols() {
            symtab.insert(
                match symbol.name() {
                    Ok(name) => name.to_owned(),
                    Err(_) => continue, // ignore entry if name can not be read
                },
                symbol.address(),
            );
        }

        let gimli_endian = match endianness {
            Endianness::Little => gimli::RunTimeEndian::Little,
            Endianness::Big => gimli::RunTimeEndian::Big,
        };

        let debug_info = obj_file.section_by_name(".debug_info").unwrap();
        let debug_info = DebugInfo::new(debug_info.data().unwrap(), gimli_endian);

        let debug_pubnames = obj_file.section_by_name(".debug_pubnames").unwrap();
        let debug_pubnames = DebugPubNames::new(debug_pubnames.data().unwrap(), gimli_endian);

        let debug_abbrev = obj_file.section_by_name(".debug_abbrev").unwrap();
        let debug_abbrev = DebugAbbrev::new(debug_abbrev.data().unwrap(), gimli_endian);

        let pc_hooks = construct_pc_hooks(pc_hooks, &debug_pubnames, &debug_info, &debug_abbrev);

        debug!("Created pc hooks: {:?}", pc_hooks);

        Ok(Project {
            start_addr: text_start,
            end_addr: text_end,
            word_size,
            endianness,
            architecture,
            program_memory: text_data,
            symtab,
            pc_hooks,
        })
    }

    pub fn get_pc_hook(&self, pc: u64) -> Option<PCHook> {
        self.pc_hooks.get(&pc).copied()
    }

    pub fn add_pc_hook(&mut self, pc: u64, hook: PCHook) {
        self.pc_hooks.insert(pc, hook);
    }

    pub fn address_in_range(&self, address: u64) -> bool {
        address >= self.start_addr && address <= self.end_addr
    }

    pub fn get_word_size(&self) -> u32 {
        self.get_ptr_size() // same for now
    }

    pub fn get_endianness(&self) -> Endianness {
        self.endianness.clone()
    }

    pub fn get_ptr_size(&self) -> u32 {
        // This is an oversimplification and not true for some architectures
        // But will do and should map to the addresses in the elf
        match self.word_size {
            WordSize::Bit64 => 64,
            WordSize::Bit32 => 32,
            WordSize::Bit16 => 16,
            WordSize::Bit8 => 8,
        }
    }

    /// Get the address of a symbol from the ELF symbol table
    pub fn get_symbol_address(&self, symbol: &str) -> Option<u64> {
        self.symtab.get(symbol).copied()
    }

    /// Get the instruction att a address
    pub fn get_instruction(&self, address: u64) -> Result<Instruction> {
        debug!("Reading instruction from address: {:#010X}", address);
        match self.get_raw_word(address)? {
            RawDataWord::Word64(d) => self.instruction_from_array_ptr(&d),
            RawDataWord::Word32(d) => self.instruction_from_array_ptr(&d),
            RawDataWord::Word16(d) => self.instruction_from_array_ptr(&d),
            RawDataWord::Word8(_) => todo!(),
        }
    }

    fn instruction_from_array_ptr(&self, data: &[u8]) -> Result<Instruction> {
        match self.architecture {
            object::Architecture::Arm => {
                // probobly right add more cheks later or custom enum etc.
                let arm_instruction = parse(data).unwrap();
                debug!("instruction read: {:?}", arm_instruction);
                Ok(arm_instruction.translate())
            }
            _ => todo!(),
        }
    }

    /// Get a byte of data from program memory.
    pub fn get_byte(&self, address: u64) -> Result<u8> {
        if address >= self.start_addr && address <= self.end_addr {
            Ok(self.program_memory[(self.start_addr - address) as usize])
        } else {
            Err(MemoryError::OutOfBounds.into())
        }
    }

    fn get_word_internal(&self, address: u64, width: WordSize) -> Result<DataWord> {
        let mem: &[u8] = self.program_memory.as_ref();
        Ok(match width {
            WordSize::Bit64 => {
                let mut data = [0; 8];
                if address >= self.start_addr && (address + 7) <= self.end_addr {
                    let address = address - self.start_addr;
                    data.copy_from_slice(&mem[address as usize..(address + 8) as usize]);

                    DataWord::Word64(match self.endianness {
                        Endianness::Little => u64::from_le_bytes(data),
                        Endianness::Big => u64::from_be_bytes(data),
                    })
                } else {
                    return Err(MemoryError::OutOfBounds.into());
                }
            }
            WordSize::Bit32 => {
                let mut data = [0; 4];
                if address >= self.start_addr && (address + 3) <= self.end_addr {
                    let address = address - self.start_addr;
                    data.copy_from_slice(&mem[address as usize..(address + 4) as usize]);

                    DataWord::Word32(match self.endianness {
                        Endianness::Little => u32::from_le_bytes(data),
                        Endianness::Big => u32::from_be_bytes(data),
                    })
                } else {
                    return Err(MemoryError::OutOfBounds.into());
                }
            }
            WordSize::Bit16 => {
                let mut data = [0; 2];
                if address >= self.start_addr && (address + 1) <= self.end_addr {
                    let address = address - self.start_addr;
                    data.copy_from_slice(&mem[address as usize..(address + 2) as usize]);

                    DataWord::Word16(match self.endianness {
                        Endianness::Little => u16::from_le_bytes(data),
                        Endianness::Big => u16::from_be_bytes(data),
                    })
                } else {
                    return Err(MemoryError::OutOfBounds.into());
                }
            }
            WordSize::Bit8 => DataWord::Word8(self.get_byte(address)?),
        })
    }

    /// Get a word from data memory
    pub fn get_word(&self, address: u64) -> Result<DataWord> {
        self.get_word_internal(address, self.word_size)
    }

    pub fn get_half_word(&self, address: u64) -> Result<DataHalfWord> {
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
            WordSize::Bit8 => return Err(ProjectError::UnabvalableOperation),
        })
    }

    pub fn get_raw_word(&self, address: u64) -> Result<RawDataWord> {
        let mem: &[u8] = self.program_memory.as_ref();
        Ok(match self.word_size {
            WordSize::Bit64 => {
                let mut data = [0; 8];
                if address >= self.start_addr && (address + 7) <= self.end_addr {
                    let address = address - self.start_addr;
                    data.copy_from_slice(&mem[address as usize..(address + 8) as usize]);
                    RawDataWord::Word64(data)
                } else {
                    return Err(MemoryError::OutOfBounds.into());
                }
            }
            WordSize::Bit32 => {
                let mut data = [0; 4];
                if address >= self.start_addr && (address + 3) <= self.end_addr {
                    let address = address - self.start_addr;
                    data.copy_from_slice(&mem[address as usize..(address + 4) as usize]);
                    RawDataWord::Word32(data)
                } else {
                    return Err(MemoryError::OutOfBounds.into());
                }
            }
            WordSize::Bit16 => {
                let mut data = [0; 2];
                if address >= self.start_addr && (address + 1) <= self.end_addr {
                    let address = address - self.start_addr;
                    data.copy_from_slice(&mem[address as usize..(address + 2) as usize]);
                    RawDataWord::Word16(data)
                } else {
                    return Err(MemoryError::OutOfBounds.into());
                }
            }
            WordSize::Bit8 => RawDataWord::Word8([self.get_byte(address)?]),
        })
    }
}

impl Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Project")
            .field("start_addr", &self.start_addr)
            .field("end_addr", &self.end_addr)
            .field("word_size", &self.word_size)
            .field("endianness", &self.endianness)
            .field("architecture", &self.architecture)
            .finish()
    }
}
