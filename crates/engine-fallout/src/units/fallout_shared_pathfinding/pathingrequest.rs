//! `fallout shared/pathfinding/pathingrequest.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e25e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequest` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006e2620` and frees 0xb0 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn pathing_request_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006e_2620, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xb0u32]);
    }
    this
}

// Translated from 006e3a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (no Xbox name in the engine map): runs the
/// destructor body at `006e39d0` and frees 0x18 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn fn_006e3a50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006e_39d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x18u32]);
    }
    this
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VPathingAvoidNode@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106D218: u32 = 0x0106_d218;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 006e3ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VPathingAvoidNode@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006e3ab0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106D218);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006e2b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`pathingrequest.cpp`). `thiscall`, `RET 4`: stores the byte
/// argument at `+0x98` of `this`.
pub fn fn_006e2b50(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x98), value);
}

// ---- Batch b039: the PathingRequest members ----------------------------------
//
// `PathingRequest` (Xbox PDB, 0xb0 bytes) offsets used below, as the PC code
// uses them (the shared layout in `pathfind.rs` names a few of them):
// `Origin` +0xc and `Destination` +0x34 (`PathingLocation`s), `LockData` +0x5c
// (`ActorLockData`), `fActorRadius` +0x68, `fActorHeight` +0x6c,
// `fGoalZDelta` +0x70, `fTargetRadius` +0x74, `fCenterRadius` +0x78, `Center`
// +0x7c (`NiPoint3`), `fSmoothingRandomness` +0x88, `fGoalAngle` +0x8c,
// `fInitialPathHeading` +0x90, `spAvoidNodeArray` +0x94, the flags
// `bCantOpenDoors` +0x98 to `bIgnoreLocks` +0xa4, `iRetryCount` +0xa8 and
// `bFailIfGoalOccupied` +0xac.

/// The `PathingRequest` vtable.
const PATHING_REQUEST_VTABLE: u32 = 0x0106_d030;
/// The default actor radius (`float`) the constructor stores.
const DEFAULT_ACTOR_RADIUS: u32 = 0x0104_ef40;
/// The default goal z delta (`float`) the constructor stores.
const DEFAULT_GOAL_Z_DELTA: u32 = 0x0106_b158;
/// The float `006e25d0` answers (the default target radius; the Xbox PDB
/// gives `PathingRequest` a static `fDefaultRadius`, not tied to this
/// address).
const DEFAULT_TARGET_RADIUS: u32 = 0x0119_e4bc;
/// The zero point (`NiPoint3`, three words) the code copies.
const ZERO_POINT: u32 = 0x011f_426c;
/// `0.6` (`double`): the factor on the actor's radius.
const RADIUS_FACTOR: u32 = 0x0102_90b8;
/// The vtables of the avoid-node array (`006e3960`'s class) and of its
/// derived class (`006e3850`): main and second (+0x10) vtables.
const AVOID_ARRAY_VTABLE: u32 = 0x0106_d204;
const AVOID_ARRAY_SECOND_VTABLE: u32 = 0x0106_d1f8;
const AVOID_ARRAY_DERIVED_VTABLE: u32 = 0x0106_d1e4;
const AVOID_ARRAY_DERIVED_SECOND_VTABLE: u32 = 0x0106_d1d8;

