//! `(unplaced)/run_00e80570` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiD3DShaderInterface::Initialize` (Xbox PDB, `00e81a90`; `thiscall`, no stack words).
const SHADER_INTERFACE_INITIALIZE: u32 = 0x00e8_1a90;
/// A global object pointer read by `00e80580` and `00e80590` (its class is unconfirmed).
const GLOBAL_OBJECT: u32 = 0x0126_f74c;
/// The vtable slot of that object `00e80590` calls with one word.
const GLOBAL_OBJECT_SLOT: u32 = 0xc;
/// The two functions `00e80630` runs (no Xbox names; neither takes a word).
const RELEASE_FIRST: u32 = 0x00e8_03f0;
const RELEASE_SECOND: u32 = 0x00e7_ed80;

// Translated from 00e80570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a jump thunk to `NiD3DShaderInterface::Initialize`
/// (Xbox PDB, `00e81a90`) with the same `this`; answers what it answers.
pub fn fn_00e80570(e: &mut Engine, this: u32) -> u32 {
    e.call(SHADER_INTERFACE_INITIALIZE, &args![this]).u32()
}

// Translated from 00e80580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable method; `this` is not read): 1 when the global
/// object pointer `0126f74c` is set, else 0.
pub fn fn_00e80580(e: &mut Engine, _this: u32) -> u32 {
    u32::from(e.mem.u32(GLOBAL_OBJECT) != 0)
}

// Translated from 00e80590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable method, `thiscall`): when the global object
/// `0126f74c` is set, calls its vtable slot +0xC with the word at `this` +0x38. Answers 0.
pub fn fn_00e80590(e: &mut Engine, this: u32) -> u32 {
    let object = e.mem.u32(GLOBAL_OBJECT);
    if object != 0 {
        let word = e.mem.u32(this + 0x38);
        e.vcall(object, GLOBAL_OBJECT_SLOT, &args![word]);
    }
    0
}

// Translated from 00e80630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: calls `00e803f0`, then tail-jumps to `00e7ed80` (neither
/// takes a word).
pub fn fn_00e80630(e: &mut Engine) {
    e.call(RELEASE_FIRST, &[]);
    e.call(RELEASE_SECOND, &[]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_0570, fn_00e80570(u32) -> u32),
        entry!(0x00e8_0580, fn_00e80580(u32) -> u32),
        entry!(0x00e8_0590, fn_00e80590(u32) -> u32),
        entry!(0x00e8_0630, fn_00e80630()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e80570_passes_this_on_and_answers_the_result() {
        let mut e = Engine::new();
        e.register(SHADER_INTERFACE_INITIALIZE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_0570, &[0x1234]).u32(), 1);
        assert_eq!(
            calls_to(&e, SHADER_INTERFACE_INITIALIZE),
            vec![vec![0x1234]]
        );
    }

    #[test]
    fn fn_00e80580_tests_the_global_pointer() {
        let mut e = Engine::new();
        e.mem.map(GLOBAL_OBJECT & !0xfff, 0x1000);
        assert_eq!(e.call(0x00e8_0580, &[0]).u32(), 0);
        e.mem.set_u32(GLOBAL_OBJECT, 0x4000);
        assert_eq!(e.call(0x00e8_0580, &[0]).u32(), 1);
    }

    #[test]
    fn fn_00e80590_calls_the_global_object_with_the_field() {
        let mut e = Engine::new();
        e.mem.map(GLOBAL_OBJECT & !0xfff, 0x1000);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x38, 0x77);
        e.call_log = Some(vec![]);
        // No object: nothing called.
        assert_eq!(e.call(0x00e8_0590, &[this]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);

        let object = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0, 0, 0x0090_1000]);
        e.mem.set_u32(object, 0x0090_0000);
        e.mem.set_u32(GLOBAL_OBJECT, object);
        e.register(0x0090_1000, |_, _| 5u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_0590, &[this]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0090_1000), vec![vec![object, 0x77]]);
    }

    #[test]
    fn fn_00e80630_runs_both_in_order() {
        let mut e = Engine::new();
        e.register(RELEASE_FIRST, |_, _| Ret::default());
        e.register(RELEASE_SECOND, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00e8_0630, &[]);
        let order: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, vec![0x00e8_0630, RELEASE_FIRST, RELEASE_SECOND]);
    }
}
