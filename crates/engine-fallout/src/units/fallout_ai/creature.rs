//! `fallout/ai/creature.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d43a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d43a0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `Creature::Creature` (Xbox PDB name in the engine map; the RTTI name of the vtable it stores at `+0`, `0x010870ac`, is
/// `Creature`): the constructor with no parameter. It runs the `Actor` constructor (`0087d4a0(this, 1)`), stores the six
/// vtable pointers (`+0`, `+0x18`, `+0x88`, `+0x94`, `+0xa4`, `+0xa8`), registers the form type 0x3c (`004f15a0`), clears
/// the byte at `+0x1b4`, copies the float at `01012054` into `+0x1b8` and `+0x1bc` and clears the words at `+0x140` and
/// `+0x13c`. The exception-unwinding frame is not translated.
pub fn creature_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0087_d4a0, &args![this, 1u32]);
    e.mem.set_u32(base, 0x0108_70ac);
    e.mem.set_u32(base + 0x18, 0x0108_70a0);
    e.mem.set_u32(base + 0x88, 0x0108_704c);
    e.mem.set_u32(base + 0x94, 0x0108_7014);
    e.mem.set_u32(base + 0xa4, 0x0108_6fe4);
    e.mem.set_u32(base + 0xa8, 0x0108_6f9c);
    e.call(0x004f_15a0, &args![this, 0x3cu32]);
    e.mem.set_u8(base + 0x1b4, 0);
    let first = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(base + 0x1b8, first);
    let second = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(base + 0x1bc, second);
    e.mem.set_u32(base + 0x140, 0);
    e.mem.set_u32(base + 0x13c, 0);
    this
}

// Translated from 008d4500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d4500 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `Creature::Creature_ov2` (Xbox PDB name in the engine map): the constructor `CreateReference` (`00564480`) runs for
/// `ACRE`. Same as [`creature_constructor`] except that the byte parameter (`RET 4`) goes to the `Actor` constructor instead
/// of the constant 1. The exception-unwinding frame is not translated.
pub fn creature_constructor_ov2(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let base = this.addr();
    e.call(0x0087_d4a0, &args![this, flag & 0xff]);
    e.mem.set_u32(base, 0x0108_70ac);
    e.mem.set_u32(base + 0x18, 0x0108_70a0);
    e.mem.set_u32(base + 0x88, 0x0108_704c);
    e.mem.set_u32(base + 0x94, 0x0108_7014);
    e.mem.set_u32(base + 0xa4, 0x0108_6fe4);
    e.mem.set_u32(base + 0xa8, 0x0108_6f9c);
    e.call(0x004f_15a0, &args![this, 0x3cu32]);
    e.mem.set_u8(base + 0x1b4, 0);
    let first = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(base + 0x1b8, first);
    let second = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(base + 0x1bc, second);
    e.mem.set_u32(base + 0x140, 0);
    e.mem.set_u32(base + 0x13c, 0);
    this
}

