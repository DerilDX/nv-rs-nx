//! `NiParticle/nipsysplanarcollider.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c32680`: the jump target of `00c28d00`.
const JUMP_TARGET_00C28D00: u32 = 0x00c3_2680;

// Translated from 00c28d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c32680` (engine map: `NiPSysCollider::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysPlanarCollider@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_planar_collider_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C28D00, &args![this, stream]).bool()
}

// Translated from 00c299d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_26c8.
pub fn ni_p_sys_planar_collider_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_26c8
}

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// Import slot of `InterlockedDecrement`.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The `NiPSysCollider` functions this unit's code calls (unit
/// `nipsyscollider.cpp`).
const COLLIDER_LINK_OBJECT: u32 = 0x00c3_2610;
const COLLIDER_PROCESS_CLONE: u32 = 0x00c3_2580;
const COLLIDER_LOAD_BINARY: u32 = 0x00c3_2840;
const COLLIDER_COPY_MEMBERS: u32 = 0x00c3_2510;
const COLLIDER_CONSTRUCT: u32 = 0x00c3_2780;
/// `NiStream::GetObjectFromLinkID` (no words).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a645b0` (unit `nistream.cpp`, no words): reads one link ID.
const READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiColor::LoadBinary` (engine map; one word: the stream): three floats.
const THREE_FLOATS_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `00a53c30` (unnamed, on the cloning process's map; two words: the key, the
/// result slot): the clone of an object, `bool` in `AL`.
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiPlane::NiPlane_ov2` (two words: the normal, the constant).
const PLANE_CONSTRUCT: u32 = 0x00a6_9970;
/// `NiTransform::MakeIdentity` (no words).
const TRANSFORM_MAKE_IDENTITY: u32 = 0x00a8_6bf0;
/// `00a5d3d0` (unit `niobject.cpp`, no words): the `NiObject` destructor.
const OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// The `NiPSysPlanarCollider` vtable.
const PLANAR_COLLIDER_VTABLE: u32 = 0x010b_eb0c;
/// `0.5` (`float`), the constructor's two half-squared sizes.
const HALF_FLOAT: u32 = 0x0101_6248;
/// The vector the constructor uses for the plane's normal and the position
/// (`011f426c`), and the nine words it copies as the rotation (`011a9448`).
const ORIGIN: u32 = 0x011f_426c;
const IDENTITY_ROTATION: u32 = 0x011a_9448;
/// The two axes `CreateClone` and the creation function pass (`011a946c`,
/// `011a9478`, three words each).
const FIRST_AXIS: u32 = 0x011a_946c;
const SECOND_AXIS: u32 = 0x011a_9478;
/// The size of a `NiPSysPlanarCollider` on PC.
const PLANAR_COLLIDER_SIZE: u32 = 0x100;

// Translated from 00c27ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::LinkObject` (Xbox PDB): the collider part
/// (`00c32610`), then the linked object becomes `m_pkColliderObj` (`+0x2C`).
pub fn ni_p_sys_planar_collider_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLLIDER_LINK_OBJECT, &args![this, stream]);
    let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.mem.set_u32(this.addr() + 0x2c, object);
}

// Translated from 00c29450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::ProcessClone` (Xbox PDB): the collider part, then
/// this collider's clone is looked up (`00a53c30`, the result slot is the
/// argument slot, set to 0 first) and, when there is a collider object, the
/// clone's becomes that object's clone, or the object itself when the map
/// has none.
pub fn ni_p_sys_planar_collider_process_clone(e: &mut Engine, this: Ptr, process: Ptr) {
    e.call(COLLIDER_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let object = e.mem.u32(this.addr() + 0x2c);
        let clone = e.mem.u32(slot.addr());
        if object == 0 {
            return;
        }
        let map = e.mem.u32(process.addr());
        let value = if e.call(CLONE_MAP_LOOKUP, &args![map, object, slot]).bool() {
            e.mem.u32(slot.addr())
        } else {
            e.mem.u32(this.addr() + 0x2c)
        };
        e.mem.set_u32(clone.wrapping_add(0x2c), value);
    });
}

