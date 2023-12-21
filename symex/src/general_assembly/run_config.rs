//! Struct to configure the symbolic execution.

use regex::Regex;

use super::project::{
    MemoryHookAddress, MemoryReadHook, MemoryWriteHook, PCHook, RegisterReadHook, RegisterWriteHook,
};

// Configures a symbolic execution run.
pub struct RunConfig {
    pub pc_hooks: Vec<(Regex, PCHook)>,
    pub register_read_hooks: Vec<(String, RegisterReadHook)>,
    pub register_write_hooks: Vec<(String, RegisterWriteHook)>,
    pub memory_write_hooks: Vec<(MemoryHookAddress, MemoryWriteHook)>,
    pub memory_read_hooks: Vec<(MemoryHookAddress, MemoryReadHook)>,
}
