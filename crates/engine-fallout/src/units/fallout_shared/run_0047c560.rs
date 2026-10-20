//! `fallout shared/run_0047c560` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Slots in the 0x98-byte component array (`FormComponentCollection::pFCArray`,
/// Xbox PDB: `BaseFormComponent*[38]`).
const SLOTS: u32 = 0x26;

/// Calls vtable slot `slot` of every non-null component pointer of the
/// array at `array`, slot 0 first.
fn each_component(e: &mut Engine, array: Ptr, slot: u32) {
    for i in 0..SLOTS {
        let component = e.mem.u32(array.addr() + 4 * i);
        if component != 0 {
            e.vcall(component, slot, &args![]);
        }
    }
}

// Translated from 0047c560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs vtable slot +0x00 (`BaseFormComponent::InitializeDataComponent`,
/// Xbox PDB) on each non-null pointer of the 38-slot component array at
/// `this` (`ECX`, no stack words). The engine map gives it no name.
pub fn fn_0047c560(e: &mut Engine, this: Ptr) {
    each_component(e, this, 0x0);
}

// Translated from 0047c5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs vtable slot +0x04 (`BaseFormComponent::ClearDataComponent`, Xbox
/// PDB) on each non-null pointer of the 38-slot component array at `this`
/// (`ECX`, no stack words). The engine map gives it no name.
pub fn fn_0047c5b0(e: &mut Engine, this: Ptr) {
    each_component(e, this, 0x4);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_c560, fn_0047c560(Ptr)),
        entry!(0x0047_c5b0, fn_0047c5b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0200_0000;
    const INIT: u32 = 0x0300_0000;
    const CLEAR: u32 = 0x0300_0010;

    /// An array with components in slots 0, 5 and 37 (the last).
    fn setup() -> (Engine, Ptr, [u32; 3]) {
        let mut e = Engine::new();
        e.put_vtable(VTABLE, &[INIT, CLEAR]);
        e.register(INIT, |_, _| Ret::default());
        e.register(CLEAR, |_, _| Ret::default());
        let array: Ptr = Ptr::new(e.mem.alloc(0x98));
        let mut components = [0u32; 3];
        for (n, slot) in [0u32, 5, 37].into_iter().enumerate() {
            let c = e.mem.alloc(4);
            e.mem.set_u32(c, VTABLE);
            e.mem.set_u32(array.addr() + 4 * slot, c);
            components[n] = c;
        }
        e.call_log = Some(vec![]);
        (e, array, components)
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn initializes_each_present_component() {
        let (mut e, array, c) = setup();
        e.call(0x0047_c560, &args![array]);
        assert_eq!(calls_to(&e, INIT), vec![vec![c[0]], vec![c[1]], vec![c[2]]]);
        assert!(calls_to(&e, CLEAR).is_empty());
    }

    #[test]
    fn clears_each_present_component() {
        let (mut e, array, c) = setup();
        e.call(0x0047_c5b0, &args![array]);
        assert_eq!(
            calls_to(&e, CLEAR),
            vec![vec![c[0]], vec![c[1]], vec![c[2]]]
        );
        assert!(calls_to(&e, INIT).is_empty());
    }
}