/// The vtables the destructor stores, by offset (`Creature` and its bases).
const CREATURE_VTABLES: [(u32, u32); 6] = [
    (0x00, 0x0108_70ac),
    (0x18, 0x0108_70a0),
    (0x88, 0x0108_704c),
    (0x94, 0x0108_7014),
    (0xa4, 0x0108_6fe4),
    (0xa8, 0x0108_6f9c),
];
/// `004077c0` (`thiscall`, a `bool` in AL; no Xbox PDB name): when it
/// answers false the destructor clears the 3D first.
const DESTRUCTOR_TEST_004077C0: u32 = 0x0040_77c0;
/// `Actor::~Actor` (`0087dcb0`, engine map; `thiscall`).
const ACTOR_DESTRUCTOR: u32 = 0x0087_dcb0;
/// The game's `operator delete` (`00401030`, `cdecl`).
const FREE: u32 = 0x0040_1030;
/// `TESObjectREFR::Set3D` (`005702e0`, engine map; `thiscall`, two words:
/// node, byte).
const REFR_SET_3D: u32 = 0x0057_02e0;
/// `004181e0` (`thiscall`): the reference's base object.
const GET_BASE_OBJECT: u32 = 0x0041_81e0;
/// `0047fef0` (`cdecl`, the base object; a 16-bit result; no Xbox PDB name).
const BASE_SHORT_0047FEF0: u32 = 0x0047_fef0;
/// `Actor::GetFatiguePercentage` (`00893530`, engine map; `thiscall`, no
/// stack words; result in ST0).
const ACTOR_GET_FATIGUE_PERCENTAGE: u32 = 0x0089_3530;
/// `00646250` (`cdecl`, two words: a `float` it ignores, an integer; the
/// integer in ST0).
const INT_TO_ST0_00646250: u32 = 0x0064_6250;
/// `_ftol2` (`00ec62c0`): the `f64` in ST0 truncated to an integer.
const FTOL: u32 = 0x00ec_62c0;
/// `00821640` (`thiscall` on the base object, result in ST0; no Xbox PDB
/// name).
const BASE_VALUE_00821640: u32 = 0x0082_1640;
/// `0.0` (`double`).
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// `00403e20` (`thiscall` on a setting): the address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The setting [`fn_008d47f0`] falls back to.
const FALLBACK_SETTING_011CFABC: u32 = 0x011c_fabc;
/// `00880580` (`thiscall` on the actor value owner, one word; no Xbox PDB
/// name).
const OWNER_CALL_00880580: u32 = 0x0088_0580;
/// `008805f0` (`thiscall` on the actor value owner, one word; no Xbox PDB
/// name).
const OWNER_CALL_008805F0: u32 = 0x0088_05f0;
/// `00880ad0` (`thiscall` on the actor value owner, three words; no Xbox
/// PDB name).
const OWNER_CALL_00880AD0: u32 = 0x0088_0ad0;
/// `00406d70` (`cdecl`: actor value, mask): whether the actor value's info
/// (table `011d61c8`) has any of the mask's flag bits (+0x44).
const ACTOR_VALUE_HAS_FLAGS: u32 = 0x0040_6d70;
/// `004a8b00` (`cdecl`, a form; no Xbox PDB name).
const FORM_TEST_004A8B00: u32 = 0x004a_8b00;
/// The recursion depth counter of [`fn_008d4ab0`].
const DEPTH_011DFFE4: u32 = 0x011d_ffe4;
/// `0045bc00` (`thiscall`, one word; no Xbox PDB name).
const NEXT_FORM_0045BC00: u32 = 0x0045_bc00;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor `.?AVTESActorBase@@`.
const TYPE_TES_ACTOR_BASE: u32 = 0x0118_46e8;
/// RTTI type descriptor `.?AVTESCreature@@`.
const TYPE_TES_CREATURE: u32 = 0x0118_3a00;
/// `00496940` (`thiscall`; no Xbox PDB name).
const HOLDER_00496940: u32 = 0x0049_6940;
/// `00537bd0` (`thiscall`; no Xbox PDB name).
const NODE_00537BD0: u32 = 0x0053_7bd0;
/// The three name getters [`creature_check_missing_limbs`] uses: the words
/// at `011c6250`, `011c6258` and `011c61ac`.
const NAME_008D4CA0: u32 = 0x008d_4ca0;
const NAME_008D4CB0: u32 = 0x008d_4cb0;
const NAME_008A30F0: u32 = 0x008a_30f0;
/// `TESHavokUtilities::RemoveHavokFromSceneGraph` (`0062bc10`, engine map;
/// `cdecl`).
const REMOVE_HAVOK_FROM_SCENE_GRAPH: u32 = 0x0062_bc10;

// Translated from 008d4490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at +0x1B4 (`thiscall`, one word; no Xbox PDB name).
pub fn fn_008d4490(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x1b4, value);
}

