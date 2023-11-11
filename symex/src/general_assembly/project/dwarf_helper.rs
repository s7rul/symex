//! Helper functions to read dwarf debug data.

use std::collections::{HashMap, HashSet};

use gimli::{
    AttributeValue, DW_AT_low_pc, DW_TAG_subprogram, DebugAbbrev, DebugInfo, DebugPubNames, Reader,
};
use tracing::trace;

use super::{PCHook, PCHooks};

/// Constructs a list of address hook pairs from a list of symbol name hook pairs.
///
/// It does this by finding the name of the symbol in the dwarf debug data and if it is a function(subprogram)
/// it adds the address and hook to the hooks list.
pub fn construct_pc_hooks<R: Reader>(
    hooks: Vec<(&str, PCHook)>,
    pub_names: &DebugPubNames<R>,
    debug_info: &DebugInfo<R>,
    debug_abbrev: &DebugAbbrev<R>,
) -> PCHooks {
    trace!("Constructing PC hooks");
    let mut ret: PCHooks = HashMap::new();
    let mut name_items = pub_names.items();
    let mut found_hooks = HashSet::new();
    while let Some(pubname) = name_items.next().unwrap() {
        let item_name = pubname.name().to_string_lossy().unwrap();
        for (name, hook) in &hooks {
            if item_name.as_ref() == *name {
                let unit_offset = pubname.unit_header_offset();
                let die_offset = pubname.die_offset();

                let unit = debug_info.header_from_offset(unit_offset).unwrap();
                let abbrev = unit.abbreviations(debug_abbrev).unwrap();
                let die = unit.entry(&abbrev, die_offset).unwrap();

                let die_type = die.tag();
                if die_type == DW_TAG_subprogram {
                    found_hooks.insert(name);
                    let addr = die.attr_value(DW_AT_low_pc).unwrap().unwrap();

                    if let AttributeValue::Addr(addr_value) = addr {
                        trace!("found hook for {} att addr: {:#X}", name, addr_value);
                        ret.insert(addr_value, *hook);
                    }
                }
            }
        }
    }
    if found_hooks.len() < hooks.len() {
        println!("Did not find addresses for all hooks.") // fix a proper error here later
    }
    ret
}
