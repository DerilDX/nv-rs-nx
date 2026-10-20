//! `(unplaced)/run_00e98aa0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedDecrement` / `InterlockedIncrement` (import slots).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

/// The inlined `NiPointer` release: `InterlockedDecrement` of the count at
/// +4, `DeleteThis` (vtable slot +4) at zero.
fn release(e: &mut Engine, object: u32) {
    if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 0x4, &[]);
    }
}

// Translated from 00e98aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`) on two
/// reference-counted pointers at +0 and +4: the first becomes `object` (the
/// old one released, the new one's count raised, nothing when they are the
/// same); the second is released and cleared.
pub fn fn_00e98aa0(e: &mut Engine, this: Ptr, object: u32) {
    let this = this.addr();
    let old = e.mem.u32(this);
    if old != object {
        release(e, old);
        e.mem.set_u32(this, object);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
        }
    }
    let second = e.mem.u32(this + 4);
    if second != 0 {
        release(e, second);
        e.mem.set_u32(this + 4, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e98aa0, fn_00e98aa0(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE: u32 = 0x0019_0004;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(0x0019_0000, &[0, DELETE]);
        e
    }

    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0019_0000);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn swaps_the_first_and_drops_the_second() {
        let mut e = engine();
        let old = counted(&mut e, 1);
        let second = counted(&mut e, 2);
        let new = counted(&mut e, 1);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, old);
        e.mem.set_u32(this + 4, second);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8aa0, &args![this, new]);
        assert_eq!(e.mem.u32(this), new);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(second + 4), 1);
        let deleted: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == DELETE)
            .map(|c| c.1)
            .collect();
        assert_eq!(deleted, vec![vec![old]]);
    }

    #[test]
    fn same_object_keeps_its_count() {
        let mut e = engine();
        let object = counted(&mut e, 1);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, object);
        e.call(0x00e9_8aa0, &args![this, object]);
        assert_eq!(e.mem.u32(object + 4), 1);
        e.call(0x00e9_8aa0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(object + 4), 0);
    }
}
