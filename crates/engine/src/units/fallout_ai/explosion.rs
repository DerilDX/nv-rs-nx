//! `fallout/ai/combat/explosion.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

// Continuation audit (2026-10-10, round 6): repaired 009b0f80's callback
// reload and added a regression test. Eight older functions remain partial for
// SEH cleanup. The translated normal paths 009ac5a0, 009ac9c0 and 009b22c0 also
// remain partial for SEH cleanup; 009ae5d0 is complete.
// Seven entries remain unimplemented or skipped; resume at 009ae6e0.
// 009acd60 skipped: the 0x178 compiler-generated local passed through
//   00404eb0/00404ee0 and its SEH state transitions are not yet identified.
// 009ad670 skipped: Init3D's per-iteration 0xe0 Havok cinfo/temporary layout
//   and nested shape/action cleanup are not verified against PC field offsets.
// 009ae280 skipped: the local cinfo passed to 0056e2d0 and its stack lifetime
//   overlap with the temporary string object are not resolved.
// 009ae6e0 skipped: large effect/damage flow; exception-state reconstruction,
//   its 0x938-byte frame, aligned collector/UI temporaries and callback state
//   are not fully established against the PC stack slots.
// 009b00a0 skipped: its 0x110-byte frame mixes LOS hits, HitData, multiple
//   virtual actor callbacks and several magic/form resolution branches; exact
//   callback reloads and hit-record lifetime remain unaudited.
// 009b1810 skipped: RunLOSPick's 0x908-byte frame has 800-byte and 812-byte
//   collector buffers plus seven bound-probe points; helper layouts and SEH
//   cleanup transitions are not sufficiently verified.
// 009b2940 skipped: its 0x4cc-byte frame contains a 1040-byte collector and
//   nested temporary vectors; collector iteration and compiler SEH cleanup are
//   not fully verified against PC stack slots.

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
        entry!(0x009ac200, explosion_ctor_ov2(Ptr, Ptr, Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x009b0e00, fn_009b0e00(Ptr, Ptr, u8, u8) -> Ptr),
        entry!(0x009b0f80, fn_009b0f80(Ptr) -> f32),
        entry!(0x009ac5a0, fn_009ac5a0(Ptr)),
        entry!(0x009ac9c0, fn_009ac9c0(Ptr, Ptr, Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x009afcc0, fn_009afcc0(Ptr)),
        entry!(0x009afef0, fn_009afef0(Ptr)),
        entry!(0x009b0920, fn_009b0920(Ptr, Ptr)),
        entry!(0x009b2040, fn_009b2040(Ptr, Ptr)),
        entry!(0x009b22c0, fn_009b22c0(Ptr, Ptr)),
        entry!(0x009b25b0, fn_009b25b0(Ptr, Ptr)),
        entry!(0x009b1510, fn_009b1510(Ptr, Ptr, Ptr, f32)),
        entry!(0x009b1630, fn_009b1630(Ptr, Ptr, i32, f32)),
        entry!(0x009b1720, fn_009b1720(Ptr, Ptr) -> Ptr),
        entry!(0x009b17a0, fn_009b17a0(Ptr, u32)),
        entry!(0x009b2770, fn_009b2770(Ptr, u32)),
        entry!(0x009b2790, fn_009b2790(Ptr, u32)),
        entry!(0x009b28a0, fn_009b28a0(Ptr, Ptr)),
        entry!(0x009b2c90, fn_009b2c90(Ptr) -> Ptr),
        entry!(0x009b2cb0, fn_009b2cb0(Ptr, u32) -> Ptr),
        entry!(0x009b2d30, fn_009b2d30(Ptr)),
        entry!(0x009b2db0, fn_009b2db0(Ptr)),
        entry!(0x009b2dd0, fn_009b2dd0(Ptr, Ptr) -> Ptr),
        entry!(0x009b2e00, fn_009b2e00(Ptr, i32)),
        entry!(0x009b2e30, fn_009b2e30(Ptr, u32) -> Ptr),
        entry!(0x009b2e60, fn_009b2e60(u32, u32) -> Ptr),
        entry!(0x009b2e90, fn_009b2e90(Ptr, i32, u8)),
        entry!(0x009b2ed0, fn_009b2ed0(Ptr, i32, i32, u8)),
        entry!(0x009b3080, fn_009b3080(Ptr, Ptr, Ptr) -> bool),
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
        entry!(0x009ae5d0, fn_009ae5d0(Ptr, u32)),
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
    fn_009ac5a0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 009ac5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the `Explosion` destruction path: removes particle and physics
/// attachments, target records, sounds and owned references, then destroys
/// the embedded members and `MobileObject` base.
// open: compiler SEH cleanup for partially destroyed members is not modeled.
// Translation incomplete: 009ac5a0 - compiler SEH cleanup for partially destroyed members
pub fn fn_009ac5a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_ee04);
    e.mem.set_u32(this.addr() + 0x18, 0x0108_edf8);

    let base = e.call(0x0043_fcd0, &args![this]).u32();
    if base != 0 {
        let has_object = e.vcall(base, 0xc, &args![]).u32();
        if has_object != 0 {
            let base = e.call(0x0043_fcd0, &args![this]).u32();
            let object = e.vcall(base, 0xc, &args![]).u32();
            if e.call(0x0057_84b0, &args![object]).bool() {
                let base = e.call(0x0043_fcd0, &args![this]).u32();
                let object = e.vcall(base, 0xc, &args![]).u32();
                e.call(0x0057_8170, &args![object]);
            }
        }
    }

    let physics = e.call(0x008d_6f30, &args![this]).u32();
    if physics != 0 && e.call(0x0045_43c0, &args![physics]).bool() {
        let physics = e.call(0x008d_6f30, &args![this]).u32();
        if physics != 0 && e.call(0x0045_43c0, &args![physics]).bool() {
            let base = e.call(0x0043_fcd0, &args![this]).u32();
            e.call(0x00c6_9ee0, &args![base, 1u32, 0u32]);
        }
    }

    fn_009b12d0(e, this);
    let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
    if phantom != 0 {
        let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
        e.vcall(phantom, 0xa0, &args![]);
    }
    e.call(0x0066_b0d0, &args![this.addr() + 0xa0, 0u32]);

    let targets = this.addr() + 0xa4;
    while !e.call(0x0082_56d0, &args![targets]).bool() {
        let value = e.call(0x0068_15c0, &args![targets]).u32();
        let record = e.mem.u32(value);
        if record != 0 {
            fn_009ac910(e, Ptr::new(record), 1);
        }
        e.call(0x0063_f7b0, &args![targets]);
    }

    if !e
        .call(0x0042_26e0, &args![e.global::<u32>(0x011c_3f2c)])
        .bool()
    {
        e.call(0x0092_ed60, &args![this, 0u32]);
    }
    explosion_stop_sounds(e, this);
    e.call(0x0042_f780, &args![this.addr() + 0xd0, 0u32]);
    let actor_cause = e.mem.u32(this.addr() + 0xdc);
    if actor_cause != 0 {
        e.vcall(actor_cause, 0, &args![1u32]);
    }

    let process_list_enabled = e.call(0x0040_8d60, &args![0x011c_7ae8u32]).u32();
    if e.mem.u8(process_list_enabled) != 0 {
        let list = 0x011c_7b8c;
        let mut node = e.call(0x0055_9450, &args![list]).u32();
        let mut removed = 0;
        while node != 0 && removed == 0 {
            let next = e.call(0x007b_52d0, &args![list, node]).u32();
            let reference = e.call(0x0063_17a0, &args![list, node]).u32();
            if e.mem.u32(reference) == this.addr() {
                removed = node;
            }
            node = next;
        }
        if removed != 0 {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), removed);
                e.call(0x0049_f590, &args![list, slot]);
            });
        }
    }

    let base = e.call(0x0043_fcd0, &args![this]).u32();
    let physics = e.call(0x008d_6f30, &args![this]).u32();
    if physics != 0 {
        let process = e.call(0x009d_9f20, &args![physics]).u32();
        if process != 0 {
            e.call(0x00c5_b370, &args![process, base]);
        }
    }

    e.call(0x0042_f760, &args![this.addr() + 0xd0]);
    e.call(0x0045_cec0, &args![this.addr() + 0xc4]);
    e.call(0x0048_3710, &args![this.addr() + 0xb8]);
    e.call(0x0048_3710, &args![this.addr() + 0xac]);
    e.call(0x0046_ffb0, &args![this.addr() + 0xa4]);
    e.call(0x0045_cec0, &args![this.addr() + 0xa0]);
    e.call(0x0092_ec40, &args![this]);
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