// Translated from 00c29770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::LoadBinary` (Xbox PDB): the collider part, the
/// collider object's link ID (`00a645b0`), width and height (four bytes each,
/// through the stream's input file `+0x24c`: its read function `+8`, `cdecl
/// (file, dest, size, &count, 1)`, the result added to its position `+4`;
/// the count word is the argument slot), then the two axes (`00a59be0`).
pub fn ni_p_sys_planar_collider_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLLIDER_LOAD_BINARY, &args![this, stream]);
    e.call(READ_LINK_ID, &args![stream]);
    e.with_stack(4, |e, count| {
        for offset in [0x30u32, 0x34] {
            let file = e.mem.u32(stream.addr() + 0x24c);
            let read = e.mem.u32(file + 8);
            e.mem.set_u32(count.addr(), 4);
            let got = e
                .call(read, &args![file, this.addr() + offset, 4u32, count, 1u32])
                .u32();
            let position = e.mem.u32(file + 4);
            e.mem.set_u32(file + 4, position.wrapping_add(got));
        }
    });
    e.call(THREE_FLOATS_LOAD_BINARY, &args![this.addr() + 0x38, stream]);
    e.call(THREE_FLOATS_LOAD_BINARY, &args![this.addr() + 0x44, stream]);
}

// Translated from 00c29900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::NiPSysPlanarCollider` (Xbox PDB; `thiscall`, 13
/// words): the collider constructor (`00c32780`) with the first four (the
/// bounce through `FLD`/`FSTP`), then the collider object (`+0x2C`), width
/// and height (`FLD`/`FSTP`), the two axes (`+0x38`, `+0x44`), both
/// half-squared sizes 0.5, the vtable, the plane (`NiPlane` from `011f426c`
/// and 0), the position from `011f426c`, the rotation from `011a9448`, and
/// two identity transforms (`+0x98`, `+0xCC`). Answers `this`.
pub fn ni_p_sys_planar_collider_ni_p_sys_planar_collider(
    e: &mut Engine,
    this: Ptr,
    bounce: f32,
    spawn_on_collide: u32,
    die_on_collide: u32,
    spawn_modifier: u32,
    collider_object: u32,
    width: f32,
    height: f32,
    x_axis_x: u32,
    x_axis_y: u32,
    x_axis_z: u32,
    y_axis_x: u32,
    y_axis_y: u32,
    y_axis_z: u32,
) -> Ptr {
    let a = this.addr();
    let bounce = x87_quiet_bits(bounce.to_bits());
    e.call(
        COLLIDER_CONSTRUCT,
        &args![
            this,
            bounce,
            spawn_on_collide,
            die_on_collide,
            spawn_modifier
        ],
    );
    e.mem.set_u32(a + 0x30, x87_quiet_bits(width.to_bits()));
    e.mem.set_u32(a + 0x34, x87_quiet_bits(height.to_bits()));
    e.mem.set_u32(a + 0x38, x_axis_x);
    let half = x87_quiet_bits(e.global::<u32>(HALF_FLOAT));
    e.mem.set_u32(a + 0x50, half);
    e.mem.set_u32(a + 0x54, half);
    e.mem.set_u32(a + 0x2c, collider_object);
    e.mem.set_u32(a + 0x3c, x_axis_y);
    e.mem.set_u32(a + 0x44, y_axis_x);
    e.mem.set_u32(a + 0x40, x_axis_z);
    e.mem.set_u32(a + 0x48, y_axis_y);
    e.mem.set_u32(a, PLANAR_COLLIDER_VTABLE);
    e.mem.set_u32(a + 0x4c, y_axis_z);
    e.call(PLANE_CONSTRUCT, &args![a + 0x58, ORIGIN, 0.0f32]);
    for i in 0..3 {
        let word = e.mem.u32(ORIGIN + 4 * i);
        e.mem.set_u32(a + 0x68 + 4 * i, word);
    }
    for i in 0..9 {
        let word = e.mem.u32(IDENTITY_ROTATION + 4 * i);
        e.mem.set_u32(a + 0x74 + 4 * i, word);
    }
    e.call(TRANSFORM_MAKE_IDENTITY, &args![a + 0x98]);
    e.call(TRANSFORM_MAKE_IDENTITY, &args![a + 0xcc]);
    this
}

// Translated from 00c299e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::_scalar_deleting_destructor_` (Xbox PDB): releases
/// `m_spNext` (`+0x28`), the `NiObject` destructor (`00a5d3d0`), then the sized
/// delete of 0x100 bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_p_sys_planar_collider_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let next = e.mem.u32(this.addr() + 0x28);
    if next != 0 && e.call(INTERLOCKED_DECREMENT, &args![next + 4]).u32() == 0 {
        e.vcall(next, 4, &[]);
    }
    e.call(OBJECT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, PLANAR_COLLIDER_SIZE]);
    }
    this
}

