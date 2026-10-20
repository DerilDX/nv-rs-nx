//! `(unplaced)/run_00bc3080` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00ba8a80`: the jump target of `00bc3770`.
const JUMP_TARGET_00BC3770: u32 = 0x00ba_8a80;

// Translated from 00bc3770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00ba8a80`, passing on `this` and its one argument; returns the target result. Return type (bool): its jump chain ends in `00a5c180` (`NiObjectNET::RegisterStreamables`, engine map, `bool` in the Xbox PDB).
pub fn fn_00bc3770(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00BC3770, &args![this, stream]).bool()
}

// The three float interpolator controllers of this unit (each 0x38 bytes,
// an `NiFloatInterpController` base; class names from the `NiRTTI` records
// their static initializers `00fbaa30`, `00fbaa60`, `00fbaa90` build):
// `BSMaterialEmittanceMultController` (vtable `010ba858`, `NiRTTI` at
// `01201f90`), `BSRefractionFirePeriodController` (`010ba948`, `01201f9c`)
// and `BSRefractionStrengthController` (`010baa38`, `01201fa8`). They share
// the deleting destructor `00bc3350` and `LoadBinary` `00bc3540`.

/// The three vtables.
const EMITTANCE_MULT_VTABLE: u32 = 0x010b_a858;
const FIRE_PERIOD_VTABLE: u32 = 0x010b_a948;
const REFRACTION_STRENGTH_VTABLE: u32 = 0x010b_aa38;
/// The three `NiRTTI` records.
const EMITTANCE_MULT_RTTI: u32 = 0x0120_1f90;
const FIRE_PERIOD_RTTI: u32 = 0x0120_1f9c;
const REFRACTION_STRENGTH_RTTI: u32 = 0x0120_1fa8;
/// The controllers' size.
const CONTROLLER_SIZE: u32 = 0x38;
/// `NiTimeController::m_uFlags` (+8), `m_fScaledTime` (+0x28), `m_pkTarget`
/// (+0x2C) and `NiSingleInterpController::m_spInterpolator` (+0x34), Xbox
/// PDB names, the same offsets on PC.
const FLAGS: u32 = 0x08;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
const INTERPOLATOR: u32 = 0x34;
/// The float stored as the scaled time while flag bit 5 is set (`01096324`).
const DISABLED_TIME: u32 = 0x0109_6324;
/// `NiTimeController::DontDoUpdate` (`00a36250`; `thiscall`, one float).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// Allocation (`00aa13e0`, `cdecl`, size) and the sized free (`00aa1460`,
/// `cdecl`, pointer and size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// The base constructor (`00a57640`, `thiscall`), the base member copy
/// (`00a57620`, `thiscall`: clone, cloning process), the base `LoadBinary`
/// (`00a57630`, `thiscall`: stream) and the destructor body (`00a360d0`:
/// releases the interpolator, then the `NiTimeController` destructor).
const BASE_CONSTRUCTOR: u32 = 0x00a5_7640;
const BASE_COPY_MEMBERS: u32 = 0x00a5_7620;
const BASE_LOAD_BINARY: u32 = 0x00a5_7630;
const DESTRUCTOR_BODY: u32 = 0x00a3_60d0;
/// `00a63430` (`thiscall` on the stream, no stack word; the next linked
/// object, by its use here) and `00a3c340` (`thiscall`, one word: a 0x18-byte
/// interpolator built on that object).
const STREAM_NEXT_LINKED_OBJECT: u32 = 0x00a6_3430;
const INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a3_c340;
/// `InterlockedDecrement` / `InterlockedIncrement` (import slots).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `_ftol2` (`00ec62c0`): the value in ST0 as a leading `f64`.
const FTOL: u32 = 0x00ec_62c0;
/// `0.5` and `-0.5` (floats): the rounding offsets of the fire period.
const HALF: u32 = 0x0101_6248;
const MINUS_HALF: u32 = 0x0102_295c;
/// The refraction strength setter on the target (`00b66700`, `thiscall`,
/// one float).
const SET_REFRACTION_STRENGTH: u32 = 0x00b6_6700;

