//! `fallout/ai/combat/explosion.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

// Continuation audit (2026-10-10): the next 40 open entries through 009b1630
// yielded 24 complete normal-path translations and the following 16 skips.
// They have no registrations; resume at 009ac200, not at 009b1720.
// 009ac200: constructor's by-value position/matrix ABI and caster allocation branches.
// 009ac5a0: destructor's nested particle, world-removal and target-list cleanup paths.
// 009ac9c0: factory's matrix ABI, perk multiplier and actor-cause ownership branches.
// 009acd60: Initialize process creation, placement/rotation and effect setup branches.
// 009ad670: Init3D's large temporary pick/effect records and cleanup lifetimes.
// 009ae280: phantom construction's temporary cinfo layout and shape ownership.
// 009ae5d0: update orchestration awaits target collection and full branch regression cases.
// 009ae6e0: large effect/damage flow; temporary record layouts and x87 branches unresolved.
// 009afcc0: target enumeration and collision/body-to-reference filtering not yet modeled.
// 009afef0: rigid-body enumeration and layer/actor exclusion branches not yet modeled.
// 009b00a0: ProcessTargets damage, actor and magic paths require full call/argument audit.
// 009b0920: PushRigidBody vector temporary lifetimes, force caps and random angular impulse.
// 009b0e00: target-list find/update/allocation loop needs matching/allocation regressions.
// 009b0f80: cached damage's weapon-condition and actor-value branches need x87 validation.
// 009b1510: target body-part selection and minimum-distance list traversal.
// 009b1630: body-part replacement versus allocation; duplicate-match traversal tests pending.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `Explosion` (Xbox PDB), PC offsets confirmed by 009abf30; the TESForm
    /// base is 0x10 smaller than the Xbox build. Embedded fields are addressed
    /// through their shared helpers rather than redeclaring container layouts.
    pub struct Explosion: 0x104 {
        0x88 fAge: f32,
        0x8c fLifetime: f32,
        0x90 fBuildTime: f32,
        0x94 fRadius: f32,
        0x98 fImageSpaceRadius: f32,
        0x9c fWeaponCondition: f32,
        0xa0 spPhantom: Ptr,
        0xc4 spLight: Ptr,
        0xc8 pOwner: Ptr,
        0xcc pExplodedRef: Ptr,
        0xd0 spActorCause: Ptr,
        0xd4 bDecalsPlaced: bool,
        0xd5 bTargetsFound: bool,
        0xd6 bTargetsProcessed: bool,
        0xd7 bForcesApplied: bool,
        0xd8 bIgnoreImageSpaceSwap: bool,
        0xd9 bUnderWater: bool,
        0xdc pCaster: Ptr,
        0xe0 pWeaponSource: Ptr,
        0xe4 iFrameCount: i32,
        0x100 fCalculatedDamage: f32,
    }
    /// `bhkLiquidAction` (Xbox PDB), same size/offsets confirmed in 009ae140.
    pub struct BhkLiquidAction: 0x40 {
        0x10 pTargetObject: Ptr,
        0x14 fNeighborDistance: f32,
        0x18 fNeighborStrength: f32,
        0x1c fInitialStickForce: f32,
        0x20 fStickStrength: f32,
        0x3c bFirstUpdate: bool,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009abf30, explosion_ctor(Ptr) -> Ptr),
        entry!(0x009ad630, tes_image_space_modifier_get_explosion_in_face() -> Ptr),
        entry!(0x009ae140, bhk_liquid_action_ctor(Ptr, Ptr) -> Ptr),
        entry!(0x009ae4a0, fn_009ae4a0(Ptr, Ptr) -> Ptr),
        entry!(0x009ae4d0, fn_009ae4d0(Ptr, Ptr) -> Ptr),
        entry!(0x009afaf0, fn_009afaf0(Ptr, u32, u32) -> Ptr),
        entry!(0x009afbe0, fn_009afbe0(Ptr)),
        entry!(0x009afed0, fn_009afed0(Ptr) -> Ptr),
        entry!(0x009b0900, fn_009b0900(Ptr) -> Ptr),
        entry!(0x009b0bd0, fn_009b0bd0(Ptr) -> f32),
        entry!(0x009b0bf0, fn_009b0bf0(Ptr, Ptr)),
        entry!(0x009b0c50, fn_009b0c50(Ptr, Ptr)),
        entry!(0x009b0c90, fn_009b0c90(Ptr, Ptr)),
        entry!(0x009b0cf0, fn_009b0cf0(Ptr, Ptr)),
        entry!(0x009b0d30, fn_009b0d30(Ptr, Ptr)),
        entry!(0x009b0d70, fn_009b0d70(Ptr, Ptr, f32)),
        entry!(0x009b0f40, fn_009b0f40(Ptr) -> Ptr),
        entry!(0x009b11c0, explosion_stop_sounds(Ptr)),
        entry!(0x009b1260, explosion_attach_light(Ptr)),
        entry!(0x009b12d0, fn_009b12d0(Ptr)),
        entry!(0x009b1090, explosion_start_sounds(Ptr)),
        entry!(0x009b1320, fn_009b1320(Ptr)),
        entry!(0x009b13a0, fn_009b13a0(Ptr)),
        entry!(0x009b1450, fn_009b1450(Ptr) -> bool),
        entry!(0x00471840, fn_00471840(Ptr) -> u32),
        entry!(0x009ac170, fn_009ac170(Ptr, Ptr)),
        entry!(0x009ac190, fn_009ac190(Ptr) -> Ptr),
        entry!(0x009ac1d0, fn_009ac1d0(Ptr, u32) -> Ptr),
        entry!(0x009ac550, fn_009ac550(Ptr) -> f32),
        entry!(0x009ac910, fn_009ac910(Ptr, u32) -> Ptr),
        entry!(0x009ac940, fn_009ac940(Ptr)),
        entry!(0x009ac9a0, fn_009ac9a0(Ptr)),
        entry!(0x009acce0, fn_009acce0(Ptr) -> Ptr),
        entry!(0x009acd00, explosion_attach_distant(Ptr)),
        entry!(0x009ad5a0, fn_009ad5a0(Ptr) -> f32),
        entry!(0x009ad5f0, fn_009ad5f0(Ptr) -> bool),
        entry!(0x009ad610, fn_009ad610(Ptr) -> Ptr),
        entry!(0x009ae0f0, fn_009ae0f0(Ptr)),
        entry!(0x009ae240, fn_009ae240(Ptr) -> Ptr),
        entry!(0x009ae250, fn_009ae250(Ptr, u32) -> Ptr),
        entry!(0x009ae540, fn_009ae540(Ptr, Ptr) -> Ptr),
        entry!(0x009ae580, fn_009ae580(Ptr, f32)),
        entry!(0x009afbb0, fn_009afbb0(Ptr, u32) -> Ptr),
        entry!(0x009afc10, fn_009afc10(Ptr)),
        entry!(0x009afc30, fn_009afc30(Ptr) -> Ptr),
        entry!(0x009afc50, fn_009afc50(Ptr) -> u32),
        entry!(0x009afc70, fn_009afc70(Ptr, u32) -> u32),
        entry!(0x009afca0, fn_009afca0(Ptr, Ptr)),
    ]
}

// Translated from 00471840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLiquidAction::QCinfoSize` (Xbox PDB): returns the fixed size 0x18.
pub fn fn_00471840(_e: &mut Engine, _this: Ptr) -> u32 {
    0x18
}

// Translated from 009ac170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the actor-cause pointer held at `this+0xD0` through the engine's
/// reference-pointer helper.
pub fn fn_009ac170(e: &mut Engine, this: Ptr, cause: Ptr) {
    e.call(0x0042_f780, &args![this.addr() + 0xd0, cause]);
}

// Translated from 009ac190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the pointer at `this+0xDC`, advanced by 0x0C when non-null.
pub fn fn_009ac190(e: &mut Engine, this: Ptr) -> Ptr {
    let pointer = e.mem.u32(this.addr() + 0xdc);
    Ptr::new(if pointer == 0 { 0 } else { pointer + 0x0c })
}