// Translated from 009ac9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Optionally creates an explosion, applies the game entry point damage
/// multiplier, attaches actor cause and performs the post-creation callbacks.
// open: compiler SEH cleanup for allocated explosion and actor-cause temporaries is not modeled.
// Translation incomplete: 009ac9c0 - compiler SEH cleanup for allocated explosion and actor-cause temporaries
#[allow(clippy::too_many_arguments)]
pub fn fn_009ac9c0(
    e: &mut Engine,
    definition: Ptr,
    owner: Ptr,
    actor_cause: Ptr,
    cell: Ptr,
    x: u32,
    y: u32,
    z: u32,
    m00: u32,
    m01: u32,
    m02: u32,
    m10: u32,
    m11: u32,
    m12: u32,
    m20: u32,
    m21: u32,
    m22: u32,
) -> Ptr {
    let player = Ptr::new(e.global::<u32>(0x011d_ea3c));
    let mut damage_multiplier = 1.0f32;
    let mut explosion = Ptr::new(0);

    if !definition.is_null() {
        if !owner.is_null() {
            if e.vcall(owner.addr(), 0x224, &args![]).bool() {
                let owner_base = e.call(0x0087_4480, &args![owner]).ptr::<()>();
                if owner_base == player {
                    let entry_value = e.call(0x0047_6c90, &args![owner]).u32();
                    e.with_stack(4, |e, output| {
                        e.mem.set_f32(output.addr(), damage_multiplier);
                        e.call(0x005e_58f0, &args![0x48u32, player, entry_value, output]);
                        damage_multiplier = e.mem.f32(output.addr());
                    });
                }
            } else if owner == player {
                let process = e.call(0x008d_8520, &args![player]).ptr::<()>();
                let saved_object = if process.is_null() {
                    0
                } else {
                    e.vcall(process.addr(), 0x148, &args![]).u32()
                };
                let entry_value = if saved_object == 0 {
                    0
                } else {
                    e.call(0x0044_ddc0, &args![saved_object]).u32()
                };
                e.with_stack(4, |e, output| {
                    e.mem.set_f32(output.addr(), damage_multiplier);
                    e.call(0x005e_58f0, &args![0x48u32, player, entry_value, output]);
                    damage_multiplier = e.mem.f32(output.addr());
                });
            }
        } else if owner == player {
            let process = e.call(0x008d_8520, &args![player]).ptr::<()>();
            let saved_object = if process.is_null() {
                0
            } else {
                e.vcall(process.addr(), 0x148, &args![]).u32()
            };
            let entry_value = if saved_object == 0 {
                0
            } else {
                e.call(0x0044_ddc0, &args![saved_object]).u32()
            };
            e.with_stack(4, |e, output| {
                e.mem.set_f32(output.addr(), damage_multiplier);
                e.call(0x005e_58f0, &args![0x48u32, player, entry_value, output]);
                damage_multiplier = e.mem.f32(output.addr());
            });
        }

        let allocation = e.call(0x0040_1000, &args![0x104u32]).ptr::<()>();
        if !allocation.is_null() {
            explosion = explosion_ctor_ov2(
                e, allocation, definition, owner, cell, x, y, z, m00, m01, m02, m10, m11, m12, m20,
                m21, m22,
            );
        }
    }

    let base_damage = e.call(0x0064_4930, &args![explosion]).f32();
    let scaled_damage = quiet_f32(((base_damage as f64) * (damage_multiplier as f64)) as f32);
    e.call(0x0040_7880, &args![explosion, scaled_damage]);

    if !explosion.is_null() {
        let current_owner = fn_009acce0(e, explosion);
        if !current_owner.is_null() {
            let attached_actor_cause = if e.vcall(current_owner.addr(), 0x100, &args![]).bool() {
                let allocation = e.call(0x0040_1000, &args![0x1cu32]).ptr::<()>();
                if allocation.is_null() {
                    0
                } else {
                    let owner = fn_009acce0(e, explosion);
                    e.call(0x0066_e010, &args![allocation, owner]).u32()
                }
            } else {
                let current_owner = fn_009acce0(e, explosion);
                e.vcall(current_owner.addr(), 0x168, &args![]).u32()
            };
            e.vcall(explosion.addr(), 0x164, &args![attached_actor_cause]);
        }

        e.call(0x0056_c7d0, &args![explosion, actor_cause]);
        if !actor_cause.is_null() && e.call(0x0057_2c80, &args![actor_cause]).bool() {
            e.vcall(actor_cause.addr(), 0x48, &args![4u32]);
        }
        if !e.call(0x008c_7aa0, &args![]).bool() {
            e.vcall(explosion.addr(), 0x304, &args![]);
        } else {
            let process = e.call(0x0045_37b0, &args![explosion]).u32();
            e.call(0x0087_ae60, &args![process]);
        }
    }
    explosion
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

// Translated from 009ae5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// On the first frame, reports the first target to the process-list callback.
/// Processes targets after the explosion's age advances, updates radiation
/// and scale while not underwater, and asks the base explosion to finish when
/// both expiration comparisons pass.
pub fn fn_009ae5d0(e: &mut Engine, this: Ptr, _unused_1: u32) {
    if e.mem.u32(this.addr() + 0xe4) == 0 {
        let mut target = 0;
        let targets = this.addr() + 0xa4;
        if !e.call(0x0082_56d0, &args![targets]).bool() {
            let value = e.call(0x0068_15c0, &args![targets]).u32();
            let record = e.mem.u32(value);
            target = e.mem.u32(record);
        }
        let owner = fn_009acce0(e, this).addr();
        e.call(0x008d_0600, &args![0x011e_0e80u32, owner, target]);
    }

    let age = e.mem.f32(this.addr() + 0x90);
    let now = e.call(0x0051_b8e0, &args![this]).f32();
    if now > age && e.mem.u8(this.addr() + 0xd5) == 0 {
        e.call(0x009a_fcc0, &args![this]);
    }

    let frame_count = e.mem.u32(this.addr() + 0xe4).wrapping_add(1);
    e.mem.set_u32(this.addr() + 0xe4, frame_count);
    if !fn_009ad5f0(e, this) {
        fn_009b13a0(e, this);
        fn_009b1320(e, this);
    }

    let now = e.call(0x0051_b8e0, &args![this]).f32();
    let expiration = e.call(0x0050_7e10, &args![this]).f32();
    if expiration < now {
        let now = e.call(0x0051_b8e0, &args![this]).f32();
        let base = e.call(0x0041_81e0, &args![this]).u32();
        let base_expiration = e.call(0x004e_3cc0, &args![base]).f32();
        if base_expiration < now {
            fn_009ac9a0(e, this);
        }
    }
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
/// Initializes the base, members and defaults.
// open: compiler SEH cleanup after constructor failure is not modeled.
// Translation incomplete: 009abf30 - compiler SEH cleanup after constructor failure
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
/// action and increments its live counter.
// open: compiler SEH cleanup after constructor failure is not modeled.
// Translation incomplete: 009ae140 - compiler SEH cleanup after constructor failure
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
/// Initializes the fixed-buffer contact collector.
// open: compiler SEH cleanup after constructor failure is not modeled.
// Translation incomplete: 009afaf0 - compiler SEH cleanup after constructor failure
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
/// their base-form records.
// open: compiler SEH cleanup after sound-handle construction failure is not modeled.
// Translation incomplete: 009b1090 - compiler SEH cleanup after sound-handle construction failure
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

/// FLD/FSTP quiets a signaling single-precision NaN without changing its payload.
fn quiet_f32(value: f32) -> f32 {
    let bits = value.to_bits();
    f32::from_bits(
        if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
            bits | 0x0040_0000
        } else {
            bits
        },
    )
}

// Translated from 009ac200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::Explosion_ov2` (Xbox PDB). RET 0x3c consumes three pointers,
/// the three position words and nine orientation words. PC 009ac2fc takes
/// position at EBP+0x14; 009ac310 copies the matrix from EBP+0x20;
/// 009ac544 returns with RET 0x3c.
// open: compiler SEH cleanup after constructor failure is not modeled.
// Translation incomplete: 009ac200 - compiler SEH cleanup after constructor failure
#[allow(clippy::too_many_arguments)]
pub fn explosion_ctor_ov2(
    e: &mut Engine,
    this: Ptr,
    definition: Ptr,
    owner: Ptr,
    cell: Ptr,
    x: u32,
    y: u32,
    z: u32,
    m00: u32,
    m01: u32,
    m02: u32,
    m10: u32,
    m11: u32,
    m12: u32,
    m20: u32,
    m21: u32,
    m22: u32,
) -> Ptr {
    e.call(0x0092_eb50, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_ee04);
    e.mem.set_u32(this.addr() + 0x18, 0x0108_edf8);
    e.call(0x0063_3c90, &args![this.addr() + 0xa0, 0u32]);
    e.call(0x0096_a2d0, &args![this.addr() + 0xa4]);
    for offset in [0xac, 0xb8] {
        e.call(0x0041_a250, &args![this.addr() + offset]);
    }
    e.call(0x0063_3c90, &args![this.addr() + 0xc4, 0u32]);
    e.call(0x0042_f730, &args![this.addr() + 0xd0, 0u32]);
    for offset in [0xe8, 0xf4] {
        e.call(0x0068_15c0, &args![this.addr() + offset]);
    }
    e.mem.set_u32(this.addr() + 0x68, 0);
    e.mem.set_u32(this.addr() + 0xc8, owner.addr());
    e.call(0x0093_0700, &args![this, cell]);
    e.with_stack(12, |e, position| {
        for (index, bits) in [x, y, z].into_iter().enumerate() {
            e.mem.set_u32(position.addr() + index as u32 * 4, bits);
        }
        e.call(0x0057_5830, &args![this, position]);
    });
    e.call(
        0x0056_fa90,
        &args![this, m00, m01, m02, m10, m11, m12, m20, m21, m22],
    );
    e.mem.set_u8(this.addr() + 0xd8, 0);
    if !definition.is_null() {
        e.call(0x0057_5690, &args![this, definition]);
        let base = e.call(0x0041_81e0, &args![this]).ptr::<()>();
        let radius = e.call(0x0047_7900, &args![base]).f32();
        e.mem.set_f32(this.addr() + 0x94, radius);
        let base = e.call(0x0041_81e0, &args![this]).ptr::<()>();
        let radius = quiet_f32(fn_009ac550(e, base));
        e.mem.set_f32(this.addr() + 0x98, radius);
        let base = e.call(0x0041_81e0, &args![this]).u32();
        let flag = e.call(0x0047_7950, &args![base, 0x40u32]).u8();
        e.mem.set_u8(this.addr() + 0xd8, flag);
        let base = e.call(0x0041_81e0, &args![this]).u32();
        if e.call(0x0047_7950, &args![base, 2u32]).bool() {
            let mut words = vec![this.addr()];
            for index in 0..9 {
                words.push(e.global::<u32>(0x011a_9448 + index * 4));
            }
            e.call(0x0056_fa90, &words);
        }
    }
    e.mem.set_u32(this.addr() + 0xe4, 0);
    let lifetime = quiet_f32(e.global::<f32>(0x0101_7718));
    e.mem.set_f32(this.addr() + 0x8c, lifetime);
    e.mem.set_f32(this.addr() + 0x90, 0.0);
    e.mem.set_f32(this.addr() + 0x88, 0.0);
    e.mem.set_f32(this.addr() + 0x9c, 1.0);
    for offset in [0xa0, 0xc4] {
        e.call(0x0066_b0d0, &args![this.addr() + offset, 0u32]);
    }
    e.call(0x0042_f780, &args![this.addr() + 0xd0, 0u32]);
    e.mem.set_u32(this.addr() + 0xe0, 0);
    for offset in [0xd4, 0xd5, 0xd6, 0xd7, 0xd9] {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    let caster = if !owner.is_null() && e.vcall(owner.addr(), 0x100, &args![]).bool() {
        0
    } else {
        let allocation = e.call(0x0040_1000, &args![0x24u32]).u32();
        if allocation == 0 {
            0
        } else {
            e.call(0x0082_5ad0, &args![allocation, this]).u32()
        }
    };
    e.mem.set_u32(this.addr() + 0xdc, caster);
    for offset in [0xe8, 0xf4] {
        for component in [0, 4, 8] {
            let bits = e.global::<u32>(0x011f_426c + component);
            e.mem.set_u32(this.addr() + offset + component, bits);
        }
    }
    let damage = quiet_f32(e.global::<f32>(0x0101_2054));
    e.mem.set_f32(this.addr() + 0x100, damage);
    this
}

// Translated from 009b0e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a reference's target record, setting only currently zero flag bytes,
/// or appends a newly allocated record. The native null-allocation dereference
/// is retained.
// open: compiler SEH cleanup after record/list construction failure is not modeled.
// Translation incomplete: 009b0e00 - compiler SEH cleanup after record/list construction failure
pub fn fn_009b0e00(e: &mut Engine, this: Ptr, reference: Ptr, first: u8, second: u8) -> Ptr {
    let mut node = this.addr().wrapping_add(0xa4);
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(slot);
        if record != 0 && e.mem.u32(record) == reference.addr() {
            for (offset, value) in [(4, first), (5, second)] {
                if value != 0 && e.mem.u8(record + offset) == 0 {
                    e.mem.set_u8(record + offset, value);
                }
            }
            return Ptr::new(record);
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    let allocation = e.call(0x0040_1000, &args![0x10u32]).ptr::<()>();
    let record = if allocation.is_null() {
        allocation
    } else {
        fn_009b0f40(e, allocation)
    };
    e.mem.set_u32(record.addr(), reference.addr());
    e.mem.set_u8(record.addr() + 4, first);
    e.mem.set_u8(record.addr() + 5, second);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), record.addr());
        e.call(0x005a_e3d0, &args![this.addr() + 0xa4, slot]);
        Ptr::new(e.mem.u32(slot.addr()))
    })
}

// Translated from 009b0f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes and caches explosion damage. FCOMP's C0 test also recomputes NaN.
/// Weapon actor-value scaling rounds at both explicit single-precision stores.
pub fn fn_009b0f80(e: &mut Engine, this: Ptr) -> f32 {
    let cached = e.mem.f32(this.addr() + 0x100);
    let threshold = e.global::<f64>(0x0101_2060);
    if matches!(
        (cached as f64).partial_cmp(&threshold),
        None | Some(std::cmp::Ordering::Less)
    ) {
        let base = e.call(0x0041_81e0, &args![this]).u32();
        let damage = e.call(0x0064_79e0, &args![base]).f32();
        e.mem.set_f32(this.addr() + 0x100, damage);
        let owner = fn_009acce0(e, this);
        if !owner.is_null() {
            let actor_owner = fn_009acce0(e, this);
            if e.vcall(actor_owner.addr(), 0x100, &args![]).bool() {
                // Native code checks LastBoundWeapon, then reloads pOwner and
                // calls LastBoundWeapon again before applying actor-value scaling.
                if !fn_009b0900(e, this).is_null() {
                    let owner = fn_009acce0(e, this);
                    let weapon = fn_009b0900(e, this);
                    let value = e.call(0x008d_85e0, &args![weapon]).u32();
                    let skill = e.vcall(owner.addr() + 0xa4, 0xc, &args![value]).f32();
                    let additive = e.call(0x0040_3e20, &args![0x011c_ece8u32]).u32();
                    let factor = e.call(0x0040_3e20, &args![0x011c_f350u32]).u32();
                    let multiplier =
                        ((skill as f64) * e.global::<f64>(0x0101_6408) * (e.mem.f32(factor) as f64)
                            + (e.mem.f32(additive) as f64)) as f32;
                    let result =
                        ((e.mem.f32(this.addr() + 0x100) as f64) * (multiplier as f64)) as f32;
                    e.mem.set_f32(this.addr() + 0x100, result);
                }
            }
        }
    }
    e.mem.f32(this.addr() + 0x100)
}

// Translated from 009afcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Enumerates collidables in spPhantom, finds their references, and records
/// eligible actor targets with their closest body-part hit distance. The
/// target-found flag is set after the scan even when the phantom is empty.
pub fn fn_009afcc0(e: &mut Engine, this: Ptr) {
    let player = Ptr::new(e.global::<u32>(0x011d_ea3c));
    let player_target = fn_009b0e00(e, this, player, 0, 1);
    let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
    if phantom != 0 {
        let mut index = 0u32;
        loop {
            let array = fn_009afed0(e, Ptr::new(phantom));
            let count = e.call(0x0072_6070, &args![array]).u32();
            if index >= count {
                break;
            }

            let array = fn_009afed0(e, Ptr::new(phantom));
            let collidable_slot = e.call(0x0062_9ab0, &args![array, index]).u32();
            let collidable = e.mem.u32(collidable_slot);
            let av_object = e.call(0x004b_5820, &args![collidable]).u32();
            let reference = e.call(0x0056_f930, &args![av_object]).u32();
            if reference != 0 {
                let actor = e.vcall(reference, 0x100, &args![]).bool();
                if !(actor && e.call(0x008a_ce90, &args![reference]).bool()) {
                    let collidable_slot = e.call(0x0062_9ab0, &args![array, index]).u32();
                    let collidable = e.mem.u32(collidable_slot);
                    let owner = e.call(0x004b_59f0, &args![collidable]).u32();
                    let flags = if owner == 0 {
                        0
                    } else {
                        u32::from(e.call(0x0051_7670, &args![owner]).u32() < 4)
                    };

                    let record = if reference == player.addr() {
                        player_target
                    } else if reference == e.mem.u32(this.addr() + 0xc8) {
                        Ptr::new(0)
                    } else {
                        fn_009b0e00(e, this, Ptr::new(reference), 0, flags as u8)
                    };
                    if !record.is_null() && e.vcall(reference, 0x100, &args![]).bool() {
                        let damage = fn_009b0f80(e, this);
                        if damage as f64 > e.global::<f64>(0x0101_2060) {
                            let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                            let distance = e.with_stack(12, |e, point| {
                                let coordinate = e
                                    .call(0x0045_bb80, &args![av_object, point, position])
                                    .u32();
                                let coordinate = e.call(0x0043_9ef0, &args![coordinate]).u32();
                                e.call(0x0045_7990, &args![coordinate]).f32()
                            });
                            fn_009b1510(e, this, record, Ptr::new(av_object), distance);
                        }
                    }
                }
            }
            index = index.wrapping_add(1);
        }
    }
    e.mem.set_u8(this.addr() + 0xd5, 1);
}

// Translated from 009afef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes eligible rigid bodies from the phantom into the explosion's force
/// list. The two native collision-layer lookups are kept as address calls;
/// their internal Havok object representation is outside this unit.
pub fn fn_009afef0(e: &mut Engine, this: Ptr) {
    let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
    if phantom != 0 {
        let form_data = e.call(0x0041_81e0, &args![this, 0x20u32]).u32();
        let always_process = e.call(0x0047_7950, &args![form_data]).bool();
        let mut index = 0u32;
        loop {
            let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
            let array = fn_009afed0(e, Ptr::new(phantom));
            let count = e.call(0x0072_6070, &args![array]).u32();
            if index >= count {
                break;
            }

            let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
            let array = fn_009afed0(e, Ptr::new(phantom));
            let slot = e.call(0x0062_9ab0, &args![array, index]).u32();
            let collidable = e.mem.u32(slot);
            let owner = e.call(0x004b_59f0, &args![collidable]).u32();
            let collision_layer = e.call(0x004b_5a80, &args![owner]).u32();
            let layer = if collision_layer == 0 {
                0
            } else {
                e.with_stack(4, |e, out| {
                    let filter = e.call(0x0043_b4f0, &args![collision_layer, out]).u32();
                    e.call(0x0043_b4d0, &args![filter]).u32()
                })
            };

            let owner_layer = if collision_layer == 0 {
                None
            } else {
                Some(e.call(0x0051_7630, &args![collision_layer]).u32())
            };
            let eligible_layer = owner_layer.is_some_and(|value| value < 4) || layer == 0x14;
            if always_process || layer == 0x1d {
                let havok_body = e.call(0x004b_5a20, &args![owner]).u32();
                let entity = if havok_body == 0 {
                    0
                } else {
                    e.call(0x0044_ddc0, &args![havok_body]).u32()
                };
                let reference = e.call(0x0056_f930, &args![entity]).u32();
                let exploded_reference = e.mem.u32(this.addr() + 0xcc);
                if !always_process || reference == exploded_reference {
                    let is_actor = reference != 0 && e.vcall(reference, 0x100, &args![]).bool();
                    let skipped_idle =
                        is_actor && e.call(0x005a_29b0, &args![reference]).u32() == 1;
                    if !skipped_idle && eligible_layer {
                        e.call(0x009b_0920, &args![this, collision_layer]);
                    }
                }
            } else if eligible_layer {
                e.call(0x009b_0920, &args![this, collision_layer]);
            }
            index = index.wrapping_add(1);
        }
    }
    e.mem.set_u8(this.addr() + 0xd7, 1);
}

