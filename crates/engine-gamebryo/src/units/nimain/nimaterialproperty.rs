//! `NiMain/nimaterialproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiMaterialProperty::ms_RTTI` (the class's `NiRTTI` record).
const NI_MATERIAL_PROPERTY_RTTI: u32 = 0x011f_4a5c;

// Translated from 00a756c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_material_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_MATERIAL_PROPERTY_RTTI)
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE};

/// `011f4a64`: the pointer `00a759e0` stores the object it builds in.
const MATERIAL_PROPERTY_SLOT: u32 = 0x011f_4a64;
/// `00a75650` (`thiscall`, engine map `NiMaterialProperty::NiMaterialProperty`): constructs
/// the object in the block and answers it.
const MATERIAL_PROPERTY_CONSTRUCTOR: u32 = 0x00a7_5650;

// Translated from 00a759e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimaterialproperty.cpp`; run by the start-up
/// `00a89cc0`, no arguments): allocates 0x4c bytes (`00aa13e0`), constructs a material
/// property in them with `00a75650` and stores the object it answers, counted, in
/// `011f4a64`, releasing the object held there before. A failed allocation stores null.
pub fn fn_00a759e0(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(MATERIAL_PROPERTY_CONSTRUCTOR, &args![block]).u32()
    };
    assign_ref_counted(e, MATERIAL_PROPERTY_SLOT, object);
}

/// `011aa070`: a counted word `00a75650` increments; the count it answers goes to `+0x18`.
const MATERIAL_COUNTER: u32 = 0x011a_a070;
/// `0101e570`: a `float` copied to `+0x38`.
const MATERIAL_FLOAT: u32 = 0x0101_e570;
/// `InterlockedIncrement` import slot.
const INTERLOCKED_INCREMENT_SLOT: u32 = 0x00fd_f1c0;

// Translated from 00a75650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::NiMaterialProperty` (Xbox PDB; `thiscall`, no stack word): runs the
/// base constructor `00a5c200`, sets the vtable `0109d6c4`, zeroes the floats at `+0x1c`
/// to `+0x30` and the word at `+0x34`, copies the float at `0101e570` to `+0x38`, sets
/// the floats at `+0x3c` and `+0x40` to 1.0, counts `011aa070` and stores the count at
/// `+0x18`, sets the word at `+0x44` to 1 and `+0x48` to 0. Returns `this`.
pub fn ni_material_property_ni_material_property(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        crate::units::nimain::nimainsdm::OBJECT_BASE_CONSTRUCTOR,
        &args![this],
    );
    let base = this.addr();
    e.mem.set_u32(base, 0x0109_d6c4);
    // `FLDZ` and `FST`/`FSTP` of 0.0 (stored repeatedly by the exe) into the six floats.
    for offset in [0x1cu32, 0x20, 0x24, 0x28, 0x2c, 0x30] {
        e.mem.set_u32(base + offset, 0.0f32.to_bits());
    }
    e.mem.set_u32(base + 0x34, 0);
    let value = e.mem.u32(MATERIAL_FLOAT);
    e.mem.set_u32(base + 0x38, x87_quiet_bits(value));
    e.mem.set_u32(base + 0x3c, 1.0f32.to_bits());
    e.mem.set_u32(base + 0x40, 1.0f32.to_bits());
    let count = e
        .call(INTERLOCKED_INCREMENT_SLOT, &args![MATERIAL_COUNTER])
        .u32();
    e.mem.set_u32(base + 0x18, count);
    e.mem.set_u32(base + 0x44, 1);
    e.mem.set_u32(base + 0x48, 0);
    this
}

/// `NiMaterialProperty`'s vtable.
const NI_MATERIAL_PROPERTY_VTABLE: u32 = 0x0109_d6c4;
/// `00a5d7a0` (`thiscall`, (clone, cloning process)): the property base's member copy.
const PROPERTY_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `00a5d450` (`thiscall`, (clone, cloning process)): records the clone in the cloning
/// process's clone map.
const OBJECT_RECORD_CLONE: u32 = 0x00a5_d450;
/// `NiProperty::LoadBinary` (`thiscall`, the stream).
const PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;
/// `NiColor::LoadBinary` (`thiscall` on the color, the stream).
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `00a5c240` (unit `niobjectnet.cpp`, `thiscall`): the object base's destructor body.
const OBJECT_NET_DESTRUCT: u32 = 0x00a5_c240;
/// The sized delete (`cdecl`, (block, size)) and the import slot of `InterlockedDecrement`.
const SIZED_DELETE: u32 = 0x00aa_1460;
const INTERLOCKED_DECREMENT_SLOT: u32 = 0x00fd_f1c4;
/// `011f4748`: the object whose virtual slot `+0xf4` the destructor tells about the material
/// (when there is one).
const MATERIAL_LISTENER: u32 = 0x011f_4748;

