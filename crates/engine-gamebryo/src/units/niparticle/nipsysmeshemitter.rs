//! `NiParticle/nipsysmeshemitter.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPSysMeshEmitter::ms_RTTI` (the class's `NiRTTI` record).
const NI_P_SYS_MESH_EMITTER_RTTI: u32 = 0x0120_2500;

// Translated from 00c1f4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshEmitter::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_p_sys_mesh_emitter_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_P_SYS_MESH_EMITTER_RTTI)
}

/// The memory manager object (`ECX` of `00aa3e40`, the allocator).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The element constructor `00b54620` handed to the vector constructor loop.
const ELEMENT_CONSTRUCTOR: u32 = 0x00b5_4620;

// Translated from 00c1d790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one stack word): allocates an array of
/// `count` four-byte elements with a one-word count in front, the shape of an
/// `operator new[]` for a class with a constructor. The size `count * 4 + 4`
/// saturates to `0xffffffff` on overflow (the compiler's `SETO`/`SETC` idiom).
/// Allocates through `00aa3e40` on the memory manager; on failure answers null.
/// Otherwise it stores `count` in the first word, runs the vector constructor
/// loop `00401050` (`stdcall`: block, element size 4, count, constructor
/// `00b54620`) over the elements and answers the address of the first element.
pub fn fn_00c1d790(e: &mut Engine, count: u32) -> Ptr {
    let bytes: u32 = (u64::from(count) * 4).try_into().unwrap_or(u32::MAX);
    let total = bytes.saturating_add(4);
    let block = e.call(0x00aa_3e40, &args![MEMORY_MANAGER, total]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.mem.set_u32(block, count);
    let elements = block + 4;
    e.call(
        0x0040_1050,
        &args![elements, 4u32, count, ELEMENT_CONSTRUCTOR],
    );
    Ptr::new(elements)
}

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `00a47600` (`cdecl`, one word): frees an array block.
const NI_FREE: u32 = 0x00a4_7600;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `_rand` (`00ecadb8`, `cdecl`, no words).
const RAND: u32 = 0x00ec_adb8;
/// `_atexit` (`00ec658f`, `cdecl`, one word).
const ATEXIT: u32 = 0x00ec_658f;
/// `_CIsqrt` (`00ec6040`): the square root of ST0, in ST0.
const SQRT: u32 = 0x00ec_6040;
/// `32767.0` (`double`).
const RAND_DIVISOR: u32 = 0x0102_9790;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `1.5` (`double`).
const ONE_AND_HALF: u32 = 0x0101_6ff0;
/// `0x3fd5555560000000` (`double`), the float nearest one third.
const ONE_THIRD: u32 = 0x0109_9ee0;
/// `4294967296.0` (`float`): the compiler's unsigned-to-float correction.
const TWO_TO_THE_32: u32 = 0x0109_9378;
/// `NiRefObject::ms_uiObjects` (`011f4400`): the live object count.
const OBJECT_COUNT: u32 = 0x011f_4400;
/// The `NiRefObject` vtable (`0101dce4`) and the `NiSkinnedEmitterData` one.
const REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const SKINNED_EMITTER_DATA_VTABLE: u32 = 0x010b_d68c;
/// The NiPSysMeshEmitter vtable and its two arrays' vtables (the
/// constructor stores `010bd698` first, then `010bd6a8`; the destructor
/// stores `010bd6a0` and `010bd698`).
const MESH_EMITTER_VTABLE: u32 = 0x010b_d7fc;
const ARRAY_BASE_VTABLE: u32 = 0x010b_d698;
const GEOMETRY_ARRAY_VTABLE: u32 = 0x010b_d6a8;
const SKINNED_ARRAY_VTABLE: u32 = 0x010b_d7f4;
const SKINNED_ARRAY_DESTROYED_VTABLE: u32 = 0x010b_d6a0;
/// The emit axis default the constructor copies (`011a946c`, three words).
const DEFAULT_EMIT_AXIS: u32 = 0x011a_946c;
/// The `NiRTTI` record `00c1eb20` looks for in the geometry's class chain.
const FALLBACK_RTTI: u32 = 0x011f_4a48;

/// Callees outside this unit (all named by address; `thiscall` unless said).
/// `NiTransform::InvertNonUniform` (one word: the result).
const INVERT_NON_UNIFORM: u32 = 0x004b_4880;
/// `0062c250` (unnamed; two words: the result, the right transform): a
/// transform product.
const TRANSFORM_PRODUCT: u32 = 0x0062_c250;
/// `00524c40` (unnamed; two words: the result, the point): a transform
/// applied to a point; returns the result.
const TRANSFORM_POINT: u32 = 0x0052_4c40;
/// `004b4500` (unnamed; two words: the result, the vector): a transform's
/// rotation applied to a vector; returns the result.
const ROTATE_VECTOR: u32 = 0x004b_4500;
/// `004a0c10` (unnamed, no words): normalizes the point (zero when its length
/// is at most 1e-6).
const NORMALIZE: u32 = 0x004a_0c10;
/// `00525340` (`cdecl`, one word): normalizes the point through the engine's
/// reciprocal square root table.
const NORMALIZE_FAST: u32 = 0x0052_5340;
/// `NiPSysEmitter::LoadBinary` (one word: the stream).
const EMITTER_LOAD_BINARY: u32 = 0x00c2_23b0;
/// `NiStream::ReadMultipleLinkIDs` (no words).
const READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
/// `NiColor::LoadBinary` (engine map; one word: the stream), here on the emit axis.
const THREE_FLOATS_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `NiPSysModifier::~NiPSysModifier`.
const MODIFIER_DESTRUCTOR: u32 = 0x00c3_1750;
/// `00c21f00` (unnamed, no words): the `NiPSysEmitter` part of the constructor.
const EMITTER_CONSTRUCT: u32 = 0x00c2_1f00;
/// `00c21f60` (unnamed; two words: the clone, the cloning process): the emitter
/// part of the member copy.
const EMITTER_COPY_MEMBERS: u32 = 0x00c2_1f60;
/// `NiPSysModifier::ProcessClone` (one word: the cloning process).
const MODIFIER_PROCESS_CLONE: u32 = 0x00c3_17e0;
/// `00a53c30` (unnamed, on the cloning process's map; two words: the key, the
/// result slot): the clone of an object, `bool` in `AL`.
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiPSysBoundUpdateModifier::LinkObject` (engine map; one word: the stream).
const MODIFIER_LINK_OBJECT: u32 = 0x00c2_d6d0;
/// `NiStream::GetNumberOfLinkIDs` and `NiStream::GetObjectFromLinkID` (no words).
const GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiTArray<NiPointer<NiControllerSequence>>::SetSize` (engine map; folded
/// code, here on the skinned-data array; one word).
const SKINNED_ARRAY_SET_SIZE: u32 = 0x00a4_7770;
/// `0066b0d0` (unnamed; one word): a `NiPointer` assignment, here of the skin
/// partition.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `00c1d170` (unnamed; one word: the geometry): fills a skinned-emitter data.
const SKINNED_DATA_UPDATE: u32 = 0x00c1_d170;
/// The emission functions `00c1f5f0` picks (all in this unit's source, not
/// yet translated): skinned ones take `(data, geometry, position, velocity)`,
/// the others `(geometry, position, velocity)`.
const EMIT_SKINNED_FACE: u32 = 0x00c1_e080;
const EMIT_SKINNED_EDGE: u32 = 0x00c1_e4e0;
const EMIT_SKINNED_VERTEX: u32 = 0x00c1_dec0;
const EMIT_EDGE: u32 = 0x00c1_dae0;
const EMIT_VERTEX: u32 = 0x00c1_d9e0;

layout! {
    /// `NiPSysMeshEmitter` (Xbox PDB), 0x88 bytes on PC.
    pub struct NiPSysMeshEmitter: 0x88 {
        /// `m_pkTarget` (Xbox PDB, `NiPSysModifier`): the particle system.
        0x10 m_pkTarget: u32,
        /// `m_fSpeed` and `m_fSpeedVar` (Xbox PDB, `NiPSysEmitter`), as bits.
        0x18 m_fSpeed: u32,
        0x1C m_fSpeedVar: u32,
        /// `m_kGeomEmitterArray` (Xbox PDB): an `NiTArray` of `NiPointer<NiGeometry>`.
        0x54 m_kGeomEmitterArray: u32,
        /// `m_kSkinnedEmitterData` (Xbox PDB): an `NiTArray` of `NiPointer<NiSkinnedEmitterData>`.
        0x64 m_kSkinnedEmitterData: u32,
        /// `m_eInitVelocityType` (Xbox PDB).
        0x74 m_eInitVelocityType: u32,
        /// `m_eEmissionType` (Xbox PDB).
        0x78 m_eEmissionType: u32,
        /// `m_kEmitAxis` (Xbox PDB): three floats.
        0x7C m_kEmitAxis: u32,
    }
}

/// Releases one reference of the `NiRefObject` at `object` (non-null):
/// `InterlockedDecrement` of its count, `DeleteThis` (vtable `+4`) at zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

/// Stores `value` in the `NiPointer` at `slot` when it differs: the old
/// object released, the new one's count incremented.
fn assign_pointer(e: &mut Engine, slot: u32, value: u32) {
    let old = e.mem.u32(slot);
    if old != value {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, value);
        if value != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
        }
    }
}

fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// `rand() / 32767.0` stored as a float.
fn random_fraction(e: &mut Engine) -> u32 {
    let value = e.call(RAND, &[]).i32();
    let divisor: f64 = e.global(RAND_DIVISOR);
    x87_store32(x87_div(f64::from(value), divisor))
}