// Translated from 009ac1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor wrapper: runs the destructor and frees `this`
/// when bit 0 of `flags` is set.
pub fn fn_009ac1d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x009a_c5a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 009ac550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSExplosion::GetImageSpaceRadiusBSUnits` (Xbox PDB): returns the stored
/// radius when the unit flag is set; otherwise scales by the game setting.
pub fn fn_009ac550(e: &mut Engine, this: Ptr) -> f32 {
    if e.call(0x0047_7950, &args![this, 1u32]).bool() {
        return e.mem.f32(this.addr() + 0x8c);
    }
    let setting = e.call(0x0040_3e20, &args![0x011d_1218u32]).ptr::<u32>();
    ((e.mem.f32(this.addr() + 0x8c) as f64) * (e.mem.f32(setting.addr()) as f64)) as f32
}

// Translated from 009ac910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor wrapper for an explosion target record.
pub fn fn_009ac910(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x009a_c940, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 009ac940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears and destroys the embedded target list at `this+8`.
pub fn fn_009ac940(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 8;
    e.call(0x0047_0470, &args![list]);
    e.call(0x0046_ffb0, &args![list]);
}

// Translated from 009ac9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the object's virtual function at byte offset 0xC4 with argument 1.
pub fn fn_009ac9a0(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0xc4, &args![1u32]);
}

// Translated from 009acce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `this+0xC8`.
pub fn fn_009acce0(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0xc8))
}

// Translated from 009acd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attaches this explosion to distant terrain when its virtual target exists.
pub fn explosion_attach_distant(e: &mut Engine, this: Ptr) {
    let target = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    if target == 0 {
        return;
    }
    let tes = e.call(0x004f_d3e0, &args![0x011d_ea10u32]).u32();
    let world = e.call(0x0058_6170, &args![tes]).u32();
    let terrain = e.call(0x0055_85e0, &args![world]).u32();
    if terrain != 0 {
        e.vcall(terrain, 0xdc, &args![target, 1u32]);
    }
}

// Translated from 009ad5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSExplosion::GetRadiationRadiusBSUnits` (Xbox PDB), using the same
/// unit-flag branch as the image-space radius accessor.
pub fn fn_009ad5a0(e: &mut Engine, this: Ptr) -> f32 {
    if e.call(0x0047_7950, &args![this, 1u32]).bool() {
        return e.mem.f32(this.addr() + 0xa0);
    }
    let setting = e.call(0x0040_3e20, &args![0x011d_1218u32]).ptr::<u32>();
    ((e.mem.f32(this.addr() + 0xa0) as f64) * (e.mem.f32(setting.addr()) as f64)) as f32
}

// Translated from 009ad5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns true when the cell pointer returned by the callee is null.
pub fn fn_009ad5f0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x008d_6f30, &args![this]).u32() == 0
}

// Translated from 009ad610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the 32-bit word at `this+0xCC`.
pub fn fn_009ad610(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0xcc))
}

// Translated from 009ae0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a four-float point record from executable constants.
pub fn fn_009ae0f0(e: &mut Engine, this: Ptr) {
    e.call(0x0053_7bb0, &args![this]);
    for (offset, address) in [
        (8, 0x0103_0020),
        (12, 0x0101_e580),
        (16, 0x0101_e6ec),
        (20, 0x0101_e580),
    ] {
        let value: f32 = e.global(address);
        e.mem.set_f32(this.addr() + offset, value);
    }
}

// Translated from 009ae240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLiquidAction::GetRTTI` (Xbox PDB): returns its RTTI descriptor address.
pub fn fn_009ae240(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(0x0126_8420)
}

// Translated from 009ae250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLiquidAction` (Xbox PDB) scalar deleting destructor wrapper; the
/// allocation size passed to the engine's delete helper is 0x40.
pub fn fn_009ae250(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00cb_c920, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x40u32]);
    }
    this
}

// Translated from 009ae540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies three floats from `source` and writes zero to the fourth component.
pub fn fn_009ae540(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    for offset in [0, 4, 8] {
        let bits = e.mem.u32(source.addr() + offset);
        // FLD/FSTP quiets signaling NaNs while preserving sign and payload.
        let bits = if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
            bits | 0x0040_0000
        } else {
            bits
        };
        e.mem.set_u32(this.addr() + offset, bits);
    }
    e.mem.set_f32(this.addr() + 0x0c, 0.0);
    this
}

// Translated from 009ae580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a setting-derived step to `this+0x88` and forwards it through the
/// object's virtual update slot at byte offset 0x2F8.
pub fn fn_009ae580(e: &mut Engine, this: Ptr, _unused_1: f32) {
    let delta = e.call(0x0084_d030, &args![0x011f_6394u32]).f32();
    e.vcall(this.addr(), 0x2f8, &args![delta]);
    let old = e.mem.f32(this.addr() + 0x88);
    e.mem
        .set_f32(this.addr() + 0x88, (old as f64 + delta as f64) as f32);
}

// Translated from 009afbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor wrapper for the Havok contact collector.
pub fn fn_009afbb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x009a_fc10, &args![this]);
    if flags & 1 != 0 {
        e.call(0x009a_fbe0, &args![this]);
    }
    this
}

// Translated from 009afc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Invokes the collector base destructor.
pub fn fn_009afc10(e: &mut Engine, this: Ptr) {
    e.call(0x0062_1100, &args![this]);
}

// Translated from 009afc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the buffer pointer at `this+8` through its accessor. The caller
/// at 009ae865 adds a contact index times 0x70 to this return value.
pub fn fn_009afc30(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0045_8b50, &args![this.addr() + 8]).ptr()
}

// Translated from 009afc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the embedded count at `this+0x14` through its accessor.
pub fn fn_009afc50(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0045_8b50, &args![this.addr() + 0x14]).u32()
}

// Translated from 009afc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores whether the embedded signed count at `this+0x14` is positive into
/// the byte pointed to by `value`, then returns that pointer unchanged.
pub fn fn_009afc70(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let count = e.call(0x0045_8b50, &args![this.addr() + 0x14]).i32();
    e.call(0x0062_2570, &args![value, u32::from(count > 0)]);
    value
}

// Translated from 009afca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the function pointer stored at 0x01268430 with `(point, this)`.
pub fn fn_009afca0(e: &mut Engine, this: Ptr, point: Ptr) {
    let target = e.global::<u32>(0x0126_8430);
    e.call(target, &args![point, this]);
}

// Translated from 009abf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::Explosion` (Xbox PDB), PC size 0x104 versus Xbox 0x114.
/// Initializes the base, members and defaults; compiler SEH unwinding is omitted.
pub fn explosion_ctor(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0092_eb50, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_ee04);
    e.mem.set_u32(this.addr() + 0x18, 0x0108_edf8);
    e.call(0x0063_3c90, &args![this.addr() + 0xa0, 0u32]);
    e.call(0x0096_a2d0, &args![this.addr() + 0xa4]);
    e.call(0x0041_a250, &args![this.addr() + 0xac]);
    e.call(0x0041_a250, &args![this.addr() + 0xb8]);
    e.call(0x0063_3c90, &args![this.addr() + 0xc4, 0u32]);
    e.call(0x0042_f730, &args![this.addr() + 0xd0, 0u32]);
    for offset in [0xe8, 0xf4] {
        e.call(0x0068_15c0, &args![this.addr() + offset]);
    }
    for offset in [0x68, 0xe4, 0x88, 0x8c, 0x90, 0x94, 0x98] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    e.mem.set_f32(this.addr() + 0x9c, 1.0);
    for offset in [0xa0, 0xc4] {
        e.call(0x0066_b0d0, &args![this.addr() + offset, 0u32]);
    }
    e.call(0x0042_f780, &args![this.addr() + 0xd0, 0u32]);
    for offset in [0xc8, 0xcc, 0xe0] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    for offset in 0xd4..=0xd9 {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    e.mem.set_u32(this.addr() + 0xdc, 0);
    for offset in [0xe8, 0xf4] {
        for component in [0, 4, 8] {
            let bits = e.global::<u32>(0x011f_426c + component);
            e.mem.set_u32(this.addr() + offset + component, bits);
        }
    }
    let damage = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(this.addr() + 0x100, damage);
    this
}