// Translated from 008d44b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x1B4 (`thiscall`; no Xbox PDB name).
pub fn fn_008d44b0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x1b4)
}

// Translated from 008d44d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `Creature` scalar deleting destructor (`thiscall`, one word: flags):
/// [`fn_008d45f0`], then with flag bit 0 the memory is freed. Returns
/// `this`.
pub fn fn_008d44d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008d45f0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 008d45f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d45f0 - the C++ exception unwinding frame (FS:[0], handler 00f15d38) is not translated
/// The `Creature` destructor (`thiscall`; no Xbox PDB name): stores the six
/// vtables, clears the 3D ([`fn_008d47d0`] with 0, 0) unless `004077c0`
/// answers true, then runs `Actor::~Actor`. The exception-unwinding frame is
/// not translated.
pub fn fn_008d45f0(e: &mut Engine, this: Ptr) {
    for (offset, vtable) in CREATURE_VTABLES {
        e.mem.set_u32(this.addr() + offset, vtable);
    }
    if !e.call(DESTRUCTOR_TEST_004077C0, &args![this]).bool() {
        fn_008d47d0(e, this, 0, 0);
    }
    e.call(ACTOR_DESTRUCTOR, &args![this]);
}

// Translated from 008d46a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An integer from the creature (`thiscall`; no Xbox PDB name):
/// `0047fef0(base object)` as a 16-bit value and `Actor::GetFatiguePercentage`
/// are handed to `00646250` (which loads the integer), its result stored as
/// a `float` and truncated by `_ftol2`.
pub fn fn_008d46a0(e: &mut Engine, this: Ptr) -> i32 {
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let word = e.call(BASE_SHORT_0047FEF0, &args![base]).u16();
    let fatigue = e.call(ACTOR_GET_FATIGUE_PERCENTAGE, &args![this]).f64();
    let fatigue = x87_quiet_bits(x87_store32(fatigue));
    let value = e
        .call(INT_TO_ST0_00646250, &args![fatigue, u32::from(word)])
        .f64();
    let value = x87_load32(x87_store32(value));
    e.call(FTOL, &args![value]).i32()
}

// Translated from 008d47d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Set3D(node, byte)` on the creature (`thiscall`, two
/// words; no Xbox PDB name).
pub fn fn_008d47d0(e: &mut Engine, this: Ptr, node: u32, flag: u8) {
    e.call(REFR_SET_3D, &args![this, node, u32::from(flag)]);
}

// Translated from 008d47f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `float` of the creature's base object (`00821640`), or the setting at
/// `011cfabc` when that is 0 or there is no base object (`thiscall`, result
/// in ST0; no Xbox PDB name).
pub fn fn_008d47f0(e: &mut Engine, this: Ptr) -> f32 {
    let mut value = x87_store32(0.0);
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if base != 0 {
        let result = e.call(BASE_VALUE_00821640, &args![base]).f64();
        value = x87_store32(result);
    }
    let zero: f64 = e.global(DOUBLE_ZERO);
    // FCOMP with the `double` 0, TEST AH,0x44 / JP: only an equal value
    // takes the setting.
    if x87_load32(value) == zero {
        let setting = e
            .call(SETTING_VALUE, &args![FALLBACK_SETTING_011CFABC])
            .u32();
        value = x87_quiet_bits(e.mem.u32(setting));
    }
    f32::from_bits(x87_quiet_bits(value))
}

// Translated from 008d4840 (decompiled, FalloutNV.exe 1.4.0.525)
/// On the creature's actor value owner (`this`, at +0xA4; `thiscall`, one
/// word; no Xbox PDB name): `00880580` with the actor value mapped by
/// [`fn_008d4a80`].
pub fn fn_008d4840(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let mapped = fn_008d4a80(e, Ptr::new(this.addr().wrapping_sub(0xa4)), index);
    e.call(OWNER_CALL_00880580, &args![this, mapped]).u32()
}

