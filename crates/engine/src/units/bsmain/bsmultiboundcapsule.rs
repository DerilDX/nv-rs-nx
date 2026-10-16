//! `BSMain/bsmultiboundcapsule.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00526ac0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A folded float getter: `FLD dword [this+0x24]`, the value in `ST0`. The
/// map names it `BSMultiBoundCapsule::QMultiBoundRadius`; the script
/// interpreter calls the same body for a `TESGlobal`'s value (operand `G`,
/// `005ac7a0`). The `FLD` quiets a signaling NaN.
pub fn fn_00526ac0(e: &mut Engine, this: Ptr) -> f64 {
    e.mem.f32(this.addr() + 0x24) as f64
}

/// `00c4fa10` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C4FA10: u32 = 0x00c4_fa10;

// Translated from 00c60ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSMultiBoundCapsule` vtable word 010c34fc, right after
/// `BSMultiBoundCapsule::LoadBinary` (010c34f8). Forwards `this` and the stream to `00c4fa10`
/// (`bsmultiboundshape.obj`, unnamed in the engine map). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c60ec0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C4FA10, &args![this, stream]);
}

// Translated from 00c61270 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01204398 without reading the receiver. The map names this body `BSMultiBoundCapsule::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c61270(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_4398
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_1270, fn_00c61270(Ptr) -> u32),
        entry!(0x00526ac0, fn_00526ac0(Ptr) -> f64),
        entry!(0x00c6_0ec0, fn_00c60ec0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_0x24_into_st0() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x28);
        e.mem.set_f32(object + 0x20, 9.0);
        for value in [1.5f32, -1.0, 0.0, f32::NEG_INFINITY] {
            e.mem.set_f32(object + 0x24, value);
            assert_eq!(e.call(0x00526ac0, &[object]).f64(), value as f64);
        }
        // A signaling single NaN comes back quiet, its payload widened.
        e.mem.set_u32(object + 0x24, 0x7f80_0001);
        assert_eq!(
            e.call(0x00526ac0, &[object]).f64().to_bits(),
            0x7ff8_0000_2000_0000
        );
    }

    #[test]
    fn forwarder_fn_00c60ec0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C4FA10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_0ec0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C4FA10)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c61270_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_1270, &args![0x1000u32]).u32(), 0x0120_4398);
        assert_eq!(e.call(0x00c6_1270, &args![0u32]).u32(), 0x0120_4398);
    }
}
