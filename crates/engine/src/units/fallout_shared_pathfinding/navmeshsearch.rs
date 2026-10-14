//! `fallout shared/pathfinding/navmeshsearch.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The object's vtable slot that releases what +4 holds.
const SLOT_RELEASE: u32 = 0x8;

// Translated from 006a8500 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Releases what the object holds at +4 through its own vtable slot +8
/// (`thiscall`, the held word its argument), then clears +4.
pub fn fn_006a8500(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr() + 4);
    e.vcall(this.addr(), SLOT_RELEASE, &args![held]);
    e.mem.set_u32(this.addr() + 4, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006a8500, fn_006a8500(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn releases_the_held_word_through_slot_8_then_clears_it() {
        let mut e = Engine::new();
        let o = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0, 0x0090_1000]);
        e.mem.set_u32(o, 0x0090_0000);
        e.mem.set_u32(o + 4, 0x1234);
        e.register(0x0090_1000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006a8500, &[o]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0090_1000, vec![o, 0x1234]));
        assert_eq!(e.mem.u32(o + 4), 0);
    }
}
