//! `(unplaced)/run_00a85580` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the destructor puts back (`01069b14`; its RTTI type descriptor
/// is `.?AVNiSkinInstance@@`).
const VTABLE_NI_SKIN_INSTANCE: u32 = 0x0106_9b14;
/// `00aa10f0`: frees the block at `this + 0x14` (`cdecl`, one word).
const FREE_00AA10F0: u32 = 0x00aa_10f0;
/// The global object (with a vtable) told about the destroyed instance
/// through its slot `0x104`, when there is one.
const GLOBAL_011F4748: u32 = 0x011f_4748;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00a5d3d0`: the base-class destructor the function ends in (`thiscall`).
const BASE_DESTRUCTOR: u32 = 0x00a5_d3d0;

/// Releases the reference-counted object in `field` (an `NiPointer`): when
/// the decremented count (`+4`) is zero, its virtual slot 4 deletes it.
fn release(e: &mut Engine, field: u32) {
    let object = e.mem.u32(field);
    if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00a85580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiSkinInstance` destructor body (the vtable it restores is
/// `NiSkinInstance`'s): frees the block at `+0x14` (`00aa10f0`), tells the
/// object at `011f4748` (slot `0x104`, `this`) when there is one, releases
/// the references at `+0x0c` and `+0x08`, then runs the base destructor
/// (`00a5d3d0`).
pub fn fn_00a85580(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let block = e.mem.u32(base + 0x14);
    e.mem.set_u32(base, VTABLE_NI_SKIN_INSTANCE);
    e.call(FREE_00AA10F0, &args![block]);
    let listener = e.global::<u32>(GLOBAL_011F4748);
    if listener != 0 {
        e.vcall(listener, 0x104, &args![base]);
    }
    release(e, base + 0x0c);
    release(e, base + 0x08);
    e.call(BASE_DESTRUCTOR, &args![base]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_5580, fn_00a85580(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn ref_object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + 4, 0x7000_0004);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn destructor_frees_tells_releases_and_chains() {
        let mut e = Engine::new();
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        e.register(0x7000_0004, |_, _| Ret::default());
        e.register(0x7000_0104, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let listener = e.mem.alloc(4);
        let listener_vtable = e.mem.alloc(0x108);
        e.mem.set_u32(listener, listener_vtable);
        e.mem.set_u32(listener_vtable + 0x104, 0x7000_0104);
        e.map(GLOBAL_011F4748, 4);
        e.set_global(GLOBAL_011F4748, listener);
        let this = e.mem.alloc(0x40);
        let last = ref_object(&mut e, 1);
        let shared = ref_object(&mut e, 2);
        e.mem.set_u32(this + 0x0c, last);
        e.mem.set_u32(this + 0x08, shared);
        e.mem.set_u32(this + 0x14, 0xabc0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_5580, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_NI_SKIN_INSTANCE);
        assert_eq!(calls_to(&e, FREE_00AA10F0), vec![vec![0xabc0]]);
        assert_eq!(calls_to(&e, 0x7000_0104), vec![vec![listener, this]]);
        assert_eq!(calls_to(&e, 0x7000_0004), vec![vec![last]]);
        assert_eq!(e.mem.u32(shared + 4), 1);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR), vec![vec![this]]);

        // No listener, no references.
        e.set_global(GLOBAL_011F4748, 0u32);
        e.mem.set_u32(this + 0x0c, 0);
        e.mem.set_u32(this + 0x08, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_5580, &args![this]);
        assert!(calls_to(&e, 0x7000_0104).is_empty());
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR).len(), 1);
    }
}