/// `FLD dword` of the float at `addr`.
fn load_float(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// The emitter's speed for one particle: `(fraction - 0.5) * m_fSpeedVar +
/// m_fSpeed` from a stored `rand()` fraction, as the float the exe stores.
fn random_speed(e: &mut Engine, this: Ptr<NiPSysMeshEmitter>) -> u32 {
    let fraction = random_fraction(e);
    let half: f64 = e.global(HALF);
    let variation = x87_m32(e.get(this, NiPSysMeshEmitter::m_fSpeedVar));
    let speed = x87_m32(e.get(this, NiPSysMeshEmitter::m_fSpeed));
    x87_store32(x87_add(
        x87_mul(x87_sub(x87_load32(fraction), half), variation),
        speed,
    ))
}

// Translated from 00c1ced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one float word, result in ST0): the
/// square root of `value` by one Newton step of the `0x5f3759df` reciprocal
/// square root estimate: `y = bits(0x5f3759df - (bits(x) >> 1))`, then
/// `x * (y * (1.5 - (0.5 * x) * y * y))`, each step stored as a float.
pub fn fn_00c1ced0(e: &mut Engine, value: f32) -> f32 {
    let x = x87_load32(value.to_bits());
    // `FST` back into the argument slot: the integer trick reads those bits.
    let stored = x87_store32(x);
    let half_x = x87_store32(x87_mul(x87_load64(e.global::<u64>(HALF)), x));
    let guess = 0x5f37_59dfu32.wrapping_sub(((stored as i32) >> 1) as u32);
    let y = x87_load32(guess);
    let half_x_y_y = x87_mul(x87_mul(x87_load32(half_x), y), y);
    let step = x87_sub(x87_load64(e.global::<u64>(ONE_AND_HALF)), half_x_y_y);
    let reciprocal = x87_store32(x87_mul(step, y));
    let result = x87_store32(x87_mul(x, x87_m32(reciprocal)));
    f32::from_bits(x87_quiet_bits(result))
}

// Translated from 00c1cf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`, no words): the `NiPSysMeshEmitter::
/// NiSkinnedEmitterData` constructor (it stores that class's vtable, the
/// one its scalar deleting destructor `00c1d660` runs from): the
/// `NiRefObject` part (vtable `0101dce4`, count 0, `ms_uiObjects`
/// incremented), then the vtable and a null skin partition (`+8`).
pub fn fn_00c1cf20(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, REF_OBJECT_VTABLE);
    e.mem.set_u32(at + 4, 0);
    e.call(INTERLOCKED_INCREMENT, &args![OBJECT_COUNT]);
    e.mem.set_u32(at, SKINNED_EMITTER_DATA_VTABLE);
    e.mem.set_u32(at + 8, 0);
    this
}

// Translated from 00c1cf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words: geometry, position,
/// velocity): with a geometry, takes its world transform (`+0x68`, 13 words)
/// into the particle system's space (the system's own world transform
/// inverted, `004b4880`, times the geometry's, `0062c250`), applies it to
/// `position` (`00524c40`) and sets `velocity` by `m_eInitVelocityType`:
/// 0 rotates it by the transform (`004b4500`); 1 makes it a random direction
/// (three `2 * rand() / 32767 - 1` components, normalized by `004a0c10`)
/// times the random speed; 2 is the emit axis rotated and normalized, times
/// the random speed; any other value leaves it. The locals sit at the exe's
/// frame offsets (one block for the frame).
pub fn fn_00c1cf50(
    e: &mut Engine,
    this: Ptr<NiPSysMeshEmitter>,
    geometry: Ptr,
    position: Ptr,
    velocity: Ptr,
) {
    if geometry.is_null() {
        return;
    }
    e.with_stack(0xc4, |e, frame| {
        let f = frame.addr();
        let geometry_world = f + 0x5c;
        let world = f + 0x28;
        let inverse = f + 0x90;
        copy_words(e, geometry.addr() + 0x68, geometry_world, 13);
        let target = e.get(this, NiPSysMeshEmitter::m_pkTarget);
        copy_words(e, target + 0x68, world, 13);
        e.call(INVERT_NON_UNIFORM, &args![world, inverse]);
        e.call(TRANSFORM_PRODUCT, &args![inverse, world, geometry_world]);
        let moved = e
            .call(TRANSFORM_POINT, &args![world, f + 0x1c, position])
            .u32();
        copy_words(e, moved, position.addr(), 3);
        let v = velocity.addr();
        match e.get(this, NiPSysMeshEmitter::m_eInitVelocityType) {
            0 => {
                let rotated = e
                    .call(ROTATE_VECTOR, &args![world, f + 0x10, velocity])
                    .u32();
                copy_words(e, rotated, v, 3);
            }
            1 => {
                let divisor: f64 = e.global(RAND_DIVISOR);
                let one: f64 = e.global(ONE);
                for i in 0..3 {
                    let value = f64::from(e.call(RAND, &[]).i32());
                    let component =
                        x87_store32(x87_sub(x87_div(x87_add(value, value), divisor), one));
                    e.mem.set_u32(v + 4 * i, component);
                }
                e.call(NORMALIZE, &args![velocity]);
                let speed = x87_load32(random_speed(e, this));
                let x = x87_store32(x87_mul(load_float(e, v), speed));
                e.mem.set_u32(v, x);
                let y = x87_store32(x87_mul(speed, x87_m32(e.mem.u32(v + 4))));
                e.mem.set_u32(v + 4, y);
                let z = x87_store32(x87_mul(speed, x87_m32(e.mem.u32(v + 8))));
                e.mem.set_u32(v + 8, z);
            }
            2 => {
                let axis = this.addr() + 0x7c;
                e.call(ROTATE_VECTOR, &args![world, f + 0x1c, axis]);
                let speed_bits = random_speed(e, this);
                e.call(NORMALIZE, &args![f + 0x1c]);
                let speed = x87_load32(speed_bits);
                let x = x87_store32(x87_mul(load_float(e, f + 0x1c), speed));
                e.mem.set_u32(v, x);
                let y = x87_store32(x87_mul(load_float(e, f + 0x20), speed));
                e.mem.set_u32(v + 4, y);
                let z = x87_store32(x87_mul(speed, x87_m32(e.mem.u32(f + 0x24))));
                e.mem.set_u32(v + 8, z);
            }
            _ => {}
        }
    });
}

// Translated from 00c1d660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshEmitter::NiSkinnedEmitterData::_scalar_deleting_destructor_`
/// (Xbox PDB): releases the skin partition (`+8`), stores the `NiRefObject`
/// vtable, decrements `ms_uiObjects`, and frees the 0xC bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_skinned_emitter_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let partition = e.mem.u32(this.addr() + 8);
    if partition != 0 {
        release(e, partition);
    }
    e.mem.set_u32(this.addr(), REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![OBJECT_COUNT]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0xcu32]);
    }
    this
}

// Translated from 00c1d740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): `operator new[]` for
/// `count` four-byte elements with a count word in front, as `fn_00c1d790`
/// but with the element constructor `006694e0`; the size `count * 4 + 4`
/// saturates to `0xffffffff`. Allocates through `00aa3e40` on the memory
/// manager; null on failure, else the first element.
pub fn fn_00c1d740(e: &mut Engine, count: u32) -> Ptr {
    let bytes: u32 = (u64::from(count) * 4).try_into().unwrap_or(u32::MAX);
    let total = bytes.saturating_add(4);
    let block = e.call(0x00aa_3e40, &args![MEMORY_MANAGER, total]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.mem.set_u32(block, count);
    let elements = block + 4;
    e.call(0x0040_1050, &args![elements, 4u32, count, 0x0066_94e0u32]);
    Ptr::new(elements)
}

/// The shared body of `00c1d7e0` and `00c1d8b0` (`NiTArray<NiPointer<..>>::
/// SetAt` for the two arrays): the function-local static null pointer at
/// `default` (guard bit 0 of `guard`, its `atexit` destructor `exit_handler`)
/// is made on first use; then the size and the effective size are kept
/// (`index` past the size grows the size to `index + 1`) and the element
/// assigned with reference counting.
fn set_at(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    value: Ptr,
    guard: u32,
    default: u32,
    exit_handler: u32,
) {
    if e.global::<u8>(guard) & 1 == 0 {
        let flags: u32 = e.global(guard);
        e.set_global(guard, flags | 1);
        e.set_global(default, 0u32);
        e.call(ATEXIT, &args![exit_handler]);
    }
    let a = this.addr();
    let size = u32::from(e.mem.u16(a + 0xa));
    let effective = e.mem.u16(a + 0xc);
    if index >= size {
        e.mem.set_u16(a + 0xa, index.wrapping_add(1) as u16);
        if e.mem.u32(value.addr()) != e.global::<u32>(default) {
            e.mem.set_u16(a + 0xc, effective.wrapping_add(1));
        }
    } else {
        let null: u32 = e.global(default);
        let base = e.mem.u32(a + 4);
        let current = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        if e.mem.u32(value.addr()) == null {
            if current != null {
                e.mem.set_u16(a + 0xc, effective.wrapping_sub(1));
            }
        } else if current == null {
            e.mem.set_u16(a + 0xc, effective.wrapping_add(1));
        }
    }
    let base = e.mem.u32(a + 4);
    let slot = base.wrapping_add(index.wrapping_mul(4));
    let new = e.mem.u32(value.addr());
    assign_pointer(e, slot, new);
}

// Translated from 00c1d7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the geometry array, two words:
/// the index, a pointer to the new `NiPointer`): `SetAt` with the null
/// pointer kept at `012024f0` (guard `012024f4`, `atexit` `00fdd3e0`).
pub fn fn_00c1d7e0(e: &mut Engine, this: Ptr, index: u32, value: Ptr) {
    set_at(e, this, index, value, 0x0120_24f4, 0x0120_24f0, 0x00fd_d3e0);
}

// Translated from 00c1d8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the skinned-data array, two
/// words): the same `SetAt` with its own null pointer at `012024f8` (guard
/// `012024fc`, `atexit` `00fdd3b0`).
pub fn fn_00c1d8b0(e: &mut Engine, this: Ptr, index: u32, value: Ptr) {
    set_at(e, this, index, value, 0x0120_24fc, 0x0120_24f8, 0x00fd_d3b0);
}

// Translated from 00c1e850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshEmitter::LoadBinary` (Xbox PDB): the emitter part (`00c223b0`),
/// the geometry link IDs (`NiStream::ReadMultipleLinkIDs`), then the initial
/// velocity type and the emission type, four bytes each, read into one local
/// word (which starts as `this`, the `ECX` the exe pushed for it) and stored
/// at `+0x74` and `+0x78`, then the emit axis (`00a59be0`). The count word is
/// the function's own argument slot.
pub fn ni_p_sys_mesh_emitter_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysMeshEmitter>,
    stream: Ptr,
) {
    e.call(EMITTER_LOAD_BINARY, &args![this, stream]);
    e.call(READ_MULTIPLE_LINK_IDS, &args![stream]);
    e.with_stack(8, |e, frame| {
        let local = frame;
        let count: Ptr = Ptr::new(frame.addr() + 4);
        e.mem.set_u32(local.addr(), this.addr());
        e.mem.set_u32(count.addr(), stream.addr());
        for field in [
            NiPSysMeshEmitter::m_eInitVelocityType,
            NiPSysMeshEmitter::m_eEmissionType,
        ] {
            let file = e.mem.u32(stream.addr() + 0x24c);
            let read = e.mem.u32(file + 8);
            e.mem.set_u32(count.addr(), 4);
            let got = e.call(read, &args![file, local, 4u32, count, 1u32]).u32();
            let position = e.mem.u32(file + 4);
            e.mem.set_u32(file + 4, position.wrapping_add(got));
            let value = e.mem.u32(local.addr());
            e.set(this, field, value);
        }
    });
    e.call(THREE_FLOATS_LOAD_BINARY, &args![this.addr() + 0x7c, stream]);
}