/// The planar collider `00c29a30` and `00c29af0` construct in `block`: bounce
/// 1, no flags, spawn modifier or object, width and height 1, the axes
/// `011a946c` and `011a9478`.
fn construct_default(e: &mut Engine, block: u32) -> Ptr {
    let one = 1.0f32;
    let word = |e: &Engine, addr: u32| e.mem.u32(addr);
    let axes = [
        word(e, FIRST_AXIS),
        word(e, FIRST_AXIS + 4),
        word(e, FIRST_AXIS + 8),
        word(e, SECOND_AXIS),
        word(e, SECOND_AXIS + 4),
        word(e, SECOND_AXIS + 8),
    ];
    ni_p_sys_planar_collider_ni_p_sys_planar_collider(
        e,
        Ptr::new(block),
        one,
        0,
        0,
        0,
        0,
        one,
        one,
        axes[0],
        axes[1],
        axes[2],
        axes[3],
        axes[4],
        axes[5],
    )
}

// Translated from 00c29a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::CreateClone` (Xbox PDB): a default planar collider
/// (null when the allocation fails, still used as the exe does), the
/// collider members copied (`00c32510`), width and height copied with
/// `FLD`/`FSTP`, the axes word by word. Answers the clone.
pub fn ni_p_sys_planar_collider_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![PLANAR_COLLIDER_SIZE]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        construct_default(e, block)
    };
    e.call(COLLIDER_COPY_MEMBERS, &args![this, clone, process]);
    let a = this.addr();
    let c = clone.addr();
    for offset in [0x30u32, 0x34] {
        let value = x87_quiet_bits(e.mem.u32(a + offset));
        e.mem.set_u32(c.wrapping_add(offset), value);
    }
    for offset in (0x38u32..0x50).step_by(4) {
        let word = e.mem.u32(a + offset);
        e.mem.set_u32(c.wrapping_add(offset), word);
    }
    clone
}

// Translated from 00c29af0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's creation function (Ghidra made no function of it; listed for
/// `NiPSysPlanarCollider` in `00c1aa00`'s loader table; `cdecl`, no words):
/// 0x100 bytes constructed as the default planar collider (the
/// constructor's result is answered), or null.
pub fn fn_00c29af0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![PLANAR_COLLIDER_SIZE]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct_default(e, block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2_7ac0, ni_p_sys_planar_collider_link_object(Ptr, Ptr)),
        entry!(
            0x00c2_9450,
            ni_p_sys_planar_collider_process_clone(Ptr, Ptr)
        ),
        entry!(0x00c2_9770, ni_p_sys_planar_collider_load_binary(Ptr, Ptr)),
        entry!(
            0x00c2_9900,
            ni_p_sys_planar_collider_ni_p_sys_planar_collider(
                Ptr, f32, u32, u32, u32, u32, f32, f32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(
            0x00c2_99e0,
            ni_p_sys_planar_collider_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c2_9a30, ni_p_sys_planar_collider_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c2_9af0, fn_00c29af0() -> Ptr),
        entry!(0x00c299d0, ni_p_sys_planar_collider_get_r_t_t_i() -> u32),
        entry!(0x00c2_8d00, ni_p_sys_planar_collider_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c28d00_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_8d00;
        const TARGET: u32 = 0x00c3_2680;
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

    #[test]
    fn ni_p_sys_planar_collider_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_99d0, &args![]).u32(), 0x0120_26c8);
    }
}

#[cfg(test)]
mod planar_tests {
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

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn link_object_takes_the_collider_object() {
        let mut e = Engine::new();
        e.register(COLLIDER_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| word(0x4321));
        let this = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        e.call(0x00c2_7ac0, &args![this, 0x6000u32]);
        assert_eq!(calls_to(&e, COLLIDER_LINK_OBJECT), vec![vec![this, 0x6000]]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x4321);
    }

