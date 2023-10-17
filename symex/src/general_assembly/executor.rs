//! General assembly executor

use crate::smt::DExpr;

use super::{project::Project, state::GAState, vm::VM, Result};

pub struct GAExecutor<'vm> {
    pub vm: &'vm mut VM,
    pub state: GAState,
    pub project: &'static Project,
}

pub enum PathResult {
    Success(Option<DExpr>),
    Faliure,
    AssumptionUnsat,
    Suppress,
}

impl<'vm> GAExecutor<'vm> {
    pub fn from_state(state: GAState, vm: &'vm mut VM, project: &'static Project) -> Self {
        Self { vm, state, project }
    }

    pub fn resume_execution(&mut self) -> Result<PathResult> {
        todo!()
    }
}