// Translated from 00c1e9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiGeometry>>::SetSize` (Xbox PDB; `thiscall`, one
/// word): nothing when the capacity is already `max_size`; otherwise the
/// elements from `max_size` up to the size are released (each non-null one
/// also lowers the effective size) and the size cut, the capacity set, and a
/// new block (`00c1d740`, none for 0) gets the kept elements with reference
/// counting, the rest of it cleared; the old block is freed (`00a47600`).
pub fn fn_00c1e9a0(e: &mut Engine, this: Ptr, max_size: u32) {
    let a = this.addr();
    if max_size == u32::from(e.mem.u16(a + 8)) {
        return;
    }
    let size = e.mem.u16(a + 0xa);
    if max_size < u32::from(size) {
        let mut index = max_size as u16;
        if index < size {
            loop {
                let base = e.mem.u32(a + 4);
                let slot = base.wrapping_add(u32::from(index) * 4);
                if e.mem.u32(slot) != 0 {
                    let object = e.mem.u32(slot);
                    if object != 0 {
                        release(e, object);
                        e.mem.set_u32(slot, 0);
                    }
                    let effective = e.mem.u16(a + 0xc);
                    e.mem.set_u16(a + 0xc, effective.wrapping_sub(1));
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u16(a + 0xa) {
                    break;
                }
            }
        }
        e.mem.set_u16(a + 0xa, max_size as u16);
    }
    let old = e.mem.u32(a + 4);
    e.mem.set_u16(a + 8, max_size as u16);
    if max_size == 0 {
        e.mem.set_u32(a + 4, 0);
        e.call(NI_FREE, &args![old]);
        return;
    }
    let new = fn_00c1d740(e, u32::from(max_size as u16)).addr();
    e.mem.set_u32(a + 4, new);
    let mut i: u32 = 0;
    while (i as u16) < e.mem.u16(a + 0xa) {
        let offset = u32::from(i as u16) * 4;
        let target = e.mem.u32(a + 4).wrapping_add(offset);
        let value = e.mem.u32(old.wrapping_add(offset));
        assign_pointer(e, target, value);
        i = i.wrapping_add(1);
    }
    let mut index = e.mem.u16(a + 0xa);
    if index < e.mem.u16(a + 8) {
        loop {
            let slot = e.mem.u32(a + 4).wrapping_add(u32::from(index) * 4);
            let object = e.mem.u32(slot);
            if object != 0 {
                release(e, object);
                e.mem.set_u32(slot, 0);
            }
            index = index.wrapping_add(1);
            if index >= e.mem.u16(a + 8) {
                break;
            }
        }
    }
    e.call(NI_FREE, &args![old]);
}

/// The x87 helpers `00c1eb20` uses on stored floats: `FLD dword` and a
/// `float` memory operand.
fn ld(bits: u32) -> f64 {
    x87_load32(bits)
}
fn op(bits: u32) -> f64 {
    x87_m32(bits)
}

// Translated from 00c1eb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words: geometry, position,
/// velocity; `bool` in `AL`): emission from a random face of a
/// triangle-based geometry. Without a geometry: false. When the geometry is
/// not triangle based (vtable `+0x1C`), its class chain (`GetRTTI`, `+8`,
/// then each record's base at `+4`) is searched for the record `011f4a48`:
/// found, the answer of `00c1dae0(geometry, position, velocity)`; else false.
/// Otherwise a random triangle (`rand() % count`, the count from the data's
/// vtable `+0xA8`; a zero count is a division fault in the exe and a panic
/// here) is fetched (`+0xAC`, three `u16` corner slots) until its corners
/// differ, at most 6 tries, moving to the next triangle each time. No
/// vertices (data `+0x20`): false. The position is the corners' centroid;
/// with initial velocity type 0 and normals (data `+0x24`) the velocity
/// becomes the normalized (`00525340`) normal centroid times the velocity's
/// length (`_CIsqrt`); with emission type 3 the position is a random point
/// of the face (the square-root step `fn_00c1ced0` of one fraction, a second
/// fraction along the second edge). Then `fn_00c1cf50` and true. Each x87
/// instruction is one helper call, in the exe's order.
pub fn fn_00c1eb20(
    e: &mut Engine,
    this: Ptr<NiPSysMeshEmitter>,
    geometry: Ptr,
    position: Ptr,
    velocity: Ptr,
) -> bool {
    if geometry.is_null() {
        return false;
    }
    let g = geometry.addr();
    if e.vcall(g, 0x1c, &[]).u32() == 0 {
        let mut rtti = e.vcall(g, 8, &[]).u32();
        while rtti != 0 {
            if rtti == FALLBACK_RTTI {
                return e
                    .call(EMIT_EDGE, &args![this, geometry, position, velocity])
                    .bool();
            }
            rtti = e.mem.u32(rtti + 4);
        }
        return false;
    }
    let data = e.mem.u32(g + 0xb8);
    let count = u32::from(e.vcall(data, 0xa8, &[]).u16());
    let random = e.call(RAND, &[]).u32();
    let mut triangle = random % count;
    if triangle == count {
        // Never true (a remainder is below the divisor); the exe checks it.
        triangle = count - 1;
    }
    // The three corner slots are words of the exe's frame; the call writes
    // their low halves.
    let (a, b, c) = e.with_stack(0xc, |e, corners| {
        let slots = corners.addr();
        let mut attempt: i32 = 0;
        loop {
            let data = e.mem.u32(g + 0xb8);
            e.vcall(data, 0xac, &args![triangle, slots, slots + 4, slots + 8]);
            let a = e.mem.u16(slots);
            let b = e.mem.u16(slots + 4);
            let c = e.mem.u16(slots + 8);
            if a != b && a != c && b != c {
                break (a, b, c);
            }
            triangle += 1;
            if triangle >= count {
                triangle = 0;
            }
            attempt += 1;
            if attempt >= 6 {
                break (a, b, c);
            }
        }
    });
    let data = e.mem.u32(g + 0xb8);
    let vertices = e.mem.u32(data + 0x20);
    let normals = e.mem.u32(data + 0x24);
    if vertices == 0 {
        return false;
    }
    let (oa, ob, oc) = (u32::from(a) * 12, u32::from(b) * 12, u32::from(c) * 12);
    let corner = |e: &Engine, base: u32, offset: u32| -> [u32; 3] {
        let at = base.wrapping_add(offset);
        [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
    };
    let [ax, ay, az] = corner(e, vertices, oa);
    let [bx, by, bz] = corner(e, vertices, ob);
    let [cx, cy, cz] = corner(e, vertices, oc);
    let (st_ax, st_ay, st_az) = (ld(ax), ld(ay), ld(az));
    let sum_x = x87_store32(x87_add(ld(bx), st_ax));
    let sum_y = x87_store32(x87_add(st_ay, op(by)));
    let sum_z = x87_store32(x87_add(st_az, op(bz)));
    let total_x = x87_store32(x87_add(ld(sum_x), op(cx)));
    let total_y = x87_store32(x87_add(ld(cy), op(sum_y)));
    let total_z = x87_store32(x87_add(ld(cz), op(sum_z)));
    let third = x87_load64(e.global::<u64>(ONE_THIRD));
    let p = position.addr();
    e.mem.set_u32(p, x87_store32(x87_mul(ld(total_x), third)));
    e.mem
        .set_u32(p + 4, x87_store32(x87_mul(ld(total_y), third)));
    e.mem
        .set_u32(p + 8, x87_store32(x87_mul(ld(total_z), third)));
    if e.get(this, NiPSysMeshEmitter::m_eInitVelocityType) == 0 && normals != 0 {
        let [nax, nay, naz] = corner(e, normals, oa);
        let [nbx, nby, nbz] = corner(e, normals, ob);
        let [ncx, ncy, ncz] = corner(e, normals, oc);
        let u_x = x87_store32(x87_add(ld(nbx), op(nax)));
        let u_y = x87_store32(x87_add(ld(nay), op(nby)));
        let u_z = x87_store32(x87_add(ld(naz), op(nbz)));
        let w_x = x87_store32(x87_add(ld(u_x), op(ncx)));
        let w_y = x87_store32(x87_add(ld(ncy), op(u_y)));
        let w_z = x87_store32(x87_add(ld(ncz), op(u_z)));
        let normal = [
            x87_store32(x87_mul(ld(w_x), third)),
            x87_store32(x87_mul(ld(w_y), third)),
            x87_store32(x87_mul(third, op(w_z))),
        ];
        let [n_x, n_y, n_z] = e.with_stack(0xc, |e, n| {
            for (i, word) in normal.iter().enumerate() {
                e.mem.set_u32(n.addr() + 4 * i as u32, *word);
            }
            e.call(NORMALIZE_FAST, &args![n]);
            corner(e, n.addr(), 0)
        });
        let v = velocity.addr();
        let vy = load_float(e, v + 4);
        let vx = load_float(e, v);
        let vz = load_float(e, v + 8);
        let xx = x87_mul(vx, vx);
        let yy = x87_mul(vy, vy);
        let sum = x87_add(yy, xx);
        let zz = x87_mul(vz, vz);
        let squared = x87_store32(x87_add(sum, zz));
        let root = e.call(SQRT, &args![ld(squared)]).f64();
        let length = ld(x87_store32(ld(x87_store32(root))));
        e.mem.set_u32(v, x87_store32(x87_mul(ld(n_x), length)));
        e.mem.set_u32(v + 4, x87_store32(x87_mul(ld(n_y), length)));
        e.mem.set_u32(v + 8, x87_store32(x87_mul(length, op(n_z))));
    }
    if e.get(this, NiPSysMeshEmitter::m_eEmissionType) != 3 {
        fn_00c1cf50(e, this, geometry, position, velocity);
        return true;
    }
    let edge1_x = x87_store32(x87_sub(ld(bx), st_ax));
    let edge1_y = x87_store32(x87_sub(ld(by), st_ay));
    let edge1_z = x87_store32(x87_sub(ld(bz), st_az));
    let edge2_x = x87_store32(x87_sub(ld(cx), st_ax));
    let edge2_y = x87_store32(x87_sub(op(cy), st_ay));
    let edge2_z = x87_store32(x87_sub(op(cz), st_az));
    let first = random_fraction(e);
    let argument = f32::from_bits(x87_store32(ld(first)));
    let scale = x87_store32(f64::from(fn_00c1ced0(e, argument)));
    let along = ld(random_fraction(e));
    let q_x = x87_store32(x87_mul(ld(edge2_x), along));
    let q_y = x87_store32(x87_mul(ld(edge2_y), along));
    let q_z = x87_store32(x87_mul(along, op(edge2_z)));
    let d_x = x87_store32(x87_sub(ld(q_x), ld(edge1_x)));
    let d_y = x87_store32(x87_sub(ld(q_y), ld(edge1_y)));
    let d_z = x87_store32(x87_sub(ld(q_z), ld(edge1_z)));
    let s = ld(scale);
    let m_x = x87_store32(x87_mul(ld(d_x), s));
    let m_y = x87_store32(x87_mul(ld(d_y), s));
    let m_z = x87_store32(x87_mul(s, op(d_z)));
    let o_x = x87_store32(x87_add(ld(m_x), op(ax)));
    let o_y = x87_store32(x87_add(ld(m_y), op(ay)));
    let o_z = x87_store32(x87_add(ld(m_z), op(az)));
    e.mem.set_u32(p, x87_store32(x87_add(ld(edge1_x), ld(o_x))));
    e.mem
        .set_u32(p + 4, x87_store32(x87_add(ld(edge1_y), op(o_y))));
    e.mem
        .set_u32(p + 8, x87_store32(x87_add(ld(edge1_z), op(o_z))));
    fn_00c1cf50(e, this, geometry, position, velocity);
    true
}

/// The shared body of `00c1f360` and `00c1f420` (`NiTArray::AddFirstEmpty`
/// shape, `thiscall`, one word: a pointer to the `NiPointer` to add): -1 for
/// a null pointer; else the first null element below the size takes it (the
/// effective size raised) and its index is answered; with none, the array
/// grows by its grow-by step when full and the pointer goes in at the size
/// (`SetAt`).
fn add_first_empty(e: &mut Engine, this: Ptr, value: Ptr, skinned: bool) -> u32 {
    let wanted = e.mem.u32(value.addr());
    if wanted == 0 {
        return 0xffff_ffff;
    }
    let a = this.addr();
    let size = e.mem.u16(a + 0xa);
    let mut i: u32 = 0;
    if size > 0 {
        let base = e.mem.u32(a + 4);
        loop {
            let index = u32::from(i as u16);
            let slot = base.wrapping_add(index * 4);
            if e.mem.u32(slot) == 0 {
                let current = e.mem.u32(slot);
                if current != wanted {
                    if current != 0 {
                        release(e, current);
                    }
                    let new = e.mem.u32(value.addr());
                    e.mem.set_u32(slot, new);
                    if new != 0 {
                        e.call(INTERLOCKED_INCREMENT, &args![new + 4]);
                    }
                }
                let effective = e.mem.u16(a + 0xc);
                e.mem.set_u16(a + 0xc, effective.wrapping_add(1));
                return index;
            }
            i = i.wrapping_add(1);
            if (i as u16) >= e.mem.u16(a + 0xa) {
                break;
            }
        }
    }
    let size = u32::from(size);
    if size >= u32::from(e.mem.u16(a + 8)) {
        let grown = u32::from(e.mem.u16(a + 0xe)) + size;
        if skinned {
            e.call(SKINNED_ARRAY_SET_SIZE, &args![this, grown]);
        } else {
            fn_00c1e9a0(e, this, grown);
        }
    }
    if skinned {
        fn_00c1d8b0(e, this, size, value);
    } else {
        fn_00c1d7e0(e, this, size, value);
    }
    size
}

// Translated from 00c1f360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds a geometry pointer to the geometry array
/// (`add_first_empty`; grows with `00c1e9a0`, stores with `00c1d7e0`).
pub fn fn_00c1f360(e: &mut Engine, this: Ptr, value: Ptr) -> u32 {
    add_first_empty(e, this, value, false)
}

// Translated from 00c1f420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds a skinned-data pointer to that array
/// (`add_first_empty`; grows with `00a47770`, stores with `00c1d8b0`).
pub fn fn_00c1f420(e: &mut Engine, this: Ptr, value: Ptr) -> u32 {
    add_first_empty(e, this, value, true)
}

// Translated from 00c1f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshEmitter::_scalar_deleting_destructor_` (Xbox PDB): the two
/// arrays' blocks freed (`00a47600`, skinned data first, each after its
/// vtable store), the modifier destructor, then the sized delete of 0x88
/// bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_p_sys_mesh_emitter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let a = this.addr();
    let skinned = e.mem.u32(a + 0x68);
    e.mem.set_u32(a + 0x64, SKINNED_ARRAY_DESTROYED_VTABLE);
    e.call(NI_FREE, &args![skinned]);
    let geometries = e.mem.u32(a + 0x58);
    e.mem.set_u32(a + 0x54, ARRAY_BASE_VTABLE);
    e.call(NI_FREE, &args![geometries]);
    e.call(MODIFIER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x88u32]);
    }
    this
}

