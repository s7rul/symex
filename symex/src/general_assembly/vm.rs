//! Descrebes the VM for general assembly

use super::{project::Project, Config};

#[derive(Debug)]
pub struct VM {
    pub project: &'static Project,
}
