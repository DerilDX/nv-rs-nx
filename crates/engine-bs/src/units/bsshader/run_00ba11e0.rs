//! `BSShader/run_00ba11e0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The destructor body `00ba0400` (unnamed in the engine map; it stores the vtable
/// `010b7ec8`, which has no RTTI locator; `thiscall`, no stack words).
const DESTRUCT_00BA0400: u32 = 0x00ba_0400;
/// The memory manager singleton (`011f6238`) and its free (`00aa4060`, `thiscall`: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;

// Translated from 00ba1230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word, `RET 4`; the only reference is the
/// vtable `010b7ec8`): the scalar deleting destructor of the class whose destructor body is
/// `00ba0400`. Runs that body, then frees `this` through the memory manager when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00ba1230(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_00BA0400, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ba_1230, fn_00ba1230(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ba1230_destructs_then_frees_on_bit_zero() {
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            let mut e = Engine::new();
            e.register(DESTRUCT_00BA0400, |_, _| Ret::default());
            e.register(MEMORY_FREE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x00ba_1230, &args![0x0e00_4000u32, flags]).u32(),
                0x0e00_4000
            );
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(DESTRUCT_00BA0400, vec![0x0e00_4000])];
            if frees {
                expected.push((MEMORY_FREE, vec![MEMORY_MANAGER, 0x0e00_4000]));
            }
            assert_eq!(log[1..].to_vec(), expected);
        }
    }
}