// Translated from 00c1f550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`, no words): the `NiPSysMeshEmitter`
/// constructor: the emitter part (`00c21f00`), the vtable, the geometry array
/// (capacity 1 from `00c1d740(1)`, grow-by 2, empty), the skinned-data array
/// (no block, grow-by 1), both types 0, the emit axis from `011a946c`.
pub fn fn_00c1f550(e: &mut Engine, this: Ptr) -> Ptr {
    let a = this.addr();
    e.call(EMITTER_CONSTRUCT, &args![this]);
    e.mem.set_u32(a, MESH_EMITTER_VTABLE);
    e.mem.set_u16(a + 0x5c, 1);
    e.mem.set_u32(a + 0x54, ARRAY_BASE_VTABLE);
    e.mem.set_u16(a + 0x62, 2);
    e.mem.set_u16(a + 0x5e, 0);
    e.mem.set_u16(a + 0x60, 0);
    let block = fn_00c1d740(e, 1).addr();
    e.mem.set_u32(a + 0x58, block);
    e.mem.set_u32(a + 0x54, GEOMETRY_ARRAY_VTABLE);
    e.mem.set_u32(a + 0x64, SKINNED_ARRAY_VTABLE);
    e.mem.set_u16(a + 0x6e, 0);
    e.mem.set_u32(a + 0x68, 0);
    e.mem.set_u16(a + 0x6c, 0);
    e.mem.set_u16(a + 0x70, 0);
    e.mem.set_u16(a + 0x72, 1);
    e.mem.set_u32(a + 0x74, 0);
    e.mem.set_u32(a + 0x78, 0);
    copy_words(e, DEFAULT_EMIT_AXIS, a + 0x7c, 3);
    this
}

// Translated from 00c1f5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshEmitter::ComputeInitialPositionAndVelocity` (Xbox PDB): with
/// geometries (effective size `+0x60`), picks one at `rand() / 32767 *
/// count` (truncated; `count` itself becomes `count - 1`); when it is below
/// the effective size and non-null, takes its skin instance (`+0xBC`) and the
/// skinned data at the same index (none past that array's effective size),
/// refreshes the data when both exist and it has no partition (`00c1d170`),
/// and emits by `m_eEmissionType`: 1 and 3 by face (`00c1e080` skinned,
/// `fn_00c1eb20` otherwise), 2 and 4 by edge (`00c1e4e0` / `00c1dae0`), the
/// rest by vertex (`00c1dec0` / `00c1d9e0`).
pub fn ni_p_sys_mesh_emitter_compute_initial_position_and_velocity(
    e: &mut Engine,
    this: Ptr<NiPSysMeshEmitter>,
    position: Ptr,
    velocity: Ptr,
) {
    let a = this.addr();
    let count = u32::from(e.mem.u16(a + 0x60));
    if count == 0 {
        return;
    }
    let fraction = random_fraction(e);
    let mut scale = f64::from(count as i32);
    if (count as i32) < 0 {
        // Never true (the count is a zero-extended u16): the compiler's
        // unsigned-to-float correction, translated as the exe has it.
        scale = x87_add(scale, x87_m32(e.global::<u32>(TWO_TO_THE_32)));
    }
    let product = x87_mul(ld(fraction), scale);
    let mut index = x87_fistp64_chop(X87Ext::load64(product)) as u32;
    if index == count {
        index = count - 1;
    }
    if index >= u32::from(e.mem.u16(a + 0x60)) {
        return;
    }
    let geometry = e.mem.u32(e.mem.u32(a + 0x58).wrapping_add(index * 4));
    if geometry == 0 {
        return;
    }
    let skin = e.mem.u32(geometry + 0xbc);
    let skinned = if index < u32::from(e.mem.u16(a + 0x70)) {
        e.mem.u32(e.mem.u32(a + 0x68).wrapping_add(index * 4))
    } else {
        0
    };
    if skin != 0 && skinned != 0 && e.mem.u32(skinned + 8) == 0 {
        e.call(SKINNED_DATA_UPDATE, &args![skinned, geometry]);
    }
    let geometry = Ptr::new(geometry);
    let (with_skin, without) = match e.get(this, NiPSysMeshEmitter::m_eEmissionType) {
        1 | 3 => (EMIT_SKINNED_FACE, None),
        2 | 4 => (EMIT_SKINNED_EDGE, Some(EMIT_EDGE)),
        _ => (EMIT_SKINNED_VERTEX, Some(EMIT_VERTEX)),
    };
    if skin != 0 {
        e.call(
            with_skin,
            &args![this, skinned, geometry, position, velocity],
        );
    } else {
        match without {
            Some(target) => {
                e.call(target, &args![this, geometry, position, velocity]);
            }
            None => {
                fn_00c1eb20(e, this, geometry, position, velocity);
            }
        }
    }
}