/// `NiRefObject` constructor (`004968b0`, `thiscall`).
const REF_OBJECT_CONSTRUCT: u32 = 0x0049_68b0;
/// `NiRefObject` destructor body (`00496910`, `thiscall`).
const REF_OBJECT_DESTRUCT: u32 = 0x0049_6910;
/// `PathingLocation::PathingLocation` (engine map), `thiscall`.
const LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;
/// `PathingLocation::PathingLocation_ov3` (engine map), `thiscall`, `RET 4`:
/// the location of an actor.
const LOCATION_CONSTRUCT_FROM_ACTOR: u32 = 0x006d_cd70;
/// `PathingLocation::operator=` (engine map), `thiscall`, `RET 4`.
const LOCATION_ASSIGN: u32 = 0x006d_cce0;
/// The `PathingLocation` destructor (`004ff7e0`, `thiscall`).
const LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
/// The `ActorLockData` constructor (`00540720`, `thiscall`).
const LOCK_DATA_CONSTRUCT: u32 = 0x0054_0720;
/// `ActorLockData::ActorLockData` (engine map), `thiscall`, `RET 4`: the
/// lock data of an actor.
const LOCK_DATA_CONSTRUCT_FROM_ACTOR: u32 = 0x0050_2670;
/// `00502630` (`thiscall`, `RET 4`): copies lock data.
const LOCK_DATA_ASSIGN: u32 = 0x0050_2630;
/// `006815c0` (`thiscall`): run on the center point before it is set.
const POINT_PREPARE: u32 = 0x0068_15c0;
/// The `NiPointer` constructor (`006b37b0`, `thiscall`, `RET 4`) and
/// destructor (`006b37e0`) of the avoid-node array pointer.
const AVOID_POINTER_CONSTRUCT: u32 = 0x006b_37b0;
const AVOID_POINTER_DESTRUCT: u32 = 0x006b_37e0;
/// `BGSSaveGameBuffer` raw save (`00865e50`, `thiscall`: data, size, 0).
const SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveVariableSizedValue` (engine map), `thiscall`.
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f60;
/// `00559450` (`thiscall`): the word at `this` (the pointed-to array).
const WORD_AT: u32 = 0x0055_9450;
/// `0044ddc0` (`thiscall`): the word at `this` (here the array's size).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `006a1400` (`thiscall`, `RET 4`): the array element at an index.
const ARRAY_ELEMENT: u32 = 0x006a_1400;
/// `006dc890` (`thiscall`, `RET 4`): an avoid node's save.
const AVOID_NODE_SAVE: u32 = 0x006d_c890;
/// `MemoryManager` allocation (`00401000`, `cdecl`: size).
const ALLOCATE: u32 = 0x0040_1000;
/// `006e3c90` (`thiscall`, `RET 8`): constructs elements (buffer, count).
const AVOID_NODES_CONSTRUCT: u32 = 0x006e_3c90;
/// The heap free (`00aa1460`, `cdecl`: block, size).
const FREE_SIZED: u32 = 0x00aa_1460;

/// `FSTP dword` of a value the callee left in `ST0`, as the float word.
fn st0_to_float(result: Ret) -> f32 {
    f32::from_bits(x87_store32(result.f64()))
}

// Translated from 006e2420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e2420 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `PathingRequest::PathingRequest` (Xbox PDB): the `NiRefObject` base, the
/// vtable, both locations, the lock data, the center point and the
/// avoid-node pointer (null) through their constructors, then the defaults:
/// actor radius from `0104ef40`, height 1.0, goal z delta from `0106b158`,
/// target radius from `006e25d0`, center radius 0, the zero center, goal
/// angle and initial heading 0, the flags (`bAllowIncompletePath`,
/// `bSoftGoalRadius` and `bCanSwim` set, the others clear), retry count 3,
/// smoothing randomness 0. Returns `this`.
pub fn pathing_request_pathing_request(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, PATHING_REQUEST_VTABLE);
    pathing_request_construct_members(e, this);
    let radius = x87_quiet_bits(e.global::<u32>(DEFAULT_ACTOR_RADIUS));
    e.mem.set_u32(base + 0x68, radius);
    e.mem.set_f32(base + 0x6c, 1.0);
    let goal_z = x87_quiet_bits(e.global::<u32>(DEFAULT_GOAL_Z_DELTA));
    e.mem.set_u32(base + 0x70, goal_z);
    let target_radius = fn_006e25d0(e);
    e.mem
        .set_u32(base + 0x74, x87_store32(f64::from(target_radius)));
    e.mem.set_f32(base + 0x78, 0.0);
    for i in 0..3 {
        let word = e.global::<u32>(ZERO_POINT + 4 * i);
        e.mem.set_u32(base + 0x7c + 4 * i, word);
    }
    e.mem.set_f32(base + 0x8c, 0.0);
    e.mem.set_f32(base + 0x90, 0.0);
    for (offset, value) in [
        (0x98u32, 0u8),
        (0x99, 0),
        (0x9a, 0),
        (0x9b, 1),
        (0x9c, 1),
        (0x9d, 1),
        (0x9e, 0),
        (0x9f, 0),
        (0xa0, 0),
        (0xa1, 0),
        (0xa2, 0),
        (0xa3, 0),
        (0xa4, 0),
    ] {
        e.mem.set_u8(base + offset, value);
    }
    e.mem.set_u32(base + 0xa8, 3);
    e.mem.set_u8(base + 0xac, 0);
    e.mem.set_f32(base + 0x88, 0.0);
    this
}

/// The member constructors both `PathingRequest` constructors run after
/// the vtable: the two locations, the lock data, the center point and the
/// avoid-node pointer (null).
fn pathing_request_construct_members(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(LOCATION_CONSTRUCT, &args![base + 0xc]);
    e.call(LOCATION_CONSTRUCT, &args![base + 0x34]);
    e.call(LOCK_DATA_CONSTRUCT, &args![base + 0x5c]);
    e.call(POINT_PREPARE, &args![base + 0x7c]);
    e.call(AVOID_POINTER_CONSTRUCT, &args![base + 0x94, 0u32]);
}

// Translated from 006e25d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `0119e4bc` (`FLD`): the target radius the constructor
/// stores.
pub fn fn_006e25d0(e: &mut Engine) -> f32 {
    x87_quiet(e.global::<f32>(DEFAULT_TARGET_RADIUS))
}

// Translated from 006e2620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e2620 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `PathingRequest` destructor body: the vtable, then the avoid-node
/// pointer (`006b37e0`), the destination and the origin (`004ff7e0`) and
/// the `NiRefObject` base (`00496910`).
pub fn fn_006e2620(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, PATHING_REQUEST_VTABLE);
    e.call(AVOID_POINTER_DESTRUCT, &args![base + 0x94]);
    e.call(LOCATION_DESTRUCT, &args![base + 0x34]);
    e.call(LOCATION_DESTRUCT, &args![base + 0xc]);
    e.call(REF_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 006e26a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e26a0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `PathingRequest::PathingRequest_ov2` (Xbox PDB), the copy constructor:
/// the base, the vtable and the members as in the default constructor, then
/// `other`'s `CopyTo` (`006e2750`) into `this`. Returns `this`.
pub fn pathing_request_pathing_request_ov2(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), PATHING_REQUEST_VTABLE);
    pathing_request_construct_members(e, this);
    pathing_request_copy_to(e, other, this);
    this
}

// Translated from 006e2750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequest::CopyTo` (Xbox PDB): copies the request into `other`
/// through the getters and setters of each part (actor `0048d150` /
/// `006d3ac0`, origin `006a9540` / `006d1bc0`, actor radius `009a7c40` /
/// `0051b840`, actor height `0069ef80` / `006d3ae0`, lock data `006c9900` /
/// `006e29b0`, target radius `009b0bd0` / `006d3b00`, center and center
/// radius `006a7910` and `006a78f0` / `006e2980`, goal z delta `0051b860` /
/// `006e2960` (twice, around the destination `0069e390` / `006d61e0`),
/// smoothing randomness `0051b8e0` / `006e29d0`), then the flags, goal
/// angle, initial heading, retry count and `bFailIfGoalOccupied` directly
/// (`bDrawDebugGeometry` at +0x9a is not copied).
pub fn pathing_request_copy_to(e: &mut Engine, this: Ptr, other: Ptr) {
    let actor = e.call(0x0048_d150, &args![this]).u32();
    e.call(0x006d_3ac0, &args![other, actor]);
    let origin = e.call(0x006a_9540, &args![this]).u32();
    e.call(0x006d_1bc0, &args![other, origin]);
    let radius = st0_to_float(e.call(0x009a_7c40, &args![this]));
    e.call(0x0051_b840, &args![other, radius]);
    let height = st0_to_float(e.call(0x0069_ef80, &args![this]));
    e.call(0x006d_3ae0, &args![other, height]);
    let lock_data = e.call(0x006c_9900, &args![this]).u32();
    fn_006e29b0(e, other, lock_data);
    let target_radius = st0_to_float(e.call(0x009b_0bd0, &args![this]));
    e.call(0x006d_3b00, &args![other, target_radius]);
    e.with_stack(0xc, |e, center| {
        e.call(0x006a_7910, &args![this, center]);
        let center_radius = st0_to_float(e.call(0x006a_78f0, &args![this]));
        fn_006e2980(e, other, center.addr(), center_radius);
    });
    let goal_z = st0_to_float(e.call(0x0051_b860, &args![this]));
    fn_006e2960(e, other, goal_z);
    let destination = e.call(0x0069_e390, &args![this]).u32();
    e.call(0x006d_61e0, &args![other, destination]);
    let goal_z = st0_to_float(e.call(0x0051_b860, &args![this]));
    fn_006e2960(e, other, goal_z);
    let randomness = st0_to_float(e.call(0x0051_b8e0, &args![this]));
    fn_006e29d0(e, other, randomness);

    let from = this.addr();
    let to = other.addr();
    let byte = e.mem.u8(from + 0x98);
    e.mem.set_u8(to + 0x98, byte);
    let angle = x87_quiet_bits(e.mem.u32(from + 0x8c));
    e.mem.set_u32(to + 0x8c, angle);
    let heading = x87_quiet_bits(e.mem.u32(from + 0x90));
    e.mem.set_u32(to + 0x90, heading);
    for offset in [
        0x99u32, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4,
    ] {
        let byte = e.mem.u8(from + offset);
        e.mem.set_u8(to + offset, byte);
    }
    let retries = e.mem.u32(from + 0xa8);
    e.mem.set_u32(to + 0xa8, retries);
    let byte = e.mem.u8(from + 0xac);
    e.mem.set_u8(to + 0xac, byte);
}

