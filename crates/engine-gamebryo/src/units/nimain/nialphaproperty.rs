//! `NiMain/nialphaproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a47360`: the jump target of `00a950c0`.
const JUMP_TARGET_00A950C0: u32 = 0x00a4_7360;

/// `00a62130`: the jump target of `00a99e40`.
const JUMP_TARGET_00A99E40: u32 = 0x00a6_2130;

// Translated from 00a950c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::LinkObject` (Xbox PDB): a 5-byte jump to `00a47360` (engine map: `NiSequenceStreamHelper::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiAlphaProperty@@UAAXAAVNiStream@@@Z`.
pub fn ni_alpha_property_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A950C0, &args![this, stream]);
}

// Translated from 00a99e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a62130` (engine map: `NiSequenceStreamHelper::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiAlphaProperty@@UAA_NAAVNiStream@@@Z`.
pub fn ni_alpha_property_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A99E40, &args![this, stream]).bool()
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `011f4410`: the pointer `00a5cef0` stores the object it builds in.
const ALPHA_PROPERTY_SLOT: u32 = 0x011f_4410;

// Translated from 00a5cef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nialphaproperty.cpp`; run by the start-up `00a89cc0`,
/// no arguments): builds a 0x1c-byte object (allocation `00aa13e0`, base constructor
/// `00a5c200`, vtable `010162dc`, the word at `+0x18` set to 0xec, the byte at `+0x1a` to
/// 0) and stores it, counted, in `011f4410`, releasing the object held there before. A
/// failed allocation stores null.
pub fn fn_00a5cef0(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, 0x0101_62dc);
        e.mem.set_u16(block + 0x18, 0xec);
        e.mem.set_u8(block + 0x1a, 0);
        block
    };
    assign_ref_counted(e, ALPHA_PROPERTY_SLOT, object);
}

/// `NiAlphaProperty`'s vtable.
const NI_ALPHA_PROPERTY_VTABLE: u32 = 0x0101_62dc;
/// `00a5d450` (`thiscall`, (clone, cloning process)): records the clone in the cloning
/// process's clone map; `00a5d7a0` (same words): the property base's member copy.
const OBJECT_RECORD_CLONE: u32 = 0x00a5_d450;
const PROPERTY_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `NiProperty::LoadBinary` (`thiscall`, the stream).
const PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;
/// `011f4404`: a byte that, when set, makes every alpha property clone a copy.
const ALWAYS_COPY_ALPHA: u32 = 0x011f_4404;
/// The import slot of `InterlockedDecrement`.
const INTERLOCKED_DECREMENT_SLOT: u32 = 0x00fd_f1c4;

/// A new alpha property (0x1c bytes, `operator new`, the base constructor, flags `0xec`,
/// threshold 0, the vtable `010162dc`); null when the allocation fails.
fn new_alpha_property(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
    e.mem.set_u16(block + 0x18, 0xec);
    e.mem.set_u32(block, NI_ALPHA_PROPERTY_VTABLE);
    e.mem.set_u8(block + 0x1a, 0);
    block
}

// Translated from 00a5ceb0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiAlphaProperty`): a new alpha property (flags `0xec`, threshold 0), or null.
pub fn fn_00a5ceb0(e: &mut Engine) -> Ptr {
    Ptr::new(new_alpha_property(e))
}

// Translated from 00a5ce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::CreateClone` (Xbox PDB): a property without controllers (`NiObjectNET`
/// +0xc null), unless the byte at `011f4404` is set, is its own clone (recorded by
/// `00a5d450`); otherwise a new alpha property (null when the allocation fails) given the
/// base's members (`00a5d7a0`) and this one's flags (+0x18) and threshold (+0x1a). Answers
/// the clone.
pub fn ni_alpha_property_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    if e.mem.u32(this.addr() + 0xc) == 0 && e.mem.u8(ALWAYS_COPY_ALPHA) == 0 {
        e.call(OBJECT_RECORD_CLONE, &args![this, this, cloning_process]);
        return this;
    }
    let clone = new_alpha_property(e);
    e.call(PROPERTY_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let flags = e.mem.u16(this.addr() + 0x18);
    e.mem.set_u16(clone.wrapping_add(0x18), flags);
    let threshold = e.mem.u8(this.addr() + 0x1a);
    e.mem.set_u8(clone.wrapping_add(0x1a), threshold);
    Ptr::new(clone)
}