// Translated from 009b0920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies linear and angular impulses to a non-null rigid body when the
/// explosion's force magnitude is strictly above the global threshold.
/// Intermediate x87 values are rounded where the native code stores floats.
pub fn fn_009b0920(e: &mut Engine, this: Ptr, rigid_body: Ptr) {
    if rigid_body.is_null() {
        return;
    }
    let data = e.call(0x0041_81e0, &args![this]).u32();
    let mut magnitude = fn_009b0bd0(e, Ptr::new(data));
    if !matches!(
        (magnitude as f64).partial_cmp(&e.global::<f64>(0x0101_2060)),
        Some(std::cmp::Ordering::Greater)
    ) {
        return;
    }

    e.with_stack(0xa4, |e, frame| {
        let target_vector = Ptr::<()>::new(frame.addr());
        let direction = Ptr::<()>::new(frame.addr() + 0x20);
        let position = Ptr::<()>::new(frame.addr() + 0x38);
        let impulse = Ptr::<()>::new(frame.addr() + 0x70);
        let torque = Ptr::<()>::new(frame.addr() + 0x7c);
        let scaled_impulse = Ptr::<()>::new(frame.addr() + 0x64);
        let angular_intermediate = Ptr::<()>::new(frame.addr() + 0x78);
        let scaled_torque = Ptr::<()>::new(frame.addr() + 0x6c);
        let output = Ptr::<()>::new(frame.addr() + 0x98);
        e.call(0x0068_15c0, &args![target_vector]);
        e.call(0x0068_15c0, &args![direction]);
        let target_vector_object = e
            .vcall(rigid_body.addr(), 0xf0, &args![target_vector])
            .u32();
        e.call(0x0045_8620, &args![direction, target_vector_object]);

        let position_vector = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let transformed = e
            .call(0x0043_9ef0, &args![direction, position, position_vector])
            .u32();
        e.call(0x004a_0c10, &args![transformed]);

        let filter = e.call(0x0043_b4f0, &args![rigid_body, output]).u32();
        if e.call(0x0062_4070, &args![filter]).bool() {
            let coordinate = e.call(0x004a_51b0, &args![position, 2u32]).u32();
            let adjustment = e.call(0x0040_3e20, &args![0x011d_0e70u32]).u32();
            let adjusted = e.mem.f32(coordinate) + e.mem.f32(adjustment);
            e.mem.set_f32(coordinate, adjusted);
        }

        let mut actor_cause = false;
        if !fn_009ad610(e, this).is_null() {
            let proxy = e.call(0x004a_e750, &args![rigid_body]).u32();
            let body = e.call(0x004b_5a20, &args![proxy]).u32();
            let node = if body == 0 {
                0
            } else {
                e.call(0x0044_ddc0, &args![body]).u32()
            };
            let reference = e.call(0x0056_f930, &args![node]).u32();
            if reference != 0 && reference == fn_009ad610(e, this).addr() {
                let filter = e.call(0x0043_b4f0, &args![rigid_body, output]).u32();
                if e.call(0x0043_b4d0, &args![filter]).u32() == 0x14 {
                    e.vcall(rigid_body.addr(), 0xe4, &args![1u32]);
                }
                let scale = e.call(0x0040_3e20, &args![0x011d_0dbc_u32]).u32();
                magnitude *= e.mem.f32(scale);
                actor_cause = true;
            }
        }

        magnitude = e
            .call(0x0062_b520, &args![magnitude, rigid_body, 1u32])
            .f32();
        if !actor_cause {
            let maximum = e.call(0x0040_3e20, &args![0x011d_0c9cu32]).u32();
            magnitude = e
                .call(0x0040_ebd0, &args![magnitude, e.mem.f32(maximum)])
                .f32();
        }

        let linear_scale = e.call(0x0040_3e20, &args![0x011c_ff64u32]).u32();
        let scaled_direction = e.call(0x0045_bb20, &args![position, impulse, magnitude]);
        let scaled_direction = e.call(
            0x0045_bb20,
            &args![
                scaled_direction.u32(),
                scaled_impulse,
                e.mem.f32(linear_scale)
            ],
        );
        fn_009b0bf0(e, rigid_body, Ptr::new(scaled_direction.u32()));

        let mut random = [0.0f32; 3];
        for component in &mut random {
            let lower = e.global::<f32>(0x0101_2054);
            *component = e.call(0x0047_6b70, &args![lower, 1.0f32]).f32();
        }
        e.call(0x0041_6870, &args![torque, random[0], random[1], random[2]]);
        let angular_scale = e.call(0x0040_3e20, &args![0x011d_02ecu32]).u32();
        let angular_magnitude =
            e.call(0x0045_bb20, &args![torque, angular_intermediate, magnitude]);
        let scaled_torque = e.call(
            0x0045_bb20,
            &args![
                angular_magnitude.u32(),
                scaled_torque,
                e.mem.f32(angular_scale)
            ],
        );
        fn_009b0c90(e, rigid_body, Ptr::new(scaled_torque.u32()));
    });
}

// Translated from 009b1510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a body-part sample for actor targets when fewer than two samples exist
/// or the distance is strictly less than their minimum. Unordered comparisons
/// do not replace the minimum or admit the sample.
pub fn fn_009b1510(e: &mut Engine, this: Ptr, target: Ptr, object: Ptr, distance: f32) {
    if target.is_null() {
        return;
    }
    let actor = e.mem.u32(target.addr());
    if actor == 0 || !e.vcall(actor, 0x100, &args![]).bool() {
        return;
    }
    let mut node = target.addr() + 8;
    let mut count = 0i32;
    let mut minimum = 0;
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        count = count.wrapping_add(1);
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(slot);
        node = e.call(0x0072_6070, &args![node]).u32();
        if minimum == 0 || e.mem.f32(record + 4) < e.mem.f32(minimum + 4) {
            minimum = record;
        }
    }
    if count < 2 || minimum == 0 || distance < e.mem.f32(minimum + 4) {
        let name = e.vcall(object.addr(), 0xc, &args![]).u32();
        let part = e.call(0x008b_3ef0, &args![actor, name]).i32();
        if part != -1 && part != 0xe {
            fn_009b1630(e, this, target, part, quiet_f32(distance));
        }
    }
}

// Translated from 009b1630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Selects the minimum-distance record while traversing, stopping when that
/// selected record matches the requested part. With fewer than two traversed
/// records it allocates, even when the first record already matched.
pub fn fn_009b1630(e: &mut Engine, _this: Ptr, target: Ptr, part: i32, distance: f32) {
    let mut node = target.addr().wrapping_add(8);
    let mut count = 0i32;
    let mut selected = 0;
    while node != 0
        && !e.call(0x0082_56d0, &args![node]).bool()
        && (selected == 0 || e.mem.i32(selected) != part)
    {
        count = count.wrapping_add(1);
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(slot);
        node = e.call(0x0072_6070, &args![node]).u32();
        if selected == 0
            || e.mem.f32(record + 4) < e.mem.f32(selected + 4)
            || e.mem.i32(selected) == part
        {
            selected = record;
        }
    }
    if selected == 0 || count < 2 {
        selected = e.call(0x0040_1000, &args![8u32]).u32();
        e.mem.set_i32(selected, part);
        e.mem.set_f32(selected + 4, quiet_f32(distance));
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), selected);
            e.call(0x005a_e3d0, &args![target.addr() + 8, slot]);
        });
    } else {
        e.mem.set_i32(selected, part);
        e.mem.set_f32(selected + 4, quiet_f32(distance));
    }
}

// Translated from 009b1720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a target and returns its embedded body-part list, or null.
pub fn fn_009b1720(e: &mut Engine, this: Ptr, reference: Ptr) -> Ptr {
    let mut node = this.addr().wrapping_add(0xa4);
    let mut found = 0;
    while found == 0 && node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(slot);
        if record != 0 && e.mem.u32(record) == reference.addr() {
            found = record + 8;
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    Ptr::new(found)
}

// Translated from 009b17a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sends a temporary update record to the 3D object only while the flag is set.
/// RET 4 preserves an unread caller argument; the pending record argument
/// belongs to 00a59c60, not the intervening virtual getter.
pub fn fn_009b17a0(e: &mut Engine, this: Ptr, _unused_1: u32) {
    if fn_009ad5f0(e, this) && e.vcall(this.addr(), 0x1d0, &args![]).u32() != 0 {
        e.with_stack(12, |e, record| {
            let time = quiet_f32(e.global::<f32>(0x011c_3c08));
            e.call(0x0043_d410, &args![record, time, 1u32, 0u32]);
            let object = e.vcall(this.addr(), 0x1d0, &args![]).u32();
            e.call(0x00a5_9c60, &args![object, record]);
        });
    }
}

// Translated from 009b2040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Serializes persistent explosion members, references, actor-cause state,
/// and its three form IDs through the supplied save buffer.
pub fn fn_009b2040(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(0x0093_2880, &args![this, buffer]);
    for (offset, size) in [
        (0x88, 4),
        (0x8c, 4),
        (0x94, 4),
        (0x98, 4),
        (0xd4, 1),
        (0xe4, 4),
        (0x90, 4),
        (0x9c, 4),
        (0xd8, 1),
        (0xd9, 1),
        (0xe8, 0xc),
        (0xf4, 0xc),
        (0x100, 4),
    ] {
        e.call(
            0x0086_5e50,
            &args![buffer, this.addr() + offset, size, 0u32],
        );
    }
    for offset in [0xc8, 0xcc, 0xe0] {
        let form = e.mem.u32(this.addr() + offset);
        e.call(0x0086_5df0, &args![buffer, form, 0u32]);
    }

    let actor_cause = e.call(0x0055_9450, &args![this.addr() + 0xd0]).u32();
    let actor_cause_id = if actor_cause == 0 {
        u32::MAX
    } else {
        e.call(0x0082_5c00, &args![actor_cause]).u32()
    };
    e.with_stack(4, |e, saved_id| {
        e.mem.set_u32(saved_id.addr(), actor_cause_id);
        e.call(0x0086_5e50, &args![buffer, saved_id, 4u32, 0u32]);
    });
    let has_actor_cause = u32::from(e.mem.u32(this.addr() + 0xdc) != 0);
    e.with_stack(1, |e, saved_flag| {
        e.mem.set_u8(saved_flag.addr(), has_actor_cause as u8);
        e.call(0x0086_5e50, &args![buffer, saved_flag, 1u32, 0u32]);
    });
    if has_actor_cause != 0 {
        let cause = e.mem.u32(this.addr() + 0xdc);
        let magic_item = e.mem.u32(cause + 0x18);
        let item_id = if magic_item == 0 {
            0
        } else {
            e.call(0x0040_a1e0, &args![magic_item]).u32()
        };
        e.call(0x0086_5db0, &args![buffer, item_id, 0u32]);

        let magic_target = e.mem.u32(cause + 0x1c);
        let target_id = if magic_target == 0 {
            0
        } else {
            e.call(0x0082_54c0, &args![magic_target]).u32()
        };
        e.call(0x0086_5db0, &args![buffer, target_id, 0u32]);
        e.call(0x0086_5df0, &args![buffer, e.mem.u32(cause + 0x20), 0u32]);
    }
}

// Translated from 009b22c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads version-gated explosion fields, resolves the weapon and actor-cause
/// forms, and restores the target-processing flags from the frame counter.
// open: compiler SEH cleanup for a partially initialized actor-cause allocation is not modeled.
// Translation incomplete: 009b22c0 - compiler SEH cleanup for partially initialized actor-cause allocation
pub fn fn_009b22c0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(0x0093_2a00, &args![this, buffer]);
    for (offset, size) in [
        (0x88, 4),
        (0x8c, 4),
        (0x94, 4),
        (0x98, 4),
        (0xd4, 1),
        (0xe4, 4),
        (0x90, 4),
    ] {
        e.call(0x0086_4980, &args![buffer, this.addr() + offset, size]);
    }
    let version = e.vcall(buffer.addr(), 0, &args![]).u8();
    if version > 13 {
        for (offset, size) in [(0x9c, 4), (0xd8, 1), (0xd9, 1), (0xe8, 0xc), (0xf4, 0xc)] {
            e.call(0x0086_4980, &args![buffer, this.addr() + offset, size]);
        }
    }
    if e.vcall(buffer.addr(), 0, &args![]).u8() > 19 {
        e.call(0x0086_4980, &args![buffer, this.addr() + 0x100, 4u32]);
    }

    for offset in [0xc8, 0xcc] {
        e.call(0x0086_48e0, &args![buffer, this.addr() + offset]);
    }
    let form = e
        .call(
            0x0086_48a0,
            &args![buffer, 0x0118_3028u32, 0x0118_3998u32, 0u32],
        )
        .u32();
    let form = e.call(0x0048_39c0, &args![form]).u32();
    let weapon = e.call(0x00ec_43fb, &args![form, 0u32]).u32();
    e.mem.set_u32(this.addr() + 0xe0, weapon);

    e.with_stack(4, |e, actor_cause_id| {
        e.mem.set_u32(actor_cause_id.addr(), u32::MAX);
        e.call(0x0086_4980, &args![buffer, actor_cause_id, 4u32]);
        let actor_cause = e
            .call(0x0066_e330, &args![e.mem.u32(actor_cause_id.addr())])
            .u32();
        e.call(0x0042_f780, &args![this.addr() + 0xd0, actor_cause]);
    });

    e.with_stack(1, |e, has_actor_cause| {
        e.mem.set_u8(has_actor_cause.addr(), 1);
        if e.vcall(buffer.addr(), 0, &args![]).u8() > 6 {
            e.call(0x0086_4980, &args![buffer, has_actor_cause, 1u32]);
        }
        if e.mem.u8(has_actor_cause.addr()) == 0 {
            return;
        }

        let mut actor_cause = e.mem.u32(this.addr() + 0xdc);
        if actor_cause == 0 {
            let allocation = e.call(0x0040_1000, &args![0x24u32]).u32();
            actor_cause = if allocation == 0 {
                0
            } else {
                e.call(0x0082_5ad0, &args![allocation, this]).u32()
            };
            e.mem.set_u32(this.addr() + 0xdc, actor_cause);
        }
        for offset in [0x18, 0x1c, 0x20] {
            e.call(
                0x0086_48e0,
                &args![buffer, actor_cause.wrapping_add(offset)],
            );
        }
    });

    if e.mem.i32(this.addr() + 0xe4) > 0 {
        for offset in [0xd5, 0xd6, 0xd7] {
            e.mem.set_u8(this.addr() + offset, 1);
        }
    }
}

