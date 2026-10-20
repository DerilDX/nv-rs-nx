//! `BSMain/bsparentvelocitymodifier.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c31650` (named `NiPSysModifier::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C31650: u32 = 0x00c3_1650;

// Translated from 00c63930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSParentVelocityModifier` vtable word 010c384c, right after
/// `BSParentVelocityModifier::LoadBinary` (010c3848). Forwards `this` and the stream to
/// `00c31650` (`NiPSysModifier::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c63930(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C31650, &args![this, stream]);
}

/// `00c31670` (named `NiPSysModifier::RegisterStreamables` in the engine map): the base
/// body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00C31670: u32 = 0x00c3_1670;

// Translated from 00c63950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSParentVelocityModifier` vtable word 010c3850, after the
/// `LinkObject` word 010c384c and before `BSParentVelocityModifier::SaveBinary` (010c3854).
/// Forwards `this` and the stream to `00c31670` (`NiPSysModifier::RegisterStreamables`).
/// Returns the `bool` in `AL`: Xbox PDB `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z`
/// returns `_N`; `stream` is the `NiStream` reference as an address. Parameter count from the
/// `RET 4` of both bodies.
pub fn fn_00c63950(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00C31670, &args![this, stream])
        .bool()
}

// Translated from 00c63aa0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043c8 without reading the receiver. The map names this body `BSParentVelocityModifier::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c63aa0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43c8
}

/// The `NiPSysModifier` constructor (`00c31720`), clone-member copy
/// (`00c31780`: `this`, the clone and the cloning process), `LoadBinary`
/// (`00c318b0`) and destructor (`00c31750`).
const NI_PSYS_MODIFIER_CONSTRUCT: u32 = 0x00c3_1720;
const NI_PSYS_MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const NI_PSYS_MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
const NI_PSYS_MODIFIER_DESTRUCT: u32 = 0x00c3_1750;
/// The Gamebryo allocator (`00aa13e0`, cdecl, the size) and the sized
/// delete (`00aa1460`, cdecl: the block and the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// Reads a float from the stream (`00a26df0`, cdecl: the stream and the
/// address).
const STREAM_READ_FLOAT: u32 = 0x00a2_6df0;
/// The `BSParentVelocityModifier` vtable (its RTTI locator names
/// `.?AVBSParentVelocityModifier@@`) and the object size.
const BS_PARENT_VELOCITY_MODIFIER_VTABLE: u32 = 0x010c_37fc;
const BS_PARENT_VELOCITY_MODIFIER_SIZE: u32 = 0x3c;
/// The point the two vectors start as (three words at `011f426c`).
const START_POINT: u32 = 0x011f_426c;
/// The float the class reads and copies (+0x20).
const DAMPING: u32 = 0x20;

// Translated from 00c63330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSParentVelocityModifier` constructor: the `NiPSysModifier` part,
/// the class's vtable, the word at +0xC 3000, the two vectors at +0x24 and
/// +0x30 the start point, the floats at +0x20 and +0x18 0.0. Returns
/// `this`.
pub fn fn_00c63330(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_PSYS_MODIFIER_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, BS_PARENT_VELOCITY_MODIFIER_VTABLE);
    e.mem.set_u32(base + 0xc, 3000);
    for vector in [0x24u32, 0x30] {
        for word in 0..3 {
            let value = e.mem.u32(START_POINT + 4 * word);
            e.mem.set_u32(base + vector + 4 * word, value);
        }
    }
    e.mem.set_f32(base + DAMPING, 0.0);
    e.mem.set_f32(base + 0x18, 0.0);
    this
}

// Translated from 00c63880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c63880 - C++ exception unwinding (FS:[0] frame) not translated
/// `BSParentVelocityModifier::CreateObject` (the class's stream factory): a
/// new 0x3C-byte object from the Gamebryo allocator, constructed; null when
/// the allocation fails.
pub fn fn_00c63880(e: &mut Engine) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![BS_PARENT_VELOCITY_MODIFIER_SIZE])
        .u32();
    if object == 0 {
        return Ptr::NULL;
    }
    fn_00c63330(e, Ptr::new(object))
}

// Translated from 00c63840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::CopyMembers` (by its use in `CreateClone`):
/// the `NiPSysModifier` part, then the float at +0x20 (two `FLD`/`FSTP`
/// pairs).
pub fn fn_00c63840(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(NI_PSYS_MODIFIER_COPY_MEMBERS, &args![this, clone, cloning]);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + DAMPING));
    e.mem.set_u32(clone.addr() + DAMPING, value);
}

