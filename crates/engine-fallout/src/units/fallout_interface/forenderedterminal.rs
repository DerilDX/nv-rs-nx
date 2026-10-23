//! `fallout/interface/forenderedterminal.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007ff8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `007ff8e0`, which tests the flag `0x4000` of the
/// word at `+0x30` through `00456630`, and returns its result (a `bool`). Parameters: one
/// word, `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX`
/// on. The return type (`bool`) is from `00456630`'s disassembly (`AND EAX,[EBP+8]; NEG EAX;
/// SBB EAX,EAX; NEG EAX`: 1 when the masked word is non-zero, else 0), which `007ff8e0`
/// returns without change.
pub fn fn_007ff8c0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x007f_f8e0, &args![this]).bool()
}

/// `00456630` (`thiscall`, `RET 4`): whether the word at +0x30 has a bit of the mask.
const FLAG_TEST_00456630: u32 = 0x0045_6630;
/// `00450b80` (`cdecl`, one word, `ECX` unused): the word at `011f91c8 + 4 * index`.
const TES_00450B80: u32 = 0x0045_0b80;
/// The holder at `011db63c`: `00559450` (`thiscall`) returns its first word, `0066b0d0`
/// (`thiscall`, `RET 4`) is called on it with 0.
const HOLDER_011DB63C: u32 = 0x011d_b63c;
const HOLDER_GET_00559450: u32 = 0x0055_9450;
const HOLDER_SET_0066B0D0: u32 = 0x0066_b0d0;
/// `ModelLoader::ReleaseModel` (`00445300`, `thiscall` on the model loader at `011c3b3c`,
/// `RET 0xc`) and the model path pointer at `011a0bb0` (the terminal interface model).
const MODEL_LOADER_RELEASE_MODEL: u32 = 0x0044_5300;
const MODEL_LOADER_011C3B3C: u32 = 0x011c_3b3c;
const TERMINAL_MODEL_PATH_011A0BB0: u32 = 0x011a_0bb0;

// Translated from 007ff8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, plain `RET`): `00456630(this, 0x4000)`, its `EAX` returned
/// unchanged (0 or 1).
pub fn fn_007ff8e0(e: &mut Engine, this: Ptr) -> bool {
    e.call(FLAG_TEST_00456630, &args![this, 0x4000u32]).bool()
}

// Translated from 007ffad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, plain `RET`; `this` is not read): `00450b80(1)`, whose `EAX` it
/// leaves as its own return value.
pub fn fn_007ffad0(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(TES_00450B80, &args![1u32]).u32()
}

// Translated from 007ffe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal::ReleaseStaticGeometry` (Xbox PDB name from the engine map; no
/// arguments): when the holder at `011db63c` holds something (`00559450`), releases the
/// terminal model (`ModelLoader::ReleaseModel(path, 1, 1)` with the path at `011a0bb0`),
/// then `0066b0d0(holder, 0)`.
pub fn fo_rendered_terminal_release_static_geometry(e: &mut Engine) {
    if e.call(HOLDER_GET_00559450, &args![HOLDER_011DB63C]).u32() != 0 {
        let path: u32 = e.global(TERMINAL_MODEL_PATH_011A0BB0);
        let loader: u32 = e.global(MODEL_LOADER_011C3B3C);
        e.call(MODEL_LOADER_RELEASE_MODEL, &args![loader, path, 1u32, 1u32]);
    }
    e.call(HOLDER_SET_0066B0D0, &args![HOLDER_011DB63C, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007f_f8c0, fn_007ff8c0(Ptr) -> bool),
        entry!(0x007f_f8e0, fn_007ff8e0(Ptr) -> bool),
        entry!(0x007f_fad0, fn_007ffad0(Ptr) -> u32),
        entry!(0x007f_fe00, fo_rendered_terminal_release_static_geometry()),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_007ff8c0_returns_the_callees_al() {
        let mut e = Engine::new();
        e.register(0x007f_f8e0, |_, _| Ret {
            eax: 0x1234_5601,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_f8c0, &args![0x0000_5000u32]).bool());
        assert_eq!(calls_to(&e, 0x007f_f8e0), vec![vec![0x5000]]);
        // Only AL counts: a non-zero upper part with AL clear is false.
        e.register(0x007f_f8e0, |_, _| Ret {
            eax: 0x1234_5600,
            ..Ret::default()
        });
        assert!(!e.call(0x007f_f8c0, &args![0x0000_5000u32]).bool());
    }
}

#[cfg(test)]
mod b069_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_007ff8e0_tests_the_flag_0x4000() {
        let mut e = Engine::new();
        e.register(FLAG_TEST_00456630, |_, a| {
            u32::from(a[1] == 0x4000).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_f8e0, &args![0x5000u32]).bool());
        assert_eq!(calls_to(&e, FLAG_TEST_00456630), vec![vec![0x5000, 0x4000]]);
    }

    #[test]
    fn fn_007ffad0_returns_the_entry_at_index_1() {
        let mut e = Engine::new();
        e.register(TES_00450B80, |_, a| (0x100 + a[0]).into_ret());
        assert_eq!(e.call(0x007f_fad0, &args![0x5000u32]).u32(), 0x101);
    }

    #[test]
    fn release_static_geometry_releases_the_model_only_when_held() {
        let mut e = Engine::new();
        e.map(MODEL_LOADER_011C3B3C & !0xfff, 0x1000);
        e.map(TERMINAL_MODEL_PATH_011A0BB0 & !0xfff, 0x1000);
        e.set_global(MODEL_LOADER_011C3B3C, 0x6000u32);
        e.set_global(TERMINAL_MODEL_PATH_011A0BB0, 0x0107_8b80u32);
        e.register(HOLDER_GET_00559450, |_, _| 1u32.into_ret());
        e.register(HOLDER_SET_0066B0D0, |_, _| Ret::default());
        e.register(MODEL_LOADER_RELEASE_MODEL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007f_fe00, &args![]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_RELEASE_MODEL),
            vec![vec![0x6000, 0x0107_8b80, 1, 1]]
        );
        assert_eq!(
            calls_to(&e, HOLDER_SET_0066B0D0),
            vec![vec![HOLDER_011DB63C, 0]]
        );
        e.register(HOLDER_GET_00559450, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007f_fe00, &args![]);
        assert!(calls_to(&e, MODEL_LOADER_RELEASE_MODEL).is_empty());
        assert_eq!(calls_to(&e, HOLDER_SET_0066B0D0).len(), 1);
    }
}