// Translated from 008d4870 (decompiled, FalloutNV.exe 1.4.0.525)
/// On the creature's actor value owner (`this`, at +0xA4; `thiscall`, one
/// word; no Xbox PDB name): `008805f0` with the actor value mapped by
/// [`fn_008d4a80`].
pub fn fn_008d4870(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let mapped = fn_008d4a80(e, Ptr::new(this.addr().wrapping_sub(0xa4)), index);
    e.call(OWNER_CALL_008805F0, &args![this, mapped]).u32()
}

// Translated from 008d4930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00880ad0(mapped actor value, value, extra)` on `this` (`thiscall`,
/// three words; no Xbox PDB name), the actor value mapped by
/// [`fn_008d4a80`].
pub fn fn_008d4930(e: &mut Engine, this: Ptr, index: u32, value: f32, extra: u32) -> u32 {
    let mapped = fn_008d4a80(e, this, index);
    e.call(
        OWNER_CALL_00880AD0,
        &args![this, mapped, x87_quiet_bits(value.to_bits()), extra],
    )
    .u32()
}

// Translated from 008d4a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Maps an actor value for a creature (`thiscall`, one word; `this` is not
/// read; no Xbox PDB name): 0x2D when the actor value's info has flag 0x400
/// (`00406d70`), else the actor value itself.
pub fn fn_008d4a80(e: &mut Engine, _this: Ptr, index: u32) -> u32 {
    if e.call(ACTOR_VALUE_HAS_FLAGS, &args![index, 0x400u32])
        .bool()
    {
        0x2d
    } else {
        index
    }
}

// Translated from 008d4ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a form or one it leads to passes `004a8b00` (`thiscall`, one
/// word; no Xbox PDB name): false without a form; with the depth counter
/// at `011dffe4` raised for the call, true when `004a8b00` answers non-zero,
/// else, while the depth is below 4, the answer for `0045bc00(0)` of the
/// form's vtable +0x0C object (when there is one).
pub fn fn_008d4ab0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let mut result = false;
    if form == 0 {
        return false;
    }
    let found = e.call(FORM_TEST_004A8B00, &args![form]).u32();
    let depth: u32 = e.global(DEPTH_011DFFE4);
    e.set_global(DEPTH_011DFFE4, depth.wrapping_add(1));
    if found != 0 {
        result = true;
    } else if (e.global::<u32>(DEPTH_011DFFE4) as i32) < 4 {
        let object = if form != 0 {
            e.vcall(form, 0xc, &args![]).u32()
        } else {
            0
        };
        if object != 0 {
            let next = e.call(NEXT_FORM_0045BC00, &args![object, 0u32]).u32();
            result = fn_008d4ab0(e, this, next);
        }
    }
    let depth: u32 = e.global(DEPTH_011DFFE4);
    e.set_global(DEPTH_011DFFE4, depth.wrapping_sub(1));
    result
}

/// Runs `0062bc10` on the vtable +0x8C object of `node` for the name a
/// getter returns.
fn remove_named(e: &mut Engine, node: u32, getter: u32) {
    let name = e.call(getter, &args![]).u32();
    let object = e.vcall(node, 0x8c, &args![name]).u32();
    e.call(REMOVE_HAVOK_FROM_SCENE_GRAPH, &args![object]);
}