// Translated from 006e2960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fGoalZDelta` (+0x70).
pub fn fn_006e2960(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x70, x87_quiet_bits(value.to_bits()));
}

// Translated from 006e2980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `Center` (+0x7c, three words from `center`) and `fCenterRadius`
/// (+0x78).
pub fn fn_006e2980(e: &mut Engine, this: Ptr, center: u32, radius: f32) {
    for i in 0..3 {
        let word = e.mem.u32(center + 4 * i);
        e.mem.set_u32(this.addr() + 0x7c + 4 * i, word);
    }
    e.mem
        .set_u32(this.addr() + 0x78, x87_quiet_bits(radius.to_bits()));
}

// Translated from 006e29b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `lock_data` into `LockData` (+0x5c) with `00502630`.
pub fn fn_006e29b0(e: &mut Engine, this: Ptr, lock_data: u32) {
    e.call(LOCK_DATA_ASSIGN, &args![this.addr() + 0x5c, lock_data]);
}

// Translated from 006e29d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fSmoothingRandomness` (+0x88).
pub fn fn_006e29d0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x88, x87_quiet_bits(value.to_bits()));
}

// Translated from 006e29f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e29f0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Fills the request from `actor` (no name in the engine map): the origin
/// becomes the actor's location (a `PathingLocation` built on the stack by
/// `006dcd70`, assigned with `006dcce0`, destroyed with `004ff7e0`), the
/// actor height 0.6 times `008be280`, the lock data from the actor
/// (`006e2b20`), `bCanSwim` and `bCanFly` from `0087f350` and `0087f390`,
/// `bCantOpenDoors` from the actor's process (`004181e0` +0x30, slot +0x28)
/// when the actor's slot +0x21c says so, and the initial heading from the
/// actor's slot +0x2bc (with 0).
pub fn fn_006e29f0(e: &mut Engine, this: Ptr, actor: u32) {
    e.with_stack(0x28, |e, local| {
        let location = e
            .call(LOCATION_CONSTRUCT_FROM_ACTOR, &args![local, actor])
            .u32();
        e.call(LOCATION_ASSIGN, &args![this.addr() + 0xc, location]);
        e.call(LOCATION_DESTRUCT, &args![local]);
    });
    let size = e.call(0x008b_e280, &args![actor]).f64();
    let factor = e.global::<f64>(RADIUS_FACTOR);
    let height = x87_store32(x87_mul(size, factor));
    e.call(0x006d_3ae0, &args![this, x87_quiet_bits(height)]);
    fn_006e2b20(e, this, actor);
    let can_swim = e.call(0x0087_f350, &args![actor]).u8();
    fn_006e2b70(e, this, can_swim);
    let can_fly = e.call(0x0087_f390, &args![actor]).u8();
    fn_006e2b90(e, this, can_fly);
    if e.vcall(actor, 0x21c, &[]).bool() {
        let process = e.call(0x0041_81e0, &args![actor]).u32().wrapping_add(0x30);
        let cant_open = e.vcall(process, 0x28, &[]).u8();
        fn_006e2b50(e, this, cant_open);
    }
    let heading = st0_to_float(e.vcall(actor, 0x2bc, &args![0u32]));
    fn_006e2bb0(e, this, heading);
}

// Translated from 006e2b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `LockData` (+0x5c) from `actor`: the actor's lock data built on the
/// stack (`00502670`), copied with `00502630`.
pub fn fn_006e2b20(e: &mut Engine, this: Ptr, actor: u32) {
    e.with_stack(0xc, |e, local| {
        let lock_data = e
            .call(LOCK_DATA_CONSTRUCT_FROM_ACTOR, &args![local, actor])
            .u32();
        e.call(LOCK_DATA_ASSIGN, &args![this.addr() + 0x5c, lock_data]);
    });
}

// Translated from 006e2b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bCanSwim` (+0x9d).
pub fn fn_006e2b70(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x9d, value);
}

// Translated from 006e2b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bCanFly` (+0x9e).
pub fn fn_006e2b90(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x9e, value);
}

// Translated from 006e2bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fInitialPathHeading` (+0x90) and `bFirstTangentUsesHeading`
/// (+0xa0).
pub fn fn_006e2bb0(e: &mut Engine, this: Ptr, heading: f32) {
    e.mem
        .set_u32(this.addr() + 0x90, x87_quiet_bits(heading.to_bits()));
    e.mem.set_u8(this.addr() + 0xa0, 1);
}

