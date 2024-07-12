mod executor;
mod hooks;
mod intrinsic;
mod path_selection;
mod project;
mod state;
mod vm;

pub use executor::*;
pub use hooks::*;
pub use intrinsic::*;
pub use path_selection::*;
pub use project::*;
pub use state::*;
pub use vm::*;

use crate::{memory::MemoryError, smt::SolverError};

/// Errors that can occur during analysis.
///
/// These errors are not related to the VM/Executor, but to the analysis itself.
/// The error path of a `Result` will never contain an `AnalysisError`, those
/// are reservered for errors related to the execution of the VM/Executor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnalysisError {
    // CallDepthExceeded,
    // IterationCountExceeded,
    // NoPath,
    Panic,
    Unreachable,
}

pub type Result<T> = std::result::Result<T, LLVMExecutorError>;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum LLVMExecutorError {
    #[error("Abort {0}")]
    Abort(i64),

    /// Function not found
    #[error("Function not found: {0}")]
    FunctionNotFound(String),

    /// Local register variable not found.
    #[error("Local not found: {0}")]
    LocalNotFound(String),

    #[error("Cannot take size of type")]
    NoSize,

    /// MalformedInstruction
    #[error("MalformedInstruction")]
    MalformedInstruction,

    /// UnsupportedInstruction
    #[error("UnsupportedInstruction {0}")]
    UnsupportedInstruction(String),

    #[error("UnexpectedZeroSize")]
    UnexpectedZeroSize,

    #[error("No active stack frame")]
    NoStackFrame,

    #[error("Memory error")]
    MemoryError(#[from] MemoryError),

    #[error("Solver error")]
    SolverError(#[from] SolverError),
}

// /// Errors why a certain path failed.
// ///
// /// Indiviual errors from the specific VM/Executors should be converted to
// this more general error #[derive(Debug, thiserror::Error, PartialEq)]
// pub enum VMError {
//     #[error("{}", UNEXPECTED_PARAMETER_MESSAGE)]
//     UnexpectedParameter,

//     #[error("Abort {0}")]
//     Abort(i64),

//     #[error("SolverError")]
//     SolverError(#[from] SolverError),

//     #[error("MemoryError")]
//     MemoryError(#[from] MemoryError),

//     #[error("Other {0}")]
//     Other(String),
// }

// const UNEXPECTED_PARAMETER_MESSAGE: &str = r#"Parameters for functions are
// not supported.

// Function parameters are not supported by the system, wrap the function inside
// another that takes not parameters.

// use symbolic_lib::symbolic;
// fn function_under_test(a: [i32; 3]) {}
// fn wrapped() {
//     let mut a = [0; 3];
//     symbolic(&mut a);
//     function_under_test(a);
// }

// Note that returning larger values may also result in the compiler generating
// code that takes the return value as a parameter instead. "#;