/// The update every controller of this unit shares (`thiscall`, one word:
/// the update data, whose first float is the time). With flag bit 5 set the
/// scaled time becomes the float at `01096324`; otherwise, when
/// `DontDoUpdate(time)` answers true, it goes on only with an interpolator
/// whose vtable slot +0xD4 answers true. Then the interpolator's float update
/// (slot +0x9C: scaled time, target, the address of the argument's own stack
/// slot, which receives the value) runs; `apply` gets the slot's bits when it
/// succeeds.
fn update_with(
    e: &mut Engine,
    this: u32,
    update_data: u32,
    apply: impl FnOnce(&mut Engine, u32, u32),
) {
    if (e.mem.u8(this + FLAGS) >> 5) & 1 != 0 {
        let time = x87_quiet_bits(e.mem.u32(DISABLED_TIME));
        e.mem.set_u32(this + SCALED_TIME, time);
    } else {
        let time = x87_quiet_bits(e.mem.u32(update_data));
        if e.call(DONT_DO_UPDATE, &args![this, time]).bool() {
            let interpolator = e.mem.u32(this + INTERPOLATOR);
            if interpolator == 0 || !e.vcall(interpolator, 0xd4, &[]).bool() {
                return;
            }
        }
    }
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    if interpolator == 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, update_data);
        let time = x87_quiet_bits(e.mem.u32(this + SCALED_TIME));
        let target = e.mem.u32(this + TARGET);
        if e.vcall(interpolator, 0x9c, &args![time, target, slot])
            .bool()
        {
            apply(e, this, slot);
        }
    });
}

/// The shared `CreateClone` body: a new controller (0x38 bytes; the base
/// constructor and `vtable`, or null when the allocation fails), then the
/// base members copied from `this`.
fn clone_with(e: &mut Engine, this: u32, cloning: u32, vtable: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![CONTROLLER_SIZE]).u32();
    let clone = if block == 0 {
        0
    } else {
        e.call(BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, vtable);
        block
    };
    e.call(BASE_COPY_MEMBERS, &args![this, clone, cloning]);
    clone
}

/// The shared `CreateObject` body: a new controller (base constructor and
/// `vtable`), or null when the allocation fails.
fn create_with(e: &mut Engine, vtable: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![CONTROLLER_SIZE]).u32();
    if block == 0 {
        return 0;
    }
    e.call(BASE_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, vtable);
    block
}

// Translated from 00bc3080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMaterialEmittanceMultController::Update` (vtable slot +0x94; name
/// from its `NiRTTI` record): the shared update; the value goes to the
/// target's float at +0x40 (through the x87 stack) and the target's counter
/// at +0x44 goes up by one (fields of the target property unnamed here).
pub fn fn_00bc3080(e: &mut Engine, this: Ptr, update_data: u32) {
    update_with(e, this.addr(), update_data, |e, this, slot| {
        let target = e.mem.u32(this + TARGET);
        let value = x87_quiet_bits(e.mem.u32(slot));
        let count = e.mem.u32(target + 0x44);
        e.mem.set_u32(target + 0x44, count.wrapping_add(1));
        e.mem.set_u32(target + 0x40, value);
    });
}

// Translated from 00bc3100 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `BSMaterialEmittanceMultController::GetRTTI` (vtable slot +8; Ghidra has
/// no function here): the class's `NiRTTI` record.
pub fn fn_00bc3100(_e: &mut Engine) -> u32 {
    EMITTANCE_MULT_RTTI
}

// Translation incomplete: 00bc3110 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc3110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMaterialEmittanceMultController::CreateClone` (vtable slot +0x48): a
/// new controller (0x38 bytes, `00aa13e0`, the base constructor `00a57640`
/// and this class's vtable; null when the allocation fails) with the base
/// members copied (`00a57620`).
pub fn fn_00bc3110(e: &mut Engine, this: Ptr, cloning: u32) -> u32 {
    clone_with(e, this.addr(), cloning, EMITTANCE_MULT_VTABLE)
}

// Translation incomplete: 00bc3190 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc3190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMaterialEmittanceMultController::CreateObject` (by its body): a new
/// controller with this class's vtable, or null when the allocation fails.
pub fn fn_00bc3190(e: &mut Engine) -> u32 {
    create_with(e, EMITTANCE_MULT_VTABLE)
}

// Translated from 00bc3290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionFirePeriodController::Update` (vtable slot +0x94): the
/// shared update; the value is rounded half away from zero (`+0.5` when
/// above 0, else `-0.5`, stored as a float, then truncated by `_ftol2`) into
/// the target's integer at +0xE4.
pub fn fn_00bc3290(e: &mut Engine, this: Ptr, update_data: u32) {
    update_with(e, this.addr(), update_data, |e, this, slot| {
        let value = x87_load32(e.mem.u32(slot));
        // FLDZ; FLD value; FCOM: C0 or C3 (or unordered) picks -0.5.
        let offset = if value > 0.0 { HALF } else { MINUS_HALF };
        let offset = x87_quiet_bits(e.mem.u32(offset));
        e.mem.set_u32(slot, offset);
        let sum = x87_add(value, x87_m32(e.mem.u32(slot)));
        e.mem.set_u32(slot, x87_store32(sum));
        let rounded = x87_load32(e.mem.u32(slot));
        let whole = e.call(FTOL, &args![rounded]).u32();
        let target = e.mem.u32(this + TARGET);
        e.mem.set_u32(target + 0xe4, whole);
    });
}

