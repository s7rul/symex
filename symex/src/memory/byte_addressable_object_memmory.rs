//! A object memmory that is fully byte addressable.

use std::collections::BTreeMap;

use crate::{
    memory::to_bytes,
    smt::{DContext, DExpr, DSolver},
};

use super::MemoryError;

#[derive(Debug)]
struct MemmoryObject {
    address: u64,
    size: u64,
    bv: DExpr,
}

impl MemmoryObject {
    pub fn bit_size(&self) -> u64 {
        self.size
    }
}

pub struct ByteAddressableObjectMemmory {
    ctx: &'static DContext,
    objects: BTreeMap<u64, MemmoryObject>,
    solver: DSolver,
    ptr_size: u32,
}

impl ByteAddressableObjectMemmory {
    pub fn new(ctx: &'static DContext, ptr_size: u32, solver: DSolver) -> Self {
        Self {
            ctx,
            objects: BTreeMap::new(),
            solver,
            ptr_size,
        }
    }

    pub fn read(&self, address: u64, bits: u32) -> Result<DExpr, MemoryError> {
        todo!()
    }

    pub fn write(&mut self, address: u64, data: DExpr, bits: u32) -> Result<(), MemoryError> {
        todo!()
    }

    fn get_affected_objects(
        &self,
        address: u64,
        bits: u32,
    ) -> Result<Vec<(u64, MemmoryObject)>, MemoryError> {
        // search equal or lower
        let mut ret = vec![];
        for obj in self.objects.range(0..address).rev() {
            // check if object in reach of read address.
            if obj.0 + (to_bytes(obj.1.bit_size())? - 1) >= address {
                todo!()
            }
            todo!()
        }
        Ok(ret)
    }
}