// Translated from 00a5cf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nialphaproperty.cpp`; the shut-down counterpart of
/// `00a5cef0`): releases the alpha property held in `011f4410` (its slot `+4` when the count
/// reaches 0) and clears the word; nothing when it is null.
pub fn fn_00a5cf60(e: &mut Engine) {
    let held = e.mem.u32(ALPHA_PROPERTY_SLOT);
    if held == 0 {
        return;
    }
    if e.call(INTERLOCKED_DECREMENT_SLOT, &args![held.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(held, 4, &args![]);
    }
    e.mem.set_u32(ALPHA_PROPERTY_SLOT, 0);
}

/// Reads `size` bytes from the stream's input (`NiStream` +0x24c, the read function +8, the
/// position +4) into `destination`, the byte-count word at `count` set to `size` first.
fn stream_read(e: &mut Engine, stream: Ptr, destination: u32, size: u32, count: u32) {
    let input = e.mem.u32(stream.addr() + 0x24c);
    let function = e.mem.u32(input + 8);
    e.mem.set_u32(count, size);
    let done = e
        .call(function, &args![input, destination, size, count, 1u32])
        .u32();
    let position = e.mem.u32(input + 4).wrapping_add(done);
    e.mem.set_u32(input + 4, position);
}

// Translated from 00a5cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary`; for files older than
/// version `0x0a000102` (`NiStream` +0xd8) the flags (+0x18) are the stream's word at +0x288
/// cut to its low 14 bits, newer ones read them (2 bytes); then the 1-byte threshold
/// (+0x1a). The stream argument's stack word is the byte-count word.
pub fn ni_alpha_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        if e.mem.u32(stream.addr() + 0xd8) < 0x0a00_0102 {
            let flags = e.mem.u16(stream.addr() + 0x288) & 0x3fff;
            e.mem.set_u16(this.addr() + 0x18, flags);
        } else {
            stream_read(e, stream, this.addr() + 0x18, 2, count.addr());
        }
        stream_read(e, stream, this.addr() + 0x1a, 1, count.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_ceb0, fn_00a5ceb0() -> Ptr),
        entry!(0x00a5_ce30, ni_alpha_property_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a5_cf60, fn_00a5cf60()),
        entry!(0x00a5_cf90, ni_alpha_property_load_binary(Ptr, Ptr)),
        entry!(0x00a9_50c0, ni_alpha_property_link_object(Ptr, Ptr)),
        entry!(0x00a9_9e40, ni_alpha_property_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a5_cef0, fn_00a5cef0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a950c0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_50c0;
        const TARGET: u32 = 0x00a4_7360;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a99e40_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_9e40;
        const TARGET: u32 = 0x00a6_2130;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_and_replaces_the_held_one() {
        let run = run_default_instance(0x00a5_cef0, ALPHA_PROPERTY_SLOT, |_| {});
        let e = &run.engine;
        assert_ne!(run.object, 0);
        assert_ne!(run.object, run.previous);
        assert_eq!(e.mem.u32(run.object), 0x0101_62dc);
        assert_eq!(e.mem.u16(run.object + 0x18), 0xec);
        assert_eq!(e.mem.u8(run.object + 0x1a), 0);
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a5_cef0, ALPHA_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }
}

#[cfg(test)]
mod alpha_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        for double in [
            OBJECT_RECORD_CLONE,
            PROPERTY_COPY_MEMBERS,
            PROPERTY_LOAD_BINARY,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn create_object_builds_the_default_property() {
        let mut e = engine();
        let object = e.call(0x00a5_ceb0, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_ALPHA_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(object + 0x18), 0xec);
        assert_eq!(e.mem.u8(object + 0x1a), 0);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a5_ceb0, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_shares_or_copies() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u16(this + 0x18, 0x1234);
        e.mem.set_u8(this + 0x1a, 0x80);
        assert_eq!(e.call(0x00a5_ce30, &args![this, 0x66u32]).u32(), this);
        assert_eq!(
            calls_to(&e, OBJECT_RECORD_CLONE),
            vec![vec![this, this, 0x66]]
        );
        // The always-copy byte makes a copy.
        e.mem.set_u8(ALWAYS_COPY_ALPHA, 1);
        let clone = e.call(0x00a5_ce30, &args![this, 0x66u32]).u32();
        assert_ne!(clone, this);
        assert_eq!(e.mem.u32(clone), NI_ALPHA_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(clone + 0x18), 0x1234);
        assert_eq!(e.mem.u8(clone + 0x1a), 0x80);
        assert_eq!(
            calls_to(&e, PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, 0x66]]
        );
        // So do controllers.
        e.mem.set_u8(ALWAYS_COPY_ALPHA, 0);
        e.mem.set_u32(this + 0xc, 1);
        assert_ne!(e.call(0x00a5_ce30, &args![this, 0x66u32]).u32(), this);
    }

    #[test]
    fn release_of_the_default_alpha_property() {
        let mut e = engine();
        e.put_vtable(0x0300_1000, &[0, 0x0300_1004]);
        e.register(0x0300_1004, |_, _| Ret::default());
        let held = e.mem.alloc(0x1c);
        e.mem.set_u32(held, 0x0300_1000);
        e.mem.set_u32(held + 4, 1);
        e.mem.set_u32(ALPHA_PROPERTY_SLOT, held);
        e.call(0x00a5_cf60, &args![]);
        assert_eq!(e.mem.u32(ALPHA_PROPERTY_SLOT), 0);
        assert_eq!(calls_to(&e, 0x0300_1004), vec![vec![held]]);
        e.call(0x00a5_cf60, &args![]);
        assert_eq!(calls_to(&e, 0x0300_1004).len(), 1);
    }

    #[test]
    fn load_binary_reads_flags_and_threshold() {
        let mut e = engine();
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0300_0000);
        e.register(0x0300_0000, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (a[2], 1));
            let value: u32 = if a[2] == 2 { 0xabcd } else { 0x7f };
            e.mem.write(a[1], &value.to_le_bytes()[..a[2] as usize]);
            a[2].into_ret()
        });
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(stream + 0xd8, 0x0a00_0102);
        e.call(0x00a5_cf90, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x18), 0xabcd);
        assert_eq!(e.mem.u8(this + 0x1a), 0x7f);
        assert_eq!(e.mem.u32(input + 4), 3);
        e.mem.set_u32(stream + 0xd8, 0x0a00_0101);
        e.mem.set_u16(stream + 0x288, 0xffff);
        e.call(0x00a5_cf90, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x18), 0x3fff);
        assert_eq!(e.mem.u32(input + 4), 4);
    }
}