// Translated from 00c1f790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshEmitter::AddGeometryEmitter` (Xbox PDB; `thiscall`, one word):
/// with a geometry, adds it to the geometry array (`00c1f360`, through a
/// `NiPointer` temporary in the argument slot), then makes a skinned-emitter
/// data (12 bytes, `00c1cf20`) whose skin partition is set from the skin
/// instance's (`+0xBC`, its `+0xC`) through `0066b0d0` when there is one
/// (also on a failed allocation, as the exe does: on address 8), adds it to
/// the skinned array (`00c1f420`), and sets bits `0x33` of the geometry
/// data's byte `+0x38`.
pub fn ni_p_sys_mesh_emitter_add_geometry_emitter(e: &mut Engine, this: Ptr, geometry: Ptr) {
    if geometry.is_null() {
        return;
    }
    let g = geometry.addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), g);
        e.call(INTERLOCKED_INCREMENT, &args![g + 4]);
        fn_00c1f360(e, Ptr::new(this.addr() + 0x54), slot);
        release(e, g);
        let skin = e.mem.u32(g + 0xbc);
        let made = |e: &mut Engine, block: u32| -> u32 {
            if block == 0 {
                0
            } else {
                fn_00c1cf20(e, Ptr::new(block)).addr()
            }
        };
        let data;
        if skin != 0 {
            let partition = e.mem.u32(skin + 0xc);
            let block = e.call(NI_ALLOCATE, &args![0xcu32]).u32();
            data = made(e, block);
            if partition != 0 {
                e.call(NI_POINTER_ASSIGN, &args![data.wrapping_add(8), partition]);
            }
        } else {
            let block = e.call(NI_ALLOCATE, &args![0xcu32]).u32();
            data = made(e, block);
        }
        e.mem.set_u32(slot.addr(), data);
        if data != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![data + 4]);
        }
        fn_00c1f420(e, Ptr::new(this.addr() + 0x64), slot);
        if data != 0 {
            release(e, data);
        }
    });
    let geometry_data = e.mem.u32(g + 0xb8);
    if geometry_data != 0 {
        let bits = e.mem.u8(geometry_data + 0x38);
        e.mem.set_u8(geometry_data + 0x38, bits | 0x33);
    }
}

// Translated from 00c1f880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshEmitter::CreateClone` (Xbox PDB): a new emitter (0x88 bytes,
/// `00c1f550`, null when the allocation fails), the emitter members copied
/// (`00c21f60`), the clone's geometry array sized to this one's size
/// (`00c1e9a0`), the two types and the emit axis copied. Answers the clone.
pub fn ni_p_sys_mesh_emitter_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x88u32]).u32();
    let clone = if block == 0 {
        0
    } else {
        fn_00c1f550(e, Ptr::new(block)).addr()
    };
    e.call(EMITTER_COPY_MEMBERS, &args![this, clone, process]);
    let size = u32::from(e.mem.u16(this.addr() + 0x5e));
    fn_00c1e9a0(e, Ptr::new(clone.wrapping_add(0x54)), size);
    for offset in [0x74u32, 0x78, 0x7c, 0x80, 0x84] {
        let word = e.mem.u32(this.addr() + offset);
        e.mem.set_u32(clone.wrapping_add(offset), word);
    }
    Ptr::new(clone)
}

// Translated from 00c1f8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshEmitter::ProcessClone` (Xbox PDB): the modifier part
/// (`00c317e0`), then this emitter's clone is looked up in the process's map
/// (`00a53c30`, the result slot is the argument slot, set to 0 first) and
/// each non-null geometry (to the effective size) is added to that clone
/// (`AddGeometryEmitter`): its own clone when the map has one, else itself.
pub fn ni_p_sys_mesh_emitter_process_clone(e: &mut Engine, this: Ptr, process: Ptr) {
    e.call(MODIFIER_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let clone = Ptr::new(e.mem.u32(slot.addr()));
        let mut i: u32 = 0;
        while i < u32::from(e.mem.u16(this.addr() + 0x60)) {
            let base = e.mem.u32(this.addr() + 0x58);
            let geometry = e.mem.u32(base.wrapping_add(i * 4));
            if geometry != 0 {
                let map = e.mem.u32(process.addr());
                let found = e.call(CLONE_MAP_LOOKUP, &args![map, geometry, slot]).bool();
                let emitter = if found {
                    e.mem.u32(slot.addr())
                } else {
                    geometry
                };
                ni_p_sys_mesh_emitter_add_geometry_emitter(e, clone, Ptr::new(emitter));
            }
            i += 1;
        }
    });
}

// Translated from 00c1f970 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's creation function (Ghidra made no function of it; listed for
/// `NiPSysMeshEmitter` in `00c1aa00`'s loader table; `cdecl`, no words): 0x88
/// bytes constructed by `00c1f550` (a tail jump), or null.
pub fn fn_00c1f970(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x88u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00c1f550(e, Ptr::new(block))
}