// Translated from 006e3230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequest::SaveGame` (Xbox PDB): saves the origin and the
/// destination (their virtual slot 0 with the buffer), the radii, height,
/// goal data, flags, center and retry count in the exe's order
/// (`00865e50`), then the number of avoid nodes (`00865f60`; 0 without an
/// array) and each avoid node (`006dc890`).
pub fn pathing_request_save_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.vcall(base + 0xc, 0, &args![buffer]);
    e.vcall(base + 0x34, 0, &args![buffer]);
    for (offset, size) in [
        (0x68u32, 4u32),
        (0x6c, 4),
        (0x70, 4),
        (0x74, 4),
        (0x78, 4),
        (0x88, 4),
        (0x8c, 4),
        (0x99, 1),
        (0x98, 1),
        (0x9a, 1),
        (0x9b, 1),
        (0x9c, 1),
        (0x9d, 1),
        (0x9f, 1),
        (0x90, 4),
        (0xa0, 1),
        (0xa1, 1),
        (0x7c, 0xc),
        (0xa2, 1),
        (0xa3, 1),
        (0xa4, 1),
        (0xa8, 4),
        (0x9e, 1),
        (0xac, 1),
    ] {
        e.call(SAVE_DATA, &args![buffer, base + offset, size, 0u32]);
    }
    let count = if e.call(WORD_AT, &args![base + 0x94]).u32() != 0 {
        let array = e.call(WORD_AT, &args![base + 0x94]).u32();
        e.call(ARRAY_SIZE, &args![array]).u32()
    } else {
        0
    };
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count]);
    let mut index = 0u32;
    while index < count {
        let array = e.call(WORD_AT, &args![base + 0x94]).u32();
        let node = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
        e.call(AVOID_NODE_SAVE, &args![node, buffer]);
        index = index.wrapping_add(1);
    }
}

// Translated from 006e3850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the class derived from the avoid-node array (no name in
/// the engine map): the base constructor (`006e3960`), then the derived
/// class's two vtables (+0 and +0x10). Returns `this`.
pub fn fn_006e3850(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006e3960(e, this);
    e.mem.set_u32(this.addr(), AVOID_ARRAY_DERIVED_VTABLE);
    e.mem
        .set_u32(this.addr() + 0x10, AVOID_ARRAY_DERIVED_SECOND_VTABLE);
    this
}

// Translated from 006e3880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the derived avoid-node array: the
/// destructor (`006e38b0`), then the 0x18-byte free (`00aa1460`) when
/// `flags & 1`. Returns `this`.
pub fn fn_006e3880(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006e38b0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x18u32]);
    }
    this
}

// Translated from 006e38b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The derived avoid-node array's destructor: the base destructor body
/// (`006e39d0`).
pub fn fn_006e38b0(e: &mut Engine, this: Ptr) {
    fn_006e39d0(e, this);
}

// Translated from 006e3960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e3960 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The avoid-node array constructor (no name in the engine map): the
/// `BSSimpleArray<PathingAvoidNode,1024>` part (`006e3a80`), the
/// `NiRefObject` part at +0x10 (`004968b0`) and the class's two vtables.
/// Returns `this`.
pub fn fn_006e3960(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006e3a80(e, this);
    e.call(REF_OBJECT_CONSTRUCT, &args![this.addr() + 0x10]);
    e.mem.set_u32(this.addr(), AVOID_ARRAY_VTABLE);
    e.mem.set_u32(this.addr() + 0x10, AVOID_ARRAY_SECOND_VTABLE);
    this
}

// Translated from 006e39d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e39d0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The avoid-node array destructor body: the class's vtables, the array's
/// release (`008454f0` with 1), the `NiRefObject` part at +0x10
/// (`00496910`) and the `BSSimpleArray` destructor (`006e3ab0`).
pub fn fn_006e39d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), AVOID_ARRAY_VTABLE);
    e.mem.set_u32(this.addr() + 0x10, AVOID_ARRAY_SECOND_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    e.call(REF_OBJECT_DESTRUCT, &args![this.addr() + 0x10]);
    fn_006e3ab0(e, this);
}

// Translated from 006e3a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<PathingAvoidNode,1024>` constructor: its vtable, then
/// the empty initialisation (`006e3de0(0, 0)`). Returns `this`.
pub fn fn_006e3a80(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106D218);
    fn_006e3de0(e, this, 0, 0);
    this
}

// Translated from 006e3c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingAvoidNode,1024>::_Allocate` (Xbox PDB): a block of
/// `count` 0x24-byte nodes (`00401000`). `this` is not read.
pub fn bs_simple_array_pathing_avoid_node_1024_allocate(
    e: &mut Engine,
    _unused_0: Ptr,
    count: u32,
) -> u32 {
    e.call(ALLOCATE, &args![count.wrapping_mul(0x24)]).u32()
}