// Translated from 008d4b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Creature::CheckMissingLimbs` (Xbox PDB, matched by the engine map;
/// `thiscall`): for a `TESCreature` base object and a vtable +0x1E4 object
/// whose `00496940` holder exists, takes the node `00537bd0` gives and, for
/// each of the base object's +0x30 component vtable slots +0x38, +0x3C and
/// +0x40 that answers true, runs `TESHavokUtilities::RemoveHavokFromSceneGraph`
/// on the node's vtable +0x8C object named by `008d4ca0`, `008d4cb0` and
/// `008a30f0` respectively.
pub fn creature_check_missing_limbs(e: &mut Engine, this: Ptr) {
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let creature = e
        .call(
            RT_DYNAMIC_CAST,
            &args![base, 0u32, TYPE_TES_ACTOR_BASE, TYPE_TES_CREATURE, 0u32],
        )
        .u32();
    if creature == 0 {
        return;
    }
    let holder_owner = e.vcall(this.addr(), 0x1e4, &args![]).u32();
    if holder_owner == 0 {
        return;
    }
    if e.call(HOLDER_00496940, &args![holder_owner]).u32() == 0 {
        return;
    }
    let holder = e.call(HOLDER_00496940, &args![holder_owner]).u32();
    let node = e.call(NODE_00537BD0, &args![holder]).u32();
    let component = creature + 0x30;
    for (slot, getter) in [
        (0x38, NAME_008D4CA0),
        (0x3c, NAME_008D4CB0),
        (0x40, NAME_008A30F0),
    ] {
        if e.vcall(component, slot, &args![]).bool() {
            remove_named(e, node, getter);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_43a0, creature_constructor(Ptr) -> Ptr),
        entry!(0x008d_4500, creature_constructor_ov2(Ptr, u32) -> Ptr),
        entry!(0x008d_4490, fn_008d4490(Ptr, u8)),
        entry!(0x008d_44b0, fn_008d44b0(Ptr) -> u8),
        entry!(0x008d_44d0, fn_008d44d0(Ptr, u32) -> Ptr),
        entry!(0x008d_45f0, fn_008d45f0(Ptr)),
        entry!(0x008d_46a0, fn_008d46a0(Ptr) -> i32),
        entry!(0x008d_47d0, fn_008d47d0(Ptr, u32, u8)),
        entry!(0x008d_47f0, fn_008d47f0(Ptr) -> f32),
        entry!(0x008d_4840, fn_008d4840(Ptr, u32) -> u32),
        entry!(0x008d_4870, fn_008d4870(Ptr, u32) -> u32),
        entry!(0x008d_4930, fn_008d4930(Ptr, u32, f32, u32) -> u32),
        entry!(0x008d_4a80, fn_008d4a80(Ptr, u32) -> u32),
        entry!(0x008d_4ab0, fn_008d4ab0(Ptr, u32) -> bool),
        entry!(0x008d_4b60, creature_check_missing_limbs(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    /// Call log, object bytes and return value of one constructor run.
    type CtorRun = (Vec<(u32, Vec<u32>)>, Vec<u8>, u32);

    fn run(addr: u32, args: &[u32]) -> CtorRun {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c0);
        e.mem.write(this, &[0xcd; 0x1c0]);
        e.mem.map(0x0101_2054, 4);
        e.mem.set_u32(0x0101_2054, 0x0100_0001);
        for a in [0x0087_d4a0u32, 0x004f_15a0] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let mut words = vec![this];
        words.extend_from_slice(args);
        let ret = e.call(addr, &words).u32();
        assert_eq!(ret, this);
        let log = e.call_log.take().unwrap();
        (
            log.into_iter().skip(1).collect(),
            e.mem.bytes(this, 0x1c0),
            this,
        )
    }

    fn expected() -> Vec<u8> {
        let mut b = vec![0xcd_u8; 0x1c0];
        let mut put = |off: usize, v: u32| b[off..off + 4].copy_from_slice(&v.to_le_bytes());
        put(0, 0x0108_70ac);
        put(0x18, 0x0108_70a0);
        put(0x88, 0x0108_704c);
        put(0x94, 0x0108_7014);
        put(0xa4, 0x0108_6fe4);
        put(0xa8, 0x0108_6f9c);
        put(0x1b8, 0x0100_0001);
        put(0x1bc, 0x0100_0001);
        put(0x140, 0);
        put(0x13c, 0);
        b[0x1b4] = 0;
        b
    }

    #[test]
    fn creature_constructor_builds_the_object() {
        let (calls, bytes, this) = run(0x008d_43a0, &[]);
        assert_eq!(
            calls,
            vec![
                (0x0087_d4a0, vec![this, 1]),
                (0x004f_15a0, vec![this, 0x3c])
            ]
        );
        assert_eq!(bytes, expected());
    }

    #[test]
    fn creature_constructor_ov2_passes_the_flag_byte_on() {
        let (calls, bytes, this) = run(0x008d_4500, &[0x1234_5600]);
        assert_eq!(
            calls,
            vec![
                (0x0087_d4a0, vec![this, 0]),
                (0x004f_15a0, vec![this, 0x3c])
            ]
        );
        assert_eq!(bytes, expected());
        let (calls, _, this) = run(0x008d_4500, &[0xab]);
        assert_eq!(calls[0], (0x0087_d4a0, vec![this, 0xab]));
    }
}

#[cfg(test)]
mod creature_tests {
    use super::*;

    const SLOT_A: u32 = 0x0001_0010;
    const SLOT_B: u32 = 0x0001_0020;
    const SLOT_C: u32 = 0x0001_0030;
    const SLOT_D: u32 = 0x0001_0040;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap().into_iter().skip(1).collect()
    }

    #[test]
    fn fn_008d4490_and_008d44b0_store_and_read_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c0);
        e.call(0x008d_4490, &args![this, 0x1234_5677u32]);
        assert_eq!(e.mem.u8(this + 0x1b4), 0x77);
        assert_eq!(e.mem.u8(this + 0x1b5), 0);
        assert_eq!(e.call(0x008d_44b0, &args![this]).u8(), 0x77);
    }

    fn destructor_doubles(e: &mut Engine, keep_3d: bool) {
        e.register_double(DESTRUCTOR_TEST_004077C0, move |_, _| {
            word(u32::from(keep_3d))
        });
        e.register(REFR_SET_3D, |_, _| Ret::default());
        e.register(ACTOR_DESTRUCTOR, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
    }

    #[test]
    fn fn_008d45f0_clears_the_3d_unless_told_not_to() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c0);
        destructor_doubles(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x008d_45f0, &args![this]);
        for (offset, vtable) in CREATURE_VTABLES {
            assert_eq!(e.mem.u32(this + offset), vtable);
        }
        assert_eq!(
            calls(&mut e),
            vec![
                (DESTRUCTOR_TEST_004077C0, vec![this]),
                (REFR_SET_3D, vec![this, 0, 0]),
                (ACTOR_DESTRUCTOR, vec![this]),
            ]
        );
        destructor_doubles(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x008d_45f0, &args![this]);
        assert!(calls(&mut e).iter().all(|c| c.0 != REFR_SET_3D));
    }

    #[test]
    fn fn_008d44d0_frees_only_with_bit_0() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c0);
        destructor_doubles(&mut e, true);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008d_44d0, &args![this, 3u32]).u32(), this);
        assert_eq!(calls(&mut e).last().unwrap(), &(FREE, vec![this]));
        e.call_log = Some(vec![]);
        e.call(0x008d_44d0, &args![this, 0u32]);
        assert!(calls(&mut e).iter().all(|c| c.0 != FREE));
    }

    #[test]
    fn fn_008d46a0_truncates_the_loaded_integer() {
        let mut e = Engine::new();
        e.register(GET_BASE_OBJECT, |_, _| word(0x5000));
        e.register(BASE_SHORT_0047FEF0, |_, a| {
            assert_eq!(a[0], 0x5000);
            word(0xffff_0007)
        });
        e.register(ACTOR_GET_FATIGUE_PERCENTAGE, |_, _| st0(0.25));
        e.register(INT_TO_ST0_00646250, |_, a| {
            assert_eq!(f32::from_bits(a[0]), 0.25);
            st0(f64::from(a[1]))
        });
        e.register(FTOL, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            word(v as i32 as u32)
        });
        assert_eq!(e.call(0x008d_46a0, &args![0x100u32]).i32(), 7);
    }

    #[test]
    fn fn_008d47d0_sets_the_3d() {
        let mut e = Engine::new();
        e.register(REFR_SET_3D, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_47d0, &args![0x100u32, 0x200u32, 0x301u32]);
        assert_eq!(calls(&mut e), vec![(REFR_SET_3D, vec![0x100, 0x200, 1])]);
    }

    #[test]
    fn fn_008d47f0_falls_back_to_the_setting() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x100);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.map(0x011c_f000, 0x1000);
        e.mem.set_f32(FALLBACK_SETTING_011CFABC + 4, 3.5);
        e.register(SETTING_VALUE, |_, a| word(a[0] + 4));
        e.register(GET_BASE_OBJECT, |_, _| word(0x5000));
        e.register(BASE_VALUE_00821640, |_, _| st0(1.25));
        assert_eq!(e.call(0x008d_47f0, &args![0x100u32]).f32(), 1.25);
        e.register(BASE_VALUE_00821640, |_, _| st0(0.0));
        assert_eq!(e.call(0x008d_47f0, &args![0x100u32]).f32(), 3.5);
        e.register(GET_BASE_OBJECT, |_, _| word(0));
        assert_eq!(e.call(0x008d_47f0, &args![0x100u32]).f32(), 3.5);
    }

    fn mapping_double(e: &mut Engine) {
        e.register(ACTOR_VALUE_HAS_FLAGS, |_, a| {
            assert_eq!(a[1], 0x400);
            word(u32::from(a[0] == 0x10))
        });
    }

    #[test]
    fn fn_008d4a80_maps_flagged_actor_values_to_0x2d() {
        let mut e = Engine::new();
        mapping_double(&mut e);
        assert_eq!(e.call(0x008d_4a80, &args![0u32, 0x10u32]).u32(), 0x2d);
        assert_eq!(e.call(0x008d_4a80, &args![0u32, 0x11u32]).u32(), 0x11);
    }

    #[test]
    fn fn_008d4840_and_008d4870_pass_the_mapped_value() {
        let mut e = Engine::new();
        mapping_double(&mut e);
        e.register(OWNER_CALL_00880580, |_, a| word(a[1] + 1));
        e.register(OWNER_CALL_008805F0, |_, a| word(a[1] + 2));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008d_4840, &args![0x1a4u32, 0x10u32]).u32(), 0x2e);
        let log = calls(&mut e);
        assert_eq!(log[0], (ACTOR_VALUE_HAS_FLAGS, vec![0x10, 0x400]));
        assert_eq!(log[1], (OWNER_CALL_00880580, vec![0x1a4, 0x2d]));
        assert_eq!(e.call(0x008d_4870, &args![0x1a4u32, 0x11u32]).u32(), 0x13);
    }

    #[test]
    fn fn_008d4930_passes_the_mapped_value_and_the_rest() {
        let mut e = Engine::new();
        mapping_double(&mut e);
        e.register(OWNER_CALL_00880AD0, |_, _| word(9));
        e.call_log = Some(vec![]);
        e.call(0x008d_4930, &args![0x100u32, 0x10u32, 1.5f32, 7u32]);
        assert_eq!(
            calls(&mut e)[1],
            (OWNER_CALL_00880AD0, vec![0x100, 0x2d, 1.5f32.to_bits(), 7])
        );
    }

    #[test]
    fn fn_008d4ab0_follows_forms_to_a_depth_of_four() {
        let mut e = Engine::new();
        e.map(0x011d_f000, 0x1000);
        // Each form's vtable +0x0C gives an object; 0045bc00 gives the
        // next form (form + 0x100); forms at 0x...30.. pass.
        let mut slots = vec![0u32; 4];
        slots[3] = SLOT_A;
        e.put_vtable(0x0002_0000, &slots);
        let forms: Vec<u32> = (0..6).map(|_| e.mem.alloc(0x10)).collect();
        for f in &forms {
            e.mem.set_u32(*f, 0x0002_0000);
        }
        let chain = forms.clone();
        e.register_double(SLOT_A, move |_, a| word(a[0]));
        e.register_double(NEXT_FORM_0045BC00, move |_, a| {
            let i = chain.iter().position(|f| *f == a[0]).unwrap();
            word(chain[i + 1])
        });
        let passing = forms[2];
        e.register_double(FORM_TEST_004A8B00, move |_, a| {
            word(u32::from(a[0] == passing))
        });
        assert!(e.call(0x008d_4ab0, &args![0u32, forms[0]]).bool());
        assert_eq!(e.global::<u32>(DEPTH_011DFFE4), 0);
        // A passing form five steps away is out of reach.
        let far = forms[5];
        e.register_double(FORM_TEST_004A8B00, move |_, a| word(u32::from(a[0] == far)));
        assert!(!e.call(0x008d_4ab0, &args![0u32, forms[0]]).bool());
        assert_eq!(e.global::<u32>(DEPTH_011DFFE4), 0);
        assert!(!e.call(0x008d_4ab0, &args![0u32, 0u32]).bool());
    }

    #[test]
    fn check_missing_limbs_removes_each_named_node() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 0x1e8 / 4];
        slots[0x1e4 / 4] = SLOT_A;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(this, 0x0002_0000);
        let creature = e.mem.alloc(0x40);
        let mut slots = vec![0u32; 0x44 / 4];
        slots[0x38 / 4] = SLOT_B;
        slots[0x3c / 4] = SLOT_C;
        slots[0x40 / 4] = SLOT_D;
        e.put_vtable(0x0002_1000, &slots);
        e.mem.set_u32(creature + 0x30, 0x0002_1000);
        let node = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 0x90 / 4];
        slots[0x8c / 4] = 0x0001_0050;
        e.put_vtable(0x0002_2000, &slots);
        e.mem.set_u32(node, 0x0002_2000);
        e.register(GET_BASE_OBJECT, |_, _| word(0x5000));
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            assert_eq!(a, &[0x5000, 0, TYPE_TES_ACTOR_BASE, TYPE_TES_CREATURE, 0]);
            word(creature)
        });
        e.register(SLOT_A, |_, _| word(0x6000));
        e.register(HOLDER_00496940, |_, _| word(0x6100));
        e.register_double(NODE_00537BD0, move |_, _| word(node));
        e.register(SLOT_B, |_, _| word(1));
        e.register(SLOT_C, |_, _| word(0));
        e.register(SLOT_D, |_, _| word(1));
        e.register(NAME_008D4CA0, |_, _| word(0xa0));
        e.register(NAME_008D4CB0, |_, _| word(0xb0));
        e.register(NAME_008A30F0, |_, _| word(0xf0));
        e.register(0x0001_0050, |_, a| word(a[1] + 0x1000));
        e.register(REMOVE_HAVOK_FROM_SCENE_GRAPH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_4b60, &args![this]);
        let removed: Vec<_> = calls(&mut e)
            .into_iter()
            .filter(|c| c.0 == REMOVE_HAVOK_FROM_SCENE_GRAPH)
            .map(|c| c.1)
            .collect();
        assert_eq!(removed, vec![vec![0x10a0], vec![0x10f0]]);

        e.register(HOLDER_00496940, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x008d_4b60, &args![this]);
        assert!(calls(&mut e).iter().all(|c| c.0 != NODE_00537BD0));

        e.register(RT_DYNAMIC_CAST, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x008d_4b60, &args![this]);
        assert!(calls(&mut e).iter().all(|c| c.0 != SLOT_A));
    }
}
