//! `BSShader/run_00baa660` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::with_allocation_context;

/// The record whose address `00baa630` answers (`01200508`).
const RECORD_01200508: u32 = 0x0120_0508;

// Translated from 00baa630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (the tall grass shader's vtable `010b8980` slot +8, where the Xbox
/// PDB's `NiObject` order has `GetRTTI`): answers the address `01200508`. No arguments.
pub fn fn_00baa630(_e: &mut Engine) -> Ptr {
    Ptr::new(RECORD_01200508)
}

/// `00e80570` (`thiscall`, no stack words; answers a bool in `AL`).
const INITIALIZE_00E80570: u32 = 0x00e8_0570;
/// The allocation context `00baa660` sets.
const ALLOCATION_CONTEXT_1C: u32 = 0x1c;

// Translated from 00baa660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the tall grass shader's vtable `010b8980` slot +0x4c), no stack
/// words. With the thread's allocation context (+0x2b4 of the thread block) at 0x1c (put back
/// at the end): runs `00e80570`; when it answers true, runs the shader's slots +0x12c, +0x134
/// and +0x130 and answers true, otherwise answers false.
// Translation incomplete: 00baa660 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
pub fn fn_00baa660(e: &mut Engine, this: Ptr) -> bool {
    let this = this.addr();
    with_allocation_context(e, ALLOCATION_CONTEXT_1C, |e| {
        if !e.call(INITIALIZE_00E80570, &args![this]).bool() {
            return false;
        }
        for slot in [0x12cu32, 0x134, 0x130] {
            e.vcall(this, slot, &args![]);
        }
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_a630, fn_00baa630() -> Ptr),
        entry!(0x00ba_a660, fn_00baa660(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOTS: u32 = 0x0e20_0000;
    const VTABLE: u32 = 0x0e21_0000;

    fn rig(answer: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let targets: Vec<u32> = (0..0x50u32).map(|i| SLOTS + 4 * i).collect();
        e.put_vtable(VTABLE, &targets);
        for slot in [0x12cu32, 0x134, 0x130] {
            e.register(SLOTS + slot, |_, _| Ret::default());
        }
        e.register_double(INITIALIZE_00E80570, move |e, _| {
            // The context is set while the shader initializes.
            let tls = e.tls();
            assert_eq!(e.mem.u32(tls + 0x2b4), ALLOCATION_CONTEXT_1C);
            answer.into_ret()
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, VTABLE);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 0x55);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_00baa630_answers_its_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ba_a630, &args![]).u32(), RECORD_01200508);
    }

    #[test]
    fn fn_00baa660_runs_the_three_slots_when_initialized() {
        let (mut e, this) = rig(0xffff_ff01);
        assert!(e.call(0x00ba_a660, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        let slots: Vec<u32> = log.iter().skip(2).map(|c| c.0).collect();
        assert_eq!(slots, vec![SLOTS + 0x12c, SLOTS + 0x134, SLOTS + 0x130]);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + 0x2b4), 0x55);
    }

    #[test]
    fn fn_00baa660_answers_false_and_restores_the_context() {
        let (mut e, this) = rig(0x100);
        assert!(!e.call(0x00ba_a660, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + 0x2b4), 0x55);
    }
}