// Translated from 006e3de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The array's initialisation (no name in the engine map; `RET 8`): clears
/// the buffer, size and capacity; `size` is raised to `used` (unsigned);
/// with a non-zero size the buffer is the array's slot +4 (allocate) and
/// the capacity is `size`; with a non-zero `used` that many nodes are
/// constructed (`006e3c90`) and the size is `used`.
pub fn fn_006e3de0(e: &mut Engine, this: Ptr, size: u32, used: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let size = size.max(used);
    if size != 0 {
        let buffer = e.vcall(base, 4, &args![size]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, size);
    }
    if used != 0 {
        let buffer = e.mem.u32(base + 4);
        e.call(AVOID_NODES_CONSTRUCT, &args![this, buffer, used]);
        e.mem.set_u32(base + 8, used);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_2b50, fn_006e2b50(Ptr, u8)),
        entry!(0x006e_25e0, pathing_request_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_3a50, fn_006e3a50(Ptr, u32) -> Ptr),
        entry!(0x006e_3ab0, fn_006e3ab0(Ptr)),
        entry!(0x006e_2420, pathing_request_pathing_request(Ptr) -> Ptr),
        entry!(0x006e_25d0, fn_006e25d0() -> f32),
        entry!(0x006e_2620, fn_006e2620(Ptr)),
        entry!(0x006e_26a0, pathing_request_pathing_request_ov2(Ptr, Ptr) -> Ptr),
        entry!(0x006e_2750, pathing_request_copy_to(Ptr, Ptr)),
        entry!(0x006e_2960, fn_006e2960(Ptr, f32)),
        entry!(0x006e_2980, fn_006e2980(Ptr, u32, f32)),
        entry!(0x006e_29b0, fn_006e29b0(Ptr, u32)),
        entry!(0x006e_29d0, fn_006e29d0(Ptr, f32)),
        entry!(0x006e_29f0, fn_006e29f0(Ptr, u32)),
        entry!(0x006e_2b20, fn_006e2b20(Ptr, u32)),
        entry!(0x006e_2b70, fn_006e2b70(Ptr, u8)),
        entry!(0x006e_2b90, fn_006e2b90(Ptr, u8)),
        entry!(0x006e_2bb0, fn_006e2bb0(Ptr, f32)),
        entry!(0x006e_3230, pathing_request_save_game(Ptr, u32)),
        entry!(0x006e_3850, fn_006e3850(Ptr) -> Ptr),
        entry!(0x006e_3880, fn_006e3880(Ptr, u32) -> Ptr),
        entry!(0x006e_38b0, fn_006e38b0(Ptr)),
        entry!(0x006e_3960, fn_006e3960(Ptr) -> Ptr),
        entry!(0x006e_39d0, fn_006e39d0(Ptr)),
        entry!(0x006e_3a80, fn_006e3a80(Ptr) -> Ptr),
        entry!(
            0x006e_3c10,
            bs_simple_array_pathing_avoid_node_1024_allocate(Ptr, u32) -> u32
        ),
        entry!(0x006e_3de0, fn_006e3de0(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn pathing_request_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006e_25e0, 0x006e_2620, 0xb0);
    }

    #[test]
    fn fn_006e3a50_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006e_3a50, 0x006e_39d0, 0x18);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_006e3ab0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006e_3ab0, ARRAY_VTABLE_0106D218, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_006e2b50_stores_the_byte_at_offset_0x98() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + 0x98, 0x1122_3344);
        e.call(0x006e_2b50, &args![this, 0xa5u8]);
        // Only the low byte changes.
        assert_eq!(e.mem.u32(this + 0x98), 0x1122_33a5);
        e.call(0x006e_2b50, &args![this, 0u8]);
        assert_eq!(e.mem.u32(this + 0x98), 0x1122_3300);
    }
}