// Translated from 009ad630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::GetExplosionInFace` (Xbox PDB): lazy lookup and RTTI cast.
pub fn tes_image_space_modifier_get_explosion_in_face(e: &mut Engine) -> Ptr {
    if e.global::<u32>(0x011c_9708) == 0 {
        let form = e.call(0x0048_39c0, &args![0x166u32]).u32();
        let modifier = e
            .call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_62b8u32, 0u32],
            )
            .u32();
        e.set_global(0x011c_9708, modifier);
    }
    Ptr::new(e.global(0x011c_9708))
}

// Translated from 009ae140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLiquidAction::bhkLiquidAction` (Xbox PDB): initializes the 0x40-byte
/// action and increments its live counter. Compiler SEH unwinding is omitted.
pub fn bhk_liquid_action_ctor(e: &mut Engine, this: Ptr, info: Ptr) -> Ptr {
    e.call(0x004b_50c0, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_f1bc);
    for offset in [0x24, 0x30] {
        e.call(0x0068_15c0, &args![this.addr() + offset]);
    }
    for offset in [0x10, 0x14, 0x18, 0x1c, 0x20] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    for offset in [0x24, 0x30] {
        for component in [0, 4, 8] {
            let bits = e.global::<u32>(0x011f_426c + component);
            e.mem.set_u32(this.addr() + offset + component, bits);
        }
    }
    e.mem.set_u8(this.addr() + 0x3c, 1);
    e.call(0x00cb_d520, &args![this, info]);
    let count = e.global::<u32>(0x0126_841c).wrapping_add(1);
    e.set_global(0x0126_841c, count);
    this
}

// Translated from 009ae4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the rotation rows and converts the translation at source+0x24.
pub fn fn_009ae4a0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    fn_009ae4d0(e, this, source);
    e.call(
        0x004b_4e50,
        &args![this.addr() + 0x30, source.addr() + 0x24],
    );
    this
}

// Translated from 009ae4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Extracts three rotation rows into four-float vectors with zero fourth lanes.
pub fn fn_009ae4d0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.with_stack(12, |e, row| {
        e.call(0x0068_15c0, &args![row]);
        for index in 0..3u32 {
            e.call(0x0043_9f50, &args![source, index, row]);
            fn_009ae540(e, Ptr::new(this.addr() + index * 16), row.cast());
        }
    });
    this
}

// Translated from 009afaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the fixed-buffer contact collector. Compiler SEH unwinding is omitted.
pub fn fn_009afaf0(e: &mut Engine, this: Ptr, buffer: u32, capacity: u32) -> Ptr {
    e.call(0x0062_1060, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_f288);
    for offset in [8, 12, 16, 20] {
        e.call(0x0068_15c0, &args![this.addr() + offset]);
    }
    for (offset, value) in [(8, buffer), (12, buffer), (16, capacity), (20, 0)] {
        e.call(0x0053_7e90, &args![this.addr() + offset, value]);
    }
    this
}

// Translated from 009afbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a non-null collector through the thread allocator, with size 0x18.
/// This is a cdecl one-argument delete, not a thiscall member.
pub fn fn_009afbe0(e: &mut Engine, allocation: Ptr) {
    if !allocation.is_null() {
        let router = e.call(0x00c8_5750, &args![]).u32();
        let allocator = e.call(0x0044_edb0, &args![router]).u32();
        e.vcall(allocator, 8, &args![allocation, 0x18u32]);
    }
}

// Translated from 009afed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the underlying Havok object's embedded array at +0x160.
pub fn fn_009afed0(e: &mut Engine, this: Ptr) -> Ptr {
    let object = e.call(0x004a_e750, &args![this]).u32();
    e.call(0x0062_d2f0, &args![object]).ptr()
}

// Translated from 009b0900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns Explosion::pWeaponSource (Xbox PDB), PC +0xE0.
pub fn fn_009b0900(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0xe0))
}

// Translated from 009b0bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float at +0x74 from the caller's object.
pub fn fn_009b0bd0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x74)
}

// Translated from 009b0bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts the supplied vector into Havok units before applying linear impulse.
pub fn fn_009b0bf0(e: &mut Engine, this: Ptr, vector: Ptr) {
    e.with_stack(16, |e, converted| {
        e.call(0x0068_15c0, &args![converted]);
        let result = e.call(0x004a_3e00, &args![converted, vector]).ptr();
        fn_009b0c50(e, this, result);
    });
}

// Translated from 009b0c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies a linear impulse only when the wrapped body exists.
pub fn fn_009b0c50(e: &mut Engine, this: Ptr, impulse: Ptr) {
    let body = e.call(0x004a_e750, &args![this]).u32();
    if body != 0 {
        e.call(0x00a2_9680, &args![this]);
        e.call(0x0081_82d0, &args![body, impulse]);
        e.call(0x00a2_9680, &args![this]);
    }
}

// Translated from 009b0c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts the supplied vector before forwarding to the activated body's motion.
pub fn fn_009b0c90(e: &mut Engine, this: Ptr, vector: Ptr) {
    e.with_stack(16, |e, converted| {
        e.call(0x0068_15c0, &args![converted]);
        let result = e.call(0x004a_3e00, &args![converted, vector]).ptr();
        fn_009b0cf0(e, this, result);
    });
}

// Translated from 009b0cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to the body's motion only when the wrapped body exists.
pub fn fn_009b0cf0(e: &mut Engine, this: Ptr, impulse: Ptr) {
    let body = e.call(0x004a_e750, &args![this]).u32();
    if body != 0 {
        e.call(0x00a2_9680, &args![this]);
        fn_009b0d30(e, Ptr::new(body), impulse);
        e.call(0x00a2_9680, &args![this]);
    }
}

// Translated from 009b0d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Activates the entity and invokes its motion's virtual slot +0x54.
pub fn fn_009b0d30(e: &mut Engine, this: Ptr, impulse: Ptr) {
    e.call(0x00c9_c1d0, &args![this]);
    let motion = e.call(0x009d_9f40, &args![this]).u32();
    e.vcall(motion, 0x54, &args![impulse]);
}

// Translated from 009b0d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the explosion position to the target's process above the setting threshold.
pub fn fn_009b0d70(e: &mut Engine, this: Ptr, target: Ptr, magnitude: f32) {
    if target.is_null() || e.call(0x008d_8520, &args![target]).u32() == 0 {
        return;
    }
    let setting = e.call(0x0040_3e20, &args![0x011d_1188u32]).u32();
    if e.mem.f32(setting) < magnitude {
        let process = e.call(0x008d_8520, &args![target]).u32();
        let position = e.vcall(this.addr(), 0x1f4, &args![0.0f32]).u32();
        let words = [
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        ];
        e.vcall(process, 0x418, &args![target, words[0], words[1], words[2]]);
    }
}

// Translated from 009b0f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes an explosion target and its embedded list at +8.
pub fn fn_009b0f40(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0096_a2d0, &args![this.addr() + 8]);
    e.mem.set_u32(this.addr(), 0);
    for offset in 4..=6 {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    this
}

// Translated from 009b11c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::StopSounds` (Xbox PDB): stops both valid handles, then rechecks
/// validity before releasing each one.
pub fn explosion_stop_sounds(e: &mut Engine, this: Ptr) {
    for offset in [0xac, 0xb8] {
        let handle = this.addr() + offset;
        if e.call(0x00ad_8ce0, &args![handle]).bool() {
            e.call(0x00ad_88f0, &args![handle]);
            if e.call(0x00ad_8ce0, &args![handle]).bool() {
                e.call(0x00ad_8d10, &args![handle]);
            }
        }
    }
}

// Translated from 009b1260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::AttachLight` (Xbox PDB): creates a dynamic light when both 3D
/// and the base explosion's light form exist, then assigns spLight.
pub fn explosion_attach_light(e: &mut Engine, this: Ptr) {
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() == 0 {
        return;
    }
    let base = e.call(0x0041_81e0, &args![this]).u32();
    if e.call(0x004f_b070, &args![base]).u32() == 0 {
        return;
    }
    let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    let base = e.call(0x0041_81e0, &args![this]).u32();
    let form = e.call(0x004f_b070, &args![base]).u32();
    let light = e.call(0x0050_d810, &args![form, this, node, 0u32]).u32();
    e.call(0x0066_b0d0, &args![this.addr() + 0xc4, light]);
}