// Translated from 00a755f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimaterialproperty.cpp`; by its body the member copy):
/// the property base's copy (`00a5d7a0`), the words from +0x1c to +0x34 copied as they are
/// and the floats at +0x38, +0x3c and +0x40 through the x87 stack.
pub fn fn_00a755f0(e: &mut Engine, this: Ptr, clone: Ptr, cloning_process: Ptr) {
    e.call(PROPERTY_COPY_MEMBERS, &args![this, clone, cloning_process]);
    for offset in (0x1cu32..=0x34).step_by(4) {
        let value = e.mem.u32(this.addr() + offset);
        e.mem.set_u32(clone.addr() + offset, value);
    }
    for offset in [0x38u32, 0x3c, 0x40] {
        let value = x87_store32(x87_load32(e.mem.u32(this.addr() + offset)));
        e.mem.set_u32(clone.addr() + offset, value);
    }
}

// Translated from 00a756d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiMaterialProperty`): allocates 0x4c bytes (`operator new`) and, when that
/// succeeds, runs the constructor (`00a75650`) on them. Answers the object, or null.
pub fn fn_00a756d0(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    ni_material_property_ni_material_property(e, Ptr::new(block))
}

// Translated from 00a756f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::_scalar_deleting_destructor_` (Xbox PDB): stores the vtable
/// `0109d6c4`, passes `this` to the virtual slot `+0xf4` of the object at `011f4748` when
/// there is one, runs the object base's destructor body (`00a5c240`) and, when bit 0 of
/// `flags` is set, the sized delete of 0x4c bytes. Answers `this`.
pub fn ni_material_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_MATERIAL_PROPERTY_VTABLE);
    let listener = e.mem.u32(MATERIAL_LISTENER);
    if listener != 0 {
        e.vcall(listener, 0xf4, &args![this]);
    }
    e.call(OBJECT_NET_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x4cu32]);
    }
    this
}

// Translated from 00a75730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::CreateClone` (Xbox PDB): a material without controllers
/// (`NiObjectNET` +0xc null) is its own clone (recorded as such by `00a5d450`); otherwise a
/// new material (0x4c bytes, the constructor; null when the allocation fails) given this
/// one's members (`00a755f0`, which runs on null too). Answers the clone.
pub fn ni_material_property_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    if e.mem.u32(this.addr() + 0xc) == 0 {
        e.call(OBJECT_RECORD_CLONE, &args![this, this, cloning_process]);
        return this;
    }
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    let clone = if block != 0 {
        ni_material_property_ni_material_property(e, Ptr::new(block))
    } else {
        Ptr::new(0)
    };
    fn_00a755f0(e, this, clone, cloning_process);
    clone
}

/// Reads one 4-byte word from the stream's input (`NiStream` +0x24c, the read function +8,
/// the position +4) into `destination`, the byte-count word at `count` set to 4 first.
fn read_word(e: &mut Engine, stream: Ptr, destination: u32, count: u32) {
    let input = e.mem.u32(stream.addr() + 0x24c);
    let function = e.mem.u32(input + 8);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![input, destination, 4u32, count, 1u32])
        .u32();
    let position = e.mem.u32(input + 4).wrapping_add(done);
    e.mem.set_u32(input + 4, position);
}

// Translated from 00a75790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary`; streams whose
/// Bethesda version (`NiStream` +4) is below 0x1a then hold two colors that are read into a
/// zeroed local and dropped, two colors at +0x1c and +0x28 (`NiColor::LoadBinary`) and two
/// 4-byte floats at +0x38 and +0x3c; newer ones the two colors and three floats, +0x38,
/// +0x3c and +0x40. The stream argument's stack word is the byte-count word.
pub fn ni_material_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    let old = e.mem.u32(stream.addr() + 4) < 0x1a;
    e.with_stack(0x10, |e, slots| {
        let count = slots.addr();
        let dropped = slots.addr() + 4;
        if old {
            for word in 0..3 {
                e.mem.set_u32(dropped + 4 * word, x87_store32(0.0));
            }
            e.call(COLOR_LOAD_BINARY, &args![dropped, stream]);
            e.call(COLOR_LOAD_BINARY, &args![dropped, stream]);
        }
        e.call(COLOR_LOAD_BINARY, &args![this.addr() + 0x1c, stream]);
        e.call(COLOR_LOAD_BINARY, &args![this.addr() + 0x28, stream]);
        read_word(e, stream, this.addr() + 0x38, count);
        read_word(e, stream, this.addr() + 0x3c, count);
        if !old {
            read_word(e, stream, this.addr() + 0x40, count);
        }
    });
}