// Translated from 009b25b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves saved owner, exploded-reference and actor-cause numeric IDs, then
/// invokes the object's post-load virtual when the base post-load predicate
/// is true.
pub fn fn_009b25b0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(0x0093_2b70, &args![this, buffer]);
    for offset in [0xc8, 0xcc] {
        let unresolved = e.mem.u32(this.addr() + offset);
        let resolved = if unresolved == 0 {
            0
        } else {
            let form = e
                .call(
                    0x0048_39c0,
                    &args![unresolved, 0x0118_3028u32, 0x0118_41ccu32, 0u32],
                )
                .u32();
            e.call(0x00ec_43fb, &args![form, 0u32]).u32()
        };
        e.mem.set_u32(this.addr() + offset, resolved);
    }

    let actor_cause = e.mem.u32(this.addr() + 0xdc);
    if actor_cause != 0 {
        let magic_item = e.mem.u32(actor_cause + 0x18);
        let magic_item = if magic_item == 0 {
            0
        } else {
            e.call(0x0040_a250, &args![magic_item]).u32()
        };
        e.mem.set_u32(actor_cause + 0x18, magic_item);

        let magic_target = e.mem.u32(actor_cause + 0x1c);
        let magic_target = if magic_target == 0 {
            0
        } else {
            e.call(0x0082_5550, &args![magic_target]).u32()
        };
        e.mem.set_u32(actor_cause + 0x1c, magic_target);

        let form = e.mem.u32(actor_cause + 0x20);
        let form = if form == 0 {
            0
        } else {
            let form = e
                .call(
                    0x0048_39c0,
                    &args![form, 0x0118_3028u32, 0x0118_41ccu32, 0u32],
                )
                .u32();
            e.call(0x00ec_43fb, &args![form, 0u32]).u32()
        };
        e.mem.set_u32(actor_cause + 0x20, form);
    }

    if fn_009ad5f0(e, this) {
        e.vcall(this.addr(), 0x304, &args![]);
    }
}

// Translated from 009b2770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the one-word load callback to MobileObject.
pub fn fn_009b2770(e: &mut Engine, this: Ptr, value: u32) {
    e.call(0x0093_2d60, &args![this, value]);
}