// Translated from 00c1f990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshEmitter::LinkObject` (Xbox PDB): the modifier part (`00c2d6d0`),
/// then the stream's link-ID count: when nonzero the geometry array is sized
/// to it and each linked object added (`AddGeometryEmitter`).
pub fn ni_p_sys_mesh_emitter_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(MODIFIER_LINK_OBJECT, &args![this, stream]);
    let count = e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    if count == 0 {
        return;
    }
    fn_00c1e9a0(e, Ptr::new(this.addr() + 0x54), count);
    let mut left = count;
    while left != 0 {
        let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        ni_p_sys_mesh_emitter_add_geometry_emitter(e, this, Ptr::new(object));
        left -= 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c1_ced0, fn_00c1ced0(f32) -> f32),
        entry!(0x00c1_cf20, fn_00c1cf20(Ptr) -> Ptr),
        entry!(
            0x00c1_cf50,
            fn_00c1cf50(Ptr<NiPSysMeshEmitter>, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00c1_d660,
            ni_skinned_emitter_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c1_d740, fn_00c1d740(u32) -> Ptr),
        entry!(0x00c1_d7e0, fn_00c1d7e0(Ptr, u32, Ptr)),
        entry!(0x00c1_d8b0, fn_00c1d8b0(Ptr, u32, Ptr)),
        entry!(
            0x00c1_e850,
            ni_p_sys_mesh_emitter_load_binary(Ptr<NiPSysMeshEmitter>, Ptr)
        ),
        entry!(0x00c1_e9a0, fn_00c1e9a0(Ptr, u32)),
        entry!(
            0x00c1_eb20,
            fn_00c1eb20(Ptr<NiPSysMeshEmitter>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00c1_f360, fn_00c1f360(Ptr, Ptr) -> u32),
        entry!(0x00c1_f420, fn_00c1f420(Ptr, Ptr) -> u32),
        entry!(
            0x00c1_f500,
            ni_p_sys_mesh_emitter_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c1_f550, fn_00c1f550(Ptr) -> Ptr),
        entry!(
            0x00c1_f5f0,
            ni_p_sys_mesh_emitter_compute_initial_position_and_velocity(
                Ptr<NiPSysMeshEmitter>,
                Ptr,
                Ptr,
            )
        ),
        entry!(
            0x00c1_f790,
            ni_p_sys_mesh_emitter_add_geometry_emitter(Ptr, Ptr)
        ),
        entry!(0x00c1_f880, ni_p_sys_mesh_emitter_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c1_f8f0, ni_p_sys_mesh_emitter_process_clone(Ptr, Ptr)),
        entry!(0x00c1_f970, fn_00c1f970() -> Ptr),
        entry!(0x00c1_f990, ni_p_sys_mesh_emitter_link_object(Ptr, Ptr)),
        entry!(0x00c1f4f0, ni_p_sys_mesh_emitter_get_rtti() -> Ptr),
        entry!(0x00c1d790, fn_00c1d790(u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_mesh_emitter_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00c1f4f0, &args![]).u32(),
            NI_P_SYS_MESH_EMITTER_RTTI
        );
    }

    #[test]
    fn fn_00c1d790_allocates_the_array_with_its_count_in_front() {
        let mut e = Engine::new();
        e.register(0x00aa_3e40, |e, a| {
            assert_eq!(a[0], MEMORY_MANAGER);
            e.mem.alloc(a[1]).into_ret()
        });
        e.register(0x0040_1050, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let elements = e.call(0x00c1_d790, &args![5u32]).u32();
        let log = e.call_log.take().unwrap();
        let alloc: Vec<_> = log.iter().filter(|c| c.0 == 0x00aa_3e40).collect();
        assert_eq!(alloc[0].1, vec![MEMORY_MANAGER, 24]);
        assert_eq!(e.mem.u32(elements - 4), 5);
        let init: Vec<_> = log.iter().filter(|c| c.0 == 0x0040_1050).collect();
        assert_eq!(init[0].1, vec![elements, 4, 5, ELEMENT_CONSTRUCTOR]);
    }

    #[test]
    fn fn_00c1d790_saturates_the_size_and_answers_null_on_failure() {
        let mut e = Engine::new();
        e.register(0x00aa_3e40, |_, _| Ret::default());
        e.register(0x0040_1050, |_, _| panic!("no elements without a block"));
        e.call_log = Some(vec![]);
        // 0x4000_0000 * 4 overflows; so does 0x3fff_ffff * 4 + 4.
        assert_eq!(e.call(0x00c1_d790, &args![0x4000_0000u32]).u32(), 0);
        assert_eq!(e.call(0x00c1_d790, &args![0x3fff_ffffu32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let sizes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == 0x00aa_3e40)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(sizes, vec![u32::MAX, u32::MAX]);
    }
}

#[cfg(test)]
mod emitter_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

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

    /// The interlocked imports as memory operations.
    fn with_refcounts(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
    }

    const DELETE_THIS: u32 = 0x0100_0004;

    /// A reference-counted object whose `DeleteThis` is the double at `DELETE_THIS`.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn set_floats(e: &mut Engine, addr: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(addr + 4 * i as u32, *v);
        }
    }

    fn floats(e: &Engine, addr: u32) -> [f32; 3] {
        [e.mem.f32(addr), e.mem.f32(addr + 4), e.mem.f32(addr + 8)]
    }

    fn constants(e: &mut Engine) {
        for (addr, value) in [
            (RAND_DIVISOR, 32767.0f64),
            (HALF, 0.5),
            (ONE, 1.0),
            (ONE_AND_HALF, 1.5),
            (ONE_THIRD, f64::from_bits(0x3fd5_5555_6000_0000)),
        ] {
            e.map(addr, 8);
            e.set_global(addr, value);
        }
    }

    fn rand_sequence(e: &mut Engine, values: Vec<u32>) {
        let mut values = values.into_iter();
        e.register_double(RAND, move |_, _| word(values.next().unwrap()));
    }

    fn map_set_at_statics(e: &mut Engine) {
        e.register(ATEXIT, |_, _| Ret::default());
        for guard in [0x0120_24f4u32, 0x0120_24f0, 0x0120_24fc, 0x0120_24f8] {
            e.map(guard, 4);
        }
    }

    fn transform_doubles(e: &mut Engine) {
        e.register(INVERT_NON_UNIFORM, |_, _| Ret::default());
        e.register(TRANSFORM_PRODUCT, |_, _| Ret::default());
        e.register(TRANSFORM_POINT, |_, a| word(a[2]));
        e.register(ROTATE_VECTOR, |_, a| word(a[2]));
    }

    #[test]
    fn fn_00c1ced0_is_one_newton_step_of_the_square_root() {
        let mut e = Engine::new();
        constants(&mut e);
        let root = e.call(0x00c1_ced0, &args![4.0f32]).f32();
        assert!((root - 2.0).abs() < 0.01, "{root}");
        assert_eq!(e.call(0x00c1_ced0, &args![0.0f32]).f32(), 0.0);
    }

    #[test]
    fn fn_00c1cf20_constructs_a_skinned_emitter_data() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.map(OBJECT_COUNT, 4);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0x77);
        assert_eq!(e.call(0x00c1_cf20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SKINNED_EMITTER_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.global::<u32>(OBJECT_COUNT), 1);
    }

    fn emitter_with_system(e: &mut Engine) -> (Ptr<NiPSysMeshEmitter>, u32) {
        let this: Ptr<NiPSysMeshEmitter> = e.new_object();
        let system = e.mem.alloc(0x100);
        e.set(this, NiPSysMeshEmitter::m_pkTarget, system);
        e.set(this, NiPSysMeshEmitter::m_fSpeed, 10.0f32.to_bits());
        e.set(this, NiPSysMeshEmitter::m_fSpeedVar, 2.0f32.to_bits());
        (this, system)
    }

    #[test]
    fn fn_00c1cf50_transforms_the_position_and_sets_the_velocity_by_type() {
        let mut e = Engine::new();
        constants(&mut e);
        e.register(INVERT_NON_UNIFORM, |_, _| Ret::default());
        e.register(TRANSFORM_PRODUCT, |_, _| Ret::default());
        e.register(TRANSFORM_POINT, |e, a| {
            set_floats(e, a[1], &[5.0, 6.0, 7.0]);
            word(a[1])
        });
        e.register(ROTATE_VECTOR, |e, a| {
            set_floats(e, a[1], &[0.0, 3.0, 4.0]);
            word(a[1])
        });
        e.register(NORMALIZE, |e, a| {
            let [x, y, z] = floats(e, a[0]);
            let length = (x * x + y * y + z * z).sqrt();
            set_floats(e, a[0], &[x / length, y / length, z / length]);
            Ret::default()
        });
        let (this, system) = emitter_with_system(&mut e);
        let geometry = e.mem.alloc(0x100);
        e.mem.set_u32(geometry + 0x68, 0x1234);
        e.mem.set_u32(system + 0x68, 0x5678);
        let position = e.mem.alloc(0xc);
        let velocity = e.mem.alloc(0xc);
        // Without a geometry nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x00c1_cf50, &args![this, 0u32, position, velocity]);
        assert_eq!(calls_to(&e, INVERT_NON_UNIFORM).len(), 0);
        // Type 0: the velocity is rotated.
        e.call(0x00c1_cf50, &args![this, geometry, position, velocity]);
        assert_eq!(floats(&e, position), [5.0, 6.0, 7.0]);
        assert_eq!(floats(&e, velocity), [0.0, 3.0, 4.0]);
        let product = &calls_to(&e, TRANSFORM_PRODUCT)[0];
        assert_eq!(e.mem.u32(product[2]), 0x1234);
        assert_eq!(calls_to(&e, INVERT_NON_UNIFORM)[0][0], product[1]);
        // Type 2: the rotated axis normalized, times speed 10 + (1 - 0.5) * 2 = 11.
        e.set(this, NiPSysMeshEmitter::m_eInitVelocityType, 2);
        rand_sequence(&mut e, vec![32767]);
        e.call(0x00c1_cf50, &args![this, geometry, position, velocity]);
        let v = floats(&e, velocity);
        assert_eq!(v[0], 0.0);
        assert!(
            (v[1] - 6.6).abs() < 1e-5 && (v[2] - 8.8).abs() < 1e-5,
            "{v:?}"
        );
        // Type 1: a random direction: (1, -1, 1) normalized, times the speed.
        e.set(this, NiPSysMeshEmitter::m_eInitVelocityType, 1);
        rand_sequence(&mut e, vec![32767, 0, 32767, 16383]);
        e.call(0x00c1_cf50, &args![this, geometry, position, velocity]);
        let speed = (((16383.0f64 / 32767.0) as f32 as f64 - 0.5) * 2.0 + 10.0) as f32;
        let unit = 1.0 / 3.0f32.sqrt();
        let v = floats(&e, velocity);
        assert!((v[0] - unit * speed).abs() < 1e-5, "{v:?}");
        assert!((v[1] + unit * speed).abs() < 1e-5, "{v:?}");
        // Another type leaves the velocity.
        e.set(this, NiPSysMeshEmitter::m_eInitVelocityType, 7);
        set_floats(&mut e, velocity, &[1.0, 2.0, 3.0]);
        e.call(0x00c1_cf50, &args![this, geometry, position, velocity]);
        assert_eq!(floats(&e, velocity), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn skinned_data_destructor_releases_the_partition() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        e.map(OBJECT_COUNT, 4);
        e.set_global(OBJECT_COUNT, 3u32);
        let partition = counted(&mut e, 1);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, partition);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c1_d660, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![partition]]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE), vec![vec![this, 0xc]]);
        assert_eq!(e.mem.u32(this), REF_OBJECT_VTABLE);
        assert_eq!(e.global::<u32>(OBJECT_COUNT), 2);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c1_d660, &args![this, 0u32]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE).len(), 1);
    }

    #[test]
    fn fn_00c1d740_allocates_with_its_element_constructor() {
        let mut e = Engine::new();
        e.register(0x00aa_3e40, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(0x0040_1050, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let elements = e.call(0x00c1_d740, &args![3u32]).u32();
        assert_eq!(calls_to(&e, 0x00aa_3e40), vec![vec![MEMORY_MANAGER, 16]]);
        assert_eq!(e.mem.u32(elements - 4), 3);
        assert_eq!(
            calls_to(&e, 0x0040_1050),
            vec![vec![elements, 4, 3, 0x0066_94e0]]
        );
        e.register(0x00aa_3e40, |_, _| Ret::default());
        assert_eq!(e.call(0x00c1_d740, &args![0x4000_0000u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x00aa_3e40)[1][1], u32::MAX);
    }

    fn array(e: &mut Engine, max: u16, size: u16, elements: &[u32]) -> u32 {
        let this = e.mem.alloc(0x10);
        let base = e.mem.alloc(4 * u32::from(max.max(1)));
        for (i, v) in elements.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *v);
        }
        e.mem.set_u32(this + 4, base);
        e.mem.set_u16(this + 8, max);
        e.mem.set_u16(this + 0xa, size);
        let effective = elements.iter().filter(|v| **v != 0).count() as u16;
        e.mem.set_u16(this + 0xc, effective);
        e.mem.set_u16(this + 0xe, 2);
        this
    }

    fn check_set_at(address: u32, guard: u32, default: u32, handler: u32) {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.register(ATEXIT, |_, _| Ret::default());
        e.map(guard, 4);
        e.map(default, 4);
        e.set_global(default, 0x55u32);
        let old = counted(&mut e, 1);
        let new = counted(&mut e, 0);
        let this = array(&mut e, 4, 2, &[old, 0, 0, 0]);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, new);
        e.call_log = Some(vec![]);
        // Replacing a non-null element keeps the effective size.
        e.call(address, &args![this, 0u32, value]);
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![handler]]);
        assert_eq!(e.global::<u32>(default), 0);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![old]]);
        let base = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(base), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        // Past the size: the size grows and the effective size with it.
        e.call(address, &args![this, 2u32, value]);
        assert_eq!(e.mem.u16(this + 0xa), 3);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        assert_eq!(calls_to(&e, ATEXIT).len(), 1);
        // A null into a set element lowers it.
        e.mem.set_u32(value, 0);
        e.call(address, &args![this, 0u32, value]);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u32(base), 0);
        // A pointer into a null element below the size raises it.
        e.mem.set_u32(value, new);
        e.call(address, &args![this, 1u32, value]);
        assert_eq!(e.mem.u16(this + 0xc), 2);
    }

    #[test]
    fn fn_00c1d7e0_sets_geometry_elements() {
        check_set_at(0x00c1_d7e0, 0x0120_24f4, 0x0120_24f0, 0x00fd_d3e0);
    }

    #[test]
    fn fn_00c1d8b0_sets_skinned_data_elements() {
        check_set_at(0x00c1_d8b0, 0x0120_24fc, 0x0120_24f8, 0x00fd_d3b0);
    }

    #[test]
    fn load_binary_reads_the_types_into_one_local_word() {
        const READ: u32 = 0x0100_0010;
        let mut e = Engine::new();
        e.register(EMITTER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_MULTIPLE_LINK_IDS, |_, _| Ret::default());
        e.register(THREE_FLOATS_LOAD_BINARY, |_, _| Ret::default());
        let mut values = vec![3u32, 2].into_iter();
        e.register_double(READ, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], values.next().unwrap());
            word(4)
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this: Ptr<NiPSysMeshEmitter> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c1_e850, &args![this, stream]);
        assert_eq!(e.get(this, NiPSysMeshEmitter::m_eInitVelocityType), 3);
        assert_eq!(e.get(this, NiPSysMeshEmitter::m_eEmissionType), 2);
        assert_eq!(e.mem.u32(file + 4), 8);
        assert_eq!(
            calls_to(&e, THREE_FLOATS_LOAD_BINARY),
            vec![vec![this.addr() + 0x7c, stream]]
        );
        assert_eq!(calls_to(&e, READ_MULTIPLE_LINK_IDS), vec![vec![stream]]);
    }

    #[test]
    fn set_size_shrinks_releasing_and_copies_into_a_new_block() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(0x00aa_3e40, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(0x0040_1050, |_, _| Ret::default());
        let first = counted(&mut e, 1);
        let dropped = counted(&mut e, 1);
        let this = array(&mut e, 4, 3, &[first, 0, dropped, 0]);
        let old = e.mem.u32(this + 4);
        e.call_log = Some(vec![]);
        // The same capacity: nothing.
        e.call(0x00c1_e9a0, &args![this, 4u32]);
        assert!(calls_to(&e, NI_FREE).is_empty());
        e.call(0x00c1_e9a0, &args![this, 2u32]);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![dropped]]);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 8), 2);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        let new = e.mem.u32(this + 4);
        assert_ne!(new, old);
        assert_eq!(e.mem.u32(new), first);
        assert_eq!(e.mem.u32(first + 4), 2);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![old]]);
        // To 0: no block.
        e.call(0x00c1_e9a0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(calls_to(&e, NI_FREE)[1], vec![new]);
    }

    const TRIANGLE: u32 = 0x0100_0028;

    /// A triangle geometry: vtable `+0x1C` answers 1; its data's `+0xA8`
    /// answers 4 triangles and `+0xAC` hands out `corners` in turn.
    fn triangle_geometry(e: &mut Engine, corners: Vec<[u16; 3]>) -> (u32, u32) {
        const IS_TRI_BASED: u32 = 0x0100_0020;
        const TRIANGLE_COUNT: u32 = 0x0100_0024;
        e.register(IS_TRI_BASED, |_, _| word(1));
        e.register(TRIANGLE_COUNT, |_, _| word(0x0002_0004));
        let corners = Rc::new(RefCell::new(corners.into_iter()));
        e.register_double(TRIANGLE, move |e, a| {
            let [x, y, z] = corners.borrow_mut().next().unwrap();
            e.mem.set_u16(a[2], x);
            e.mem.set_u16(a[3], y);
            e.mem.set_u16(a[4], z);
            Ret::default()
        });
        let geometry_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(geometry_vtable + 0x1c, IS_TRI_BASED);
        let geometry = e.mem.alloc(0x100);
        e.mem.set_u32(geometry, geometry_vtable);
        let data_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(data_vtable + 0xa8, TRIANGLE_COUNT);
        e.mem.set_u32(data_vtable + 0xac, TRIANGLE);
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(data, data_vtable);
        e.mem.set_u32(geometry + 0xb8, data);
        (geometry, data)
    }

    #[test]
    fn fn_00c1eb20_emits_from_the_centroid_of_a_distinct_triangle() {
        let mut e = Engine::new();
        constants(&mut e);
        transform_doubles(&mut e);
        let (geometry, data) = triangle_geometry(&mut e, vec![[1, 1, 2], [0, 1, 2]]);
        let vertices = e.mem.alloc(0x24);
        set_floats(
            &mut e,
            vertices,
            &[0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 6.0, 0.0],
        );
        e.mem.set_u32(data + 0x20, vertices);
        let normals = e.mem.alloc(0x24);
        set_floats(
            &mut e,
            normals,
            &[0.0, 0.0, 2.0, 0.0, 0.0, 2.0, 0.0, 0.0, 2.0],
        );
        e.mem.set_u32(data + 0x24, normals);
        e.register(NORMALIZE_FAST, |e, a| {
            set_floats(e, a[0], &[0.0, 0.0, 1.0]);
            Ret::default()
        });
        e.register(SQRT, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            value.sqrt().into_ret()
        });
        rand_sequence(&mut e, vec![5]);
        let (this, _) = emitter_with_system(&mut e);
        let position = e.mem.alloc(0xc);
        let velocity = e.mem.alloc(0xc);
        set_floats(&mut e, velocity, &[3.0, 4.0, 0.0]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00c1_eb20, &args![this, geometry, position, velocity])
            .bool());
        // 5 % 4 = 1 first: its corners 1 1 2 repeat, so triangle 2 is used.
        let tries: Vec<u32> = calls_to(&e, TRIANGLE).iter().map(|w| w[1]).collect();
        assert_eq!(tries, vec![1, 2]);
        assert_eq!(floats(&e, position), [1.0, 2.0, 0.0]);
        assert_eq!(floats(&e, velocity), [0.0, 0.0, 5.0]);
    }

    #[test]
    fn fn_00c1eb20_without_triangles_searches_the_class_chain() {
        const NOT_TRI: u32 = 0x0100_0040;
        const GET_RTTI: u32 = 0x0100_0044;
        let mut e = Engine::new();
        e.register(NOT_TRI, |_, _| word(0));
        let base_rtti = e.mem.alloc(8);
        e.mem.set_u32(base_rtti + 4, FALLBACK_RTTI);
        e.register_double(GET_RTTI, move |_, _| word(base_rtti));
        e.map(FALLBACK_RTTI, 8);
        e.register(EMIT_EDGE, |_, _| word(0x101));
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x1c, NOT_TRI);
        e.mem.set_u32(vtable + 8, GET_RTTI);
        let geometry = e.mem.alloc(0x100);
        e.mem.set_u32(geometry, vtable);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00c1_eb20, &args![0x9000u32, geometry, 1u32, 2u32])
            .bool());
        assert_eq!(calls_to(&e, EMIT_EDGE), vec![vec![0x9000, geometry, 1, 2]]);
        // Not in the chain: false; no geometry: false.
        e.mem.set_u32(base_rtti + 4, 0);
        assert!(!e
            .call(0x00c1_eb20, &args![0x9000u32, geometry, 1u32, 2u32])
            .bool());
        assert!(!e
            .call(0x00c1_eb20, &args![0x9000u32, 0u32, 1u32, 2u32])
            .bool());
    }

    #[test]
    fn fn_00c1eb20_area_emission_and_no_vertices() {
        let mut e = Engine::new();
        constants(&mut e);
        transform_doubles(&mut e);
        e.register(NORMALIZE, |_, _| Ret::default());
        let (geometry, data) = triangle_geometry(&mut e, vec![[0, 1, 2], [0, 1, 2]]);
        rand_sequence(&mut e, vec![0, 0, 32767, 0, 0, 0, 0, 16383]);
        let (this, _) = emitter_with_system(&mut e);
        let position = e.mem.alloc(0xc);
        let velocity = e.mem.alloc(0xc);
        assert!(!e
            .call(0x00c1_eb20, &args![this, geometry, position, velocity])
            .bool());
        let vertices = e.mem.alloc(0x24);
        set_floats(
            &mut e,
            vertices,
            &[1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 5.0, 1.0],
        );
        e.mem.set_u32(data + 0x20, vertices);
        e.set(this, NiPSysMeshEmitter::m_eEmissionType, 3);
        e.set(this, NiPSysMeshEmitter::m_eInitVelocityType, 1);
        assert!(e
            .call(0x00c1_eb20, &args![this, geometry, position, velocity])
            .bool());
        // The square-root step of 1 is about 1 and the edge fraction 0:
        // e1 + (0 - e1) * 1 + a is about a.
        let p = floats(&e, position);
        assert!(
            (p[0] - 1.0).abs() < 1e-2 && (p[1] - 1.0).abs() < 1e-2,
            "{p:?}"
        );
    }

    #[test]
    fn add_first_empty_fills_a_hole_or_appends() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        map_set_at_statics(&mut e);
        e.register(SKINNED_ARRAY_SET_SIZE, |_, _| Ret::default());
        let object = counted(&mut e, 0);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, object);
        let other = counted(&mut e, 1);
        let this = array(&mut e, 4, 2, &[other, 0, 0, 0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c1_f360, &args![this, value]).u32(), 1);
        assert_eq!(e.mem.u32(e.mem.u32(this + 4) + 4), object);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        // Full below the size: appended at the size (no growth needed).
        assert_eq!(e.call(0x00c1_f420, &args![this, value]).u32(), 2);
        assert_eq!(e.mem.u16(this + 0xa), 3);
        assert!(calls_to(&e, SKINNED_ARRAY_SET_SIZE).is_empty());
        // A full array grows by the grow-by step first.
        let full = array(&mut e, 1, 1, &[other]);
        let grown = e.mem.alloc(8);
        e.mem.set_u32(grown, other);
        e.register_double(SKINNED_ARRAY_SET_SIZE, move |e, a| {
            e.mem.set_u32(a[0] + 4, grown);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        assert_eq!(e.call(0x00c1_f420, &args![full, value]).u32(), 1);
        assert_eq!(calls_to(&e, SKINNED_ARRAY_SET_SIZE), vec![vec![full, 3]]);
        e.mem.set_u32(value, 0);
        assert_eq!(e.call(0x00c1_f360, &args![this, value]).u32(), u32::MAX);
    }

    #[test]
    fn mesh_emitter_destructor_frees_both_blocks() {
        let mut e = Engine::new();
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(MODIFIER_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x88);
        e.mem.set_u32(this + 0x58, 0x111);
        e.mem.set_u32(this + 0x68, 0x222);
        e.call_log = Some(vec![]);
        e.call(0x00c1_f500, &args![this, 1u32]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x222], vec![0x111]]);
        assert_eq!(e.mem.u32(this + 0x54), ARRAY_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x64), SKINNED_ARRAY_DESTROYED_VTABLE);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE), vec![vec![this, 0x88]]);
        e.call(0x00c1_f500, &args![this, 0u32]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE).len(), 1);
    }

    #[test]
    fn constructor_and_create_function() {
        let mut e = Engine::new();
        e.register(EMITTER_CONSTRUCT, |_, _| Ret::default());
        e.register(0x00aa_3e40, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(0x0040_1050, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.map(DEFAULT_EMIT_AXIS, 0xc);
        e.mem.set_f32(DEFAULT_EMIT_AXIS, 1.0);
        assert_eq!(e.call(0x00c1_f970, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.call(0x00c1_f970, &args![]).u32();
        assert_eq!(e.mem.u32(this), MESH_EMITTER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x54), GEOMETRY_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x64), SKINNED_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(this + 0x5c), 1);
        assert_eq!(e.mem.u16(this + 0x62), 2);
        assert_eq!(e.mem.u16(this + 0x72), 1);
        assert_ne!(e.mem.u32(this + 0x58), 0);
        assert_eq!(e.mem.f32(this + 0x7c), 1.0);
        assert_eq!(e.call(0x00c1_f550, &args![this]).u32(), this);
    }

    #[test]
    fn compute_initial_position_picks_a_geometry_and_an_emission() {
        let mut e = Engine::new();
        constants(&mut e);
        for target in [EMIT_SKINNED_FACE, EMIT_EDGE, EMIT_VERTEX] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(SKINNED_DATA_UPDATE, |_, _| Ret::default());
        let this: Ptr<NiPSysMeshEmitter> = e.new_object();
        let a = this.addr();
        let geometry = e.mem.alloc(0x100);
        let geometries = e.mem.alloc(8);
        e.mem.set_u32(geometries + 4, geometry);
        e.mem.set_u32(a + 0x58, geometries);
        e.mem.set_u16(a + 0x60, 2);
        e.call_log = Some(vec![]);
        // rand 32767: index 2 becomes 1.
        rand_sequence(&mut e, vec![32767, 32767, 32767]);
        e.set(this, NiPSysMeshEmitter::m_eEmissionType, 4);
        e.call(0x00c1_f5f0, &args![this, 1u32, 2u32]);
        assert_eq!(calls_to(&e, EMIT_EDGE), vec![vec![a, geometry, 1, 2]]);
        e.set(this, NiPSysMeshEmitter::m_eEmissionType, 9);
        e.call(0x00c1_f5f0, &args![this, 1u32, 2u32]);
        assert_eq!(calls_to(&e, EMIT_VERTEX), vec![vec![a, geometry, 1, 2]]);
        // With a skin instance and skinned data without a partition.
        let skinned = e.mem.alloc(0xc);
        let skins = e.mem.alloc(8);
        e.mem.set_u32(skins + 4, skinned);
        e.mem.set_u32(a + 0x68, skins);
        e.mem.set_u16(a + 0x70, 2);
        e.mem.set_u32(geometry + 0xbc, 0x4444);
        e.set(this, NiPSysMeshEmitter::m_eEmissionType, 3);
        e.call(0x00c1_f5f0, &args![this, 1u32, 2u32]);
        assert_eq!(
            calls_to(&e, SKINNED_DATA_UPDATE),
            vec![vec![skinned, geometry]]
        );
        assert_eq!(
            calls_to(&e, EMIT_SKINNED_FACE),
            vec![vec![a, skinned, geometry, 1, 2]]
        );
        // No geometries: no rand.
        e.mem.set_u16(a + 0x60, 0);
        e.call(0x00c1_f5f0, &args![this, 1u32, 2u32]);
        assert_eq!(calls_to(&e, RAND).len(), 3);
    }

    /// An emitter whose two arrays are empty with room for four.
    fn emitter_with_arrays(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x88);
        let geometries = array(e, 4, 0, &[]);
        let skins = array(e, 4, 0, &[]);
        copy_words(e, geometries, this + 0x54, 4);
        copy_words(e, skins, this + 0x64, 4);
        this
    }

    #[test]
    fn add_geometry_emitter_adds_the_geometry_and_its_skinned_data() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        map_set_at_statics(&mut e);
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.map(OBJECT_COUNT, 4);
        let this = emitter_with_arrays(&mut e);
        let geometry = counted(&mut e, 1);
        let skin = e.mem.alloc(0x10);
        e.mem.set_u32(skin + 0xc, 0x5151);
        e.mem.set_u32(geometry + 0xbc, skin);
        let geometry_data = e.mem.alloc(0x40);
        e.mem.set_u32(geometry + 0xb8, geometry_data);
        e.call_log = Some(vec![]);
        e.call(0x00c1_f790, &args![this, geometry]);
        let base = e.mem.u32(this + 0x58);
        assert_eq!(e.mem.u32(base), geometry);
        assert_eq!(e.mem.u32(geometry + 4), 2);
        let data = e.mem.u32(e.mem.u32(this + 0x68));
        assert_eq!(e.mem.u32(data), SKINNED_EMITTER_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(
            calls_to(&e, NI_POINTER_ASSIGN),
            vec![vec![data + 8, 0x5151]]
        );
        assert_eq!(e.mem.u8(geometry_data + 0x38), 0x33);
        e.call(0x00c1_f790, &args![this, 0u32]);
        assert_eq!(calls_to(&e, NI_ALLOCATE).len(), 1);
    }

    #[test]
    fn create_clone_copies_types_and_axis() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(EMITTER_CONSTRUCT, |_, _| Ret::default());
        e.register(EMITTER_COPY_MEMBERS, |_, _| Ret::default());
        e.register(0x00aa_3e40, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(0x0040_1050, |_, _| Ret::default());
        e.register(NI_FREE, |_, _| Ret::default());
        e.map(DEFAULT_EMIT_AXIS, 0xc);
        let this = e.mem.alloc(0x88);
        e.mem.set_u16(this + 0x5e, 3);
        for (i, offset) in [0x74u32, 0x78, 0x7c, 0x80, 0x84].iter().enumerate() {
            e.mem.set_u32(this + offset, 0x10 + i as u32);
        }
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c1_f880, &args![this, 0x7000u32]).u32();
        assert_eq!(
            calls_to(&e, EMITTER_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        assert_eq!(e.mem.u16(clone + 0x5c), 3);
        assert_eq!(e.mem.u32(clone + 0x84), 0x14);
        assert_eq!(e.mem.u32(clone + 0x74), 0x10);
    }

    #[test]
    fn process_clone_adds_each_geometry_or_its_clone() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        map_set_at_statics(&mut e);
        e.register(MODIFIER_PROCESS_CLONE, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        let clone = emitter_with_arrays(&mut e);
        let kept = counted(&mut e, 1);
        let cloned_from = counted(&mut e, 1);
        let replacement = counted(&mut e, 1);
        let this = e.mem.alloc(0x88);
        let source = array(&mut e, 4, 2, &[kept, cloned_from]);
        copy_words(&mut e, source, this + 0x54, 4);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            let found = if a[1] == this {
                clone
            } else if a[1] == cloned_from {
                replacement
            } else {
                return word(0);
            };
            e.mem.set_u32(a[2], found);
            word(1)
        });
        let process = e.mem.alloc(4);
        e.mem.set_u32(process, 0x6060);
        e.call(0x00c1_f8f0, &args![this, process]);
        let base = e.mem.u32(clone + 0x58);
        assert_eq!(e.mem.u32(base), kept);
        assert_eq!(e.mem.u32(base + 4), replacement);
        assert_eq!(e.mem.u16(clone + 0x60), 2);
    }

    #[test]
    fn link_object_sizes_the_array_and_adds_every_linked_object() {
        let mut e = Engine::new();
        e.register(MODIFIER_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_NUMBER_OF_LINK_IDS, |_, _| word(2));
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| word(0));
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(0x00aa_3e40, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(0x0040_1050, |_, _| Ret::default());
        let this = e.mem.alloc(0x88);
        let geometries = array(&mut e, 1, 0, &[]);
        copy_words(&mut e, geometries, this + 0x54, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c1_f990, &args![this, 0x7000u32]);
        assert_eq!(e.mem.u16(this + 0x5c), 2);
        assert_eq!(calls_to(&e, GET_OBJECT_FROM_LINK_ID).len(), 2);
        e.register(GET_NUMBER_OF_LINK_IDS, |_, _| word(0));
        e.call(0x00c1_f990, &args![this, 0x7000u32]);
        assert_eq!(calls_to(&e, GET_OBJECT_FROM_LINK_ID).len(), 2);
    }
}
