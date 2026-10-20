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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
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