// Translated from 00bc3340 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `BSRefractionFirePeriodController::GetRTTI` (vtable slot +8; Ghidra has
/// no function here): the class's `NiRTTI` record.
pub fn fn_00bc3340(_e: &mut Engine) -> u32 {
    FIRE_PERIOD_RTTI
}

// Translated from 00bc3350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor the three controllers share (vtable slot
/// 0): the destructor body (`00a360d0`), then with bit 0 of `flags` the
/// 0x38 bytes freed (`00aa1460`). Returns `this`.
pub fn fn_00bc3350(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, CONTROLLER_SIZE]);
    }
    this
}

// Translation incomplete: 00bc3370 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc3370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionFirePeriodController::CreateClone` (vtable slot +0x48), as
/// `00bc3110` with this class's vtable.
pub fn fn_00bc3370(e: &mut Engine, this: Ptr, cloning: u32) -> u32 {
    clone_with(e, this.addr(), cloning, FIRE_PERIOD_VTABLE)
}

// Translation incomplete: 00bc33f0 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc33f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionFirePeriodController::CreateObject` (by its body), as
/// `00bc3190` with this class's vtable.
pub fn fn_00bc33f0(e: &mut Engine) -> u32 {
    create_with(e, FIRE_PERIOD_VTABLE)
}

// Translated from 00bc34c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionStrengthController::Update` (vtable slot +0x94): the shared
/// update; the value goes to the target through `00b66700` (`thiscall`, one
/// float).
pub fn fn_00bc34c0(e: &mut Engine, this: Ptr, update_data: u32) {
    update_with(e, this.addr(), update_data, |e, this, slot| {
        let value = x87_quiet_bits(e.mem.u32(slot));
        let target = e.mem.u32(this + TARGET);
        e.call(SET_REFRACTION_STRENGTH, &args![target, value]);
    });
}

// Translation incomplete: 00bc3540 - the FS:[0] exception frame (unwinding when the interpolator constructor throws) is not translated
// Translated from 00bc3540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LoadBinary` the three controllers share (vtable slot +0x50): the
/// base `LoadBinary` (`00a57630`); for files older than `0x0A010068` (the
/// stream's version at +0xD8) a 0x18-byte interpolator is built
/// (`00a3c340`) on the stream's next linked object (`00a63430`), or null
/// when the allocation fails, and becomes `m_spInterpolator` (the old one
/// released, `DeleteThis` at a zero count, the new one's count raised),
/// whose vtable slot +0xBC then runs (no arguments; through a null
/// interpolator as the exe does).
pub fn fn_00bc3540(e: &mut Engine, this: Ptr, stream: u32) {
    let this = this.addr();
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream + 0xd8) >= 0x0a01_0068 {
        return;
    }
    let data = e.call(STREAM_NEXT_LINKED_OBJECT, &args![stream]).u32();
    let block = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
    let interpolator = if block == 0 {
        0
    } else {
        e.call(INTERPOLATOR_CONSTRUCTOR, &args![block, data]).u32()
    };
    let old = e.mem.u32(this + INTERPOLATOR);
    if old != interpolator {
        if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
            e.vcall(old, 0x4, &[]);
        }
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        if interpolator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![interpolator + 4]);
        }
    }
    let current = e.mem.u32(this + INTERPOLATOR);
    e.vcall(current, 0xbc, &[]);
}

// Translated from 00bc3610 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `BSRefractionStrengthController::GetRTTI` (vtable slot +8; Ghidra has no
/// function here): the class's `NiRTTI` record.
pub fn fn_00bc3610(_e: &mut Engine) -> u32 {
    REFRACTION_STRENGTH_RTTI
}

// Translation incomplete: 00bc3620 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc3620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionStrengthController::CreateClone` (vtable slot +0x48), as
/// `00bc3110` with this class's vtable.
pub fn fn_00bc3620(e: &mut Engine, this: Ptr, cloning: u32) -> u32 {
    clone_with(e, this.addr(), cloning, REFRACTION_STRENGTH_VTABLE)
}