// Translated from 009b2790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets transient explosion flags, actor cause and vectors, then destroys
/// every non-null target and removes every list entry.
pub fn fn_009b2790(e: &mut Engine, this: Ptr, value: u32) {
    e.call(0x0093_2f60, &args![this, value]);
    for offset in 0xd4..=0xd7 {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    e.call(0x0042_f780, &args![this.addr() + 0xd0, 0u32]);
    e.mem.set_f32(this.addr() + 0x9c, 1.0);
    for offset in [0xd8, 0xd9] {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    for offset in [0xe8, 0xf4] {
        for component in [0, 4, 8] {
            let bits = e.global::<u32>(0x011f_426c + component);
            e.mem.set_u32(this.addr() + offset + component, bits);
        }
    }
    let list = this.addr() + 0xa4;
    while !e.call(0x0082_56d0, &args![list]).bool() {
        let slot = e.call(0x0068_15c0, &args![list]).u32();
        let target = e.mem.u32(slot);
        if target != 0 {
            fn_009ac910(e, Ptr::new(target), 1);
        }
        e.call(0x0063_f7b0, &args![list]);
    }
}

// Translated from 009b28a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears matching owner/exploded references and invalidates every matching
/// target record. It saves the next node before reading the current payload.
pub fn fn_009b28a0(e: &mut Engine, this: Ptr, reference: Ptr) {
    if fn_009acce0(e, this) == reference {
        e.call(0x0040_30d0, &args![this, 0u32]);
    }
    if fn_009ad610(e, this) == reference {
        e.call(0x0056_c7d0, &args![this, 0u32]);
    }
    let mut node = this.addr().wrapping_add(0xa4);
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let next = e.call(0x0072_6070, &args![node]).u32();
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let target = e.mem.u32(slot);
        if target != 0 && e.mem.u32(target) == reference.addr() {
            e.mem.set_u8(target + 6, 1);
            e.mem.set_u32(target, 0);
        }
        node = next;
    }
}

// Translated from 009b2c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns this plus the signed byte at +5, with 32-bit wrapping.
pub fn fn_009b2c90(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(
        this.addr()
            .wrapping_add(e.mem.u8(this.addr() + 5) as i8 as i32 as u32),
    )
}

// Translated from 009b2cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the array, allocates aligned contact storage and marks its
/// capacity with the high bit.
// open: compiler SEH cleanup after aligned-storage allocation failure is not modeled.
// Translation incomplete: 009b2cb0 - compiler SEH cleanup after aligned-storage allocation failure
pub fn fn_009b2cb0(e: &mut Engine, this: Ptr, capacity: u32) -> Ptr {
    e.call(0x0062_99a0, &args![this]);
    let storage = fn_009b2e60(e, capacity, 0);
    e.mem.set_u32(this.addr(), storage.addr());
    e.mem.set_u32(this.addr() + 8, capacity | 0x8000_0000);
    e.mem.set_u32(this.addr() + 12, storage.addr());
    this
}

// Translated from 009b2d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Conditionally clears changed storage, releases the original allocation,
/// then destroys the array base.
// open: compiler SEH cleanup during array destruction is not modeled.
// Translation incomplete: 009b2d30 - compiler SEH cleanup during array destruction
pub fn fn_009b2d30(e: &mut Engine, this: Ptr) {
    e.with_stack(1, |e, result| {
        let flag = fn_009b2dd0(e, this, result);
        if !e.call(0x0056_09f0, &args![flag]).bool() {
            e.call(0x0062_6870, &args![this]);
        }
    });
    let allocation = e.mem.u32(this.addr() + 12);
    e.call(0x005e_0b60, &args![allocation]);
    e.call(0x0062_1b60, &args![this]);
}

// Translated from 009b2db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards destruction of the embedded array base.
pub fn fn_009b2db0(e: &mut Engine, this: Ptr) {
    e.call(0x0057_c530, &args![this]);
}

// Translated from 009b2dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the caller's boolean result from current versus original storage.
pub fn fn_009b2dd0(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let changed = e.mem.u32(this.addr() + 12) != e.mem.u32(this.addr());
    e.call(0x0062_2570, &args![result, changed]);
    result
}

// Translated from 009b2e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sorts contact records with a zero comparator-state byte; the decompiler's
/// standard-library name is an incorrect identical-body match.
pub fn fn_009b2e00(e: &mut Engine, records: Ptr, count: i32) {
    fn_009b2e90(e, records, count, 0);
}

// Translated from 009b2e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an embedded-capacity array over storage beginning at +12.
pub fn fn_009b2e30(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    e.call(0x0062_e230, &args![this, this.addr() + 12, count, 0x80u32]);
    this
}

// Translated from 009b2e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates count times 0x70 bytes, rounded up to a 0x80-byte boundary.
/// Cdecl has two words; the identity helper leaves both pending for 005e0ad0.
pub fn fn_009b2e60(e: &mut Engine, count: u32, flags: u32) -> Ptr {
    let allocator = e.call(0x00c8_5750, &args![]).u32();
    let bytes = count.wrapping_mul(0x70).wrapping_add(0x7f) & 0xffff_ff80;
    let allocator = e.call(0x0068_15c0, &args![allocator]).u32();
    e.call(0x005e_0ad0, &args![allocator, bytes, flags]).ptr()
}

// Translated from 009b2e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the recursive sort only for signed counts greater than one.
pub fn fn_009b2e90(e: &mut Engine, records: Ptr, count: i32, state: u8) {
    if count > 1 {
        fn_009b2ed0(e, records, 0, count - 1, state);
    }
}

// Translated from 009b2ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hkpRootCdPoint,...>` (Xbox PDB).
/// Copies the middle record as pivot, partitions through the game's comparator,
/// recursively sorts the left range and iterates the right range.
pub fn fn_009b2ed0(e: &mut Engine, records: Ptr, mut first: i32, last: i32, state: u8) {
    e.with_stack(0x104, |e, temporary| {
        let pivot = temporary;
        let swap = Ptr::<()>::new(temporary.addr() + 0x80);
        let comparator = Ptr::<()>::new(temporary.addr() + 0x100);
        e.mem.set_u8(comparator.addr(), state);
        e.call(0x0062_1c40, &args![pivot]);
        loop {
            let mut low = first;
            let mut high = last;
            let middle = first.wrapping_add(last) >> 1;
            let address = |index: i32| {
                Ptr::<()>::new(
                    records
                        .addr()
                        .wrapping_add((index as u32).wrapping_mul(0x70)),
                )
            };
            e.call(0x0062_12c0, &args![pivot, address(middle)]);
            loop {
                while fn_009b3080(e, comparator, address(low), pivot) {
                    low = low.wrapping_add(1);
                }
                while fn_009b3080(e, comparator, pivot, address(high)) {
                    high = high.wrapping_sub(1);
                }
                if high >= low {
                    if high != low {
                        e.call(0x0062_1c40, &args![swap]);
                        e.call(0x0062_12c0, &args![swap, address(high)]);
                        e.call(0x0062_12c0, &args![address(high), address(low)]);
                        e.call(0x0062_12c0, &args![address(low), swap]);
                    }
                    high = high.wrapping_sub(1);
                    low = low.wrapping_add(1);
                }
                if low > high {
                    break;
                }
            }
            if first < high {
                fn_009b2ed0(e, records, first, high, state);
            }
            if low >= last {
                break;
            }
            first = low;
        }
    });
}

// Translated from 009b3080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Contact comparator wrapper: constructs a temporary boolean and returns AL.
pub fn fn_009b3080(e: &mut Engine, _this: Ptr, left: Ptr, right: Ptr) -> bool {
    e.with_stack(1, |e, flag| {
        let result = e.call(0x009b_30b0, &args![left, flag, right]).u32();
        e.call(0x0056_09f0, &args![result]).bool()
    })
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
        for flags in [0u32, 1, 2] {
            let mut e = empty_explosion_destructor_engine();
            fn_009ac1d0(&mut e, Ptr::new(0x1000), flags);
            let calls = e.call_log.unwrap();
            assert_eq!(
                calls.iter().any(|call| call.0 == 0x0040_1030),
                flags & 1 != 0
            );
        }
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

    fn map_generated_constant_pages(e: &mut Engine) {
        for address in [
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x011a_9000,
            0x011c_3000,
            0x011f_4000,
        ] {
            e.map(address, 0x1000);
        }
    }

    fn list_doubles(e: &mut Engine) {
        // Generated two-word list nodes: the helpers expose payload and next.
        e.register(0x0082_56d0, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0096_a2d0, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(0x005a_e3d0, |e, a| {
            let record = e.mem.u32(a[1]);
            e.mem.set_u32(0x1f00, a[0]);
            e.mem.set_u32(0x1f04, record);
            Ret::default()
        });
        e.register(0x0040_1000, |_, a| {
            assert!([8, 16].contains(&a[0]));
            0x3000u32.into_ret()
        });
    }

    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x1000, 0x6000);
        map_generated_constant_pages(&mut e);
        e.call_log = Some(vec![]);
        list_doubles(&mut e);
        e
    }

    fn empty_explosion_destructor_engine() -> Engine {
        let mut e = list_engine();
        for address in [0x0043_fcd0, 0x008d_6f30, 0x0055_9450] {
            e.register(address, |_, _| 0u32.into_ret());
        }
        for address in [
            0x0042_f760,
            0x0045_cec0,
            0x0048_3710,
            0x0046_ffb0,
            0x0066_b0d0,
            0x0042_f780,
            0x0092_ec40,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0042_26e0, |_, _| true.into_ret());
        e.register(0x0040_8d60, |_, _| 0x2600u32.into_ret());
        e.register(0x0040_1030, |_, _| Ret::default());
        e.register(0x00ad_8ce0, |_, _| false.into_ret());
        e.mem.set_u8(0x2600, 0);
        e
    }

    #[test]
    fn parameterized_constructor_preserves_word_abi_and_owner_allocation_branches() {
        for (owner, actor, allocated) in [
            (0u32, false, 0u32),
            (0, false, 0x6000),
            (0x4000, false, 0x6000),
            (0x4000, true, 0),
        ] {
            for definition in [0u32, 0x3000] {
                for orient in [false, true] {
                    let mut e = engine_with(&[
                        0x0092_eb50,
                        0x0063_3c90,
                        0x0096_a2d0,
                        0x0041_a250,
                        0x0042_f730,
                        0x0068_15c0,
                        0x0093_0700,
                        0x0057_5690,
                        0x0066_b0d0,
                        0x0042_f780,
                        0x0056_fa90,
                    ]);
                    e.map(0x1000, 0x6000);
                    map_generated_constant_pages(&mut e);
                    for offset in (0..0x104).step_by(4) {
                        e.mem.set_u32(0x1000 + offset, 0xa5a5_a5a5);
                    }
                    e.mem.set_u32(0x4000, 0x5000);
                    e.mem.set_u32(0x5100, 0x6000_0100);
                    e.register_double(0x6000_0100, move |_, _| actor.into_ret());
                    e.register_double(0x0040_1000, move |_, a| {
                        assert_eq!(a, &[0x24]);
                        allocated.into_ret()
                    });
                    e.register(0x0082_5ad0, |_, a| {
                        assert_eq!(a, &[0x6000, 0x1000]);
                        0x6004u32.into_ret()
                    });
                    e.register(0x0057_5830, |e, a| {
                        assert_eq!(a[0], 0x1000);
                        assert_eq!(
                            [e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8)],
                            [11, 12, 13]
                        );
                        Ret::default()
                    });
                    e.register(0x0041_81e0, |_, a| {
                        assert_eq!(a, &[0x1000]);
                        0x3000u32.into_ret()
                    });
                    e.register(0x0047_7900, |_, a| {
                        assert_eq!(a, &[0x3000]);
                        8.0f32.into_ret()
                    });
                    e.register_double(0x0047_7950, move |_, a| match a[1] {
                        1 => true.into_ret(),
                        2 => orient.into_ret(),
                        0x40 => 0x82u32.into_ret(),
                        _ => panic!("unexpected mask"),
                    });
                    e.mem.set_f32(0x308c, 9.0);
                    e.set_global(0x0101_7718, 3.0f32);
                    e.set_global(0x0101_2054, -1.0f32);
                    for i in 0..3 {
                        e.set_global(0x011f_426c + i * 4, 90 + i);
                    }
                    for i in 0..9 {
                        e.set_global(0x011a_9448 + i * 4, 100 + i);
                    }
                    let result = e.call(
                        0x009a_c200,
                        &[
                            0x1000, definition, owner, 0x7000, 11, 12, 13, 21, 22, 23, 24, 25, 26,
                            27, 28, 29,
                        ],
                    );
                    assert_eq!(result.u32(), 0x1000);
                    assert_eq!(e.mem.u32(0x10c8), owner);
                    assert_eq!(e.mem.u32(0x10cc), 0xa5a5_a5a5); // This overload does not write +0xCC after the base constructor.
                    assert_eq!(
                        e.mem.u32(0x10dc),
                        if actor || allocated == 0 { 0 } else { 0x6004 }
                    );
                    assert_eq!(e.mem.f32(0x108c), 3.0);
                    assert_eq!(e.mem.f32(0x1100), -1.0);
                    assert_eq!(e.mem.u8(0x10d8), if definition == 0 { 0 } else { 0x82 });
                    assert_eq!(
                        e.mem.u32(0x1094),
                        if definition == 0 {
                            0xa5a5_a5a5
                        } else {
                            8.0f32.to_bits()
                        }
                    );
                    assert_eq!(
                        e.mem.u32(0x1098),
                        if definition == 0 {
                            0xa5a5_a5a5
                        } else {
                            9.0f32.to_bits()
                        }
                    );
                    for base in [0x10e8, 0x10f4] {
                        for i in 0..3 {
                            assert_eq!(e.mem.u32(base + i * 4), 90 + i);
                        }
                    }
                    let calls = e.call_log.unwrap();
                    assert_eq!(calls.iter().any(|c| c.0 == 0x0040_1000), !actor);
                    assert_eq!(
                        calls.iter().any(|c| c.0 == 0x0082_5ad0),
                        !actor && allocated != 0
                    );
                    let rotations: Vec<_> = calls.iter().filter(|c| c.0 == 0x0056_fa90).collect();
                    assert_eq!(
                        rotations[0].1,
                        vec![0x1000, 21, 22, 23, 24, 25, 26, 27, 28, 29]
                    );
                    assert_eq!(
                        rotations.len(),
                        if definition != 0 && orient { 2 } else { 1 }
                    );
                    if rotations.len() == 2 {
                        assert_eq!(
                            rotations[1].1,
                            vec![0x1000, 100, 101, 102, 103, 104, 105, 106, 107, 108]
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn target_lookup_updates_zero_bytes_only_and_skips_null_payloads() {
        let mut e = list_engine();
        e.mem.set_u32(0x10a8, 0x1200);
        e.mem.set_u32(0x1200, 0x2000);
        e.mem.set_u32(0x2000, 0x9000);
        e.mem.set_u8(0x2004, 0);
        e.mem.set_u8(0x2005, 0x7f);
        assert_eq!(
            fn_009b0e00(&mut e, Ptr::new(0x1000), Ptr::new(0x9000), 0x80, 0xff).addr(),
            0x2000
        );
        assert_eq!(e.mem.u8(0x2004), 0x80);
        assert_eq!(e.mem.u8(0x2005), 0x7f);
        fn_009b0e00(&mut e, Ptr::new(0x1000), Ptr::new(0x9000), 0, 0);
        assert_eq!(e.mem.u8(0x2004), 0x80);
        assert!(!e.call_log.unwrap().iter().any(|c| c.0 == 0x0040_1000));
    }

    #[test]
    fn target_lookup_allocates_after_nonmatch_and_initializes_embedded_list() {
        let mut e = list_engine();
        e.mem.set_u32(0x10a4, 0x2000);
        e.mem.set_u32(0x2000, 0x8888);
        assert_eq!(
            fn_009b0e00(&mut e, Ptr::new(0x1000), Ptr::new(0x9999), 2, 3).addr(),
            0x3000
        );
        assert_eq!(e.mem.u32(0x3000), 0x9999);
        assert_eq!(e.mem.u8(0x3004), 2);
        assert_eq!(e.mem.u8(0x3005), 3);
        assert_eq!(e.mem.u8(0x3006), 0);
        assert_eq!(e.mem.u32(0x3008), 0);
        assert_eq!(e.mem.u32(0x1f00), 0x10a4);
        assert_eq!(e.mem.u32(0x1f04), 0x3000);
    }

    #[test]
    fn cached_damage_handles_cache_nan_owner_and_weapon_gates() {
        for cached in [12.0f32, -1.0, f32::NAN] {
            for owner in [0u32, 0x2000] {
                for actor in [false, true] {
                    for weapon in [0u32, 0x3000] {
                        let mut e = list_engine();
                        e.set_global(0x0101_2060, 0.0f64);
                        e.set_global(0x0101_6408, 0.01f64);
                        e.mem.set_f32(0x1100, cached);
                        e.mem.set_u32(0x10c8, owner);
                        e.mem.set_u32(0x10e0, weapon);
                        e.mem.set_u32(0x2000, 0x4000);
                        e.mem.set_u32(0x4100, 0x6000_0100);
                        e.mem.set_u32(0x20a4, 0x5000);
                        e.mem.set_u32(0x500c, 0x6000_000c);
                        e.register_double(0x6000_0100, move |_, _| actor.into_ret());
                        e.register(0x6000_000c, |_, a| {
                            assert_eq!(a, &[0x20a4, 42]);
                            25.0f32.into_ret()
                        });
                        e.register(0x0041_81e0, |_, _| 0x7000u32.into_ret());
                        e.register(0x0064_79e0, |_, a| {
                            assert_eq!(a, &[0x7000]);
                            10.0f32.into_ret()
                        });
                        e.register(0x008d_85e0, |_, a| {
                            assert_eq!(a, &[0x3000]);
                            42u32.into_ret()
                        });
                        e.register(0x0040_3e20, |_, a| {
                            if a[0] == 0x011c_ece8 {
                                0x6000u32
                            } else {
                                assert_eq!(a[0], 0x011c_f350);
                                0x6004u32
                            }
                            .into_ret()
                        });
                        e.mem.set_f32(0x6000, 1.0);
                        e.mem.set_f32(0x6004, 2.0);
                        let expected = if cached == 12.0 {
                            12.0
                        } else if owner != 0 && actor && weapon != 0 {
                            15.0
                        } else {
                            10.0
                        };
                        assert_eq!(fn_009b0f80(&mut e, Ptr::new(0x1000)), expected);
                        assert_eq!(e.mem.f32(0x1100), expected);
                        let count = e.call_log.as_ref().unwrap().len();
                        assert_eq!(fn_009b0f80(&mut e, Ptr::new(0x1000)), expected);
                        assert_eq!(e.call_log.unwrap().len(), count);
                    }
                }
            }
        }
    }

    #[test]
    fn cached_damage_reloads_owner_and_weapon_after_is_actor_callback() {
        let mut e = list_engine();
        e.set_global(0x0101_2060, 0.0f64);
        e.set_global(0x0101_6408, 0.01f64);
        e.mem.set_f32(0x1100, -1.0);
        e.mem.set_u32(0x10c8, 0x2000);
        e.mem.set_u32(0x10e0, 0x3000);
        e.mem.set_u32(0x2000, 0x4100);
        e.mem.set_u32(0x30a4, 0x5100);
        e.mem.set_u32(0x510c, 0x6000_000c);
        e.register_double(0x6000_0100, |e, _| {
            e.mem.set_u32(0x10c8, 0x3000);
            e.mem.set_u32(0x10e0, 0x4000);
            true.into_ret()
        });
        // The first owner's actor predicate is at its original vtable slot.
        e.mem.set_u32(0x4100 + 0x100, 0x6000_0100);
        e.register(0x0041_81e0, |_, _| 0x7000u32.into_ret());
        e.register(0x0064_79e0, |_, _| 10.0f32.into_ret());
        e.register(0x008d_85e0, |_, a| {
            assert_eq!(a, &[0x4000]);
            42u32.into_ret()
        });
        e.register(0x6000_000c, |_, a| {
            assert_eq!(a, &[0x30a4, 42]);
            25.0f32.into_ret()
        });
        e.register(0x0040_3e20, |_, a| {
            match a[0] {
                0x011c_ece8 => 0x6000u32,
                0x011c_f350 => 0x6004u32,
                _ => panic!("unexpected game setting"),
            }
            .into_ret()
        });
        e.mem.set_f32(0x6000, 1.0);
        e.mem.set_f32(0x6004, 2.0);

        assert_eq!(fn_009b0f80(&mut e, Ptr::new(0x1000)), 15.0);
        assert!(e
            .call_log
            .unwrap()
            .iter()
            .any(|call| { call.0 == 0x6000_000c && call.1 == [0x30a4, 42] }));
    }

    #[test]
    fn target_scan_marks_complete_and_creates_player_record_without_phantom() {
        let mut e = list_engine();
        e.map(0x011d_e000, 0x1000);
        e.set_global(0x011d_ea3c, 0x9000u32);
        e.register(0x0055_9450, |_, a| {
            assert_eq!(a, &[0x10a0]);
            0u32.into_ret()
        });

        fn_009afcc0(&mut e, Ptr::new(0x1000));

        assert_eq!(e.mem.u8(0x10d5), 1);
        assert_eq!(e.mem.u32(0x3000), 0x9000);
        assert_eq!(e.mem.u8(0x3004), 0);
        assert_eq!(e.mem.u8(0x3005), 1);
        assert_eq!(e.mem.u32(0x1f00), 0x10a4);
        assert_eq!(e.mem.u32(0x1f04), 0x3000);
    }

    #[test]
    fn rigid_body_scan_marks_complete_when_phantom_is_null() {
        let mut e = list_engine();
        e.register(0x0055_9450, |_, a| {
            assert_eq!(a, &[0x10a0]);
            0u32.into_ret()
        });

        fn_009afef0(&mut e, Ptr::new(0x1000));

        assert_eq!(e.mem.u8(0x10d7), 1);
        let calls = e.call_log.unwrap();
        assert_eq!(calls.len(), 1);
        assert!(calls.iter().all(|call| call.0 == 0x0055_9450));
    }

    #[test]
    fn rigid_body_scan_pushes_low_layer_body() {
        let mut e = list_engine();
        let push_state = std::rc::Rc::new(std::cell::Cell::new(false));
        let callback_state = std::rc::Rc::clone(&push_state);
        e.register(0x0055_9450, |_, a| {
            assert_eq!(a, &[0x10a0]);
            0x2000u32.into_ret()
        });
        e.register(0x0041_81e0, |_, a| {
            assert_eq!(a, &[0x1000, 0x20]);
            0x7000u32.into_ret()
        });
        e.register(0x0047_7950, |_, a| {
            assert_eq!(a, &[0x7000]);
            false.into_ret()
        });
        e.register(0x004a_e750, |_, a| {
            assert_eq!(a, &[0x2000]);
            0x2100u32.into_ret()
        });
        e.register(0x0062_d2f0, |_, a| {
            assert_eq!(a, &[0x2100]);
            0x3000u32.into_ret()
        });
        e.register(0x0072_6070, |_, a| {
            assert_eq!(a, &[0x3000]);
            1u32.into_ret()
        });
        e.register(0x0062_9ab0, |_, a| {
            assert_eq!(a, &[0x3000, 0]);
            0x3100u32.into_ret()
        });
        e.mem.set_u32(0x3100, 0x4000);
        e.register(0x004b_59f0, |_, a| {
            assert_eq!(a, &[0x4000]);
            0x5000u32.into_ret()
        });
        e.register(0x004b_5a80, |_, a| {
            assert_eq!(a, &[0x5000]);
            0x6000u32.into_ret()
        });
        e.register(0x0043_b4f0, |_, a| {
            assert_eq!(a[0], 0x6000);
            0x6100u32.into_ret()
        });
        e.register(0x0043_b4d0, |_, a| {
            assert_eq!(a, &[0x6100]);
            0u32.into_ret()
        });
        e.register(0x0051_7630, |_, a| {
            assert_eq!(a, &[0x6000]);
            1u32.into_ret()
        });
        e.register_double(0x009b_0920, move |_, a| {
            assert_eq!(a, &[0x1000, 0x6000]);
            callback_state.set(true);
            Ret::default()
        });

        fn_009afef0(&mut e, Ptr::new(0x1000));

        assert!(push_state.get());
        assert_eq!(e.mem.u8(0x10d7), 1);
    }

    #[test]
    fn rigid_body_impulse_rejects_magnitude_at_global_threshold() {
        let mut e = list_engine();
        e.set_global(0x0101_2060, 2.0f64);
        e.mem.set_f32(0x2074, 2.0);
        e.register(0x0041_81e0, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x2000u32.into_ret()
        });

        fn_009b0920(&mut e, Ptr::new(0x1000), Ptr::new(0x3000));

        let calls = e.call_log.unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, 0x0041_81e0);
    }

    #[test]
    fn rigid_body_impulse_scales_linear_and_random_angular_vectors() {
        let mut e = list_engine();
        e.set_global(0x0101_2060, 0.0f64);
        e.set_global(0x0101_2054, -1.0f32);
        e.mem.set_u32(0x1000, 0x4000);
        e.mem.set_u32(0x4000 + 0x1f4, 0x6000_01f4);
        e.mem.set_u32(0x3000, 0x4100);
        e.mem.set_u32(0x4100 + 0xf0, 0x6000_00f0);
        e.mem.set_u32(0x10cc, 0);
        e.mem.set_f32(0x2074, 8.0);
        e.mem.set_f32(0x5000, 16.0);
        e.mem.set_f32(0x5004, 0.5);
        e.mem.set_f32(0x5008, 0.25);
        e.mem.set_f32(0x5010, 0.75);
        e.register(0x0041_81e0, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x2000u32.into_ret()
        });
        e.register(0x6000_01f4, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x3400u32.into_ret()
        });
        e.register(0x6000_00f0, |e, a| {
            assert_eq!(a.len(), 2);
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[1] + 4, 2.0);
            e.mem.set_f32(a[1] + 8, 3.0);
            0x3300u32.into_ret()
        });
        e.register(0x0045_8620, |e, a| {
            assert_eq!(a[1], 0x3300);
            for offset in [0, 4, 8] {
                e.mem.set_f32(a[0] + offset, offset as f32 + 1.0);
            }
            Ret::default()
        });
        e.register(0x0043_9ef0, |e, a| {
            assert_eq!(a.len(), 3);
            assert_ne!(a[0], 0);
            assert_eq!(a[2], 0x3400);
            e.mem.set_f32(a[1], 4.0);
            e.mem.set_f32(a[1] + 4, 5.0);
            e.mem.set_f32(a[1] + 8, 6.0);
            a[1].into_ret()
        });
        e.register(0x004a_0c10, |_, a| {
            assert_eq!(a.len(), 1);
            Ret::default()
        });
        e.register(0x0043_b4f0, |_, a| {
            assert_eq!(a[0], 0x3000);
            0x3500u32.into_ret()
        });
        e.register(0x0062_4070, |_, a| {
            assert_eq!(a, &[0x3500]);
            false.into_ret()
        });
        e.register(0x0062_b520, |_, a| {
            assert_eq!(a, &[8.0f32.to_bits(), 0x3000, 1]);
            8.0f32.into_ret()
        });
        e.register(0x0040_3e20, |_, a| match a[0] {
            0x011d_0c9c => 0x5000u32.into_ret(),
            0x011c_ff64 => 0x5004u32.into_ret(),
            0x011d_02ec => 0x5008u32.into_ret(),
            other => panic!("unexpected setting {other:#x}"),
        });
        e.register(0x0040_ebd0, |_, a| {
            assert_eq!(a, &[8.0f32.to_bits(), 16.0f32.to_bits()]);
            8.0f32.into_ret()
        });
        let scaled = std::rc::Rc::new(std::cell::Cell::new(0));
        let scaled_calls = std::rc::Rc::clone(&scaled);
        e.register_double(0x0045_bb20, move |e, a| {
            assert_eq!(a.len(), 3);
            let scale = f32::from_bits(a[2]);
            for offset in [0, 4, 8] {
                let value = e.mem.f32(a[0] + offset) * scale;
                e.mem.set_f32(a[1] + offset, value);
            }
            scaled_calls.set(scaled_calls.get() + 1);
            a[1].into_ret()
        });
        e.register(0x0047_6b70, |_, a| {
            assert_eq!(a, &[(-1.0f32).to_bits(), 1.0f32.to_bits()]);
            0.5f32.into_ret()
        });
        e.register(0x0041_6870, |e, a| {
            assert_eq!(a.len(), 4);
            for (index, value) in a[1..].iter().copied().enumerate() {
                e.mem
                    .set_f32(a[0] + index as u32 * 4, f32::from_bits(value));
            }
            a[0].into_ret()
        });
        e.register(0x004a_3e00, |_, a| {
            assert_eq!(a.len(), 2);
            a[0].into_ret()
        });
        e.register(0x004a_e750, |_, a| {
            assert_eq!(a, &[0x3000]);
            0u32.into_ret()
        });

        fn_009b0920(&mut e, Ptr::new(0x1000), Ptr::new(0x3000));

        assert_eq!(scaled.get(), 4);
        let calls = e.call_log.unwrap();
        assert_eq!(calls.iter().filter(|call| call.0 == 0x0045_bb20).count(), 4);
        assert_eq!(calls.iter().filter(|call| call.0 == 0x0047_6b70).count(), 3);
    }

    #[test]
    fn explosion_save_serializes_base_fields_and_optional_caster_ids() {
        for has_caster in [false, true] {
            let mut e = list_engine();
            let this = Ptr::new(0x1000);
            let buffer = Ptr::new(0x2000);
            e.mem.set_u32(0x10c8, 0x3100);
            e.mem.set_u32(0x10cc, 0x3200);
            e.mem.set_u32(0x10e0, 0x3300);
            if has_caster {
                e.mem.set_u32(0x10dc, 0x3400);
                e.mem.set_u32(0x3418, 0x3500);
                e.mem.set_u32(0x341c, 0x3600);
                e.mem.set_u32(0x3420, 0x3700);
            }
            e.register(0x0093_2880, |_, a| {
                assert_eq!(a, &[0x1000, 0x2000]);
                Ret::default()
            });
            e.register(0x0086_5e50, |_e, a| {
                assert_eq!(a.len(), 4);
                assert_eq!(a[0], 0x2000);
                assert!(matches!(a[2], 1 | 4 | 12));
                assert_eq!(a[3], 0);
                Ret::default()
            });
            e.register(0x0086_5df0, |_, a| {
                assert_eq!(a.len(), 3);
                assert_eq!(a[0], 0x2000);
                assert_eq!(a[2], 0);
                Ret::default()
            });
            e.register(0x0086_5db0, |_, a| {
                assert_eq!(a.len(), 3);
                assert_eq!(a[0], 0x2000);
                assert_eq!(a[2], 0);
                Ret::default()
            });
            e.register(0x0055_9450, move |e, a| {
                assert_eq!(a, &[0x10d0]);
                e.mem.u32(0x10dc).into_ret()
            });
            e.register(0x0082_5c00, |_, a| {
                assert_eq!(a, &[0x3400]);
                0x44u32.into_ret()
            });
            e.register(0x0040_a1e0, |_, a| {
                assert_eq!(a, &[0x3500]);
                0x55u32.into_ret()
            });
            e.register(0x0082_54c0, |_, a| {
                assert_eq!(a, &[0x3600]);
                0x66u32.into_ret()
            });

            fn_009b2040(&mut e, this, buffer);

            let calls = e.call_log.unwrap();
            assert_eq!(
                calls.iter().filter(|call| call.0 == 0x0086_5e50).count(),
                15
            );
            assert_eq!(
                calls.iter().filter(|call| call.0 == 0x0086_5df0).count(),
                3 + if has_caster { 1 } else { 0 }
            );
            assert_eq!(
                calls.iter().filter(|call| call.0 == 0x0086_5db0).count(),
                2 * if has_caster { 1 } else { 0 }
            );
        }
    }

    #[test]
    fn explosion_load_obeys_version_gates_and_optional_caster_allocation() {
        for (version, has_caster) in [(13u8, false), (20, true)] {
            let mut e = list_engine();
            e.mem.set_u32(0x2000, 0x4000);
            e.mem.set_u32(0x4000, 0x6000_0000);
            e.mem.set_i32(0x10e4, 3);
            e.register(0x0093_2a00, |_, a| {
                assert_eq!(a, &[0x1000, 0x2000]);
                Ret::default()
            });
            e.register_double(0x6000_0000, move |_, a| {
                assert_eq!(a, &[0x2000]);
                version.into_ret()
            });
            e.register_double(0x0086_4980, move |e, a| {
                assert_eq!(a[0], 0x2000);
                let destination = a[1];
                if a[2] == 1 {
                    e.mem.set_u8(destination, u8::from(has_caster));
                } else if destination == 0x10e4 {
                    e.mem.set_i32(destination, 3);
                } else {
                    e.mem.set_u32(destination, 0x44);
                }
                Ret::default()
            });
            e.register(0x0086_48e0, |_, a| {
                assert_eq!(a[0], 0x2000);
                Ret::default()
            });
            e.register(0x0086_48a0, |_, a| {
                assert_eq!(a, &[0x2000, 0x0118_3028, 0x0118_3998, 0]);
                0x100u32.into_ret()
            });
            e.register(0x0048_39c0, |_, a| {
                assert_eq!(a, &[0x100]);
                0x200u32.into_ret()
            });
            e.register(0x00ec_43fb, |_, a| {
                assert_eq!(a, &[0x200, 0]);
                0x300u32.into_ret()
            });
            e.register(0x0066_e330, |_, a| {
                assert_eq!(a, &[0x44]);
                0u32.into_ret()
            });
            e.register(0x0042_f780, |e, a| {
                assert_eq!(a, &[0x10d0, 0]);
                e.mem.set_u32(a[0] + 0xc, a[1]);
                Ret::default()
            });
            e.register(0x0040_1000, |_, a| {
                assert_eq!(a, &[0x24]);
                0x5000u32.into_ret()
            });
            e.register(0x0082_5ad0, |_, a| {
                assert_eq!(a, &[0x5000, 0x1000]);
                0x6000u32.into_ret()
            });

            fn_009b22c0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));

            let calls = e.call_log.unwrap();
            let loaded = calls
                .iter()
                .filter(|call| call.0 == 0x0086_4980)
                .map(|call| call.1[1])
                .collect::<Vec<_>>();
            assert_eq!(loaded.contains(&0x109c), version > 13);
            assert_eq!(loaded.contains(&0x1100), version > 19);
            assert_eq!(e.mem.u32(0x10e0), 0x300);
            assert_eq!(e.mem.u32(0x10dc), if has_caster { 0x6000 } else { 0 });
            assert_eq!(e.mem.u8(0x10d5), 1);
            assert_eq!(e.mem.u8(0x10d6), 1);
            assert_eq!(e.mem.u8(0x10d7), 1);
            assert_eq!(
                calls.iter().filter(|call| call.0 == 0x0040_1000).count(),
                if has_caster { 1 } else { 0 }
            );
        }
    }

    #[test]
    fn explosion_init_load_resolves_reference_and_caster_members() {
        let mut e = list_engine();
        e.mem.set_u32(0x10c8, 0x111);
        e.mem.set_u32(0x10cc, 0x222);
        e.mem.set_u32(0x10dc, 0x3000);
        e.mem.set_u32(0x3018, 0x333);
        e.mem.set_u32(0x301c, 0x444);
        e.mem.set_u32(0x3020, 0x555);
        e.mem.set_u32(0x1000, 0x4000);
        e.mem.set_u32(0x4000 + 0x304, 0x6000_0304);
        e.register(0x0093_2b70, |_, a| {
            assert_eq!(a, &[0x1000, 0x2000]);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| {
            assert_eq!(a.len(), 4);
            assert_eq!(&a[1..], &[0x0118_3028, 0x0118_41cc, 0]);
            (a[0] + 0x100).into_ret()
        });
        e.register(0x00ec_43fb, |_, a| {
            assert_eq!(a[1], 0);
            (a[0] + 0x1000).into_ret()
        });
        e.register(0x0040_a250, |_, a| {
            assert_eq!(a, &[0x333]);
            0x433u32.into_ret()
        });
        e.register(0x0082_5550, |_, a| {
            assert_eq!(a, &[0x444]);
            0x544u32.into_ret()
        });
        e.register(0x008d_6f30, |_, a| {
            assert_eq!(a, &[0x1000]);
            0u32.into_ret()
        });
        e.register(0x6000_0304, |_, a| {
            assert_eq!(a, &[0x1000]);
            Ret::default()
        });

        fn_009b25b0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));

        assert_eq!(e.mem.u32(0x10c8), 0x1211);
        assert_eq!(e.mem.u32(0x10cc), 0x1322);
        assert_eq!(e.mem.u32(0x3018), 0x433);
        assert_eq!(e.mem.u32(0x301c), 0x544);
        assert_eq!(e.mem.u32(0x3020), 0x1655);
    }

    #[test]
    fn explosion_update_handles_first_target_age_and_expiration_gates() {
        let mut e = list_engine();
        e.mem.set_u32(0x10a4, 0x2100);
        e.mem.set_u32(0x2100, 0x2200);
        e.mem.set_u32(0x2200, 0x2300);
        e.mem.set_u32(0x10c8, 0x2000);
        e.mem.set_u32(0x1000, 0x4000);
        e.mem.set_u32(0x40c4, 0x6000_00c4);
        e.mem.set_f32(0x1090, 1.0);
        e.register(0x0082_56d0, |_, a| {
            assert_eq!(a, &[0x10a4]);
            false.into_ret()
        });
        e.register(0x0068_15c0, |_, a| {
            assert_eq!(a, &[0x10a4]);
            0x2100u32.into_ret()
        });
        e.register(0x008d_0600, |_, a| {
            assert_eq!(a, &[0x011e_0e80, 0x2000, 0x2300]);
            Ret::default()
        });
        let mut times = [2.0f32, 5.0, 6.0].into_iter();
        e.register_double(0x0051_b8e0, move |_, a| {
            assert_eq!(a, &[0x1000]);
            times.next().unwrap().into_ret()
        });
        e.register(0x0050_7e10, |_, a| {
            assert_eq!(a, &[0x1000]);
            4.0f32.into_ret()
        });
        e.register(0x009a_fcc0, |_, a| {
            assert_eq!(a, &[0x1000]);
            Ret::default()
        });
        e.register(0x008d_6f30, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x3000u32.into_ret()
        });
        e.register(0x0041_81e0, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x3100u32.into_ret()
        });
        e.register(0x004e_3cc0, |_, a| {
            assert_eq!(a, &[0x3100]);
            5.0f32.into_ret()
        });
        e.register(0x008d_01e0, |_, a| {
            assert_eq!(a, &[0x3100]);
            0.0f32.into_ret()
        });
        e.register(0x0055_9450, |_, _| 0u32.into_ret());
        e.register(0x6000_00c4, |_, a| {
            assert_eq!(a, &[0x1000, 1]);
            Ret::default()
        });

        fn_009ae5d0(&mut e, Ptr::new(0x1000), 0);

        assert_eq!(e.mem.u32(0x10e4), 1);
        let calls = e.call_log.unwrap();
        assert!(calls.iter().any(|call| call.0 == 0x009a_fcc0));
        assert!(calls.iter().any(|call| call.0 == 0x008d_0600));
        assert!(calls.iter().any(|call| call.0 == 0x6000_00c4));
    }

    #[test]
    fn explosion_destructor_runs_conditional_cleanup_and_member_destruction() {
        let mut e = list_engine();
        let this = Ptr::new(0x1000);
        e.mem.set_u32(0x10dc, 0x2500);
        e.mem.set_u32(0x2500, 0x4500);
        e.mem.set_u32(0x4500, 0x6000_0003);
        e.mem.set_u32(0x2000, 0x4000);
        e.mem.set_u32(0x400c, 0x6000_0001);
        e.mem.set_u32(0x2400, 0x4400);
        e.mem.set_u32(0x44a0, 0x6000_0002);
        e.mem.set_u32(0x011c_3f2c, 0x2b00);
        e.mem.set_u32(0x2800, this.addr());
        e.mem.set_u8(0x2600, 1);
        let list_visits = std::cell::Cell::new(0);

        e.register(0x0043_fcd0, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x2000u32.into_ret()
        });
        e.register(0x6000_0001, |_, a| {
            assert_eq!(a, &[0x2000]);
            0x5000u32.into_ret()
        });
        e.register(0x0057_84b0, |_, a| {
            assert_eq!(a, &[0x5000]);
            true.into_ret()
        });
        e.register(0x0057_8170, |_, a| {
            assert_eq!(a, &[0x5000]);
            Ret::default()
        });
        e.register(0x008d_6f30, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x2100u32.into_ret()
        });
        e.register(0x0045_43c0, |_, a| {
            assert_eq!(a, &[0x2100]);
            true.into_ret()
        });
        e.register(0x00c6_9ee0, |_, a| {
            assert_eq!(a, &[0x2000, 1, 0]);
            Ret::default()
        });
        e.register(0x0055_9450, |_, a| match a[0] {
            0x10a0 => 0x2400u32.into_ret(),
            0x10c4 => 0u32.into_ret(),
            0x011c_7b8c => 0x2700u32.into_ret(),
            _ => panic!("unexpected smart pointer or list"),
        });
        e.register(0x6000_0002, |_, a| {
            assert_eq!(a, &[0x2400]);
            Ret::default()
        });
        e.register(0x0066_b0d0, |_, a| {
            assert_eq!(a, &[0x10a0, 0]);
            Ret::default()
        });
        e.register_double(0x0082_56d0, move |_, a| {
            assert_eq!(a, &[0x10a4]);
            let empty = list_visits.get() != 0;
            list_visits.set(list_visits.get() + 1);
            empty.into_ret()
        });
        e.register(0x0068_15c0, |_, a| {
            assert_eq!(a, &[0x10a4]);
            0x2200u32.into_ret()
        });
        e.mem.set_u32(0x2200, 0x2300);
        e.register(0x0047_0470, |_, a| {
            assert_eq!(a, &[0x2308]);
            Ret::default()
        });
        e.register(0x0040_1030, |_, a| {
            assert_eq!(a, &[0x2300]);
            Ret::default()
        });
        e.register(0x0063_f7b0, |_, a| {
            assert_eq!(a, &[0x10a4]);
            Ret::default()
        });
        e.register(0x0042_26e0, |_, a| {
            assert_eq!(a, &[0x2b00]);
            true.into_ret()
        });
        e.register(0x00ad_8ce0, |_, a| {
            assert!(a == [0x10ac] || a == [0x10b8]);
            false.into_ret()
        });
        e.register(0x0042_f780, |_, a| {
            assert_eq!(a, &[0x10d0, 0]);
            Ret::default()
        });
        e.register(0x6000_0003, |_, a| {
            assert_eq!(a, &[0x2500, 1]);
            Ret::default()
        });
        e.register(0x0040_8d60, |_, a| {
            assert_eq!(a, &[0x011c_7ae8]);
            0x2600u32.into_ret()
        });
        e.register(0x007b_52d0, |_, a| {
            assert_eq!(a, &[0x011c_7b8c, 0x2700]);
            0u32.into_ret()
        });
        e.register(0x0063_17a0, |_, a| {
            assert_eq!(a, &[0x011c_7b8c, 0x2700]);
            0x2800u32.into_ret()
        });
        e.register(0x0049_f590, |e, a| {
            assert_eq!(a[0], 0x011c_7b8c);
            assert_eq!(e.mem.u32(a[1]), 0x2700);
            Ret::default()
        });
        e.register(0x009d_9f20, |_, a| {
            assert_eq!(a, &[0x2100]);
            0x2900u32.into_ret()
        });
        e.register(0x00c5_b370, |_, a| {
            assert_eq!(a, &[0x2900, 0x2000]);
            Ret::default()
        });
        for address in [0x0042_f760, 0x0045_cec0, 0x0048_3710, 0x0046_ffb0] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0092_ec40, |_, a| {
            assert_eq!(a, &[0x1000]);
            Ret::default()
        });

        fn_009ac5a0(&mut e, this);

        assert_eq!(e.mem.u32(0x1000), 0x0108_ee04);
        assert_eq!(e.mem.u32(0x1018), 0x0108_edf8);
        assert!(e.call_log.unwrap().iter().any(|call| call.0 == 0x0049_f590));
    }

    #[test]
    fn explosion_factory_skips_creation_for_null_definition() {
        let mut e = list_engine();
        e.map(0x011d_e000, 0x1000);
        e.register(0x0064_4930, |_, a| {
            assert_eq!(a, &[0]);
            8.0f32.into_ret()
        });
        e.register(0x0040_7880, |_, a| {
            assert_eq!(a, &[0, 8.0f32.to_bits()]);
            Ret::default()
        });

        let result = fn_009ac9c0(
            &mut e,
            Ptr::new(0),
            Ptr::new(0),
            Ptr::new(0),
            Ptr::new(0),
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
        );
        assert!(result.is_null());
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (0x0064_4930, vec![0]),
                (0x0040_7880, vec![0, 8.0f32.to_bits()]),
            ]
        );
    }

    #[test]
    fn explosion_factory_skips_perk_and_construction_after_null_allocation() {
        let mut e = list_engine();
        e.map(0x011d_e000, 0x1000);
        e.set_global(0x011d_ea3c, 0x3000u32);
        e.mem.set_u32(0x2000, 0x4000);
        e.mem.set_u32(0x4224, 0x6000_0224);
        e.register(0x6000_0224, |_, a| {
            assert_eq!(a, &[0x2000]);
            false.into_ret()
        });
        e.register(0x0040_1000, |_, a| {
            assert_eq!(a, &[0x104]);
            0u32.into_ret()
        });
        e.register(0x0064_4930, |_, a| {
            assert_eq!(a, &[0]);
            8.0f32.into_ret()
        });
        e.register(0x0040_7880, |_, a| {
            assert_eq!(a, &[0, 8.0f32.to_bits()]);
            Ret::default()
        });

        let result = fn_009ac9c0(
            &mut e,
            Ptr::new(0x5000),
            Ptr::new(0x2000),
            Ptr::new(0),
            Ptr::new(0x3000),
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
        );
        assert!(result.is_null());
        assert!(!e
            .call_log
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0x005e_58f0 || *address == 0x009a_c200));
    }

    #[test]
    fn body_part_insertion_allocates_for_empty_or_single_matching_sample() {
        for existing in [false, true] {
            let mut e = list_engine();
            if existing {
                e.mem.set_u32(0x2008, 0x2100);
                e.mem.set_i32(0x2100, 7);
                e.mem.set_f32(0x2104, 2.0);
            }
            fn_009b1630(
                &mut e,
                Ptr::new(0x1000),
                Ptr::new(0x2000),
                7,
                f32::from_bits(0x7f80_0001),
            );
            assert_eq!(e.mem.i32(0x3000), 7);
            assert_eq!(e.mem.u32(0x3004), 0x7fc0_0001);
            assert_eq!(e.mem.u32(0x1f00), 0x2008);
            if existing {
                assert_eq!(e.mem.f32(0x2104), 2.0);
            }
        }
    }

    #[test]
    fn body_part_replacement_uses_minimum_and_stops_on_selected_match() {
        for second_distance in [1.0f32, 8.0, f32::NAN] {
            let mut e = list_engine();
            e.mem.set_u32(0x2008, 0x2100);
            e.mem.set_u32(0x200c, 0x2200);
            e.mem.set_u32(0x2200, 0x2300);
            e.mem.set_i32(0x2100, 1);
            e.mem.set_f32(0x2104, 4.0);
            e.mem.set_i32(0x2300, 2);
            e.mem.set_f32(0x2304, second_distance);
            fn_009b1630(&mut e, Ptr::new(0x1000), Ptr::new(0x2000), 9, 0.5);
            let chosen = if second_distance == 1.0 {
                0x2300
            } else {
                0x2100
            };
            assert_eq!(e.mem.i32(chosen), 9);
            assert_eq!(e.mem.f32(chosen + 4), 0.5);
            assert_eq!(e.mem.u32(0x1f04), 0);
        }
    }

    #[test]
    fn body_part_sampling_checks_actor_minimum_and_excluded_parts() {
        for sample_count in [0, 1, 2] {
            for distance in [1.0f32, 3.0, f32::NAN] {
                for part in [-1, 14, 7] {
                    let mut e = list_engine();
                    e.mem.set_u32(0x2000, 0x4000);
                    e.mem.set_u32(0x4000, 0x5000);
                    e.mem.set_u32(0x5100, 0x6000_0100);
                    e.register(0x6000_0100, |_, _| true.into_ret());
                    e.mem.set_u32(0x6000, 0x5000);
                    e.mem.set_u32(0x500c, 0x6000_000c);
                    e.register(0x6000_000c, |_, a| {
                        assert_eq!(a, &[0x6000]);
                        42u32.into_ret()
                    });
                    e.register_double(0x008b_3ef0, move |_, a| {
                        assert_eq!(a, &[0x4000, 42]);
                        part.into_ret()
                    });
                    if sample_count > 0 {
                        e.mem.set_u32(0x2008, 0x2100);
                        e.mem.set_i32(0x2100, 1);
                        e.mem.set_f32(0x2104, 2.0);
                    }
                    if sample_count > 1 {
                        e.mem.set_u32(0x200c, 0x2200);
                        e.mem.set_u32(0x2200, 0x2300);
                        e.mem.set_i32(0x2300, 2);
                        e.mem.set_f32(0x2304, 4.0);
                    }
                    fn_009b1510(
                        &mut e,
                        Ptr::new(0x1000),
                        Ptr::new(0x2000),
                        Ptr::new(0x6000),
                        distance,
                    );
                    let allowed = sample_count < 2 || distance < 2.0;
                    assert_eq!(
                        e.call_log
                            .as_ref()
                            .unwrap()
                            .iter()
                            .any(|c| c.0 == 0x008b_3ef0),
                        allowed
                    );
                    if allowed && part == 7 {
                        assert_eq!(e.mem.i32(if sample_count < 2 { 0x3000 } else { 0x2100 }), 7);
                    } else {
                        assert_eq!(e.mem.u32(0x1f04), 0);
                        assert_eq!(e.mem.i32(0x2100), if sample_count > 0 { 1 } else { 0 });
                    }
                }
            }
        }
        let mut e = list_engine();
        fn_009b1510(&mut e, Ptr::new(0), Ptr::new(0), Ptr::new(0), 0.0);
        fn_009b1510(&mut e, Ptr::new(0), Ptr::new(0x2000), Ptr::new(0), 0.0);
        e.mem.set_u32(0x2000, 0x4000);
        e.mem.set_u32(0x4000, 0x5000);
        e.mem.set_u32(0x5100, 0x6000_0100);
        e.register(0x6000_0100, |_, _| false.into_ret());
        fn_009b1510(&mut e, Ptr::new(0), Ptr::new(0x2000), Ptr::new(0), 0.0);
    }

    #[test]
    fn target_body_list_lookup_finds_match_after_null_and_returns_null_for_missing() {
        let mut e = list_engine();
        e.mem.set_u32(0x10a8, 0x1200);
        e.mem.set_u32(0x1200, 0x2000);
        e.mem.set_u32(0x2000, 0x9000);
        assert_eq!(
            fn_009b1720(&mut e, Ptr::new(0x1000), Ptr::new(0x9000)).addr(),
            0x2008
        );
        assert!(fn_009b1720(&mut e, Ptr::new(0x1000), Ptr::new(0x9001)).is_null());
        assert!(fn_009b1720(&mut e, Ptr::new(0x1300), Ptr::new(0x9000)).is_null());
    }

    #[test]
    fn update_record_obeys_both_gates_and_passes_pending_argument_to_update() {
        for cell in [0u32, 0x7000] {
            for object in [0u32, 0x2000] {
                let mut e = list_engine();
                e.mem.set_u32(0x1000, 0x4000);
                e.mem.set_u32(0x41d0, 0x6000_01d0);
                e.register_double(0x008d_6f30, move |_, _| cell.into_ret());
                e.register_double(0x6000_01d0, move |_, a| {
                    assert_eq!(a, &[0x1000]);
                    object.into_ret()
                });
                e.set_global(0x011c_3c08, 1.25f32);
                e.register(0x0043_d410, |e, a| {
                    assert_eq!(&a[1..], &[1.25f32.to_bits(), 1, 0]);
                    e.mem.set_u32(a[0], 123);
                    Ret::default()
                });
                e.register(0x00a5_9c60, |e, a| {
                    assert_eq!(a[0], 0x2000);
                    assert_eq!(e.mem.u32(a[1]), 123);
                    Ret::default()
                });
                e.call(0x009b_17a0, &args![0x1000u32, 0xfeedu32]);
                assert_eq!(
                    e.call_log.unwrap().iter().any(|c| c.0 == 0x00a5_9c60),
                    cell == 0 && object != 0
                );
            }
        }
    }

    #[test]
    fn load_forwarder_preserves_this_and_value() {
        let mut e = engine_with(&[0x0093_2d60]);
        fn_009b2770(&mut e, Ptr::new(4), 7);
        assert_eq!(e.call_log.unwrap()[0], (0x0093_2d60, vec![4, 7]));
    }

    #[test]
    fn transient_reset_removes_null_and_nonnull_entries_and_resets_vectors() {
        let mut e = list_engine();
        e.register(0x0093_2f60, |_, a| {
            assert_eq!(a, &[0x1000, 7]);
            Ret::default()
        });
        e.register(0x0042_f780, |_, a| {
            assert_eq!(a, &[0x10d0, 0]);
            Ret::default()
        });
        e.register(0x009a_c940, |_, a| {
            assert_eq!(a, &[0x2000]);
            Ret::default()
        });
        e.register(0x0040_1030, |_, a| {
            assert_eq!(a, &[0x2000]);
            Ret::default()
        });
        let mut calls = 0;
        e.register_double(0x0082_56d0, move |_, a| {
            assert_eq!(a, &[0x10a4]);
            calls += 1;
            (calls > 2).into_ret()
        });
        let mut payloads = 0;
        e.register_double(0x0068_15c0, move |e, _| {
            payloads += 1;
            e.mem
                .set_u32(0x1200, if payloads == 1 { 0 } else { 0x2000 });
            0x1200u32.into_ret()
        });
        e.register(0x0063_f7b0, |_, a| {
            assert_eq!(a, &[0x10a4]);
            Ret::default()
        });
        for offset in 0xd4..=0xd9 {
            e.mem.set_u8(0x1000 + offset, 0xff);
        }
        for i in 0..3 {
            e.set_global(0x011f_426c + i * 4, 90 + i);
        }
        fn_009b2790(&mut e, Ptr::new(0x1000), 7);
        for offset in 0xd4..=0xd9 {
            assert_eq!(e.mem.u8(0x1000 + offset), 0);
        }
        assert_eq!(e.mem.f32(0x109c), 1.0);
        for base in [0x10e8, 0x10f4] {
            for i in 0..3 {
                assert_eq!(e.mem.u32(base + i * 4), 90 + i);
            }
        }
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .filter(|c| c.0 == 0x0063_f7b0)
                .count(),
            2
        );
    }

    #[test]
    fn reference_invalidation_clears_only_matching_fields_and_all_matching_targets() {
        for owner in [0x9000u32, 0x9001] {
            for exploded in [0x9000u32, 0x9001] {
                let mut e = list_engine();
                e.mem.set_u32(0x10c8, owner);
                e.mem.set_u32(0x10cc, exploded);
                e.register(0x0040_30d0, |_, a| {
                    assert_eq!(a, &[0x1000, 0]);
                    Ret::default()
                });
                e.register(0x0056_c7d0, |_, a| {
                    assert_eq!(a, &[0x1000, 0]);
                    Ret::default()
                });
                for (node, next, record, reference) in [
                    (0x10a4, 0x1200, 0x2000, 0x9000),
                    (0x1200, 0x1300, 0x2100, 0x9001),
                    (0x1300, 0, 0x2200, 0x9000),
                ] {
                    e.mem.set_u32(node, record);
                    e.mem.set_u32(node + 4, next);
                    e.mem.set_u32(record, reference);
                }
                fn_009b28a0(&mut e, Ptr::new(0x1000), Ptr::new(0x9000));
                for record in [0x2000, 0x2200] {
                    assert_eq!(e.mem.u32(record), 0);
                    assert_eq!(e.mem.u8(record + 6), 1);
                }
                assert_eq!(e.mem.u32(0x2100), 0x9001);
                assert_eq!(e.mem.u8(0x2106), 0);
                let calls = e.call_log.unwrap();
                assert_eq!(calls.iter().any(|c| c.0 == 0x0040_30d0), owner == 0x9000);
                assert_eq!(calls.iter().any(|c| c.0 == 0x0056_c7d0), exploded == 0x9000);
            }
        }
    }

    #[test]
    fn signed_byte_pointer_adjustment_covers_both_signs() {
        let mut e = list_engine();
        for value in [0u8, 1, 127, 128, 255] {
            e.mem.set_u8(0x1005, value);
            assert_eq!(
                fn_009b2c90(&mut e, Ptr::new(0x1000)).addr(),
                0x1000u32.wrapping_add(value as i8 as i32 as u32)
            );
        }
    }

    fn allocator_doubles(e: &mut Engine) {
        e.register(0x00c8_5750, |_, a| {
            assert!(a.is_empty());
            0x5000u32.into_ret()
        });
        e.register(0x0068_15c0, |_, a| {
            assert_eq!(a, &[0x5000]);
            0x5004u32.into_ret()
        });
        e.register(0x005e_0ad0, |_, a| {
            assert_eq!(a[0], 0x5004);
            0x6000u32.into_ret()
        });
    }

    #[test]
    fn contact_allocator_rounds_with_wrapping_and_preserves_flags() {
        let mut e = list_engine();
        allocator_doubles(&mut e);
        for count in [0u32, 1, 2, 8, u32::MAX] {
            assert_eq!(fn_009b2e60(&mut e, count, 0x80).addr(), 0x6000);
            assert_eq!(
                e.call_log.as_ref().unwrap().last().unwrap().1,
                vec![
                    0x5004,
                    count.wrapping_mul(0x70).wrapping_add(0x7f) & 0xffff_ff80,
                    0x80
                ]
            );
        }
    }

    #[test]
    fn contact_array_constructor_marks_capacity_and_preserves_base_count() {
        let mut e = list_engine();
        allocator_doubles(&mut e);
        e.register(0x0062_99a0, |e, a| {
            e.mem.set_u32(a[0] + 4, 17);
            Ret::default()
        });
        assert_eq!(fn_009b2cb0(&mut e, Ptr::new(0x1000), 3).addr(), 0x1000);
        assert_eq!(e.mem.u32(0x1000), 0x6000);
        assert_eq!(e.mem.u32(0x1004), 17);
        assert_eq!(e.mem.u32(0x1008), 0x8000_0003);
        assert_eq!(e.mem.u32(0x100c), 0x6000);
    }

    fn boolean_doubles(e: &mut Engine) {
        e.register(0x0062_2570, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            a[0].into_ret()
        });
        e.register(0x0056_09f0, |e, a| e.mem.u8(a[0]).into_ret());
    }

    #[test]
    fn storage_comparison_constructs_result_for_equal_and_changed_storage() {
        let mut e = list_engine();
        boolean_doubles(&mut e);
        for original in [0x2000u32, 0x3000] {
            e.mem.set_u32(0x1000, 0x2000);
            e.mem.set_u32(0x100c, original);
            assert_eq!(
                fn_009b2dd0(&mut e, Ptr::new(0x1000), Ptr::new(0x1200)).addr(),
                0x1200
            );
            assert_eq!(e.mem.u8(0x1200), u8::from(original != 0x2000));
        }
    }

    #[test]
    fn array_destructor_clears_equal_storage_and_always_releases_original_then_base() {
        for original in [0x2000u32, 0x3000] {
            let mut e = list_engine();
            boolean_doubles(&mut e);
            for address in [0x0062_6870, 0x005e_0b60, 0x0062_1b60] {
                e.register(address, |_, _| Ret::default());
            }
            e.mem.set_u32(0x1000, 0x2000);
            e.mem.set_u32(0x100c, original);
            fn_009b2d30(&mut e, Ptr::new(0x1000));
            let calls = e.call_log.unwrap();
            assert_eq!(calls.iter().any(|c| c.0 == 0x0062_6870), original == 0x2000);
            assert_eq!(
                &calls[calls.len() - 2..],
                &[(0x005e_0b60, vec![original]), (0x0062_1b60, vec![0x1000])]
            );
        }
    }

    #[test]
    fn array_base_destructor_forwarder_preserves_this() {
        let mut e = engine_with(&[0x0057_c530]);
        fn_009b2db0(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.unwrap()[0].1, vec![0x1000]);
    }

    #[test]
    fn embedded_array_initializer_passes_storage_count_and_capacity() {
        let mut e = engine_with(&[0x0062_e230]);
        assert_eq!(fn_009b2e30(&mut e, Ptr::new(0x1000), 19).addr(), 0x1000);
        assert_eq!(e.call_log.unwrap()[0].1, vec![0x1000, 0x100c, 19, 0x80]);
    }

    fn sorting_doubles(e: &mut Engine) {
        e.register(0x0062_1c40, |_, _| Ret::default());
        e.register(0x0062_12c0, |e, a| {
            for offset in (0..0x70).step_by(4) {
                let bits = e.mem.u32(a[1] + offset);
                e.mem.set_u32(a[0] + offset, bits);
            }
            a[0].into_ret()
        });
        e.register(0x009b_30b0, |e, a| {
            let less = e.mem.f32(a[0]) < e.mem.f32(a[2]);
            e.mem.set_u8(a[1], u8::from(less));
            a[1].into_ret()
        });
        e.register(0x0056_09f0, |e, a| e.mem.u8(a[0]).into_ret());
    }

    #[test]
    fn contact_comparator_wrapper_passes_left_result_right_and_uses_low_byte() {
        let mut e = list_engine();
        e.register(0x009b_30b0, |_, a| {
            assert_eq!(a[0], 0x2000);
            assert_eq!(a[2], 0x3000);
            a[1].into_ret()
        });
        for raw in [0x100u32, 0x101] {
            e.register_double(0x0056_09f0, move |_, _| raw.into_ret());
            assert_eq!(
                fn_009b3080(&mut e, Ptr::new(0), Ptr::new(0x2000), Ptr::new(0x3000)),
                raw == 0x101
            );
        }
    }

    #[test]
    fn contact_sort_preserves_full_records_for_reversed_duplicate_and_nan_inputs() {
        for input in [
            vec![5.0f32, 4.0, 3.0, 2.0, 1.0],
            vec![2.0, 1.0, 2.0, 1.0, 3.0],
            vec![f32::NAN, 3.0, 1.0],
            vec![1.0],
        ] {
            let mut e = list_engine();
            sorting_doubles(&mut e);
            for (index, value) in input.iter().enumerate() {
                let record = 0x2000 + index as u32 * 0x70;
                e.mem.set_f32(record, *value);
                for offset in (4..0x70).step_by(4) {
                    e.mem.set_u32(record + offset, index as u32);
                }
            }
            fn_009b2ed0(&mut e, Ptr::new(0x2000), 0, input.len() as i32 - 1, 0x8a);
            let mut identities = vec![];
            let mut output = vec![];
            for index in 0..input.len() {
                let record = 0x2000 + index as u32 * 0x70;
                let identity = e.mem.u32(record + 4);
                identities.push(identity);
                output.push(e.mem.f32(record));
                assert_eq!(e.mem.u32(record), input[identity as usize].to_bits());
                for offset in (4..0x70).step_by(4) {
                    assert_eq!(e.mem.u32(record + offset), identity);
                }
            }
            identities.sort();
            assert_eq!(identities, (0..input.len() as u32).collect::<Vec<_>>());
            if input.iter().all(|v| !v.is_nan()) {
                assert!(output.windows(2).all(|w| w[0] <= w[1]));
            }
        }
    }

    #[test]
    fn sort_entry_points_skip_signed_small_counts_and_sort_nonempty_ranges() {
        for count in [-1, 0, 1] {
            let mut e = Engine::new();
            e.call_log = Some(vec![]);
            fn_009b2e90(&mut e, Ptr::new(0), count, 9);
            fn_009b2e00(&mut e, Ptr::new(0), count);
            assert!(e.call_log.unwrap().is_empty());
        }
        for explicit_state in [false, true] {
            let mut e = list_engine();
            sorting_doubles(&mut e);
            e.mem.set_f32(0x2000, 3.0);
            e.mem.set_f32(0x2070, 1.0);
            if explicit_state {
                fn_009b2e90(&mut e, Ptr::new(0x2000), 2, 8);
            } else {
                fn_009b2e00(&mut e, Ptr::new(0x2000), 2);
            }
            assert_eq!(e.mem.f32(0x2000), 1.0);
            assert_eq!(e.mem.f32(0x2070), 3.0);
        }
    }

    #[test]
    fn contact_sort_matches_native_generated_record_permutations() {
        // nv-call, FalloutNV.exe 1.4.0.525, SHA256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f.
        // AMD Ryzen AI 9 HX 370; FCW 027F, MXCSR 1F80. Five fault-free
        // generated vectors; no executable bytes or raw recordings embedded.
        let cases: &[(&[f32], &[u32])] = &[
            (&[5.0, 4.0, 3.0, 2.0, 1.0], &[5, 4, 3, 2, 1]),
            (&[2.0, 1.0, 2.0, 1.0, 3.0], &[2, 4, 3, 1, 5]),
            (&[f32::NAN, 3.0, 1.0], &[3, 2, 1]),
            (
                &[0.0, -0.0, f32::INFINITY, f32::NEG_INFINITY],
                &[4, 2, 1, 3],
            ),
            (&[1.0], &[1]),
        ];
        for &(values, expected) in cases {
            let mut e = list_engine();
            sorting_doubles(&mut e);
            // 009b30b0 compares +0x1c via 009b3100/00560d30.
            e.register(0x009b_30b0, |e, a| {
                let less = e.mem.f32(a[0] + 0x1c) < e.mem.f32(a[2] + 0x1c);
                e.mem.set_u8(a[1], u8::from(less));
                a[1].into_ret()
            });
            for (index, value) in values.iter().enumerate() {
                let record = 0x2000 + index as u32 * 0x70;
                e.mem.set_f32(record + 0x1c, *value);
                e.mem.set_u32(record + 0x20, index as u32 + 1);
            }
            fn_009b2ed0(&mut e, Ptr::new(0x2000), 0, values.len() as i32 - 1, 0x8a);
            for (index, &identity) in expected.iter().enumerate() {
                let record = 0x2000 + index as u32 * 0x70;
                assert_eq!(e.mem.u32(record + 0x20), identity);
                assert_eq!(
                    e.mem.u32(record + 0x1c),
                    values[identity as usize - 1].to_bits()
                );
                for offset in (0..0x70).step_by(4) {
                    if offset != 0x1c && offset != 0x20 {
                        assert_eq!(e.mem.u32(record + offset), 0);
                    }
                }
            }
        }
    }

    #[test]
    fn target_allocation_failure_keeps_native_null_dereference() {
        let mut e = list_engine();
        e.register(0x0040_1000, |_, _| 0u32.into_ret());
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            fn_009b0e00(&mut e, Ptr::new(0x1000), Ptr::new(0x9000), 1, 1)
        }));
        assert!(result.is_err());
        assert!(!e.call_log.unwrap().iter().any(|c| c.0 == 0x005a_e3d0));
    }

    #[test]
    fn selected_second_body_part_match_stops_before_third_payload() {
        let mut e = list_engine();
        for (node, next, record, part, distance) in [
            (0x2008, 0x2200, 0x2100, 1, 4.0),
            (0x2200, 0x2400, 0x2300, 7, 2.0),
            (0x2400, 0, 0x2500, 8, 1.0),
        ] {
            e.mem.set_u32(node, record);
            e.mem.set_u32(node + 4, next);
            e.mem.set_i32(record, part);
            e.mem.set_f32(record + 4, distance);
        }
        fn_009b1630(&mut e, Ptr::new(0x1000), Ptr::new(0x2000), 7, 0.5);
        assert_eq!(e.mem.f32(0x2304), 0.5);
        assert_eq!(e.mem.f32(0x2504), 1.0);
        assert!(!e
            .call_log
            .unwrap()
            .iter()
            .any(|c| c.0 == 0x0068_15c0 && c.1 == [0x2400]));
    }

    #[test]
    fn parameterized_constructor_quiets_float_stores_but_preserves_copied_nan_words() {
        let mut e = engine_with(&[
            0x0092_eb50,
            0x0063_3c90,
            0x0096_a2d0,
            0x0041_a250,
            0x0042_f730,
            0x0068_15c0,
            0x0057_5830,
            0x0057_5690,
            0x0066_b0d0,
            0x0042_f780,
            0x0056_fa90,
        ]);
        e.map(0x1000, 0x5000);
        map_generated_constant_pages(&mut e);
        e.register(0x0093_0700, |_, a| {
            assert_eq!(a, &[0x1000, 0x7000]);
            Ret::default()
        });
        e.register(0x0041_81e0, |_, _| 0x3000u32.into_ret());
        e.register(0x0047_7900, |_, _| f32::from_bits(0x7f80_0001).into_ret());
        e.register(0x0047_7950, |_, a| {
            match a[1] {
                1 => 1u32,
                2 => 0x100,
                0x40 => 0x182,
                _ => panic!("mask"),
            }
            .into_ret()
        });
        e.register(0x0040_1000, |_, _| 0u32.into_ret());
        e.set_global(0x0101_7718, 0x7f80_0001u32);
        e.set_global(0x0101_2054, 0xff80_0001u32);
        e.mem.set_u32(0x308c, 0x7f80_0002);
        for i in 0..3 {
            e.set_global(0x011f_426c + i * 4, 0x7f80_0010u32 + i);
        }
        e.call(
            0x009a_c200,
            &[
                0x1000,
                0x3000,
                0,
                0x7000,
                0x7f80_0001,
                0,
                0,
                0x7f80_0002,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        );
        assert_eq!(e.mem.u32(0x108c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(0x1094), 0x7fc0_0001);
        assert_eq!(e.mem.u32(0x1098), 0x7fc0_0002);
        assert_eq!(e.mem.u32(0x1100), 0xffc0_0001);
        assert_eq!(e.mem.u8(0x10d8), 0x82);
        assert_eq!(e.mem.u32(0x1088), 0);
        assert_eq!(e.mem.u32(0x1090), 0);
        for base in [0x10e8, 0x10f4] {
            for i in 0..3 {
                assert_eq!(e.mem.u32(base + i * 4), 0x7f80_0010 + i);
            }
        }
        let rotations: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == 0x0056_fa90)
            .collect();
        assert_eq!(rotations.len(), 1);
        assert_eq!(rotations[0].1[1], 0x7f80_0002);
    }
}