#[cfg(test)]
mod b039_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The order of the calls to the given callees.
    fn order(e: &Engine, callees: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| callees.contains(address))
            .cloned()
            .collect()
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    const MEMBER_CONSTRUCTORS: [u32; 5] = [
        REF_OBJECT_CONSTRUCT,
        LOCATION_CONSTRUCT,
        LOCK_DATA_CONSTRUCT,
        POINT_PREPARE,
        AVOID_POINTER_CONSTRUCT,
    ];

    fn constructor_doubles(e: &mut Engine) {
        for callee in MEMBER_CONSTRUCTORS {
            e.register(callee, nothing);
        }
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = Engine::new();
        constructor_doubles(&mut e);
        e.map(0x0104_e000, 0x1000);
        e.map(0x0106_b000, 0x1000);
        e.map(0x0119_e000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.set_global(DEFAULT_ACTOR_RADIUS, 35.0f32);
        e.set_global(DEFAULT_GOAL_Z_DELTA, 0x7f80_0001u32);
        e.set_global(DEFAULT_TARGET_RADIUS, 12.5f32);
        e.set_global(ZERO_POINT, 1.0f32);
        let this = e.mem.alloc(0xb0);
        for i in 0..0x2c {
            e.mem.set_u32(this + 4 * i, 0xcccc_cccc);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_2420, &args![this]).u32(), this);
        assert_eq!(
            order(&e, &MEMBER_CONSTRUCTORS),
            vec![
                (REF_OBJECT_CONSTRUCT, vec![this]),
                (LOCATION_CONSTRUCT, vec![this + 0xc]),
                (LOCATION_CONSTRUCT, vec![this + 0x34]),
                (LOCK_DATA_CONSTRUCT, vec![this + 0x5c]),
                (POINT_PREPARE, vec![this + 0x7c]),
                (AVOID_POINTER_CONSTRUCT, vec![this + 0x94, 0]),
            ]
        );
        assert_eq!(e.mem.u32(this), PATHING_REQUEST_VTABLE);
        assert_eq!(e.mem.f32(this + 0x68), 35.0);
        assert_eq!(e.mem.f32(this + 0x6c), 1.0);
        assert_eq!(e.mem.u32(this + 0x70), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x74), 12.5);
        assert_eq!(e.mem.u32(this + 0x78), 0);
        assert_eq!(e.mem.f32(this + 0x7c), 1.0);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.mem.u32(this + 0x88), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 0);
        assert_eq!(e.mem.u32(this + 0x90), 0);
        assert_eq!(e.mem.u32(this + 0x98), 0x0100_0000);
        assert_eq!(e.mem.u32(this + 0x9c), 0x0000_0101);
        assert_eq!(e.mem.u32(this + 0xa0), 0);
        assert_eq!(e.mem.u8(this + 0xa4), 0);
        assert_eq!(e.mem.u32(this + 0xa8), 3);
        assert_eq!(e.mem.u8(this + 0xac), 0);
        // Not written: the actor word and the padding after +0xa4.
        assert_eq!(e.mem.u32(this + 8), 0xcccc_cccc);
        assert_eq!(e.mem.u8(this + 0xa5), 0xcc);
    }

    #[test]
    fn fn_006e25d0_reads_the_default_target_radius() {
        let mut e = Engine::new();
        e.map(0x0119_e000, 0x1000);
        e.set_global(DEFAULT_TARGET_RADIUS, 64.0f32);
        assert_eq!(e.call(0x006e_25d0, &[]).f64(), 64.0);
    }

    #[test]
    fn destructor_body_destroys_the_members_in_reverse() {
        let mut e = Engine::new();
        let callees = [
            AVOID_POINTER_DESTRUCT,
            LOCATION_DESTRUCT,
            REF_OBJECT_DESTRUCT,
        ];
        for callee in callees {
            e.register(callee, nothing);
        }
        let this = e.mem.alloc(0xb0);
        e.call_log = Some(vec![]);
        e.call(0x006e_2620, &args![this]);
        assert_eq!(e.mem.u32(this), PATHING_REQUEST_VTABLE);
        assert_eq!(
            order(&e, &callees),
            vec![
                (AVOID_POINTER_DESTRUCT, vec![this + 0x94]),
                (LOCATION_DESTRUCT, vec![this + 0x34]),
                (LOCATION_DESTRUCT, vec![this + 0xc]),
                (REF_OBJECT_DESTRUCT, vec![this]),
            ]
        );
    }

    const COPY_SETTERS: [u32; 6] = [
        0x006d_3ac0,
        0x006d_1bc0,
        0x0051_b840,
        0x006d_3ae0,
        0x006d_3b00,
        0x006d_61e0,
    ];

    /// Getters answering fixed values; the float getters 1.5, 2.5, ...
    fn copy_doubles(e: &mut Engine) {
        e.register(0x0048_d150, |_, _| word(0xa1));
        e.register(0x006a_9540, |_, _| word(0xa2));
        e.register(0x006c_9900, |_, _| word(0xa3));
        e.register(0x0069_e390, |_, _| word(0xa4));
        e.register(0x006a_7910, |e, a| {
            e.mem.set_f32(a[1], 7.0);
            e.mem.set_f32(a[1] + 4, 8.0);
            e.mem.set_f32(a[1] + 8, 9.0);
            Ret::default()
        });
        e.register(0x009a_7c40, |_, _| float(1.5));
        e.register(0x0069_ef80, |_, _| float(2.5));
        e.register(0x009b_0bd0, |_, _| float(3.5));
        e.register(0x006a_78f0, |_, _| float(4.5));
        e.register(0x0051_b860, |_, _| float(5.5));
        e.register(0x0051_b8e0, |_, _| float(6.5));
        for callee in COPY_SETTERS {
            e.register(callee, nothing);
        }
        e.register(LOCK_DATA_ASSIGN, nothing);
    }

    fn check_copy(e: &Engine, from: u32, to: u32) {
        assert_eq!(
            order(e, &COPY_SETTERS),
            vec![
                (0x006d_3ac0, vec![to, 0xa1]),
                (0x006d_1bc0, vec![to, 0xa2]),
                (0x0051_b840, vec![to, 1.5f32.to_bits()]),
                (0x006d_3ae0, vec![to, 2.5f32.to_bits()]),
                (0x006d_3b00, vec![to, 3.5f32.to_bits()]),
                (0x006d_61e0, vec![to, 0xa4]),
            ]
        );
        assert_eq!(calls_to(e, LOCK_DATA_ASSIGN), vec![vec![to + 0x5c, 0xa3]]);
        assert_eq!(calls_to(e, 0x006a_7910)[0][0], from);
        assert_eq!(e.mem.f32(to + 0x7c), 7.0);
        assert_eq!(e.mem.f32(to + 0x84), 9.0);
        assert_eq!(e.mem.f32(to + 0x78), 4.5);
        assert_eq!(e.mem.f32(to + 0x70), 5.5);
        assert_eq!(e.mem.f32(to + 0x88), 6.5);
        assert_eq!(e.mem.f32(to + 0x8c), 0.25);
        assert_eq!(e.mem.u32(to + 0x90), 0x7fc0_0002);
        assert_eq!(e.mem.u8(to + 0x98), 1);
        assert_eq!(e.mem.u8(to + 0x99), 2);
        assert_eq!(e.mem.u8(to + 0x9a), 0xee);
        assert_eq!(e.mem.u8(to + 0xa4), 0x0d);
        assert_eq!(e.mem.u32(to + 0xa8), 0x1234_5678);
        assert_eq!(e.mem.u8(to + 0xac), 0x77);
    }

    fn source_request(e: &mut Engine) -> u32 {
        let from = e.mem.alloc(0xb0);
        e.mem.set_f32(from + 0x8c, 0.25);
        e.mem.set_u32(from + 0x90, 0x7f80_0002);
        for (i, offset) in (0x98u32..=0xa4).enumerate() {
            e.mem.set_u8(from + offset, i as u8 + 1);
        }
        e.mem.set_u32(from + 0xa8, 0x1234_5678);
        e.mem.set_u8(from + 0xac, 0x77);
        from
    }

    #[test]
    fn copy_to_copies_every_part() {
        let mut e = Engine::new();
        copy_doubles(&mut e);
        let from = source_request(&mut e);
        let to = e.mem.alloc(0xb0);
        e.mem.set_u8(to + 0x9a, 0xee);
        e.call_log = Some(vec![]);
        e.call(0x006e_2750, &args![from, to]);
        check_copy(&e, from, to);
        // The getters run on the source.
        assert_eq!(calls_to(&e, 0x0048_d150), vec![vec![from]]);
        assert_eq!(calls_to(&e, 0x0051_b860), vec![vec![from], vec![from]]);
    }

    #[test]
    fn copy_constructor_builds_then_copies_from_the_other() {
        let mut e = Engine::new();
        constructor_doubles(&mut e);
        copy_doubles(&mut e);
        let from = source_request(&mut e);
        let to = e.mem.alloc(0xb0);
        e.mem.set_u8(to + 0x9a, 0xee);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_26a0, &args![to, from]).u32(), to);
        assert_eq!(e.mem.u32(to), PATHING_REQUEST_VTABLE);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCT), vec![vec![to]]);
        assert_eq!(
            calls_to(&e, AVOID_POINTER_CONSTRUCT),
            vec![vec![to + 0x94, 0]]
        );
        check_copy(&e, from, to);
    }

    #[test]
    fn fn_006e2960_sets_the_goal_z_delta() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.call(0x006e_2960, &args![this, 0x7f80_0003u32]);
        assert_eq!(e.mem.u32(this + 0x70), 0x7fc0_0003);
        e.call(0x006e_2960, &args![this, 3.0f32]);
        assert_eq!(e.mem.f32(this + 0x70), 3.0);
    }

    #[test]
    fn fn_006e2980_sets_the_center_and_its_radius() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        let center = e.mem.alloc(0xc);
        e.mem.set_u32(center, 1);
        e.mem.set_u32(center + 4, 2);
        e.mem.set_u32(center + 8, 3);
        e.call(0x006e_2980, &args![this, center, 4.0f32]);
        assert_eq!(e.mem.u32(this + 0x7c), 1);
        assert_eq!(e.mem.u32(this + 0x80), 2);
        assert_eq!(e.mem.u32(this + 0x84), 3);
        assert_eq!(e.mem.f32(this + 0x78), 4.0);
    }

    #[test]
    fn fn_006e29b0_copies_the_lock_data() {
        let mut e = Engine::new();
        e.register(LOCK_DATA_ASSIGN, nothing);
        e.call_log = Some(vec![]);
        e.call(0x006e_29b0, &args![0x5000u32, 0x6000u32]);
        assert_eq!(calls_to(&e, LOCK_DATA_ASSIGN), vec![vec![0x505c, 0x6000]]);
    }

    #[test]
    fn fn_006e29d0_sets_the_smoothing_randomness() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.call(0x006e_29d0, &args![this, 0.5f32]);
        assert_eq!(e.mem.f32(this + 0x88), 0.5);
    }

    #[test]
    fn the_flag_setters_set_their_bytes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.call(0x006e_2b70, &args![this, 1u8]);
        assert_eq!(e.mem.u8(this + 0x9d), 1);
        assert_eq!(e.mem.u8(this + 0x9e), 0);
        e.call(0x006e_2b90, &args![this, 1u8]);
        assert_eq!(e.mem.u8(this + 0x9e), 1);
        e.call(0x006e_2b70, &args![this, 0u8]);
        assert_eq!(e.mem.u8(this + 0x9d), 0);
    }

    #[test]
    fn fn_006e2b90_sets_can_fly() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.mem.set_u8(this + 0x9e, 7);
        e.call(0x006e_2b90, &args![this, 0u8]);
        assert_eq!(e.mem.u8(this + 0x9e), 0);
    }

    #[test]
    fn fn_006e2bb0_sets_the_heading_and_its_flag() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.call(0x006e_2bb0, &args![this, 1.25f32]);
        assert_eq!(e.mem.f32(this + 0x90), 1.25);
        assert_eq!(e.mem.u8(this + 0xa0), 1);
    }

    #[test]
    fn fn_006e2b20_builds_the_actors_lock_data() {
        let mut e = Engine::new();
        e.register(LOCK_DATA_CONSTRUCT_FROM_ACTOR, |_, a| word(a[0]));
        e.register(LOCK_DATA_ASSIGN, nothing);
        e.call_log = Some(vec![]);
        e.call(0x006e_2b20, &args![0x5000u32, 0x7000u32]);
        let built = calls_to(&e, LOCK_DATA_CONSTRUCT_FROM_ACTOR);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1], 0x7000);
        assert_eq!(
            calls_to(&e, LOCK_DATA_ASSIGN),
            vec![vec![0x505c, built[0][0]]]
        );
    }

    fn actor(e: &mut Engine, swims: bool) -> u32 {
        let actor = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0; 0xb0]);
        e.mem.set_u32(0x0300_0000 + 0x21c, 0x0300_1000);
        e.mem.set_u32(0x0300_0000 + 0x2bc, 0x0300_2000);
        e.mem.set_u32(actor, 0x0300_0000);
        e.register_double(0x0300_1000, move |_, _| word(u32::from(swims)));
        e.register(0x0300_2000, |_, _| float(0.75));
        let process = e.mem.alloc(0x40);
        e.put_vtable(0x0300_4000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0300_3000]);
        e.mem.set_u32(process + 0x30, 0x0300_4000);
        e.register(0x0300_3000, |_, _| word(0x101));
        e.register_double(0x0041_81e0, move |_, _| word(process));
        actor
    }

    #[test]
    fn fn_006e29f0_fills_the_request_from_the_actor() {
        let mut e = Engine::new();
        e.map(0x0102_9000, 0x1000);
        e.set_global(RADIUS_FACTOR, 0.5f64);
        e.register(LOCATION_CONSTRUCT_FROM_ACTOR, |_, a| word(a[0]));
        e.register(LOCATION_ASSIGN, nothing);
        e.register(LOCATION_DESTRUCT, nothing);
        e.register(0x008b_e280, |_, _| float(80.0));
        e.register(0x006d_3ae0, nothing);
        e.register(LOCK_DATA_CONSTRUCT_FROM_ACTOR, |_, a| word(a[0]));
        e.register(LOCK_DATA_ASSIGN, nothing);
        e.register(0x0087_f350, |_, _| word(0xff01));
        e.register(0x0087_f390, |_, _| word(0));
        let this = e.mem.alloc(0xb0);
        let actor = actor(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x006e_29f0, &args![this, actor]);
        let local = calls_to(&e, LOCATION_CONSTRUCT_FROM_ACTOR)[0][0];
        assert_eq!(
            calls_to(&e, LOCATION_CONSTRUCT_FROM_ACTOR),
            vec![vec![local, actor]]
        );
        assert_eq!(calls_to(&e, LOCATION_ASSIGN), vec![vec![this + 0xc, local]]);
        assert_eq!(calls_to(&e, LOCATION_DESTRUCT), vec![vec![local]]);
        assert_eq!(calls_to(&e, 0x006d_3ae0), vec![vec![this, 40f32.to_bits()]]);
        assert_eq!(calls_to(&e, LOCK_DATA_ASSIGN).len(), 1);
        assert_eq!(e.mem.u8(this + 0x9d), 1);
        assert_eq!(e.mem.u8(this + 0x9e), 0);
        // The process's slot +0x28 answers 0x101: its low byte goes in.
        assert_eq!(e.mem.u8(this + 0x98), 1);
        assert_eq!(calls_to(&e, 0x0300_2000), vec![vec![actor, 0]]);
        assert_eq!(e.mem.f32(this + 0x90), 0.75);
        assert_eq!(e.mem.u8(this + 0xa0), 1);
    }

    #[test]
    fn fn_006e29f0_skips_the_door_flag_when_the_slot_says_no() {
        let mut e = Engine::new();
        e.map(0x0102_9000, 0x1000);
        e.set_global(RADIUS_FACTOR, 0.5f64);
        e.register(LOCATION_CONSTRUCT_FROM_ACTOR, |_, a| word(a[0]));
        e.register(LOCATION_ASSIGN, nothing);
        e.register(LOCATION_DESTRUCT, nothing);
        e.register(0x008b_e280, |_, _| float(80.0));
        e.register(0x006d_3ae0, nothing);
        e.register(LOCK_DATA_CONSTRUCT_FROM_ACTOR, |_, a| word(a[0]));
        e.register(LOCK_DATA_ASSIGN, nothing);
        e.register(0x0087_f350, |_, _| word(0));
        e.register(0x0087_f390, |_, _| word(1));
        let this = e.mem.alloc(0xb0);
        e.mem.set_u8(this + 0x98, 0x55);
        let actor = actor(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x006e_29f0, &args![this, actor]);
        assert!(calls_to(&e, 0x0041_81e0).is_empty());
        assert_eq!(e.mem.u8(this + 0x98), 0x55);
        assert_eq!(e.mem.u8(this + 0x9e), 1);
    }

    #[test]
    fn save_game_saves_in_the_exe_order() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.put_vtable(0x0300_0000, &[0x0300_1000]);
        e.mem.set_u32(this + 0xc, 0x0300_0000);
        e.mem.set_u32(this + 0x34, 0x0300_0000);
        e.register(0x0300_1000, nothing);
        e.register(SAVE_DATA, nothing);
        e.register(SAVE_VARIABLE_SIZED_VALUE, nothing);
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, 2);
        e.mem.set_u32(this + 0x94, array);
        e.register(WORD_AT, |e, a| word(e.mem.u32(a[0])));
        e.register(ARRAY_SIZE, |e, a| word(e.mem.u32(a[0])));
        e.register(ARRAY_ELEMENT, |_, a| word(0x8000 + a[1]));
        e.register(AVOID_NODE_SAVE, nothing);
        e.call_log = Some(vec![]);
        e.call(0x006e_3230, &args![this, 0x9000u32]);
        assert_eq!(
            calls_to(&e, 0x0300_1000),
            vec![vec![this + 0xc, 0x9000], vec![this + 0x34, 0x9000]]
        );
        let saves = calls_to(&e, SAVE_DATA);
        assert_eq!(saves.len(), 24);
        assert_eq!(saves[0], vec![0x9000, this + 0x68, 4, 0]);
        assert_eq!(saves[7], vec![0x9000, this + 0x99, 1, 0]);
        assert_eq!(saves[17], vec![0x9000, this + 0x7c, 0xc, 0]);
        assert_eq!(saves[23], vec![0x9000, this + 0xac, 1, 0]);
        assert_eq!(
            calls_to(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0x9000, 2]]
        );
        assert_eq!(
            calls_to(&e, AVOID_NODE_SAVE),
            vec![vec![0x8000, 0x9000], vec![0x8001, 0x9000]]
        );
        // Without an array the count is 0.
        e.mem.set_u32(this + 0x94, 0);
        e.call_log = Some(vec![]);
        e.call(0x006e_3230, &args![this, 0x9000u32]);
        assert_eq!(
            calls_to(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0x9000, 0]]
        );
        assert!(calls_to(&e, AVOID_NODE_SAVE).is_empty());
    }

    fn array_doubles(e: &mut Engine) {
        e.register(REF_OBJECT_CONSTRUCT, nothing);
        e.register(REF_OBJECT_DESTRUCT, nothing);
        e.register(ARRAY_RELEASE_008454F0, nothing);
    }

    #[test]
    fn avoid_array_constructor_sets_both_vtables() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 8, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_3960, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), AVOID_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), AVOID_ARRAY_SECOND_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCT), vec![vec![this + 0x10]]);
    }

    #[test]
    fn derived_avoid_array_constructor_sets_its_vtables() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x006e_3850, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), AVOID_ARRAY_DERIVED_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), AVOID_ARRAY_DERIVED_SECOND_VTABLE);
    }

    #[test]
    fn avoid_array_destructor_body_releases_in_order() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x006e_39d0, &args![this]);
        assert_eq!(
            order(&e, &[ARRAY_RELEASE_008454F0, REF_OBJECT_DESTRUCT]),
            vec![
                (ARRAY_RELEASE_008454F0, vec![this, 1]),
                (REF_OBJECT_DESTRUCT, vec![this + 0x10]),
                (ARRAY_RELEASE_008454F0, vec![this, 1]),
            ]
        );
        // `006e3ab0` leaves the base array's vtable.
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_0106D218);
        assert_eq!(e.mem.u32(this + 0x10), AVOID_ARRAY_SECOND_VTABLE);
    }

    #[test]
    fn fn_006e38b0_runs_the_base_destructor_body() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x006e_38b0, &args![this]);
        assert_eq!(calls_to(&e, REF_OBJECT_DESTRUCT), vec![vec![this + 0x10]]);
    }

    #[test]
    fn fn_006e3880_frees_with_bit_zero() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        e.register(FREE_SIZED, nothing);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_3880, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, FREE_SIZED).is_empty());
        assert_eq!(e.call(0x006e_3880, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, FREE_SIZED), vec![vec![this, 0x18]]);
    }

    #[test]
    fn fn_006e3a80_sets_the_vtable_and_empties_the_array() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for i in 1..4 {
            e.mem.set_u32(this + 4 * i, 9);
        }
        assert_eq!(e.call(0x006e_3a80, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_0106D218);
        for i in 1..4 {
            assert_eq!(e.mem.u32(this + 4 * i), 0);
        }
    }

    #[test]
    fn allocate_asks_for_0x24_bytes_a_node() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, a| word(a[0] + 0x10_0000));
        assert_eq!(e.call(0x006e_3c10, &args![0u32, 3u32]).u32(), 0x10_006c);
    }

    #[test]
    fn fn_006e3de0_allocates_and_constructs() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0, 0x0300_1000]);
        e.mem.set_u32(this, 0x0300_0000);
        e.register(0x0300_1000, |_, a| word(0x7000 + a[1]));
        e.register(AVOID_NODES_CONSTRUCT, nothing);
        e.call_log = Some(vec![]);
        e.call(0x006e_3de0, &args![this, 2u32, 5u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7005);
        assert_eq!(e.mem.u32(this + 8), 5);
        assert_eq!(e.mem.u32(this + 0xc), 5);
        assert_eq!(
            calls_to(&e, AVOID_NODES_CONSTRUCT),
            vec![vec![this, 0x7005, 5]]
        );
        e.call(0x006e_3de0, &args![this, 4u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7004);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        assert_eq!(calls_to(&e, AVOID_NODES_CONSTRUCT).len(), 1);
    }
}
