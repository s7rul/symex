use crate::memory::MemoryError;

use self::project::ProjectError;

pub mod instruction;
pub mod path_selection;
pub mod project;
pub mod state;
pub mod translator;
pub mod translators;
pub mod vm;

pub type Result<T> = std::result::Result<T, GAError>;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum GAError {
    #[error("Project error: {0}")]
    ProjectError(#[from] ProjectError),

    #[error("Memmory error: {0}")]
    MemmoryError(#[from] MemoryError),

    #[error("Entry function {0} not found.")]
    EntryFunctionNotFound(String),

    #[error("Writing to static memory not permited.")]
    WritingToStaticMemoryProhibited,
}

#[derive(Debug, Clone, Copy)]
pub enum WordSize {
    Bit64,
    Bit32,
    Bit16,
    Bit8,
}

#[derive(Debug, Clone, Copy)]
pub enum DataWord {
    Word64(u64),
    Word32(u32),
    Word16(u16),
    Word8(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum RawDataWord {
    Word64([u8; 8]),
    Word32([u8; 4]),
    Word16([u8; 2]),
    Word8([u8; 1]),
}

#[derive(Debug, Clone, Copy)]
pub enum DataHalfWord {
    HalfWord64(u32),
    HalfWord32(u16),
    HalfWord16(u8),
}

#[derive(Debug)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, Clone)]
pub struct Config {
    /// Maximum call stack depth. Default is `1000`.
    pub max_call_depth: usize,

    /// Maximum iteration count. Default is `1000`.
    pub max_iter_count: usize,

    /// Maximum amount of concretizations for function pointers. Default is `1`.
    pub max_fn_ptr_resolutions: usize,

    /// Maximum amount of concretizations for a memory address. This does not apply for e.g.
    /// ArrayMemory, but does apply for ObjectMemory. Default is `100`.
    pub max_memory_access_resolutions: usize,

    /// Maximum amount of concretizations for memmove, memcpy, memset and other intrisic functions.
    /// Default is `100`.
    pub max_intrinsic_concretizations: usize,
}