// Translated from 009b12d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes a non-null spLight from the shadow scene and clears the smart pointer.
pub fn fn_009b12d0(e: &mut Engine, this: Ptr) {
    let member = this.addr() + 0xc4;
    if e.call(0x0055_9450, &args![member]).u32() != 0 {
        let light = e.call(0x0055_9450, &args![member]).u32();
        let scene = e.call(0x0045_0b80, &args![0u32]).u32();
        e.call(0x00b5_eed0, &args![scene, light]);
        e.call(0x0066_b0d0, &args![member, 0u32]);
    }
}
// Translated from 009b1090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::StartSounds` (Xbox PDB): replaces the two sound handles from
/// their base-form records. Compiler SEH unwinding is omitted.
pub fn explosion_start_sounds(e: &mut Engine, this: Ptr) {
    explosion_stop_sounds(e, this);
    for (getter, offset) in [(0x004f_d3c0, 0xac), (0x0091_85e0, 0xb8)] {
        let base = e.call(0x0041_81e0, &args![this]).u32();
        if e.call(getter, &args![base]).u32() != 0 {
            let base = e.call(0x0041_81e0, &args![this]).u32();
            let sound = e.call(getter, &args![base]).u32();
            let id = e.call(0x0084_e3a0, &args![sound]).u32();
            e.with_stack(12, |e, handle| {
                let result = e
                    .call(
                        0x0093_3150,
                        &args![this, handle, id, 0u32, 0x4000_0182u32, 1u32],
                    )
                    .u32();
                e.call(0x0041_8900, &args![this.addr() + offset, result]);
                e.call(0x0048_3710, &args![handle]);
            });
        }
    }
}

// Translated from 009b1320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the current light by the age/lifetime fade, clamped through the exe helpers.
pub fn fn_009b1320(e: &mut Engine, this: Ptr) {
    let member = this.addr() + 0xc4;
    if e.call(0x0055_9450, &args![member]).u32() != 0 {
        let age = e.call(0x0051_b8e0, &args![this]).f32() as f64;
        let scaled = age * e.global::<f64>(0x0101_1590);
        let lifetime = e.call(0x0050_7e10, &args![this]).f32() as f64;
        let factor = (1.0 - scaled / lifetime) as f32;
        let factor = e.call(0x0040_ebd0, &args![1.0f32, factor]).f32();
        let factor = e.call(0x0040_4010, &args![0.0f32, factor]).f32();
        let light = e.call(0x0055_9450, &args![member]).u32();
        e.call(0x0050_dd20, &args![light, factor]);
    }
}

// Translated from 009b13a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates ExtraRadiation from positive BGSExplosionData::fRadiationRads and
/// fRadiationDissipationTime (Xbox PDB), setting the reference change flag.
pub fn fn_009b13a0(e: &mut Engine, this: Ptr) {
    let base = e.call(0x0041_81e0, &args![this]).u32();
    let rads = e.call(0x008d_01e0, &args![base]).f32();
    let base = e.call(0x0041_81e0, &args![this]).u32();
    let duration = e.call(0x004e_3cc0, &args![base]).f32();
    let zero = e.global::<f64>(0x0101_2060);
    if rads as f64 > zero && duration as f64 > zero {
        let age = e.call(0x0051_b8e0, &args![this]).f32() as f64;
        let mut scaled = ((1.0 - age / duration as f64) * rads as f64) as f32;
        // TEST AH,0x41; JP skips the replacement for unordered or strictly greater.
        if scaled as f64 <= zero {
            scaled = e.global::<f32>(0x0101_3ea4);
        }
        let member = e.call(0x005d_43c0, &args![this]).u32();
        e.call(0x0042_2350, &args![member, scaled]);
        e.vcall(this.addr(), 0x48, &args![0x4000_0000u32]);
    }
}