// Translated from 00c637b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c637b0 - C++ exception unwinding (FS:[0] frame) not translated
/// `BSParentVelocityModifier::CreateClone` (Xbox PDB): a new object as
/// `CreateObject` makes it (null when the allocation fails, used as is),
/// then the members copied into it (`00c63840`).
pub fn bs_parent_velocity_modifier_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = fn_00c63880(e);
    fn_00c63840(e, this, clone, cloning);
    clone
}

// Translated from 00c63900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::LoadBinary` (Xbox PDB): the `NiPSysModifier`
/// part, then the float at +0x20 (`00a26df0`).
pub fn bs_parent_velocity_modifier_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_PSYS_MODIFIER_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_READ_FLOAT, &args![stream, this.addr() + DAMPING]);
}

// Translated from 00c63a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::_scalar_deleting_destructor_` (Xbox PDB): the
/// `NiPSysModifier` destructor, then the sized delete of 0x3C bytes when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bs_parent_velocity_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(NI_PSYS_MODIFIER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, BS_PARENT_VELOCITY_MODIFIER_SIZE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_3aa0, fn_00c63aa0(Ptr) -> u32),
        entry!(0x00c6_3930, fn_00c63930(Ptr, Ptr)),
        entry!(0x00c6_3950, fn_00c63950(Ptr, Ptr) -> bool),
        entry!(0x00c63330, fn_00c63330(Ptr) -> Ptr),
        entry!(0x00c63880, fn_00c63880() -> Ptr),
        entry!(0x00c63840, fn_00c63840(Ptr, Ptr, Ptr)),
        entry!(
            0x00c637b0,
            bs_parent_velocity_modifier_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00c63900,
            bs_parent_velocity_modifier_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c63a70,
            bs_parent_velocity_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c63930_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C31650, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_3930, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C31650)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c63950_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00C31670, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c6_3950, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == REGISTER_STREAMABLES_TARGET_00C31670)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c63aa0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_3aa0, &args![0x1000u32]).u32(), 0x0120_43c8);
        assert_eq!(e.call(0x00c6_3aa0, &args![0u32]).u32(), 0x0120_43c8);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn constructor_factory_and_clone() {
        let mut e = Engine::new();
        e.register(NI_PSYS_MODIFIER_CONSTRUCT, nothing);
        e.register(NI_PSYS_MODIFIER_COPY_MEMBERS, nothing);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.map(START_POINT, 12);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(START_POINT + 4 * i as u32, *v);
        }
        let this = e.mem.alloc(0x3c);
        e.mem.write(this, &[0xaa; 0x3c]);
        assert_eq!(e.call(0x00c6_3330, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BS_PARENT_VELOCITY_MODIFIER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 3000);
        assert_eq!(e.mem.f32(this + 0x28), 2.0);
        assert_eq!(e.mem.f32(this + 0x38), 3.0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0xaaaa_aaaa);
        let object = e.call(0x00c6_3880, &args![]).u32();
        assert_eq!(e.mem.u32(object), BS_PARENT_VELOCITY_MODIFIER_VTABLE);
        e.mem.set_f32(this + 0x20, 0.75);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c6_37b0, &args![this, 0x99u32]).u32();
        assert_eq!(
            calls_to(&e, NI_PSYS_MODIFIER_COPY_MEMBERS),
            vec![vec![this, clone, 0x99]]
        );
        assert_eq!(e.mem.f32(clone + 0x20), 0.75);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c6_3880, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_and_deleting_destructor() {
        let mut e = Engine::new();
        e.register(NI_PSYS_MODIFIER_LOAD_BINARY, nothing);
        e.register(STREAM_READ_FLOAT, nothing);
        e.register(NI_PSYS_MODIFIER_DESTRUCT, nothing);
        e.register(SIZED_DELETE, nothing);
        e.call_log = Some(vec![]);
        e.call(0x00c6_3900, &args![0x100u32, 0x200u32]);
        assert_eq!(
            calls_to(&e, NI_PSYS_MODIFIER_LOAD_BINARY),
            vec![vec![0x100, 0x200]]
        );
        assert_eq!(calls_to(&e, STREAM_READ_FLOAT), vec![vec![0x200, 0x120]]);
        assert_eq!(e.call(0x00c6_3a70, &args![0x100u32, 0u32]).u32(), 0x100);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00c6_3a70, &args![0x100u32, 1u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![0x100, 0x3c]]);
        assert_eq!(calls_to(&e, NI_PSYS_MODIFIER_DESTRUCT).len(), 2);
    }
}
