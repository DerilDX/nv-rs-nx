//! `fallout/magic/magicshaderhiteffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006b3eb0(this, 0, 0)`: the array-shaped constructor (unit
/// `pathsmootherpov`).
const ARRAY_CONSTRUCTOR: u32 = 0x006b_3eb0;
/// The vtable `00822600` stores.
const VTABLE: u32 = 0x0107_b84c;

// Translated from 00822600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `0107b84c` and runs
/// `006b3eb0(this, 0, 0)` (an empty array: no reserved size, no elements).
/// Returns `this`. The class is not confirmed from the exe.
pub fn fn_00822600(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(ARRAY_CONSTRUCTOR, &args![this, 0u32, 0u32]);
    this
}

/// `operator new` (`00401000`, `cdecl` (size)).
const OPERATOR_NEW: u32 = 0x0040_1000;

// Translated from 008225d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A map's bucket-array allocator (`thiscall`, `RET 4`; `this` unused):
/// `operator new` of `count * 4` bytes.
pub fn fn_008225d0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(OPERATOR_NEW, &args![count << 2]).u32()
}

/// `0081bf20` (named `MagicHitEffect::InitLoadGame_ov2` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const INIT_LOAD_GAME_TARGET_0081BF20: u32 = 0x0081_bf20;

// Translated from 0081d9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::InitLoadGame` overload 2 (engine map vtable name; vtable words
/// 0107b304 and 0107b7c4): forwards `this` and one pointer to the base body `0081bf20`
/// (`MagicHitEffect::InitLoadGame_ov2`, Xbox PDB
/// `?InitLoadGame@MagicHitEffect@@UAAXPAVBGSLoadGameBuffer@@@Z`, void). Returns nothing.
pub fn magic_shader_hit_effect_init_load_game_ov2(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(INIT_LOAD_GAME_TARGET_0081BF20, &args![this, buffer]);
}

// Translated from 0081f310 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011dc804 without reading the receiver. The map names this body `MagicShaderHitEffect::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0081f310(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_c804
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_f310, fn_0081f310(Ptr) -> u32),
        entry!(
            0x0081_d9d0,
            magic_shader_hit_effect_init_load_game_ov2(Ptr, Ptr)
        ),
        entry!(0x00822600, fn_00822600(Ptr) -> Ptr),
        entry!(0x008225d0, fn_008225d0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bucket_array_is_four_bytes_a_bucket() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (0x5000 + a[0]).into_ret());
        assert_eq!(e.call(0x008225d0, &[0, 37]).u32(), 0x5000 + 37 * 4);
    }

    #[test]
    fn stores_the_vtable_then_builds_an_empty_array() {
        let mut e = Engine::new();
        e.register(ARRAY_CONSTRUCTOR, |e, a| {
            // The vtable is already in place when the constructor runs.
            assert_eq!(e.mem.u32(a[0]), VTABLE);
            Ret::default()
        });
        let this = e.mem.alloc(16);
        e.call_log = Some(vec![]);
        let r = e.call(0x00822600, &args![this]);
        assert_eq!(r.u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ARRAY_CONSTRUCTOR, vec![this, 0, 0]));
    }

    #[test]
    fn forwarder_magic_shader_hit_effect_init_load_game_ov2_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(INIT_LOAD_GAME_TARGET_0081BF20, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0081_d9d0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == INIT_LOAD_GAME_TARGET_0081BF20)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_0081f310_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0081_f310, &args![0x1000u32]).u32(), 0x011d_c804);
        assert_eq!(e.call(0x0081_f310, &args![0u32]).u32(), 0x011d_c804);
    }
}
