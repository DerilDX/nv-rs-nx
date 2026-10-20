//! `fallout shared/pathfinding/run_006f31f0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The array's "full" test (`00438b90`, `thiscall`, `bool` in `AL`), its
/// next-capacity rule (`009a3910`, `thiscall`) and its resize (`009f6540`,
/// `thiscall (this, capacity, count)`, `RET 8`); the engine map has no
/// names for them.
const IS_FULL: u32 = 0x0043_8b90;
const NEXT_CAPACITY: u32 = 0x009a_3910;
const RESIZE: u32 = 0x009f_6540;

// Translated from 006f31f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes room for one more element of a growable array (data +4, count
/// +8, capacity +0xC; the engine map has no name) and answers its index:
/// when the array is full, an empty one allocates 4 elements (vtable +4
/// with 4, the answer stored as the data) and a non-empty one is resized
/// to the next capacity (`009a3910`, `009f6540`); then the count grows by
/// one and the old count is answered.
pub fn fn_006f31f0(e: &mut Engine, this: Ptr) -> u32 {
    let array = this.addr();
    if e.call(IS_FULL, &args![this]).bool() {
        if e.mem.u32(array + 0xc) == 0 {
            let capacity = 4u32;
            let data = e.vcall(array, 4, &args![capacity]).u32();
            e.mem.set_u32(array + 4, data);
            e.mem.set_u32(array + 0xc, capacity);
        } else {
            let capacity = e.call(NEXT_CAPACITY, &args![this]).u32();
            let count = e.mem.u32(array + 8);
            e.call(RESIZE, &args![this, capacity, count]);
            e.mem.set_u32(array + 0xc, capacity);
        }
    }
    let count = e.mem.u32(array + 8).wrapping_add(1);
    e.mem.set_u32(array + 8, count);
    e.mem.u32(array + 8).wrapping_sub(1)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006f_31f0, fn_006f31f0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_grows_a_full_array_and_answers_the_new_index() {
        const ALLOCATE: u32 = 0x0300_0000;
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, a| (0x9000 + a[1]).into_ret());
        e.register(NEXT_CAPACITY, |e, a| (2 * e.mem.u32(a[0] + 0xc)).into_ret());
        e.register(RESIZE, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, ALLOCATE);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        // Not full: only the count.
        e.register(IS_FULL, |_, _| Ret::default());
        e.mem.set_u32(this + 8, 2);
        assert_eq!(e.call(0x006f_31f0, &args![this]).u32(), 2);
        assert_eq!(e.mem.u32(this + 8), 3);
        // Full and empty: four elements allocated.
        e.register(IS_FULL, |_, _| 1u32.into_ret());
        e.mem.set_u32(this + 8, 0);
        assert_eq!(e.call(0x006f_31f0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 0x9004);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        // Full with a capacity: resized to the next one.
        e.mem.set_u32(this + 8, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_31f0, &args![this]).u32(), 4);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == RESIZE && call.1 == vec![this, 8, 4]));
    }
}