// Translated from 00a75a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimaterialproperty.cpp`; the shut-down counterpart of
/// `00a759e0`): releases the material held in `011f4a64` (its slot `+4` when the count
/// reaches 0) and clears the word; nothing when it is null.
pub fn fn_00a75a40(e: &mut Engine) {
    let held = e.mem.u32(MATERIAL_PROPERTY_SLOT);
    if held == 0 {
        return;
    }
    if e.call(INTERLOCKED_DECREMENT_SLOT, &args![held.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(held, 4, &args![]);
    }
    e.mem.set_u32(MATERIAL_PROPERTY_SLOT, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_55f0, fn_00a755f0(Ptr, Ptr, Ptr)),
        entry!(0x00a7_56d0, fn_00a756d0() -> Ptr),
        entry!(
            0x00a7_56f0,
            ni_material_property_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a7_5730, ni_material_property_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a7_5790, ni_material_property_load_binary(Ptr, Ptr)),
        entry!(0x00a7_5a40, fn_00a75a40()),
        entry!(0x00a756c0, ni_material_property_get_rtti() -> Ptr),
        entry!(0x00a7_59e0, fn_00a759e0()),
        entry!(0x00a7_5650, ni_material_property_ni_material_property(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_material_property_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a756c0, &args![]).u32(),
            NI_MATERIAL_PROPERTY_RTTI
        );
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn stores_the_object_the_constructor_answers() {
        let run = run_default_instance(0x00a7_59e0, MATERIAL_PROPERTY_SLOT, |e| {
            e.register(MATERIAL_PROPERTY_CONSTRUCTOR, |e, a| {
                e.mem.set_u32(a[0] + 0x40, 0xabcd);
                a[0].into_ret()
            });
        });
        assert_ne!(run.object, 0);
        assert_eq!(run.engine.mem.u32(run.object + 0x40), 0xabcd);
        assert_eq!(run.engine.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a7_59e0, MATERIAL_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn constructs_the_default_material() {
        let mut e = engine_with_counting();
        e.map(0x011a_a000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.mem.set_u32(MATERIAL_COUNTER, 41);
        e.mem.set_u32(MATERIAL_FLOAT, 0x7fa0_0001);
        let this = e.mem.alloc(0x4c);
        e.mem.write(this, &[0xcc; 0x4c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_5650, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, OBJECT_BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x0109_d6c4);
        for offset in [0x1cu32, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        // The signalling NaN is quieted by the FLD/FSTP copy.
        assert_eq!(e.mem.u32(this + 0x38), 0x7fe0_0001);
        assert_eq!(e.mem.u32(this + 0x3c), 0x3f80_0000);
        assert_eq!(e.mem.u32(this + 0x40), 0x3f80_0000);
        assert_eq!(e.mem.u32(MATERIAL_COUNTER), 42);
        assert_eq!(e.mem.u32(this + 0x18), 42);
        assert_eq!(e.mem.u32(this + 0x44), 1);
        assert_eq!(e.mem.u32(this + 0x48), 0);
    }
}

#[cfg(test)]
mod material_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x011a_a000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        for double in [
            PROPERTY_COPY_MEMBERS,
            OBJECT_RECORD_CLONE,
            PROPERTY_LOAD_BINARY,
            OBJECT_NET_DESTRUCT,
            SIZED_DELETE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn copy_members_copies_the_colors_and_floats() {
        let mut e = engine();
        let this = e.mem.alloc(0x4c);
        let clone = e.mem.alloc(0x4c);
        for offset in (0x1cu32..0x44).step_by(4) {
            e.mem.set_u32(this + offset, 0x100 + offset);
        }
        e.mem.set_u32(this + 0x38, 0x7fa0_0001);
        e.call(0x00a7_55f0, &args![this, clone, 0x55u32]);
        assert_eq!(
            calls_to(&e, PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, 0x55]]
        );
        for offset in (0x1cu32..=0x34).step_by(4) {
            assert_eq!(e.mem.u32(clone + offset), 0x100 + offset);
        }
        assert_eq!(e.mem.u32(clone + 0x38), 0x7fe0_0001);
        assert_eq!(e.mem.u32(clone + 0x40), 0x140);
    }

    #[test]
    fn create_object_constructs_a_material() {
        let mut e = engine();
        let object = e.call(0x00a7_56d0, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_MATERIAL_PROPERTY_VTABLE);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x4c]]);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a7_56d0, &args![]).u32(), 0);
    }

    #[test]
    fn scalar_deleting_destructor_tells_the_listener() {
        let mut e = engine();
        let this = e.mem.alloc(0x4c);
        assert_eq!(e.call(0x00a7_56f0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_MATERIAL_PROPERTY_VTABLE);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x4c]]);
        let listener = e.mem.alloc(8);
        let mut slots = vec![0u32; 0x3e];
        slots[0xf4 / 4] = 0x0300_00f4;
        e.put_vtable(0x0300_0000, &slots);
        e.register(0x0300_00f4, |_, _| Ret::default());
        e.mem.set_u32(listener, 0x0300_0000);
        e.mem.set_u32(MATERIAL_LISTENER, listener);
        e.call(0x00a7_56f0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, 0x0300_00f4), vec![vec![listener, this]]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
        assert_eq!(calls_to(&e, OBJECT_NET_DESTRUCT).len(), 2);
    }

    #[test]
    fn create_clone_shares_a_material_without_controllers() {
        let mut e = engine();
        let this = e.mem.alloc(0x4c);
        assert_eq!(e.call(0x00a7_5730, &args![this, 0x66u32]).u32(), this);
        assert_eq!(
            calls_to(&e, OBJECT_RECORD_CLONE),
            vec![vec![this, this, 0x66]]
        );
        e.mem.set_u32(this + 0xc, 0x1234);
        e.mem.set_u32(this + 0x20, 0x77);
        let clone = e.call(0x00a7_5730, &args![this, 0x66u32]).u32();
        assert_ne!(clone, this);
        assert_eq!(e.mem.u32(clone), NI_MATERIAL_PROPERTY_VTABLE);
        assert_eq!(e.mem.u32(clone + 0x20), 0x77);
    }

    #[test]
    fn load_binary_reads_old_and_new_layouts() {
        let mut e = engine();
        e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0300_0000);
        let mut next = 1u32;
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], next);
            next += 1;
            4u32.into_ret()
        });
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(stream + 4, 0x19);
        e.call(0x00a7_5790, &args![this, stream]);
        let colors: Vec<u32> = calls_to(&e, COLOR_LOAD_BINARY)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(colors.len(), 4);
        assert_eq!(colors[0], colors[1]);
        assert_eq!(colors[2..], [this + 0x1c, this + 0x28]);
        assert_eq!(
            (
                e.mem.u32(this + 0x38),
                e.mem.u32(this + 0x3c),
                e.mem.u32(this + 0x40)
            ),
            (1, 2, 0)
        );
        e.mem.set_u32(stream + 4, 0x1a);
        e.call_log = Some(vec![]);
        e.call(0x00a7_5790, &args![this, stream]);
        assert_eq!(calls_to(&e, COLOR_LOAD_BINARY).len(), 2);
        assert_eq!(
            (
                e.mem.u32(this + 0x38),
                e.mem.u32(this + 0x3c),
                e.mem.u32(this + 0x40)
            ),
            (3, 4, 5)
        );
        assert_eq!(e.mem.u32(input + 4), 20);
    }

    #[test]
    fn release_of_the_default_material() {
        let mut e = engine();
        e.put_vtable(0x0300_1000, &[0, 0x0300_1004]);
        e.register(0x0300_1004, |_, _| Ret::default());
        let held = e.mem.alloc(0x4c);
        e.mem.set_u32(held, 0x0300_1000);
        e.mem.set_u32(held + 4, 2);
        e.mem.set_u32(MATERIAL_PROPERTY_SLOT, held);
        e.call(0x00a7_5a40, &args![]);
        assert_eq!(e.mem.u32(MATERIAL_PROPERTY_SLOT), 0);
        assert_eq!(e.mem.u32(held + 4), 1);
        assert!(calls_to(&e, 0x0300_1004).is_empty());
        e.mem.set_u32(MATERIAL_PROPERTY_SLOT, held);
        e.call(0x00a7_5a40, &args![]);
        assert_eq!(calls_to(&e, 0x0300_1004), vec![vec![held]]);
        e.call(0x00a7_5a40, &args![]);
        assert_eq!(calls_to(&e, 0x0300_1004).len(), 1);
    }
}