// Translated from 009b1450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Retains the explosion at the current process level or when its cell is loaded;
/// otherwise invokes its scalar destructor and reports false.
pub fn fn_009b1450(e: &mut Engine, this: Ptr) -> bool {
    let process = e.mem.u32(this.addr() + 0x68);
    let level = e.call(0x0045_cd60, &args![process]).u32();
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    let visible = node != 0 || {
        let manager = e.global::<u32>(0x011c_3b3c);
        e.call(0x0044_5750, &args![manager, this]).bool()
    };
    let desired = if visible && {
        let tes = e.global::<u32>(0x011d_ea10);
        e.call(0x0045_11e0, &args![tes, cell, 1u32]).bool()
    } {
        0
    } else {
        3
    };
    if level != desired && desired != 0 {
        if !this.is_null() {
            e.vcall(this.addr(), 0x10, &args![1u32]);
        }
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine_with(addrs: &[u32]) -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    fn mapped(e: &mut Engine, address: u32) -> Ptr {
        e.map(address & !0xfff, 0x1000);
        Ptr::new(address)
    }

    #[test]
    fn qcinfo_size_is_fixed() {
        assert_eq!(fn_00471840(&mut Engine::new(), Ptr::new(0)), 0x18);
    }

    #[test]
    fn actor_cause_release_passes_adjusted_member_and_pointer() {
        let mut e = engine_with(&[0x0042_f780]);
        fn_009ac170(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));
        assert_eq!(e.call_log.unwrap()[0].1, vec![0x10d0, 0x2000]);
    }

    #[test]
    fn caster_accessor_adjusts_non_null_pointer() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        e.mem.set_u32(this.addr() + 0xdc, 0x2000);
        assert_eq!(fn_009ac190(&mut e, this).addr(), 0x200c);
        e.mem.set_u32(this.addr() + 0xdc, 0);
        assert!(fn_009ac190(&mut e, this).is_null());
    }

    #[test]
    fn scalar_destructor_frees_only_when_flag_is_set() {
        let mut e = engine_with(&[0x009a_c5a0, 0x0040_1030]);
        fn_009ac1d0(&mut e, Ptr::new(0x1000), 0);
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn image_space_radius_uses_stored_value_when_flag_is_set() {
        let mut e = Engine::new();
        e.register(0x0047_7950, |_, _| true.into_ret());
        let this = mapped(&mut e, 0x1000);
        e.mem.set_f32(this.addr() + 0x8c, 42.5);
        assert_eq!(fn_009ac550(&mut e, this), 42.5);
    }

    #[test]
    fn target_scalar_destructor_honors_flag() {
        let mut e = engine_with(&[0x009a_c940, 0x0040_1030]);
        fn_009ac910(&mut e, Ptr::new(0x1000), 1);
        assert_eq!(e.call_log.unwrap().len(), 2);
    }

    #[test]
    fn target_list_destruction_uses_adjusted_member() {
        let mut e = engine_with(&[0x0047_0470, 0x0046_ffb0]);
        fn_009ac940(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.unwrap()[0].1[0], 0x1008);
    }

    #[test]
    fn explosion_activation_uses_slot_c4_with_one() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        e.map(0x3000, 0x1000);
        e.mem.set_u32(this.addr(), 0x3000);
        e.mem.set_u32(0x30c4, 0x6000_00c4);
        e.register(0x6000_00c4, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        fn_009ac9a0(&mut e, this);
        assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![0x1000, 1]);
    }

    #[test]
    fn word_at_c8_is_returned() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        e.mem.set_u32(this.addr() + 0xc8, 0x2000);
        assert_eq!(fn_009acce0(&mut e, this).addr(), 0x2000);
    }

    #[test]
    fn attach_distant_stops_without_target() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        e.map(0x3000, 0x1000);
        e.mem.set_u32(this.addr(), 0x3000);
        e.mem.set_u32(0x31d0, 0x6000_01d0);
        e.register(0x6000_01d0, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        explosion_attach_distant(&mut e, this);
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn radiation_radius_uses_stored_value_when_flag_is_set() {
        let mut e = Engine::new();
        e.register(0x0047_7950, |_, _| true.into_ret());
        let this = mapped(&mut e, 0x1000);
        e.mem.set_f32(this.addr() + 0xa0, 20.0);
        assert_eq!(fn_009ad5a0(&mut e, this), 20.0);
    }

    #[test]
    fn null_cell_is_reported_as_true() {
        let mut e = Engine::new();
        e.register(0x008d_6f30, |_, _| 0u32.into_ret());
        assert!(fn_009ad5f0(&mut e, Ptr::new(0x1000)));
    }

    #[test]
    fn word_at_cc_is_returned() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        e.mem.set_u32(this.addr() + 0xcc, 1);
        assert_eq!(fn_009ad610(&mut e, this).addr(), 1);
    }

    #[test]
    fn point_record_constructor_copies_constants() {
        let mut e = engine_with(&[0x0053_7bb0]);
        let this = mapped(&mut e, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.map(0x0103_0000, 0x1000);
        e.set_global(0x0103_0020, 1.0f32);
        e.set_global(0x0101_e580, 2.0f32);
        e.set_global(0x0101_e6ec, 3.0f32);
        fn_009ae0f0(&mut e, this);
        assert_eq!(e.mem.f32(this.addr() + 0x10), 3.0);
        assert_eq!(e.mem.f32(this.addr() + 0x14), 2.0);
    }

    #[test]
    fn rtti_accessor_returns_constant_address() {
        assert_eq!(
            fn_009ae240(&mut Engine::new(), Ptr::new(0)).addr(),
            0x0126_8420
        );
    }

    #[test]
    fn collector_scalar_destructor_honors_flag() {
        let mut e = engine_with(&[0x00cb_c920, 0x00aa_1460]);
        fn_009ae250(&mut e, Ptr::new(0x1000), 1);
        assert_eq!(e.call_log.unwrap().len(), 2);
    }

    #[test]
    fn point_copy_writes_zero_w_component() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        let source = mapped(&mut e, 0x2000);
        e.mem.set_f32(source.addr(), 1.0);
        e.mem.set_f32(source.addr() + 4, -2.0);
        e.mem.set_f32(source.addr() + 8, 3.5);
        fn_009ae540(&mut e, this, source);
        assert_eq!(e.mem.f32(this.addr() + 8), 3.5);
        assert_eq!(e.mem.f32(this.addr() + 12), 0.0);
    }

    #[test]
    fn frame_step_calls_virtual_update_and_adds_delta() {
        let mut e = Engine::new();
        e.map(0x3000, 0x1000);
        let this = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(this.addr(), 0x3000);
        e.mem.set_u32(0x3000 + 0x2f8, 0x6000_02f8);
        e.register(0x0084_d030, |_, _| 2.5f32.into_ret());
        e.register(0x6000_02f8, |_, a| a[1].into_ret());
        e.call_log = Some(vec![]);
        fn_009ae580(&mut e, this, 9.0);
        assert_eq!(e.mem.f32(this.addr() + 0x88), 2.5);
        assert!(e
            .call_log
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0x6000_02f8));
    }

    #[test]
    fn collector_base_destructor_is_called() {
        let mut e = engine_with(&[0x0062_1100]);
        fn_009afc10(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn contact_collector_deleting_destructor_honors_flag() {
        let mut e = engine_with(&[0x009a_fc10, 0x009a_fbe0]);
        let this = Ptr::new(0x1000);
        assert_eq!(fn_009afbb0(&mut e, this, 0).addr(), this.addr());
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn buffer_accessor_adjusts_this_by_eight() {
        let mut e = engine_with(&[0x0045_8b50]);
        fn_009afc30(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.unwrap()[0].1[0], 0x1008);
    }

    #[test]
    fn proxy_count_adjusts_this_by_twenty() {
        let mut e = engine_with(&[0x0045_8b50]);
        fn_009afc50(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.unwrap()[0].1[0], 0x1014);
    }

    #[test]
    fn proxy_boolean_returns_input() {
        let mut e = engine_with(&[0x0045_8b50, 0x0062_2570]);
        let value = 0x1234;
        assert_eq!(fn_009afc70(&mut e, Ptr::new(0x1000), value), value);
        assert_eq!(e.call_log.unwrap().len(), 2);
    }

    #[test]
    fn contact_callback_uses_global_target_and_ordered_arguments() {
        let mut e = engine_with(&[0x7000]);
        e.map(0x0126_8000, 0x1000);
        e.set_global(0x0126_8430, 0x7000);
        fn_009afca0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));
        assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![0x2000, 0x1000]);
    }
    #[test]
    fn native_point_copy_vectors_preserve_abi_and_quiet_nan() {
        // Generated nv-call vectors, FalloutNV.exe 1.4.0.525, 2026-10-10.
        for (input, expected) in [
            (
                [0x3f80_0000, 0xc000_0000, 0x8000_0000],
                [0x3f80_0000, 0xc000_0000, 0x8000_0000],
            ),
            (
                [0x7f80_0001, 0xff80_0001, 0x7fc1_2345],
                [0x7fc0_0001, 0xffc0_0001, 0x7fc1_2345],
            ),
        ] {
            let mut e = Engine::new();
            let dst = mapped(&mut e, 0x1000);
            let src = mapped(&mut e, 0x2000);
            for (i, bits) in input.into_iter().enumerate() {
                e.mem.set_u32(src.addr() + i as u32 * 4, bits);
            }
            assert_eq!(e.call(0x009a_e540, &args![dst, src]).u32(), dst.addr());
            for (i, bits) in expected.into_iter().enumerate() {
                assert_eq!(e.mem.u32(dst.addr() + i as u32 * 4), bits);
            }
            assert_eq!(e.mem.u32(dst.addr() + 12), 0);
        }
    }

    #[test]
    fn collector_accessors_return_callee_word_through_registry() {
        for (address, offset) in [(0x009a_fc30, 8), (0x009a_fc50, 0x14)] {
            let mut e = engine_with(&[]);
            e.register(0x0045_8b50, |_, a| {
                assert!(a[0] == 0x1008 || a[0] == 0x1014);
                0xffff_fffeu32.into_ret()
            });
            assert_eq!(e.call(address, &args![0x1000u32]).u32(), 0xffff_fffe);
            assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![0x1000 + offset]);
        }
    }

    #[test]
    fn count_boolean_uses_signed_positive_test_and_output_address() {
        for count in [-1i32, 0, 1] {
            let mut e = engine_with(&[]);
            e.register_double(0x0045_8b50, move |_, a| {
                assert_eq!(a, &[0x1014]);
                count.into_ret()
            });
            e.register(0x0062_2570, |e, a| {
                e.mem.set_u8(a[0], a[1] as u8);
                a[0].into_ret()
            });
            mapped(&mut e, 0x2000);
            assert_eq!(
                e.call(0x009a_fc70, &args![0x1000u32, 0x2000u32]).u32(),
                0x2000
            );
            assert_eq!(e.mem.u8(0x2000), u8::from(count > 0));
        }
    }

    #[test]
    fn both_radius_accessors_scale_only_without_unit_flag() {
        for (address, offset) in [(0x009a_c550, 0x8c), (0x009a_d5a0, 0xa0)] {
            let mut e = engine_with(&[]);
            mapped(&mut e, 0x1000);
            mapped(&mut e, 0x2000);
            e.mem.set_f32(0x1000 + offset, 3.5);
            e.mem.set_f32(0x2000, 2.25);
            e.register(0x0047_7950, |_, a| {
                assert_eq!(a, &[0x1000, 1]);
                false.into_ret()
            });
            e.register(0x0040_3e20, |_, a| {
                assert_eq!(a, &[0x011d_1218]);
                0x2000u32.into_ret()
            });
            assert_eq!(e.call(address, &args![0x1000u32]).f32(), 7.875);
        }
    }

    #[test]
    fn distant_target_attaches_only_when_terrain_exists() {
        for terrain in [0u32, 0x4000] {
            let mut e = engine_with(&[0x6000_00dc]);
            for address in [0x1000, 0x3000, 0x4000, 0x5000] {
                mapped(&mut e, address);
            }
            e.mem.set_u32(0x1000, 0x3000);
            e.mem.set_u32(0x31d0, 0x6000_01d0);
            e.mem.set_u32(0x4000, 0x5000);
            e.mem.set_u32(0x50dc, 0x6000_00dc);
            e.register(0x6000_01d0, |_, _| 0x9000u32.into_ret());
            e.register(0x004f_d3e0, |_, a| {
                assert_eq!(a, &[0x011d_ea10]);
                0x7000u32.into_ret()
            });
            e.register(0x0058_6170, |_, a| {
                assert_eq!(a, &[0x7000]);
                0x8000u32.into_ret()
            });
            e.register_double(0x0055_85e0, move |_, a| {
                assert_eq!(a, &[0x8000]);
                terrain.into_ret()
            });
            explosion_attach_distant(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            assert_eq!(log.len(), if terrain == 0 { 4 } else { 5 });
            if terrain != 0 {
                assert_eq!(log.last().unwrap().1, vec![terrain, 0x9000, 1]);
            }
        }
    }

    #[test]
    fn deleting_wrappers_preserve_this_and_gate_free_on_low_bit() {
        for (address, destructor, delete, size) in [
            (0x009a_c1d0, 0x009a_c5a0, 0x0040_1030, None),
            (0x009a_c910, 0x009a_c940, 0x0040_1030, None),
            (0x009a_e250, 0x00cb_c920, 0x00aa_1460, Some(0x40)),
            (0x009a_fbb0, 0x009a_fc10, 0x009a_fbe0, None),
        ] {
            for flags in [0u32, 1, 2] {
                let mut e = engine_with(&[destructor, delete]);
                assert_eq!(e.call(address, &args![0x1000u32, flags]).u32(), 0x1000);
                let log = e.call_log.unwrap();
                assert_eq!(log[1], (destructor, vec![0x1000]));
                assert_eq!(log.len(), if flags & 1 == 0 { 2 } else { 3 });
                if flags & 1 != 0 {
                    assert_eq!(
                        log[2],
                        (delete, size.map_or(vec![0x1000], |size| vec![0x1000, size]))
                    );
                }
            }
        }
    }
    fn constructor_engine() -> Engine {
        let mut e = engine_with(&[
            0x0092_eb50,
            0x0063_3c90,
            0x0096_a2d0,
            0x0041_a250,
            0x0042_f730,
            0x0068_15c0,
            0x0066_b0d0,
            0x0042_f780,
            0x004b_50c0,
            0x00cb_d520,
        ]);
        mapped(&mut e, 0x1000);
        mapped(&mut e, 0x011f_4000);
        mapped(&mut e, 0x0101_2000);
        mapped(&mut e, 0x0126_8000);
        for (offset, value) in [(0, 1u32), (4, 2), (8, 3)] {
            e.set_global(0x011f_426c + offset, value);
        }
        e.set_global(0x0101_2054, -1.0f32);
        e
    }

    #[test]
    fn explosion_default_constructor_initializes_pc_fields_and_order() {
        let mut e = constructor_engine();
        for offset in 0..0x104 {
            e.mem.set_u8(0x1000 + offset, 0xaa);
        }
        assert_eq!(e.call(0x009a_bf30, &args![0x1000u32]).u32(), 0x1000);
        assert_eq!(e.mem.u32(0x1000), 0x0108_ee04);
        assert_eq!(e.mem.u32(0x1018), 0x0108_edf8);
        for offset in [
            0x68, 0xe4, 0x88, 0x8c, 0x90, 0x94, 0x98, 0xc8, 0xcc, 0xe0, 0xdc,
        ] {
            assert_eq!(e.mem.u32(0x1000 + offset), 0);
        }
        for offset in 0xd4..=0xd9 {
            assert_eq!(e.mem.u8(0x1000 + offset), 0);
        }
        assert_eq!(e.mem.u8(0x10da), 0xaa);
        assert_eq!(e.mem.f32(0x109c), 1.0);
        assert_eq!(e.mem.f32(0x1100), -1.0);
        for offset in [0xe8, 0xf4] {
            for i in 0..3 {
                assert_eq!(e.mem.u32(0x1000 + offset + i * 4), i + 1);
            }
        }
        let log = e.call_log.unwrap();
        assert_eq!(log[1], (0x0092_eb50, vec![0x1000]));
        assert_eq!(log[2], (0x0063_3c90, vec![0x10a0, 0]));
        assert_eq!(log.last().unwrap(), &(0x0042_f780, vec![0x10d0, 0]));
    }

    #[test]
    fn modifier_lookup_caches_success_and_retries_null() {
        for cast_result in [0u32, 0x9000] {
            let mut e = engine_with(&[]);
            mapped(&mut e, 0x011c_9000);
            e.register(0x0048_39c0, |_, a| {
                assert_eq!(a, &[0x166]);
                0x8000u32.into_ret()
            });
            e.register_double(0x00ec_43fb, move |_, a| {
                assert_eq!(a, &[0x8000, 0, 0x0118_3028, 0x0118_62b8, 0]);
                cast_result.into_ret()
            });
            for _ in 0..2 {
                assert_eq!(
                    tes_image_space_modifier_get_explosion_in_face(&mut e).addr(),
                    cast_result
                );
            }
            assert_eq!(
                e.call_log.unwrap().len(),
                if cast_result == 0 { 4 } else { 2 }
            );
        }
    }

    #[test]
    fn liquid_constructor_sets_defaults_then_initializes_and_wraps_counter() {
        let mut e = constructor_engine();
        e.set_global(0x0126_841c, u32::MAX);
        assert_eq!(
            bhk_liquid_action_ctor(&mut e, Ptr::new(0x1000), Ptr::new(0x8000)).addr(),
            0x1000
        );
        assert_eq!(e.mem.u32(0x1000), 0x0108_f1bc);
        assert_eq!(e.mem.u8(0x103c), 1);
        assert_eq!(e.global::<u32>(0x0126_841c), 0);
        for offset in [0x24, 0x30] {
            for i in 0..3 {
                assert_eq!(e.mem.u32(0x1000 + offset + i * 4), i + 1);
            }
        }
        assert_eq!(
            e.call_log.unwrap().last().unwrap(),
            &(0x00cb_d520, vec![0x1000, 0x8000])
        );
    }

    fn matrix_engine() -> Engine {
        let mut e = engine_with(&[0x0068_15c0, 0x004b_4e50]);
        mapped(&mut e, 0x1000);
        e.register(0x0043_9f50, |e, a| {
            assert_eq!(a[0], 0x2000);
            for i in 0..3 {
                e.mem.set_f32(a[2] + i * 4, (a[1] * 3 + i) as f32);
            }
            Ret::default()
        });
        e
    }

    #[test]
    fn matrix_rows_convert_to_padded_vectors() {
        let mut e = matrix_engine();
        assert_eq!(
            fn_009ae4d0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000)).addr(),
            0x1000
        );
        for row in 0..3 {
            for column in 0..3 {
                assert_eq!(
                    e.mem.f32(0x1000 + row * 16 + column * 4),
                    (row * 3 + column) as f32
                );
            }
            assert_eq!(e.mem.u32(0x100c + row * 16), 0);
        }
    }

    #[test]
    fn transform_copy_forwards_translation_after_rows() {
        let mut e = matrix_engine();
        assert_eq!(
            fn_009ae4a0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000)).addr(),
            0x1000
        );
        assert_eq!(
            e.call_log.unwrap().last().unwrap(),
            &(0x004b_4e50, vec![0x1030, 0x2024])
        );
    }

    #[test]
    fn fixed_buffer_constructor_initializes_each_member() {
        let mut e = engine_with(&[0x0062_1060, 0x0068_15c0]);
        mapped(&mut e, 0x1000);
        e.register(0x0053_7e90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        assert_eq!(
            fn_009afaf0(&mut e, Ptr::new(0x1000), 0x8000, 12).addr(),
            0x1000
        );
        assert_eq!(e.mem.u32(0x1000), 0x0108_f288);
        for (offset, value) in [(8, 0x8000), (12, 0x8000), (16, 12), (20, 0)] {
            assert_eq!(e.mem.u32(0x1000 + offset), value);
        }
    }

    #[test]
    fn fixed_buffer_delete_handles_null_and_allocator_size() {
        let mut e = engine_with(&[0x6000_0008]);
        mapped(&mut e, 0x1000);
        mapped(&mut e, 0x2000);
        e.mem.set_u32(0x1000, 0x2000);
        e.mem.set_u32(0x2008, 0x6000_0008);
        e.register(0x00c8_5750, |_, a| {
            assert!(a.is_empty());
            0x7000u32.into_ret()
        });
        e.register(0x0044_edb0, |_, a| {
            assert_eq!(a, &[0x7000]);
            0x1000u32.into_ret()
        });
        fn_009afbe0(&mut e, Ptr::NULL);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        fn_009afbe0(&mut e, Ptr::new(0x8000));
        assert_eq!(
            e.call_log.unwrap().last().unwrap().1,
            vec![0x1000, 0x8000, 0x18]
        );
    }

    #[test]
    fn embedded_array_uses_resolved_havok_object() {
        let mut e = engine_with(&[0x0062_d2f0]);
        e.register(0x004a_e750, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x2000u32.into_ret()
        });
        e.register(0x0062_d2f0, |_, a| (a[0] + 0x160).into_ret());
        assert_eq!(e.call(0x009a_fed0, &args![0x1000u32]).u32(), 0x2160);
        assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![0x2000]);
    }

    #[test]
    fn weapon_source_accessor_reads_pc_offset() {
        let mut e = Engine::new();
        mapped(&mut e, 0x1000);
        e.mem.set_u32(0x10e0, 0x9876);
        assert_eq!(e.call(0x009b_0900, &args![0x1000u32]).u32(), 0x9876);
    }

    #[test]
    fn float_accessor_preserves_negative_zero() {
        let mut e = Engine::new();
        mapped(&mut e, 0x1000);
        e.mem.set_u32(0x1074, 0x8000_0000);
        assert_eq!(
            e.call(0x009b_0bd0, &args![0x1000u32]).f32().to_bits(),
            0x8000_0000
        );
    }

    fn impulse_engine(body: u32) -> Engine {
        let mut e = engine_with(&[
            0x0068_15c0,
            0x00a2_9680,
            0x0081_82d0,
            0x00c9_c1d0,
            0x6000_0054,
        ]);
        mapped(&mut e, 0x3000);
        mapped(&mut e, 0x4000);
        e.mem.set_u32(0x3000, 0x4000);
        e.mem.set_u32(0x4054, 0x6000_0054);
        e.register_double(0x004a_e750, move |_, a| {
            assert_eq!(a, &[0x1000]);
            body.into_ret()
        });
        e.register(0x009d_9f40, |_, a| {
            assert_eq!(a, &[0x2000]);
            0x3000u32.into_ret()
        });
        e.register(0x004a_3e00, |e, a| {
            assert_eq!(a[1], 0x9000);
            e.mem.set_f32(a[0], 5.0);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn linear_impulse_branches_on_body_and_brackets_call() {
        for body in [0, 0x2000] {
            let mut e = impulse_engine(body);
            fn_009b0c50(&mut e, Ptr::new(0x1000), Ptr::new(0x9000));
            let log = e.call_log.unwrap();
            assert_eq!(log.len(), if body == 0 { 1 } else { 4 });
            if body != 0 {
                assert_eq!(log[2], (0x0081_82d0, vec![body, 0x9000]));
                assert_eq!(log[1], log[3]);
            }
        }
    }

    #[test]
    fn converted_linear_impulse_passes_stack_result() {
        let mut e = impulse_engine(0x2000);
        e.register(0x0081_82d0, |e, a| {
            assert_eq!(a[0], 0x2000);
            assert_eq!(e.mem.f32(a[1]), 5.0);
            Ret::default()
        });
        fn_009b0bf0(&mut e, Ptr::new(0x1000), Ptr::new(0x9000));
        assert_eq!(e.call_log.unwrap().len(), 6);
    }

    #[test]
    fn motion_impulse_branches_on_body_and_brackets_call() {
        for body in [0, 0x2000] {
            let mut e = impulse_engine(body);
            fn_009b0cf0(&mut e, Ptr::new(0x1000), Ptr::new(0x9000));
            let log = e.call_log.unwrap();
            assert_eq!(log.len(), if body == 0 { 1 } else { 6 });
            if body != 0 {
                assert_eq!(log[4], (0x6000_0054, vec![0x3000, 0x9000]));
                assert_eq!(log[1], log[5]);
            }
        }
    }

    #[test]
    fn converted_motion_impulse_passes_stack_result() {
        let mut e = impulse_engine(0x2000);
        e.register(0x6000_0054, |e, a| {
            assert_eq!(a[0], 0x3000);
            assert_eq!(e.mem.f32(a[1]), 5.0);
            Ret::default()
        });
        fn_009b0c90(&mut e, Ptr::new(0x1000), Ptr::new(0x9000));
        assert_eq!(e.call_log.unwrap().len(), 8);
    }

    #[test]
    fn motion_forwarding_activates_before_lookup() {
        let mut e = impulse_engine(0x2000);
        fn_009b0d30(&mut e, Ptr::new(0x2000), Ptr::new(0x9000));
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (0x00c9_c1d0, vec![0x2000]),
                (0x009d_9f40, vec![0x2000]),
                (0x6000_0054, vec![0x3000, 0x9000])
            ]
        );
    }

    #[test]
    fn target_constructor_leaves_padding_and_initializes_list() {
        let mut e = engine_with(&[0x0096_a2d0]);
        mapped(&mut e, 0x1000);
        e.mem.set_u32(0x1004, u32::MAX);
        assert_eq!(fn_009b0f40(&mut e, Ptr::new(0x1000)).addr(), 0x1000);
        assert_eq!(e.mem.u32(0x1000), 0);
        assert_eq!(e.mem.u32(0x1004), 0xff00_0000);
        assert_eq!(e.call_log.unwrap(), vec![(0x0096_a2d0, vec![0x1008])]);
    }

    #[test]
    fn sounds_recheck_validity_after_stop() {
        for validity in [
            vec![false, false],
            vec![true, false, false],
            vec![true, true, true, true],
        ] {
            let mut e = engine_with(&[0x00ad_88f0, 0x00ad_8d10]);
            let expected_releases = if validity.len() == 4 { 2 } else { 0 };
            let mut values = validity.into_iter();
            e.register_double(0x00ad_8ce0, move |_, _| values.next().unwrap().into_ret());
            explosion_stop_sounds(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            assert_eq!(
                log.iter()
                    .filter(|(address, _)| *address == 0x00ad_8d10)
                    .count(),
                expected_releases
            );
            assert!(log.iter().all(|(_, a)| a[0] == 0x10ac || a[0] == 0x10b8));
        }
    }
    #[test]
    fn target_process_impulse_obeys_null_threshold_and_by_value_position() {
        for (target, process, magnitude, forwarded) in [
            (0, 0x3000, 3.0, false),
            (0x2000, 0, 3.0, false),
            (0x2000, 0x3000, 2.0, false),
            (0x2000, 0x3000, f32::NAN, false),
            (0x2000, 0x3000, 3.0, true),
        ] {
            let mut e = engine_with(&[0x6000_0418]);
            for address in [0x1000, 0x3000, 0x4000, 0x5000, 0x6000, 0x7000] {
                mapped(&mut e, address);
            }
            e.mem.set_u32(0x1000, 0x4000);
            e.mem.set_u32(0x41f4, 0x6000_01f4);
            e.mem.set_u32(0x3000, 0x5000);
            e.mem.set_u32(0x5418, 0x6000_0418);
            e.mem.set_f32(0x6000, 2.0);
            for i in 0..3 {
                e.mem.set_u32(0x7000 + i * 4, i + 7);
            }
            e.register_double(0x008d_8520, move |_, a| {
                assert_eq!(a, &[target]);
                process.into_ret()
            });
            e.register(0x0040_3e20, |_, a| {
                assert_eq!(a, &[0x011d_1188]);
                0x6000u32.into_ret()
            });
            e.register(0x6000_01f4, |_, a| {
                assert_eq!(a, &[0x1000, 0]);
                0x7000u32.into_ret()
            });
            fn_009b0d70(&mut e, Ptr::new(0x1000), Ptr::new(target), magnitude);
            let log = e.call_log.unwrap();
            assert_eq!(
                log.iter().any(|(address, _)| *address == 0x6000_0418),
                forwarded
            );
            if forwarded {
                assert_eq!(log.last().unwrap().1, vec![0x3000, 0x2000, 7, 8, 9]);
            }
        }
    }

    #[test]
    fn sound_start_handles_each_optional_record_and_temporary_cleanup() {
        for present in [false, true] {
            let mut e = engine_with(&[0x0041_8900, 0x0048_3710]);
            e.register(0x00ad_8ce0, |_, _| false.into_ret());
            e.register(0x0041_81e0, |_, a| {
                assert_eq!(a, &[0x1000]);
                0x2000u32.into_ret()
            });
            for getter in [0x004f_d3c0, 0x0091_85e0] {
                e.register_double(getter, move |_, a| {
                    assert_eq!(a, &[0x2000]);
                    (if present { 0x3000u32 } else { 0 }).into_ret()
                });
            }
            e.register(0x0084_e3a0, |_, a| {
                assert_eq!(a, &[0x3000]);
                77u32.into_ret()
            });
            e.register(0x0093_3150, |_, a| {
                assert_eq!(a[0], 0x1000);
                assert_eq!(&a[2..], &[77, 0, 0x4000_0182, 1]);
                a[1].into_ret()
            });
            explosion_start_sounds(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            let assignments: Vec<_> = log
                .iter()
                .filter(|(address, _)| *address == 0x0041_8900)
                .collect();
            assert_eq!(assignments.len(), if present { 2 } else { 0 });
            if present {
                assert_eq!(assignments[0].1[0], 0x10ac);
                assert_eq!(assignments[1].1[0], 0x10b8);
            }
            assert_eq!(
                log.iter()
                    .filter(|(address, _)| *address == 0x0048_3710)
                    .count(),
                assignments.len()
            );
        }
    }

    #[test]
    fn light_attach_checks_node_and_form_before_assignment() {
        for (node, form) in [(0u32, 0x3000u32), (0x2000, 0), (0x2000, 0x3000)] {
            let mut e = engine_with(&[0x0066_b0d0]);
            mapped(&mut e, 0x1000);
            mapped(&mut e, 0x4000);
            e.mem.set_u32(0x1000, 0x4000);
            e.mem.set_u32(0x41d0, 0x6000_01d0);
            e.register_double(0x6000_01d0, move |_, a| {
                assert_eq!(a, &[0x1000]);
                node.into_ret()
            });
            e.register(0x0041_81e0, |_, _| 0x5000u32.into_ret());
            e.register_double(0x004f_b070, move |_, a| {
                assert_eq!(a, &[0x5000]);
                form.into_ret()
            });
            e.register(0x0050_d810, |_, a| {
                assert_eq!(a, &[0x3000, 0x1000, 0x2000, 0]);
                0x7000u32.into_ret()
            });
            explosion_attach_light(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            if node != 0 && form != 0 {
                assert_eq!(log.last().unwrap(), &(0x0066_b0d0, vec![0x10c4, 0x7000]));
            } else {
                assert!(!log.iter().any(|(address, _)| *address == 0x0066_b0d0));
            }
        }
    }

    #[test]
    fn light_remove_checks_pointer_and_preserves_pending_stack_argument() {
        for light in [0u32, 0x2000] {
            let mut e = engine_with(&[0x00b5_eed0, 0x0066_b0d0]);
            e.register_double(0x0055_9450, move |_, a| {
                assert_eq!(a, &[0x10c4]);
                light.into_ret()
            });
            e.register(0x0045_0b80, |_, a| {
                assert_eq!(a, &[0]);
                0x3000u32.into_ret()
            });
            fn_009b12d0(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            assert_eq!(log.len(), if light == 0 { 1 } else { 5 });
            if light != 0 {
                assert_eq!(log[3], (0x00b5_eed0, vec![0x3000, light]));
                assert_eq!(log[4], (0x0066_b0d0, vec![0x10c4, 0]));
            }
        }
    }

    #[test]
    fn light_fade_passes_computed_factor_through_both_clamps() {
        for light in [0u32, 0x2000] {
            let mut e = engine_with(&[0x0050_dd20]);
            mapped(&mut e, 0x0101_1000);
            e.set_global(0x0101_1590, 2.0f64);
            e.register_double(0x0055_9450, move |_, _| light.into_ret());
            e.register(0x0051_b8e0, |_, _| 1.0f32.into_ret());
            e.register(0x0050_7e10, |_, _| 8.0f32.into_ret());
            e.register(0x0040_ebd0, |_, a| {
                assert_eq!(a, &[1.0f32.to_bits(), 0.75f32.to_bits()]);
                0.75f32.into_ret()
            });
            e.register(0x0040_4010, |_, a| {
                assert_eq!(a, &[0, 0.75f32.to_bits()]);
                0.75f32.into_ret()
            });
            fn_009b1320(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            if light == 0 {
                assert_eq!(log.len(), 1);
            } else {
                assert_eq!(
                    log.last().unwrap(),
                    &(0x0050_dd20, vec![light, 0.75f32.to_bits()])
                );
            }
        }
    }

    #[test]
    fn radiation_update_requires_positive_inputs_and_floors_nonpositive_result() {
        for (radius, duration, age, expected) in [
            (0.0f32, 4.0f32, 1.0f32, None),
            (2.0, 0.0, 1.0, None),
            (f32::NAN, 4.0, 1.0, None),
            (2.0, 4.0, 1.0, Some(1.5)),
            (2.0, 4.0, 4.0, Some(0.01)),
            (2.0, 4.0, 5.0, Some(0.01)),
        ] {
            let mut e = engine_with(&[0x0042_2350, 0x6000_0048]);
            for address in [0x1000, 0x2000, 0x0101_2000, 0x0101_3000] {
                mapped(&mut e, address);
            }
            e.mem.set_u32(0x1000, 0x2000);
            e.mem.set_u32(0x2048, 0x6000_0048);
            e.set_global(0x0101_2060, 0.0f64);
            e.set_global(0x0101_3ea4, 0.01f32);
            e.register(0x0041_81e0, |_, _| 0x3000u32.into_ret());
            e.register_double(0x008d_01e0, move |_, _| radius.into_ret());
            e.register_double(0x004e_3cc0, move |_, _| duration.into_ret());
            e.register_double(0x0051_b8e0, move |_, _| age.into_ret());
            e.register(0x005d_43c0, |_, a| {
                assert_eq!(a, &[0x1000]);
                0x4000u32.into_ret()
            });
            fn_009b13a0(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            let update = log.iter().find(|(address, _)| *address == 0x0042_2350);
            if let Some(expected) = expected {
                assert_eq!(update.unwrap().1, vec![0x4000, f32::to_bits(expected)]);
                assert_eq!(log.last().unwrap().1, vec![0x1000, 0x4000_0000]);
            } else {
                assert!(update.is_none());
            }
        }
    }

    #[test]
    fn process_level_retains_loaded_cell_or_matching_level() {
        for (node, fallback, loaded, level, retained) in [
            (0u32, false, false, 2u32, false),
            (1, false, true, 2, true),
            (0, true, true, 2, true),
            (1, false, false, 3, true),
            (1, false, false, 2, false),
        ] {
            let mut e = engine_with(&[0x6000_0010]);
            for address in [0x1000, 0x2000, 0x011c_3000, 0x011d_e000] {
                mapped(&mut e, address);
            }
            e.mem.set_u32(0x1000, 0x2000);
            e.mem.set_u32(0x21d0, 0x6000_01d0);
            e.mem.set_u32(0x2010, 0x6000_0010);
            e.mem.set_u32(0x1068, 0x3000);
            e.set_global(0x011c_3b3c, 0x4000u32);
            e.set_global(0x011d_ea10, 0x5000u32);
            e.register_double(0x0045_cd60, move |_, a| {
                assert_eq!(a, &[0x3000]);
                level.into_ret()
            });
            e.register(0x008d_6f30, |_, _| 0x6000u32.into_ret());
            e.register_double(0x6000_01d0, move |_, _| node.into_ret());
            e.register_double(0x0044_5750, move |_, a| {
                assert_eq!(a, &[0x4000, 0x1000]);
                fallback.into_ret()
            });
            e.register_double(0x0045_11e0, move |_, a| {
                assert_eq!(a, &[0x5000, 0x6000, 1]);
                loaded.into_ret()
            });
            assert_eq!(fn_009b1450(&mut e, Ptr::new(0x1000)), retained);
            assert_eq!(
                e.call_log
                    .unwrap()
                    .iter()
                    .any(|(address, _)| *address == 0x6000_0010),
                !retained
            );
        }
    }
}
