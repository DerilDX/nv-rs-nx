//! `fallout/misc/run_00878860` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Reads the word at its `this` (`00559450`, thiscall, no stack words).
const READ_POINTER: u32 = 0x0055_9450;
/// Reads the word at +0x0c of its `this` (`0084e3a0`, thiscall; the form id
/// when `this` is a form).
const FORM_ID: u32 = 0x0084_e3a0;
/// The pointer holder the function reads (+0x10 of `this`).
const HOLDER: u32 = 0x10;

// Translated from 00878f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form id of the object held at +0x10 (unnamed in the engine map): the
/// pointer is read through `00559450(this + 0x10)`; null gives 0, otherwise
/// the pointer is read again and `0084e3a0` gives the word at its +0x0c.
pub fn fn_00878f20(e: &mut Engine, this: Ptr) -> u32 {
    let holder = this.addr() + HOLDER;
    if e.call(READ_POINTER, &args![holder]).u32() == 0 {
        return 0;
    }
    let form = e.call(READ_POINTER, &args![holder]).u32();
    e.call(FORM_ID, &args![form]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0087_8f20, fn_00878f20(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_the_held_forms_id_or_zero() {
        let mut e = Engine::new();
        e.register(READ_POINTER, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(FORM_ID, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x0c),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_8f20, &args![this]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|(a, _)| *a == READ_POINTER).count(), 1);
        assert!(!log.iter().any(|(a, _)| *a == FORM_ID));

        let form = e.mem.alloc(0x18);
        e.mem.set_u32(form + 0x0c, 0x0001_2345);
        e.mem.set_u32(this + HOLDER, form);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_8f20, &args![this]).u32(), 0x0001_2345);
        let log = e.call_log.take().unwrap();
        let reads: Vec<_> = log.iter().filter(|(a, _)| *a == READ_POINTER).collect();
        assert_eq!(reads.len(), 2);
        assert!(reads.iter().all(|(_, w)| *w == vec![this + HOLDER]));
        let ids: Vec<_> = log.iter().filter(|(a, _)| *a == FORM_ID).collect();
        assert_eq!(ids[0].1, vec![form]);
    }
}
