//! Simple runner that starts symbolic execution on LLVM bitcode.
//!
//!
use std::time::Instant;

use regex::Regex;
use tracing::{debug, info};

use crate::{
    elf_util::{ErrorReason, PathStatus, VisualPathResult},
    general_assembly::{self, executor::PathResult, project::PCHook, GAError},
    smt::DContext,
};

#[derive(Debug)]
pub struct RunConfig {
    /// Which paths should the solver be invoked on.
    pub solve_for: SolveFor,

    /// If concretized inputs should be shown.
    pub solve_inputs: bool,

    /// If concretized values should be displayed for variables passed to `symbolic`.
    pub solve_symbolics: bool,

    /// If concretized output values should be shown.
    pub solve_output: bool,
}

// impl RunConfig {
//     /// Determine if the solver should be invoked this specific result.
//     ///
//     /// Returns true of all paths should be solved, or if the result variant matches the given
//     /// `SolveFor`.
//     fn should_solve(&self, result: &vm::PathResult) -> bool {
//         match self.solve_for {
//             SolveFor::All => true,
//             SolveFor::Error => matches!(result, vm::PathResult::Success(_)),
//             SolveFor::Success => matches!(result, vm::PathResult::Failure(_)),
//         }
//     }
// }

/// Determine for which types of paths the solver should be invoked on.
#[derive(Debug)]
pub enum SolveFor {
    /// All paths.
    All,

    /// Paths which return errors. Currently this is both internal executor errors and program errors.
    Error,

    /// Paths which are sucessful.
    Success,
}

/// Run symbolic execution on a elf file where `path` is the path to the ELF file and
/// `function` is the function the execution starts at.
pub fn run_elf(
    path: &str,
    function: &str,
    // _cfg: &RunConfig,
) -> Result<Vec<VisualPathResult>, GAError> {
    let context = Box::new(DContext::new());
    let context = Box::leak(context);

    let end_pc = 0xFFFFFFFE;

    let hooks = vec![
        (Regex::new(r"^panic$").unwrap(), PCHook::EndFaliure("panic")),
        (
            Regex::new(r"^panic_cold_explicit$").unwrap(),
            PCHook::EndFaliure("explicit panic"),
        ),
        (
            Regex::new(r"^panic_bounds_check$").unwrap(),
            PCHook::EndFaliure("bounds check panic"),
        ),
        (Regex::new(r"^suppress_path$").unwrap(), PCHook::Suppress),
        (
            Regex::new(r"^unreachable_unchecked$").unwrap(),
            PCHook::EndFaliure("reach a unreachable unchecked call undefined behavior"),
        ),
    ];

    let project = Box::new(general_assembly::project::Project::from_path(path, hooks)?);
    let project = Box::leak(project);
    project.add_pc_hook(end_pc, PCHook::EndSuccess);
    debug!("Created project: {:?}", project);

    info!("create VM");
    let mut vm = general_assembly::vm::VM::new(project, context, function, end_pc)?;

    run_elf_paths(&mut vm)
}

/// Runs all paths in the vm
fn run_elf_paths(vm: &mut general_assembly::vm::VM) -> Result<Vec<VisualPathResult>, GAError> {
    let mut path_num = 0;
    let start = Instant::now();
    let mut path_results = vec![];
    while let Some((path_result, state)) = vm.run()? {
        if matches!(path_result, PathResult::Suppress) {
            debug!("Suppressing path");
            continue;
        }
        if matches!(path_result, PathResult::AssumptionUnsat) {
            println!("Encountered an unsatisfiable assumption, ignoring this path");
            continue;
        }

        path_num += 1;

        let v_path_result = match path_result {
            general_assembly::executor::PathResult::Success(_v) => PathStatus::Ok(None),
            general_assembly::executor::PathResult::Faliure(reason) => {
                PathStatus::Failed(ErrorReason {
                    error_message: reason.to_owned(),
                })
            }
            general_assembly::executor::PathResult::AssumptionUnsat => todo!(),
            general_assembly::executor::PathResult::Suppress => todo!(),
        };

        let result = VisualPathResult::from_state(state, path_num, v_path_result)?;
        println!("{}", result);
        path_results.push(result);
    }
    println!("time: {:?}", start.elapsed());
    Ok(path_results)
}