    #[test]
    fn process_clone_maps_the_collider_object() {
        let mut e = Engine::new();
        e.register(COLLIDER_PROCESS_CLONE, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        let clone = e.mem.alloc(0x100);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            let found = match a[1] {
                k if k == this => clone,
                0x4440 => 0x4441,
                _ => return word(0),
            };
            e.mem.set_u32(a[2], found);
            word(1)
        });
        let process = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        // No object: one lookup only.
        e.call(0x00c2_9450, &args![this, process]);
        assert_eq!(calls_to(&e, CLONE_MAP_LOOKUP).len(), 1);
        e.mem.set_u32(this + 0x2c, 0x4440);
        e.call(0x00c2_9450, &args![this, process]);
        assert_eq!(e.mem.u32(clone + 0x2c), 0x4441);
        e.mem.set_u32(this + 0x2c, 0x5550);
        e.call(0x00c2_9450, &args![this, process]);
        assert_eq!(e.mem.u32(clone + 0x2c), 0x5550);
    }

    #[test]
    fn load_binary_reads_size_and_axes() {
        const READ: u32 = 0x0100_0010;
        let mut e = Engine::new();
        e.register(COLLIDER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        e.register(THREE_FLOATS_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            e.mem.set_f32(a[1], 2.0);
            word(4)
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        e.call(0x00c2_9770, &args![this, stream]);
        assert_eq!(e.mem.f32(this + 0x30), 2.0);
        assert_eq!(e.mem.f32(this + 0x34), 2.0);
        assert_eq!(
            calls_to(&e, THREE_FLOATS_LOAD_BINARY),
            vec![vec![this + 0x38, stream], vec![this + 0x44, stream]]
        );
        assert_eq!(calls_to(&e, READ_LINK_ID), vec![vec![stream]]);
    }

    fn constructor_doubles(e: &mut Engine) {
        e.register(COLLIDER_CONSTRUCT, |_, a| word(a[0]));
        e.register(PLANE_CONSTRUCT, |_, _| Ret::default());
        e.register(TRANSFORM_MAKE_IDENTITY, |_, _| Ret::default());
        for addr in [
            HALF_FLOAT,
            ORIGIN,
            IDENTITY_ROTATION,
            FIRST_AXIS,
            SECOND_AXIS,
        ] {
            e.map(addr, 0x24);
        }
        e.mem.set_f32(HALF_FLOAT, 0.5);
        e.mem.set_f32(IDENTITY_ROTATION, 1.0);
        e.mem.set_f32(FIRST_AXIS, 1.0);
        e.mem.set_f32(SECOND_AXIS + 4, 1.0);
    }

    #[test]
    fn constructor_lays_out_the_planar_collider() {
        let mut e = Engine::new();
        constructor_doubles(&mut e);
        let this = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        let words = args![
            this,
            0x7f80_0001u32,
            1u32,
            0u32,
            0x44u32,
            0x55u32,
            2.0f32,
            3.0f32,
            1u32,
            2u32,
            3u32,
            4u32,
            5u32,
            6u32
        ];
        assert_eq!(e.call(0x00c2_9900, &words).u32(), this);
        assert_eq!(
            calls_to(&e, COLLIDER_CONSTRUCT),
            vec![vec![this, 0x7fc0_0001, 1, 0, 0x44]]
        );
        assert_eq!(
            calls_to(&e, PLANE_CONSTRUCT),
            vec![vec![this + 0x58, ORIGIN, 0]]
        );
        assert_eq!(
            calls_to(&e, TRANSFORM_MAKE_IDENTITY),
            vec![vec![this + 0x98], vec![this + 0xcc]]
        );
        assert_eq!(e.mem.u32(this), PLANAR_COLLIDER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x2c), 0x55);
        assert_eq!(e.mem.f32(this + 0x30), 2.0);
        assert_eq!(e.mem.f32(this + 0x34), 3.0);
        let axes: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 0x38 + 4 * i)).collect();
        assert_eq!(axes, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(e.mem.f32(this + 0x50), 0.5);
        assert_eq!(e.mem.f32(this + 0x54), 0.5);
        assert_eq!(e.mem.f32(this + 0x74), 1.0);
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_next_collider() {
        const DELETE_THIS: u32 = 0x0100_0004;
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.register(OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let next = e.mem.alloc(8);
        e.mem.set_u32(next, vtable);
        e.mem.set_u32(next + 4, 1);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x28, next);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_99e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![next]]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE), vec![vec![this, 0x100]]);
        e.mem.set_u32(this + 0x28, 0);
        e.call(0x00c2_99e0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE).len(), 1);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn create_clone_and_create_function_build_the_default_collider() {
        let mut e = Engine::new();
        constructor_doubles(&mut e);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(COLLIDER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x30, 0x7f80_0003);
        e.mem.set_u32(this + 0x4c, 0x99);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_9a30, &args![this, 0x7000u32]).u32();
        assert_eq!(
            calls_to(&e, COLLIDER_CONSTRUCT),
            vec![vec![clone, 0x3f80_0000, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, COLLIDER_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        assert_eq!(e.mem.u32(clone + 0x30), 0x7fc0_0003);
        assert_eq!(e.mem.u32(clone + 0x4c), 0x99);
        let made = e.call(0x00c2_9af0, &args![]).u32();
        assert_eq!(e.mem.u32(made), PLANAR_COLLIDER_VTABLE);
        assert_eq!(e.mem.f32(made + 0x38), 1.0);
        assert_eq!(e.mem.f32(made + 0x48), 1.0);
        assert_eq!(e.mem.f32(made + 0x34), 1.0);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_9af0, &args![]).u32(), 0);
    }
}