// Translation incomplete: 00bc36a0 - the FS:[0] exception frame (unwinding when the constructor throws) is not translated
// Translated from 00bc36a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRefractionStrengthController::CreateObject` (by its body), as
/// `00bc3190` with this class's vtable.
pub fn fn_00bc36a0(e: &mut Engine) -> u32 {
    create_with(e, REFRACTION_STRENGTH_VTABLE)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_3770, fn_00bc3770(Ptr, Ptr) -> bool),
        entry!(0x00bc3080, fn_00bc3080(Ptr, u32)),
        entry!(0x00bc3100, fn_00bc3100() -> u32),
        entry!(0x00bc3110, fn_00bc3110(Ptr, u32) -> u32),
        entry!(0x00bc3190, fn_00bc3190() -> u32),
        entry!(0x00bc3290, fn_00bc3290(Ptr, u32)),
        entry!(0x00bc3340, fn_00bc3340() -> u32),
        entry!(0x00bc3350, fn_00bc3350(Ptr, u32) -> Ptr),
        entry!(0x00bc3370, fn_00bc3370(Ptr, u32) -> u32),
        entry!(0x00bc33f0, fn_00bc33f0() -> u32),
        entry!(0x00bc34c0, fn_00bc34c0(Ptr, u32)),
        entry!(0x00bc3540, fn_00bc3540(Ptr, u32)),
        entry!(0x00bc3610, fn_00bc3610() -> u32),
        entry!(0x00bc3620, fn_00bc3620(Ptr, u32) -> u32),
        entry!(0x00bc36a0, fn_00bc36a0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00bc3770_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00bc_3770;
        const TARGET: u32 = 0x00ba_8a80;
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

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A controller with an interpolator whose float update (slot +0x9C)
    /// writes `value` into its out slot and answers true, and a target.
    fn controller(e: &mut Engine, value: f32) -> (u32, u32) {
        e.map(0x0109_6000, 0x1000);
        e.set_global(DISABLED_TIME, -2.0f32);
        e.register(DONT_DO_UPDATE, |_, _| Ret::default());
        e.register_double(0x0015_0000, move |e, a| {
            e.mem.set_f32(a[3], value);
            1u32.into_ret()
        });
        e.register(0x0015_0010, |_, _| 0u32.into_ret());
        let vtable = e.mem.alloc(0xd8);
        e.mem.set_u32(vtable + 0x9c, 0x0015_0000);
        e.mem.set_u32(vtable + 0xd4, 0x0015_0010);
        let interpolator = e.mem.alloc(8);
        e.mem.set_u32(interpolator, vtable);
        let this = e.mem.alloc(CONTROLLER_SIZE);
        let target = e.mem.alloc(0x100);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.mem.set_u32(this + TARGET, target);
        e.mem.set_f32(this + SCALED_TIME, 1.5);
        (this, target)
    }

    #[test]
    fn emittance_update_stores_the_value_and_counts() {
        let mut e = Engine::new();
        let (this, target) = controller(&mut e, 0.25);
        let data = e.mem.alloc(4);
        e.mem.set_f32(data, 4.0);
        e.call_log = Some(vec![]);
        e.call(0x00bc_3080, &args![this, data]);
        assert_eq!(
            calls_to(&e, DONT_DO_UPDATE),
            vec![vec![this, 4.0f32.to_bits()]]
        );
        let update = calls_to(&e, 0x0015_0000);
        assert_eq!(
            &update[0][..3],
            [e.mem.u32(this + INTERPOLATOR), 1.5f32.to_bits(), target]
        );
        assert_eq!(e.mem.f32(target + 0x40), 0.25);
        assert_eq!(e.mem.u32(target + 0x44), 1);
        // Flag bit 5: the disabled time is the scaled time.
        e.mem.set_u16(this + FLAGS, 0x20);
        e.call(0x00bc_3080, &args![this, data]);
        assert_eq!(e.mem.f32(this + SCALED_TIME), -2.0);
        assert_eq!(e.mem.u32(target + 0x44), 2);
        // DontDoUpdate true and an inactive interpolator: nothing.
        e.mem.set_u16(this + FLAGS, 0);
        e.register(DONT_DO_UPDATE, |_, _| 1u32.into_ret());
        e.call(0x00bc_3080, &args![this, data]);
        assert_eq!(e.mem.u32(target + 0x44), 2);
    }

    #[test]
    fn fire_period_update_rounds_half_away_from_zero() {
        for (value, expected) in [(2.5f32, 3i32), (2.4, 2), (-2.5, -3), (0.0, 0)] {
            let mut e = Engine::new();
            e.map(0x0101_6000, 0x1000);
            e.set_global(HALF, 0.5f32);
            e.map(0x0102_2000, 0x1000);
            e.set_global(MINUS_HALF, -0.5f32);
            e.register(FTOL, |_, a| {
                let value = f64::take(a, &mut 0);
                (value as i32).into_ret()
            });
            let (this, target) = controller(&mut e, value);
            let data = e.mem.alloc(4);
            e.call(0x00bc_3290, &args![this, data]);
            assert_eq!(e.mem.u32(target + 0xe4) as i32, expected, "{value}");
        }
    }

    #[test]
    fn strength_update_calls_the_setter() {
        let mut e = Engine::new();
        let (this, target) = controller(&mut e, 0.75);
        e.register(SET_REFRACTION_STRENGTH, |_, _| Ret::default());
        let data = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00bc_34c0, &args![this, data]);
        assert_eq!(
            calls_to(&e, SET_REFRACTION_STRENGTH),
            vec![vec![target, 0.75f32.to_bits()]]
        );
    }

    #[test]
    fn rtti_create_and_clone_use_each_class_vtable() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00bc_3100, &[]).u32(), EMITTANCE_MULT_RTTI);
        assert_eq!(e.call(0x00bc_3340, &[]).u32(), FIRE_PERIOD_RTTI);
        assert_eq!(e.call(0x00bc_3610, &[]).u32(), REFRACTION_STRENGTH_RTTI);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for (create, clone, vtable) in [
            (0x00bc_3190u32, 0x00bc_3110u32, EMITTANCE_MULT_VTABLE),
            (0x00bc_33f0, 0x00bc_3370, FIRE_PERIOD_VTABLE),
            (0x00bc_36a0, 0x00bc_3620, REFRACTION_STRENGTH_VTABLE),
        ] {
            let object = e.call(create, &[]).u32();
            assert_eq!(e.mem.u32(object), vtable);
            let copy = e.call(clone, &args![object, 0x77u32]).u32();
            assert_eq!(e.mem.u32(copy), vtable);
            assert!(calls_to(&e, BASE_COPY_MEMBERS).contains(&vec![object, copy, 0x77]));
        }
        // A failed allocation: null, and the clone copies into null.
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00bc_3190, &[]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_3110, &args![0x1234u32, 0u32]).u32(), 0);
        assert_eq!(calls_to(&e, BASE_COPY_MEMBERS), vec![vec![0x1234, 0, 0]]);
    }

    #[test]
    fn destructor_frees_on_flag() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_BODY, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_3350, &args![0x5000u32, 1u32]).u32(), 0x5000);
        assert_eq!(
            calls_to(&e, NI_FREE_SIZED),
            vec![vec![0x5000, CONTROLLER_SIZE]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00bc_3350, &args![0x5000u32, 0u32]);
        assert_eq!(calls_to(&e, DESTRUCTOR_BODY).len(), 1);
        assert!(calls_to(&e, NI_FREE_SIZED).is_empty());
    }

    #[test]
    fn load_binary_builds_an_interpolator_for_old_files() {
        let mut e = Engine::new();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_NEXT_LINKED_OBJECT, |_, _| 0x6000u32.into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(INTERPOLATOR_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x0016_0000);
            a[0].into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x0016_1000, |_, _| Ret::default());
        e.register(0x0016_1004, |_, _| Ret::default());
        e.put_vtable(0x0016_0000, &[0, 0x0016_1004]);
        e.mem.map(0x0016_00bc, 4);
        e.mem.set_u32(0x0016_00bc, 0x0016_1000);
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, 0x0016_0000);
        e.mem.set_u32(old + 4, 1);
        let this = e.mem.alloc(CONTROLLER_SIZE);
        e.mem.set_u32(this + INTERPOLATOR, old);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 0xd8, 0x0a01_0067);
        e.call_log = Some(vec![]);
        e.call(0x00bc_3540, &args![this, stream]);
        let made = e.mem.u32(this + INTERPOLATOR);
        assert_ne!(made, old);
        assert_eq!(e.mem.u32(made + 4), 1);
        assert_eq!(
            calls_to(&e, INTERPOLATOR_CONSTRUCTOR),
            vec![vec![made, 0x6000]]
        );
        assert_eq!(calls_to(&e, 0x0016_1004), vec![vec![old]]);
        assert_eq!(calls_to(&e, 0x0016_1000), vec![vec![made]]);
        // A current file: only the base LoadBinary.
        e.mem.set_u32(stream + 0xd8, 0x0a01_0068);
        e.call_log = Some(vec![]);
        e.call(0x00bc_3540, &args![this, stream]);
        assert_eq!(calls_to(&e, BASE_LOAD_BINARY).len(), 1);
        assert!(calls_to(&e, STREAM_NEXT_LINKED_OBJECT).is_empty());
    }
}
