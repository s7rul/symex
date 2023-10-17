//! Descrebes the VM for general assembly

use crate::{
    general_assembly::{path_selection::Path, state::GAState},
    smt::{DContext, DSolver},
};

use super::{
    executor::{self, GAExecutor},
    path_selection::DFSPathSelection,
    project::Project,
    Config, GAError, Result,
};

#[derive(Debug)]
pub struct VM {
    pub project: &'static Project,
    pub paths: DFSPathSelection,
}

impl VM {
    pub fn new(project: &'static Project, ctx: &'static DContext, fn_name: &str) -> Result<Self> {
        let entry_addr = match project.get_symbol_address(fn_name) {
            Some(addr) => addr,
            None => return Err(GAError::EntryFunctionNotFound(fn_name.to_owned())),
        };

        let mut vm = Self {
            project,
            paths: DFSPathSelection::new(),
        };

        let solver = DSolver::new(ctx);
        let mut state = GAState::new(ctx, project, solver, fn_name)?;

        vm.paths.save_path(Path::new(state, None));

        Ok(vm)
    }

    pub fn run(&mut self) -> Result<Option<GAState>> {
        while let Some(path) = self.paths.get_path() {
            // try stuff
            let next_instruction = path.state.get_next_instruction().unwrap();
            let mut executor = GAExecutor::from_state(path.state, self, self.project);

            for constraint in path.constraints {
                executor.state.constraints.assert(&constraint);
            }
        }
        Ok(None)
    }
}
