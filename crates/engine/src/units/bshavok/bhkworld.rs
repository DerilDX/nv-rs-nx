//! `BSHavok/bhkworld.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

const HEAP_ALLOC: u32 = 0x00aa_3e40;
const HEAP_FREE: u32 = 0x00aa_4060;
const HEAP_SINGLETON: u32 = 0x011f_6238;
const MEMORY_ALLOCATOR_DESTRUCTOR: u32 = 0x00cb_f1a0;
const HAVOK_WORLD_DESTRUCTOR: u32 = 0x00c9_3e50;
const GLOBAL_WORLD_MODE: u32 = 0x011a_fe5c;
const GLOBAL_PICK_COUNT_OVERRIDE: u32 = 0x0126_77a3;
const GLOBAL_WORLD_FLOAT: u32 = 0x0126_77c4;
const GLOBAL_WORLD_INTEGER: u32 = 0x0126_77c8;
const ALLOCATOR_VTABLE: u32 = 0x010c_3af8;
const HAVOK_WORLD_VTABLE: u32 = 0x010c_3bc4;

layout! {
    /// `bhkWorld` (Xbox PDB), 0xA0 bytes; offsets used by this unit agree with the PC image.
    pub struct BhkWorld: 0xa0 {
        /// `pInfo` (Xbox PDB), the `hkSerializableCinfo*` stored by the world.
        0x0c pInfo: Ptr,
        /// `pDebugger` (Xbox PDB).
        0x10 pDebugger: Ptr,
        /// `bAdded` (Xbox PDB).
        0x14 bAdded: bool,
        /// `bEnabled` (Xbox PDB).
        0x15 bEnabled: bool,
        /// `bDebugDisplay` (Xbox PDB).
        0x16 bDebugDisplay: bool,
        /// `iBatchAdd` (Xbox PDB).
        0x18 iBatchAdd: u32,
        /// `iBatchRemove` (Xbox PDB).
        0x1c iBatchRemove: u32,
        /// `Origin` (Xbox PDB), a four-float `hkVector4`.
        0x60 origin: [f32; 4],
        /// `pEntityListenerA.data` (Xbox PDB), the `hkArray` pointer.
        0x70 entityListeners: Ptr,
        /// `pEntityListenerA.size` (Xbox PDB).
        0x74 entityListenerCount: i32,
    }

    /// `hkQuaternion` (Xbox PDB), a four-float `hkVector4`.
    pub struct HkQuaternion: 0x10 {
        0x00 x: f32,
        0x04 y: f32,
        0x08 z: f32,
        0x0c w: f32,
    }

    /// The `m_rootCollidable` field read by the two `bhkPickData` helpers (Xbox PDB).
    pub struct BhkPickData: 0xb0 {
        0x80 rootCollidable: Ptr,
    }
}

// Translated from 00c66170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMemoryAllocator::blockAlloc` (Xbox PDB): allocates `size + 16` bytes
/// and returns the first 16-byte-aligned address after reserving a trailing
/// byte for the alignment distance.
pub fn bhk_memory_allocator_block_alloc(e: &mut Engine, size: u32) -> Ptr {
    let raw = e
        .call(HEAP_ALLOC, &args![HEAP_SINGLETON, size.wrapping_add(0x10)])
        .u32();
    let padding = 0x10 - (raw & 0xf);
    let aligned = raw.wrapping_add(padding);
    e.mem.set_u8(aligned - 1, padding as u8);
    Ptr::new(aligned)
}

// Translated from 00c661a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMemoryAllocator::blockFree` (Xbox PDB): recovers the raw allocation
/// from the byte immediately before `block`; the second stack word is unused.
pub fn bhk_memory_allocator_block_free(e: &mut Engine, block: Ptr, _unused_1: u32) {
    if !block.is_null() {
        let padding = e.mem.u8(block.addr() - 1) as u32;
        e.call(
            HEAP_FREE,
            &args![HEAP_SINGLETON, block.addr().wrapping_sub(padding)],
        );
    }
}

// Translated from 00c661c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the second of its two stack arguments; `RET 8` confirms both words
/// are part of the function's ABI even though the first is not read.
pub fn bhk_world_return_second_argument(_e: &mut Engine, _unused_0: u32, value: u32) -> u32 {
    value
}

// Translated from 00c661d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the physics update mode global to 1 for argument 0, 3 for argument
/// 4, and 2 for every other value.
pub fn bhk_world_set_update_mode(e: &mut Engine, mode: u32) {
    let value = match mode {
        0 => 1,
        4 => 3,
        _ => 2,
    };
    e.set_global(GLOBAL_WORLD_MODE, value);
}

// Translated from 00c66210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the world's integer and float step accumulators.
pub fn bhk_world_reset_step_accumulators(e: &mut Engine) {
    e.set_global(GLOBAL_WORLD_INTEGER, 0u32);
    e.set_global(GLOBAL_WORLD_FLOAT, 0.0f32);
}

// Translated from 00c66230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::SetPickCountOverride` (Xbox PDB): writes the override byte.
pub fn bhk_world_set_pick_count_override(e: &mut Engine, enabled: u8) {
    e.set_global(GLOBAL_PICK_COUNT_OVERRIDE, enabled);
}

// Translated from 00c66240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::SethkObject` (Xbox PDB): forwards `this` to
/// `bhkRefObject::SethkObject` (`00c87c50`).
pub fn bhk_world_set_hk_object(e: &mut Engine, this: Ptr<BhkWorld>) {
    e.call(0x00c8_7c50, &args![this]);
}

// Translated from 00c66300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Increments the counter at `this + 0x18`.
pub fn bhk_world_increment_batch_add(e: &mut Engine, this: Ptr) {
    let count = e.mem.u32(this.addr() + 0x18);
    e.mem.set_u32(this.addr() + 0x18, count.wrapping_add(1));
}

// Translated from 00c66310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Increments the counter at `this + 0x1C`.
pub fn bhk_world_increment_batch_remove(e: &mut Engine, this: Ptr) {
    let count = e.mem.u32(this.addr() + 0x1c);
    e.mem.set_u32(this.addr() + 0x1c, count.wrapping_add(1));
}

// Translated from 00c66320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkQuaternion::setMul` (Xbox PDB): stores the product of `left` and
/// `right` in `this`, using the PC build's SSE single-precision operations.
pub fn hk_quaternion_set_mul(
    e: &mut Engine,
    this: Ptr<HkQuaternion>,
    left: Ptr<HkQuaternion>,
    right: Ptr<HkQuaternion>,
) {
    let [lx, ly, lz, lw] = [
        e.get(left, HkQuaternion::x),
        e.get(left, HkQuaternion::y),
        e.get(left, HkQuaternion::z),
        e.get(left, HkQuaternion::w),
    ];
    let [rx, ry, rz, rw] = [
        e.get(right, HkQuaternion::x),
        e.get(right, HkQuaternion::y),
        e.get(right, HkQuaternion::z),
        e.get(right, HkQuaternion::w),
    ];
    let result = [
        rw * lx + lw * rx + (rz * ly - ry * lz),
        rw * ly + lw * ry + (rx * lz - rz * lx),
        rw * lz + lw * rz + (ry * lx - rx * ly),
        rw * lw - (rz * lz + ry * ly + rx * lx),
    ];
    e.set(this, HkQuaternion::x, result[0]);
    e.set(this, HkQuaternion::y, result[1]);
    e.set(this, HkQuaternion::z, result[2]);
    e.set(this, HkQuaternion::w, result[3]);
}

// Translated from 00c663d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns a referenced-object pointer: decrements the old object's count,
/// deletes it at zero, then copies and increments the new object's count.
pub fn hk_ref_counted_pointer_assign(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let old = e.mem.u32(this.addr());
    let new = e.mem.u32(source.addr());
    if old != new {
        if old != 0 {
            let count = e.call(0x00fd_f1c4, &args![old + 4]).i32();
            if count == 0 {
                e.vcall(old, 4, &[]);
            }
        }
        e.mem.set_u32(this.addr(), new);
        if new != 0 {
            e.call(0x00fd_f1c0, &args![new + 4]);
        }
    }
    this
}

// Translated from 00c66420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollidableCollidableFilter` deleting destructor (Xbox PDB).
pub fn hkp_collidable_collidable_filter_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_3b80);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c66450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRayCollidableFilter` deleting destructor (Xbox PDB).
pub fn hkp_ray_collidable_filter_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_3b8c);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c66480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionFilter` deleting destructor (Xbox PDB).
pub fn hkp_shape_collection_filter_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_3b98);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c664b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRayShapeCollectionFilter` deleting destructor (Xbox PDB).
pub fn hkp_ray_shape_collection_filter_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_3bac);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c664e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadPhaseCastCollector` deleting destructor (Xbox PDB).
pub fn hkp_broad_phase_cast_collector_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_3bb8);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c66510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ahkpWorld` deleting destructor (Xbox PDB): installs its vtable,
/// destroys the `hkpWorld` base, and conditionally releases `this` through
/// the Havok memory router's vtable slot `+8` with the object's 16-bit size.
pub fn ahkp_world_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), HAVOK_WORLD_VTABLE);
    e.call(HAVOK_WORLD_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(0x00c8_5750, &[]).u32();
        let router_allocator = e.mem.u32(router + 0x10);
        let size = e.mem.u16(this.addr() + 4) as u32;
        e.vcall(router_allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00c66540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMemoryAllocator` deleting destructor (Xbox PDB): writes its vtable,
/// calls the base allocator cleanup, and returns `this`; the stack flag is
/// unused by this body.
pub fn bhk_memory_allocator_delete(e: &mut Engine, this: Ptr, _unused_0: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ALLOCATOR_VTABLE);
    e.call(MEMORY_ALLOCATOR_DESTRUCTOR, &args![this]);
    this
}

// Translated from 00c66560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryAllocator` deleting destructor (Xbox PDB): calls the base
/// allocator cleanup and returns `this`; its stack flag is unused.
pub fn hk_memory_allocator_delete(e: &mut Engine, this: Ptr, _unused_0: u32) -> Ptr {
    e.call(MEMORY_ALLOCATOR_DESTRUCTOR, &args![this]);
    this
}

// Translated from 00c66570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Performs the thread-local Havok startup callback at `01268450` with
/// argument 1, temporarily setting the TLS Havok memory state to 0x10.
/// Structured exception unwinding is not modeled.
pub fn bhk_on_startup_callback_invoke(e: &mut Engine) {
    let tls = e.tls();
    let previous = e.mem.u32(tls + 0x2b4);
    e.mem.set_u32(tls + 0x2b4, 0x10);
    let callback = e.global::<Ptr>(0x0126_8450);
    if !callback.is_null() {
        e.vcall(callback.addr(), 8, &args![1u32]);
    }
    e.mem.set_u32(tls + 0x2b4, previous);
}

// Translated from 00c66600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkOnStartupCallback` deleting destructor (Xbox PDB).
pub fn bhk_on_startup_callback_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010a_47b8);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c66630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjustor thunk: subtracts 8 from the `this` subobject and tail-calls
/// `00dff470`; the byte flag is forwarded unchanged.
pub fn bhk_on_startup_callback_delete_adjustor_8(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_f470, &args![this.addr() - 8, flags]).ptr()
}

// Translated from 00c66640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjustor thunk: subtracts 12 from the `this` subobject and tail-calls
/// `00dff470`; the byte flag is forwarded unchanged.
pub fn bhk_on_startup_callback_delete_adjustor_12(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_f470, &args![this.addr() - 0xc, flags]).ptr()
}

// Translated from 00c66650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjustor thunk: subtracts 16 from the `this` subobject and tail-calls
/// `00dff470`; the byte flag is forwarded unchanged.
pub fn bhk_on_startup_callback_delete_adjustor_16(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_f470, &args![this.addr() - 0x10, flags]).ptr()
}

// Translated from 00c66660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjustor thunk: subtracts 20 from the `this` subobject and tail-calls
/// `00dff470`; the byte flag is forwarded unchanged.
pub fn bhk_on_startup_callback_delete_adjustor_20(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_f470, &args![this.addr() - 0x14, flags]).ptr()
}

// Translated from 00c66670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tears down the Havok globals in the order used by the executable.
/// Structured exception unwinding is not modeled.
pub fn bhk_world_shutdown(e: &mut Engine) {
    bhk_on_startup_callback_invoke(e);
    e.call(0x00c6_d340, &[]);
    e.call(0x00c6_d3c0, &[]);
    let collision_filter = e.global::<Ptr>(0x0126_77a8);
    if !collision_filter.is_null() {
        e.call(0x00c9_05b0, &args![collision_filter]);
        e.set_global(0x0126_77a8, 0u32);
    }
    let data = e.global::<Ptr>(0x0126_77d0);
    if !data.is_null() {
        e.call(0x00cb_f430, &args![data]);
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, data]);
    }
    e.call(0x00cb_bcb0, &[]);
    let callback = e.global::<Ptr>(0x0126_77d4);
    e.vcall(callback.addr(), 8, &args![3u32]);
    e.vcall(callback.addr(), 0, &args![0u32]);
    e.set_global(0x0126_77d4, 0u32);
    e.call(0x00cb_bfb0, &args![0u32]);
}

// Translated from 00c66760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances the Havok step accumulator. The mode switch, thresholds, count
/// limits and smoothing are taken from the PC code; x87 stores are rounded
/// back to `f32` after each intermediate store.
pub fn bhk_world_accumulate_step(e: &mut Engine, delta: f32, single_step: u8, forced: u8) {
    const DELTA: u32 = 0x011a_fe64;
    const LAST_DELTA: u32 = 0x0126_77b0;
    const STEP_COUNT: u32 = 0x0126_77b4;
    const STEP_SIZE: u32 = 0x0126_77ac;
    const MODE: u32 = 0x0126_77b8;
    const FORCE: u32 = 0x0126_7b4c;
    const MAX_DELTA: u32 = 0x010c_3c28;
    const DEFAULT_SKIP_THRESHOLD: u32 = 0x010c_3c24;
    const FREQUENCY: u32 = 0x0126_7b38;
    const SCALE: u32 = 0x011a_c3a0;
    const SMOOTH_VALUE: u32 = 0x011a_fe6c;
    const SMOOTH_RATE: u32 = 0x011a_fe70;

    let previous_delta = e.global::<f32>(LAST_DELTA);
    let accumulated = (previous_delta as f64 + delta as f64) as f32;
    e.set_global(0x011a_fe68, delta);
    let force_step = forced != 0 || e.global::<u8>(FORCE) != 0;
    let max_delta = e.global::<f32>(MAX_DELTA);
    let accumulated = if accumulated <= max_delta {
        if accumulated < 0.0 {
            e.set_global(STEP_COUNT, 0u32);
            e.set_global(STEP_SIZE, 0.0f32);
            return;
        }
        accumulated
    } else {
        max_delta
    };
    e.set_global(DELTA, accumulated);
    let max_steps = if single_step == 0 { 3 } else { 1 };

    match e.global::<u32>(MODE) {
        0 => {
            let step_size =
                (e.global::<f32>(FREQUENCY) as f64 * e.global::<f32>(SCALE) as f64) as f32;
            e.set_global(STEP_SIZE, step_size);
            let mut steps = (accumulated as f64 / step_size as f64) as u32;
            e.set_global(STEP_COUNT, steps);
            if steps > max_steps {
                steps = max_steps;
                e.set_global(STEP_COUNT, steps);
            }
            if steps != 0 {
                let remaining = (accumulated as f64 - steps as f64 * step_size as f64) as f32;
                e.set_global(LAST_DELTA, remaining);
                let used = (accumulated as f64 - remaining as f64) as f32;
                e.set_global(DELTA, used);
                let clock = e.call(0x0040_3e20, &[]).u32();
                let clock_value = e.mem.f32(clock);
                let scaled = e.call(0x0040_ebd0, &args![remaining, clock_value]).f32();
                e.set_global(LAST_DELTA, scaled);
                return;
            }
            let half = e.global::<f64>(0x0101_1588);
            if !force_step && (accumulated as f64) < step_size as f64 * half {
                e.set_global(LAST_DELTA, accumulated);
                e.set_global(DELTA, 0.0f32);
                e.set_global(STEP_SIZE, 0.0f32);
                return;
            }
            e.set_global(STEP_SIZE, accumulated);
            e.set_global(STEP_COUNT, 1u32);
        }
        2 => {
            e.set_global(STEP_COUNT, 1u32);
            e.set_global(STEP_SIZE, delta);
        }
        10 => {
            let smooth = e.global::<f32>(SMOOTH_VALUE);
            let rate = e.global::<f32>(SMOOTH_RATE);
            let next = (smooth as f64 + rate as f64 * (accumulated as f64 - smooth as f64)) as f32;
            e.set_global(SMOOTH_VALUE, next);
            e.set_global(STEP_SIZE, next);
            e.set_global(STEP_COUNT, 1u32);
            e.set_global(LAST_DELTA, 0.0f32);
        }
        _ => {
            let cutoff = e.global::<f32>(DEFAULT_SKIP_THRESHOLD);
            if !force_step && accumulated < cutoff {
                e.set_global(STEP_COUNT, 0u32);
                e.set_global(LAST_DELTA, accumulated);
                e.set_global(DELTA, 0.0f32);
                e.set_global(STEP_SIZE, 0.0f32);
                return;
            }
            e.set_global(LAST_DELTA, 0.0f32);
            let frequency = e.global::<f32>(FREQUENCY);
            let mut steps = (accumulated as f64 / frequency as f64) as u32;
            if steps > max_steps {
                steps = max_steps;
            }
            if steps == 0 {
                e.set_global(STEP_SIZE, accumulated);
                e.set_global(LAST_DELTA, 0.0f32);
                e.set_global(STEP_COUNT, 1u32);
                return;
            }
            e.set_global(STEP_SIZE, (accumulated as f64 / steps as f64) as f32);
            e.set_global(STEP_COUNT, steps);
        }
    }
}

// Translated from 00c66a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the active solver's tau and damping according to the current
/// update mode, then updates the step count and reciprocal.
pub fn bhk_world_scale_solver_info(e: &mut Engine, this: Ptr) {
    const MODE: u32 = 0x0126_77b8;
    const STEP_SIZE: u32 = 0x0126_77ac;
    const FREQUENCY: u32 = 0x0126_7b38;
    const MINIMUM: u32 = 0x0126_7b14;
    const MAXIMUM: u32 = 0x0126_7b20;
    const ORIGINAL_TAU: u32 = 0x011a_fe60;
    const TAU: u32 = 0x0126_7b50;
    let world = if this.is_null() {
        0
    } else {
        e.vcall(this.addr(), 0x94, &[]).u32()
    };
    if e.global::<u32>(MODE) != 2 {
        let quotient = e.global::<f32>(STEP_SIZE) as f64 / e.global::<f32>(FREQUENCY) as f64;
        let scaled = ((quotient * e.global::<f64>(0x0101_db80)) as f32)
            .max(e.global::<f64>(0x0101_1590) as f32);
        let step_count = e.call(0x00ec_62c0, &args![scaled as f64]).i32();
        e.mem.set_i32(world + 0x2f4, step_count);
        e.mem.set_f32(world + 0x300, 1.0 / step_count as f32);
        let original_tau = e.global::<f32>(ORIGINAL_TAU);
        e.set_global(TAU, original_tau);
        let blend = scaled as f64;
        let tau = ((1.0 - original_tau as f64) * 0.6 + 0.6 * original_tau as f64 * blend) as f32;
        e.mem.set_f32(world + 0x1e4, tau);
        let damping = e.mem.f32(world + 0x1e8);
        e.mem.set_f32(world + 0x230, damping / tau);
        e.mem.set_f32(world + 0x234, tau / damping);
        return;
    }

    let ratio = e.global::<f32>(0x011a_fe68) / e.global::<f32>(MINIMUM);
    let rounded = e.call(0x00ec_9e10, &args![ratio as f64]).f32();
    let mut tau = e.global::<f32>(MINIMUM);
    let mut scale = rounded;
    if scale < e.global::<f32>(MAXIMUM) {
        tau = e.global::<f32>(0x011a_fe68) / e.global::<f32>(MAXIMUM);
        scale = e.global::<f32>(MAXIMUM);
    }
    let tau_field = e.mem.f32(world + 0x1e4);
    e.mem.set_f32(this.addr() + 0x88, tau_field);
    let damping_field = e.mem.f32(world + 0x1e8);
    e.mem.set_f32(this.addr() + 0x8c, damping_field);
    let ratio = e.global::<f32>(0x011a_fe68) / (tau * scale);
    let tau_new = ratio - 1.0 + tau_field;
    let damping_new = (damping_field as f64 + ratio as f64 - 1.0) as f32;
    e.mem.set_f32(world + 0x1e4, tau_new);
    e.mem.set_f32(world + 0x1e8, damping_new);
    let step_count = e.call(0x00cb_fef0, &args![tau_new]).i32();
    e.mem.set_i32(world + 0x2f4, step_count);
    e.mem.set_f32(world + 0x300, 1.0 / step_count as f32);
}

// Translated from 00c66be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the vector at `input` into this world's `Origin` vector by calling
/// `fn_00458620` (`hkVector4` scale helper).
pub fn bhk_world_scale_origin(e: &mut Engine, this: Ptr<BhkWorld>, input: Ptr) {
    e.call(0x0045_8620, &args![input, this.addr() + 0x60]);
}

// Translated from 00c66c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shifts the Havok broad phase to the requested origin and stores the
/// resulting origin on the `bhkWorld`.
pub fn bhk_world_shift_broad_phase(e: &mut Engine, this: Ptr<BhkWorld>, requested: Ptr) {
    let target = e.mem.bytes(requested.addr(), 16);
    let origin = e.mem.bytes(this.addr() + 0x60, 16);
    let mut delta = [0.0f32; 4];
    for i in 0..4 {
        let a = f32::from_le_bytes(target[i * 4..i * 4 + 4].try_into().unwrap());
        let b = f32::from_le_bytes(origin[i * 4..i * 4 + 4].try_into().unwrap());
        delta[i] = a - b;
    }
    e.with_stack(32, |e, vectors| {
        let delta_bytes: Vec<u8> = delta.into_iter().flat_map(f32::to_le_bytes).collect();
        e.mem.write(vectors.addr(), &delta_bytes);
        let output = vectors.addr() + 16;
        let world = e.vcall(this.addr(), 0x94, &[]).u32();
        e.call(0x00c9_6a20, &args![world, vectors.addr(), output, 0u32]);
        let shifted = e.mem.bytes(output, 16);
        e.vcall(this.addr(), 0x94, &[]);
        let mut new_origin = [0u8; 16];
        for i in 0..4 {
            let prior = f32::from_le_bytes(origin[i * 4..i * 4 + 4].try_into().unwrap());
            let moved = f32::from_le_bytes(shifted[i * 4..i * 4 + 4].try_into().unwrap());
            new_origin[i * 4..i * 4 + 4].copy_from_slice(&(prior + moved).to_le_bytes());
        }
        e.mem.write(this.addr() + 0x60, &new_origin);
    });
}

// Translated from 00c66c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Init` (Xbox PDB): allocates and constructs the 0x360-byte
/// Havok world, installs its game vtable, attaches the optional collision
/// filter, then invokes the world's setup virtuals. The trailing stack word
/// is the construction-info pointer. Structured exception unwinding is not modeled.
pub fn bhk_world_init(e: &mut Engine, this: Ptr<BhkWorld>, cinfo: Ptr) {
    let tls = e.tls();
    let saved_tls = e.mem.u32(tls + 0x2b4);
    e.mem.set_u32(tls + 0x2b4, 0x10);
    let router = e.call(0x00c8_5750, &[]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let havok_world = e.vcall(allocator, 4, &args![0x360u32]).u32();
    e.mem.set_u16(havok_world + 4, 0x360);
    let cinfo_base: Ptr = if cinfo.is_null() {
        Ptr::NULL
    } else {
        Ptr::new(cinfo.addr() - 0xf0)
    };
    e.call(0x00c9_5a80, &args![havok_world, cinfo_base, 0x111d4u32]);
    e.mem.set_u32(havok_world, HAVOK_WORLD_VTABLE);
    e.mem.set_u32(havok_world + 0x350, 0);
    let filter = e.global::<u32>(0x0126_77a8);
    if filter != 0 {
        e.call(0x00c9_77c0, &args![havok_world, filter, 1u32, 0u32, 1u32]);
    }
    e.vcall(this.addr(), 0x8c, &args![havok_world]);
    e.call(0x00c9_05b0, &args![havok_world]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    let world = e.vcall(this.addr(), 0x94, &[]).u32();
    if world != 0 {
        e.call(0x00cb_fd20, &args![e.mem.u32(world + 0x80)]);
    }
    e.mem.set_u32(tls + 0x2b4, saved_tls);
}

// Translated from 00c66de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Link` (Xbox PDB): after linking the base reference object,
/// stores `this` in the linked Havok world's `+0x350` field when bit 0 is set.
pub fn bhk_world_link(e: &mut Engine, this: Ptr<BhkWorld>, linked: u32) {
    if !this.is_null() {
        let world = e.vcall(this.addr(), 0x94, &[]).u32();
        if world != 0 {
            e.mem.set_u32(
                world + 0x350,
                if linked & 0xff != 0 { this.addr() } else { 0 },
            );
        }
    }
    e.call(0x00c8_7c90, &args![this, linked]);
}

// Translated from 00c66e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls slot `+0x18` on every non-null entry in `pEntityListenerA` when the
/// world's Havok object exists and `value` is nonzero.
pub fn bhk_world_set_solver_value(e: &mut Engine, this: Ptr<BhkWorld>, value: f32) {
    if e.mem.u32(this.addr() + 8) == 0 || value == 0.0 {
        return;
    }
    let items = e.get(this, BhkWorld::entityListeners).addr();
    let count = e.get(this, BhkWorld::entityListenerCount);
    for i in 0..count.max(0) as u32 {
        let listener = e.mem.u32(items + i * 4);
        if listener != 0 {
            e.vcall(listener, 0x18, &args![value]);
        }
    }
}

// Translated from 00c66e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::KillCinfo` (Xbox PDB): when requested, destroys the
/// `hkSerializableCinfo` base at `pInfo - 0xF0` and clears `pInfo`.
/// Structured exception unwinding is not modeled.
pub fn bhk_world_kill_cinfo(e: &mut Engine, this: Ptr<BhkWorld>, destroy: u8) {
    let tls = e.tls();
    let saved_tls = e.mem.u32(tls + 0x2b4);
    e.mem.set_u32(tls + 0x2b4, 0x10);
    if destroy != 0 {
        let info = e.get(this, BhkWorld::pInfo);
        if !info.is_null() {
            let base = info.addr() - 0xf0;
            e.vcall(base, 0, &args![1u32]);
        }
        e.set(this, BhkWorld::pInfo, Ptr::NULL);
    }
    e.mem.set_u32(tls + 0x2b4, saved_tls);
}

// Translated from 00c66f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns a collision filter's 16-bit group when empty. The group is taken
/// from the collidable's filter-info table, or from the wrapping global
/// counter (which skips zero and resumes at 10); existing groups are reused.
pub fn bhk_world_assign_collision_group(e: &mut Engine, collidable: Ptr, filter: Ptr) -> bool {
    let world = e.mem.u32(collidable.addr() + 8);
    let filter_info = if world == 0 || world == 0xffff_fff0 {
        0
    } else {
        e.mem.u32(world + 0x2c)
    };
    let encoded_group = filter_info >> 16;
    let existing = e.mem.u32(filter.addr() + 0x10);
    if existing != 0 {
        e.call(0x00c8_5ce0, &args![existing]);
        return true;
    }
    let group = if encoded_group != 0 {
        encoded_group
    } else {
        let index = filter_info & 0x7f;
        let configured = e.mem.u32(0x0126_7e70 + index * 4);
        if configured != 0 {
            configured
        } else {
            let next = e.global::<u32>(0x011b_0808).wrapping_add(1) & 0xffff;
            let next = if next == 0 { 10 } else { next };
            e.set_global(0x011b_0808, next);
            next
        }
    };
    e.mem.set_u32(filter.addr() + 0x10, group);
    if encoded_group == 0 {
        e.call(0x00c8_5ce0, &args![group]);
    }
    true
}

// Translated from 00c66f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::HasActiveItems` (Xbox PDB): true when the `hkpWorld` returned
/// by slot `+0x94` has a nonzero active-island count at `+0x2C`.
pub fn bhk_world_has_active_items(e: &mut Engine, this: Ptr<BhkWorld>) -> bool {
    let world = e.vcall(this.addr(), 0x94, &[]).u32();
    world != 0 && e.mem.u32(world + 0x2c) != 0
}

// Translated from 00c66fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPickData::GetNiAVObject` (Xbox PDB): resolves the root collidable
/// through `bhkUtilFunctions::GetNiAVObject` (`00c7fa90`), or returns null.
pub fn bhk_pick_data_get_ni_av_object(e: &mut Engine, this: Ptr<BhkPickData>) -> Ptr {
    let collidable = e.get(this, BhkPickData::rootCollidable);
    if collidable.is_null() {
        Ptr::NULL
    } else {
        e.call(0x00c7_fa90, &args![collidable]).ptr::<()>()
    }
}

// Translated from 00c66fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPickData::GethkRigidBody` (Xbox PDB): returns the body subobject
/// inside `m_rootCollidable` when its byte at `+0x18` equals 1.
pub fn bhk_pick_data_get_hk_rigid_body(e: &mut Engine, this: Ptr<BhkPickData>) -> Ptr {
    let collidable = e.get(this, BhkPickData::rootCollidable);
    if collidable.is_null() || e.mem.u8(collidable.addr() + 0x18) != 1 {
        return Ptr::NULL;
    }
    Ptr::new(
        collidable
            .addr()
            .wrapping_add_signed(e.mem.i8(collidable.addr() + 0x10) as i32),
    )
}

// Translated from 00c66ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the Havok collision properties held by `filter` to the collidable
/// held by `agent`, including collision flags, world links, zeroed contact
/// data, and the activation call. All virtual calls retain their PC slots.
pub fn bhk_collision_agent_update(e: &mut Engine, agent: Ptr, filter: Ptr) {
    let collidable = e.mem.u32(agent.addr() + 0x10);
    if collidable != 0 {
        let property = e.vcall(agent.addr(), 0x3c, &[]).u32();
        if property != 0 {
            let flags = e.mem.u16(filter.addr() + 0x14);
            let current = e.mem.u16(property + 0xc);
            e.mem.set_u16(
                property + 0xc,
                if flags == 0 {
                    current | 0x100
                } else {
                    current & 0xfeff
                },
            );
        }
        let flags = e.mem.u16(agent.addr() + 0xc);
        if e.mem.u8(filter.addr() + 0xc) & 1 != 0 {
            e.mem.set_u16(agent.addr() + 0xc, flags | 0x0c);
        } else {
            e.mem.set_u16(agent.addr() + 0xc, flags & 0xfffb);
        }
        if e.global::<u8>(0x011a_fe58) != 0 {
            bhk_world_assign_collision_group(e, Ptr::new(collidable), filter);
        }
        let filter_world = e.vcall(e.mem.u32(filter.addr()), 0x94, &[]).u32();
        let object_world = e.vcall(collidable, 0x94, &[]).u32();
        if object_world != filter_world {
            let current = e.mem.u16(agent.addr() + 0xc);
            if e.mem.u32(filter.addr() + 0x18) == 0 {
                e.mem.set_u16(agent.addr() + 0xc, current & 0xffbf);
            } else {
                e.mem.set_u16(agent.addr() + 0xc, current | 0x40);
            }
            e.vcall(collidable, 0x9c, &args![e.mem.u32(filter.addr())]);
        }
        e.with_stack(12, |e, contacts| {
            e.mem.set_f32(contacts.addr(), 0.0);
            e.mem.write(contacts.addr() + 4, &[0; 5]);
            let data_owner = e.mem.u32(agent.addr() + 8);
            e.vcall(data_owner, 0xb8, &args![contacts.addr()]);
        });
        let filter_object = e.mem.u32(filter.addr());
        if e.mem.u8(filter_object + 0x16) != 0 {
            e.vcall(agent.addr(), 0xc0, &args![1u32]);
        }
    }
    let filter_flags = e.mem.u32(filter.addr() + 0xc);
    e.mem.set_u32(filter.addr() + 0xc, filter_flags & !1);
    let count = e.mem.u32(filter.addr() + 0x14);
    e.mem.set_u32(filter.addr() + 0x14, count.wrapping_add(1));
}

// Translated from 00c67320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates a Havok shape's transform from the four-float vector at
/// `transform + 0xC`; scales it by an optional property and skips a redundant
/// near-zero offset before calling shape slots `+0xD4` and `+0xDC`.
pub fn bhk_collision_agent_update_shape_transform(e: &mut Engine, agent: Ptr, transform: Ptr) {
    let material = e.vcall(agent.addr(), 0x3c, &[]).u32();
    let shape_owner = e.mem.u32(agent.addr() + 0x10);
    if shape_owner == 0 {
        return;
    }
    let shape = e.vcall(shape_owner, 0x40, &[]).u32();
    if shape == 0 {
        return;
    }
    let vector = e.mem.u32(transform.addr() + 0xc);
    if vector == 0 {
        return;
    }
    let mut offset = [0.0f32; 4];
    for (i, value) in offset.iter_mut().enumerate() {
        *value = e.mem.f32(vector + i as u32 * 4);
    }
    if material != 0 {
        let scale = e.mem.f32(material + 0x18);
        if scale < 1.0 {
            if scale == 0.0 {
                return;
            }
            let multiplier = e.mem.f32(material + 0x1c);
            for component in &mut offset {
                *component *= multiplier as f64 as f32;
            }
            let zero_vector = [
                e.global::<f32>(0x0126_7e30),
                e.global::<f32>(0x0126_7e34),
                e.global::<f32>(0x0126_7e38),
                e.global::<f32>(0x0126_7e3c),
            ];
            let epsilon = e.global::<f32>(0x0101_7d00);
            if (0..3).all(|i| (offset[i] - zero_vector[i]).abs() <= epsilon) {
                return;
            }
        }
    }
    e.with_stack(32, |e, vectors| {
        e.vcall(shape, 0xd4, &args![vectors.addr() + 16]);
        let mut result = [0u8; 16];
        for i in 0..4 {
            let base = e.mem.f32(vectors.addr() + 16 + i as u32 * 4);
            result[i * 4..i * 4 + 4].copy_from_slice(&(base + offset[i]).to_le_bytes());
        }
        e.mem.write(vectors.addr() + 16, &result);
        e.vcall(shape, 0xdc, &args![vectors.addr() + 16]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_6170, bhk_memory_allocator_block_alloc(u32) -> Ptr),
        entry!(0x00c6_61a0, bhk_memory_allocator_block_free(Ptr, u32)),
        entry!(0x00c6_61c0, bhk_world_return_second_argument(u32, u32) -> u32),
        entry!(0x00c6_61d0, bhk_world_set_update_mode(u32)),
        entry!(0x00c6_6210, bhk_world_reset_step_accumulators()),
        entry!(0x00c6_6230, bhk_world_set_pick_count_override(u8)),
        entry!(0x00c6_6240, bhk_world_set_hk_object(Ptr<BhkWorld>)),
        entry!(0x00c6_6300, bhk_world_increment_batch_add(Ptr)),
        entry!(0x00c6_6310, bhk_world_increment_batch_remove(Ptr)),
        entry!(
            0x00c6_6320,
            hk_quaternion_set_mul(Ptr<HkQuaternion>, Ptr<HkQuaternion>, Ptr<HkQuaternion>)
        ),
        entry!(0x00c6_63d0, hk_ref_counted_pointer_assign(Ptr, Ptr) -> Ptr),
        entry!(0x00c6_6420, hkp_collidable_collidable_filter_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_6450, hkp_ray_collidable_filter_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_6480, hkp_shape_collection_filter_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_64b0, hkp_ray_shape_collection_filter_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_64e0, hkp_broad_phase_cast_collector_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_6510, ahkp_world_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_6540, bhk_memory_allocator_delete(Ptr, u32) -> Ptr),
        entry!(0x00c6_6560, hk_memory_allocator_delete(Ptr, u32) -> Ptr),
        entry!(0x00c6_6570, bhk_on_startup_callback_invoke()),
        entry!(0x00c6_6600, bhk_on_startup_callback_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_6630, bhk_on_startup_callback_delete_adjustor_8(Ptr, u8) -> Ptr),
        entry!(0x00c6_6640, bhk_on_startup_callback_delete_adjustor_12(Ptr, u8) -> Ptr),
        entry!(0x00c6_6650, bhk_on_startup_callback_delete_adjustor_16(Ptr, u8) -> Ptr),
        entry!(0x00c6_6660, bhk_on_startup_callback_delete_adjustor_20(Ptr, u8) -> Ptr),
        entry!(0x00c6_6670, bhk_world_shutdown()),
        entry!(0x00c6_6760, bhk_world_accumulate_step(f32, u8, u8)),
        entry!(0x00c6_6a00, bhk_world_scale_solver_info(Ptr)),
        entry!(0x00c6_6be0, bhk_world_scale_origin(Ptr<BhkWorld>, Ptr)),
        entry!(0x00c6_6c00, bhk_world_shift_broad_phase(Ptr<BhkWorld>, Ptr)),
        entry!(0x00c6_6c90, bhk_world_init(Ptr<BhkWorld>, Ptr)),
        entry!(0x00c6_6de0, bhk_world_link(Ptr<BhkWorld>, u32)),
        entry!(0x00c6_6e20, bhk_world_set_solver_value(Ptr<BhkWorld>, f32)),
        entry!(0x00c6_6e70, bhk_world_kill_cinfo(Ptr<BhkWorld>, u8)),
        entry!(0x00c6_6f10, bhk_world_assign_collision_group(Ptr, Ptr) -> bool),
        entry!(
            0x00c6_6f90,
            bhk_world_has_active_items(Ptr<BhkWorld>) -> bool
        ),
        entry!(
            0x00c6_6fb0,
            bhk_pick_data_get_ni_av_object(Ptr<BhkPickData>) -> Ptr
        ),
        entry!(
            0x00c6_6fd0,
            bhk_pick_data_get_hk_rigid_body(Ptr<BhkPickData>) -> Ptr
        ),
        entry!(0x00c6_6ff0, bhk_collision_agent_update(Ptr, Ptr)),
        entry!(
            0x00c6_7320,
            bhk_collision_agent_update_shape_transform(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn block_alloc_adds_slack_and_stores_the_alignment_distance() {
        let mut e = engine();
        let raw = 0x0300_0003;
        e.map(0x0300_0000, 0x1000);
        e.register_double(HEAP_ALLOC, move |_, _| ret(raw));
        e.call_log = Some(vec![]);
        let block = e.call(0x00c6_6170, &args![0x20u32]).ptr::<()>();
        assert_eq!(block.addr(), raw + 0xd);
        assert_eq!(e.mem.u8(block.addr() - 1), 0xd);
        let call = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(a, _)| *a == HEAP_ALLOC)
            .unwrap();
        assert_eq!(call.1, [HEAP_SINGLETON, 0x30]);
    }

    #[test]
    fn block_free_recovers_the_raw_address_and_skips_null() {
        let mut e = engine();
        e.register_double(HEAP_FREE, move |_, _| Ret::default());
        let block = 0x0300_1010;
        e.map(0x0300_1000, 0x1000);
        e.mem.set_u8(block - 1, 8);
        e.call_log = Some(vec![]);
        e.call(0x00c6_61a0, &args![block, 0xfeedu32]);
        e.call(0x00c6_61a0, &args![0u32, 0xfeedu32]);
        let frees: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == HEAP_FREE)
            .collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, [HEAP_SINGLETON, block - 8]);
    }

    #[test]
    fn second_argument_helper_returns_the_second_word() {
        let mut e = engine();
        assert_eq!(e.call(0x00c6_61c0, &args![17u32, 29u32]).u32(), 29);
    }

    #[test]
    fn world_mode_mapping_matches_the_three_executable_cases() {
        let mut e = engine();
        for (input, expected) in [(0, 1), (1, 2), (4, 3), (2, 2), (u32::MAX, 2)] {
            e.call(0x00c6_61d0, &args![input]);
            assert_eq!(e.global::<u32>(GLOBAL_WORLD_MODE), expected);
        }
    }

    #[test]
    fn reset_world_step_clears_both_accumulators() {
        let mut e = engine();
        e.set_global(GLOBAL_WORLD_INTEGER, 17u32);
        e.set_global(GLOBAL_WORLD_FLOAT, 1.25f32);
        e.call(0x00c6_6210, &[]);
        assert_eq!(e.global::<u32>(GLOBAL_WORLD_INTEGER), 0);
        assert_eq!(e.global::<f32>(GLOBAL_WORLD_FLOAT), 0.0);
    }

    #[test]
    fn pick_count_override_stores_the_low_byte() {
        let mut e = engine();
        e.call(0x00c6_6230, &args![0x1ffu32]);
        assert_eq!(e.global::<u8>(GLOBAL_PICK_COUNT_OVERRIDE), 0xff);
    }

    #[test]
    fn set_hk_object_forwards_this_to_the_base_method() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        e.register_double(0x00c8_7c50, move |_, a| {
            assert_eq!(a, [this.addr()]);
            Ret::default()
        });
        e.call(0x00c6_6240, &args![this]);
    }

    #[test]
    fn first_batch_counter_wraps() {
        let mut e = engine();
        let this: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr() + 0x18, u32::MAX);
        e.call(0x00c6_6300, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 0);
    }

    #[test]
    fn second_batch_counter_increments_at_its_own_offset() {
        let mut e = engine();
        let this: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr() + 0x18, 6);
        e.mem.set_u32(this.addr() + 0x1c, 9);
        e.call(0x00c6_6310, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 6);
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 10);
    }

    #[test]
    fn quaternion_product_writes_the_sse_product() {
        let mut e = engine();
        let out: Ptr<HkQuaternion> = e.new_object();
        let left: Ptr<HkQuaternion> = e.new_object();
        let right: Ptr<HkQuaternion> = e.new_object();
        e.set(left, HkQuaternion::x, 1.0);
        e.set(left, HkQuaternion::y, 2.0);
        e.set(left, HkQuaternion::z, 3.0);
        e.set(left, HkQuaternion::w, 4.0);
        e.set(right, HkQuaternion::x, 5.0);
        e.set(right, HkQuaternion::y, 6.0);
        e.set(right, HkQuaternion::z, 7.0);
        e.set(right, HkQuaternion::w, 8.0);
        e.call(0x00c6_6320, &args![out, left, right]);
        assert_eq!(e.get(out, HkQuaternion::x), 24.0);
        assert_eq!(e.get(out, HkQuaternion::y), 48.0);
        assert_eq!(e.get(out, HkQuaternion::z), 48.0);
        assert_eq!(e.get(out, HkQuaternion::w), -6.0);
    }

    #[test]
    fn shared_pointer_assignment_releases_old_and_retains_new() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        let source = e.mem.alloc(4);
        let old = e.mem.alloc(8);
        let new = e.mem.alloc(8);
        e.map(0x0400_0000, 0x1000);
        e.mem.set_u32(this, old);
        e.mem.set_u32(source, new);
        e.mem.set_u32(old, 0x0400_0000);
        e.mem.set_i32(old + 4, 1);
        e.mem.set_i32(new + 4, 4);
        e.put_vtable(0x0400_0000, &[0, 0x0400_0100]);
        e.register_double(0x00fd_f1c4, move |e, a| {
            let n = e.mem.i32(a[0]) - 1;
            e.mem.set_i32(a[0], n);
            ret(n as u32)
        });
        e.register_double(0x00fd_f1c0, move |e, a| {
            let n = e.mem.i32(a[0]) + 1;
            e.mem.set_i32(a[0], n);
            ret(n as u32)
        });
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a[0], old);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c6_63d0, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u32(this), new);
        assert_eq!(e.mem.i32(old + 4), 0);
        assert_eq!(e.mem.i32(new + 4), 5);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x0400_0100));
    }

    fn assert_filter_destructor(address: u32, expected_vtable: u32) {
        let mut e = engine();
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.register_double(HEAP_FREE, move |_, a| {
            assert_eq!(a, [HEAP_SINGLETON, this]);
            Ret::default()
        });
        e.call(address, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this), expected_vtable);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == HEAP_FREE)
                .count(),
            1
        );
    }

    #[test]
    fn collidable_filter_destructor_sets_vtable_and_frees_when_flagged() {
        assert_filter_destructor(0x00c6_6420, 0x010c_3b80);
    }

    #[test]
    fn ray_collidable_filter_destructor_sets_vtable_and_frees_when_flagged() {
        assert_filter_destructor(0x00c6_6450, 0x010c_3b8c);
    }

    #[test]
    fn shape_collection_filter_destructor_sets_vtable_and_frees_when_flagged() {
        assert_filter_destructor(0x00c6_6480, 0x010c_3b98);
    }

    #[test]
    fn ray_shape_filter_destructor_sets_vtable_and_frees_when_flagged() {
        assert_filter_destructor(0x00c6_64b0, 0x010c_3bac);
    }

    #[test]
    fn broad_phase_collector_destructor_sets_vtable_and_frees_when_flagged() {
        assert_filter_destructor(0x00c6_64e0, 0x010c_3bb8);
    }

    #[test]
    fn ahkp_world_destructor_destroys_base_and_uses_router_for_delete() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let router = e.mem.alloc(0x14);
        let allocator = e.mem.alloc(4);
        e.map(0x0400_0000, 0x1000);
        e.mem.set_u16(this + 4, 0x360);
        e.mem.set_u32(router + 0x10, allocator);
        e.put_vtable(0x0400_0000, &[0, 0, 0x0400_0100]);
        e.mem.set_u32(allocator, 0x0400_0000);
        e.register_double(HAVOK_WORLD_DESTRUCTOR, move |_, a| {
            assert_eq!(a, [this]);
            Ret::default()
        });
        e.register_double(0x00c8_5750, move |_, _| ret(router));
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a, [allocator, this, 0x360]);
            Ret::default()
        });
        e.call(0x00c6_6510, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this), HAVOK_WORLD_VTABLE);
    }

    #[test]
    fn bhk_memory_allocator_destructor_sets_vtable_and_calls_base_cleanup() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        e.register_double(MEMORY_ALLOCATOR_DESTRUCTOR, move |_, a| {
            assert_eq!(a[0], this);
            Ret::default()
        });
        e.call(0x00c6_6540, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), ALLOCATOR_VTABLE);
    }

    #[test]
    fn hk_memory_allocator_destructor_calls_base_without_changing_vtable() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x1234);
        e.register_double(MEMORY_ALLOCATOR_DESTRUCTOR, move |_, a| {
            assert_eq!(a[0], this);
            Ret::default()
        });
        e.call(0x00c6_6560, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), 0x1234);
    }

    #[test]
    fn startup_callback_uses_vtable_slot_eight_and_restores_tls() {
        let mut e = engine();
        let callback = e.mem.alloc(4);
        let table = 0x0400_0000;
        e.put_vtable(table, &[0, 0, 0x0400_0100]);
        e.mem.set_u32(callback, table);
        e.set_global(0x0126_8450, callback);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 7);
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a, [callback, 1]);
            Ret::default()
        });
        e.call(0x00c6_6570, &[]);
        assert_eq!(e.mem.u32(tls + 0x2b4), 7);
    }

    #[test]
    fn startup_callback_destructor_installs_vtable_and_obeys_free_flag() {
        assert_filter_destructor(0x00c6_6600, 0x010a_47b8);
    }

    fn assert_adjustor(address: u32, adjustment: u32) {
        let mut e = engine();
        let object = 0x0500_1000;
        e.register_double(0x00df_f470, move |_, a| {
            assert_eq!(a[0], object - adjustment);
            assert_eq!(a[1], 1);
            ret(a[0])
        });
        assert_eq!(
            e.call(address, &args![object, 1u32]).u32(),
            object - adjustment
        );
    }

    #[test]
    fn adjustor_thunk_subtracts_eight() {
        assert_adjustor(0x00c6_6630, 8);
    }

    #[test]
    fn adjustor_thunk_subtracts_twelve() {
        assert_adjustor(0x00c6_6640, 12);
    }

    #[test]
    fn adjustor_thunk_subtracts_sixteen() {
        assert_adjustor(0x00c6_6650, 16);
    }

    #[test]
    fn adjustor_thunk_subtracts_twenty() {
        assert_adjustor(0x00c6_6660, 20);
    }

    #[test]
    fn teardown_runs_callbacks_and_clears_owned_globals_in_order() {
        let mut e = engine();
        let startup = e.mem.alloc(4);
        let callback = e.mem.alloc(4);
        e.put_vtable(0x0400_0000, &[0x0400_0100, 0, 0x0400_0108]);
        e.mem.set_u32(startup, 0x0400_0000);
        e.mem.set_u32(callback, 0x0400_0000);
        e.set_global(0x0126_8450, startup);
        e.set_global(0x0126_77d4, callback);
        e.set_global(0x0126_77a8, 0u32);
        e.set_global(0x0126_77d0, 0u32);
        for address in [
            0x00c6_d340,
            0x00c6_d3c0,
            0x00cb_bcb0,
            0x00cb_bfb0,
            0x0400_0100,
            0x0400_0108,
        ] {
            e.register_double(address, move |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00c6_6670, &[]);
        let addresses: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|(a, _)| a)
            .collect();
        let expected = [
            0x00c6_6670,
            0x0400_0108,
            0x00c6_d340,
            0x00c6_d3c0,
            0x00cb_bcb0,
            0x0400_0108,
            0x0400_0100,
            0x00cb_bfb0,
        ];
        assert_eq!(addresses, expected);
        assert_eq!(e.global::<u32>(0x0126_77d4), 0);
    }

    #[test]
    fn step_accumulator_handles_negative_and_manual_modes() {
        let mut e = engine();
        e.set_global(0x010c_3c28, 166.66667f32);
        e.set_global(0x010c_3c24, 0.008333334f32);
        e.set_global(0x0126_77b8, 2u32);
        e.set_global(0x0126_77b0, 0.0f32);
        e.call(0x00c6_6760, &args![-1.0f32, 0u32, 0u32]);
        assert_eq!(e.global::<u32>(0x0126_77b4), 0);
        assert_eq!(e.global::<f32>(0x0126_77ac), 0.0);
        e.call(0x00c6_6760, &args![0.25f32, 1u32, 0u32]);
        assert_eq!(e.global::<u32>(0x0126_77b4), 1);
        assert_eq!(e.global::<f32>(0x0126_77ac), 0.25);
    }

    #[test]
    fn scale_solver_info_uses_mode_one_and_writes_solver_fields() {
        let mut e = engine();
        let bhk: Ptr = Ptr::new(e.mem.alloc(4));
        let world = e.mem.alloc(0x304);
        let table = 0x0400_0000;
        let mut bhk_slots = vec![0; 0x26];
        bhk_slots[0x94 / 4] = 0x0400_0094;
        e.put_vtable(table, &bhk_slots);
        e.mem.set_u32(bhk.addr(), table);
        e.register_double(0x0400_0094, move |_, _| ret(world));
        e.set_global(0x0126_77b8, 0u32);
        e.set_global(0x0126_77ac, 0.5f32);
        e.set_global(0x0126_7b38, 1.0f32);
        e.set_global(0x011a_fe60, 0.5f32);
        e.set_global(0x0101_db80, 4.0f64);
        e.set_global(0x0101_1590, 2.0f64);
        e.register_double(0x00ec_62c0, move |_, _| ret(2));
        e.mem.set_f32(world + 0x1e8, 0.75);
        e.call(0x00c6_6a00, &args![bhk]);
        assert_eq!(e.mem.i32(world + 0x2f4), 2);
        assert_eq!(e.mem.f32(world + 0x300), 0.5);
        assert_eq!(e.global::<f32>(0x0126_7b50), 0.5);
        assert_eq!(e.mem.f32(world + 0x1e4), 0.9);
    }

    #[test]
    fn origin_scale_helper_passes_source_then_destination() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let input = e.mem.alloc(16);
        e.register_double(0x0045_8620, move |_, a| {
            assert_eq!(a, [input, this.addr() + 0x60]);
            Ret::default()
        });
        e.call(0x00c6_6be0, &args![this, input]);
    }

    #[test]
    fn broad_phase_shift_updates_origin_from_the_shifted_delta() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let requested = e.mem.alloc(16);
        let world = e.mem.alloc(4);
        let world_table = 0x0400_0000;
        let bhk_table = 0x0400_1000;
        e.put_vtable(world_table, &[0; 0x26]);
        let mut bhk_slots = vec![0; 0x26];
        bhk_slots[0x94 / 4] = 0x0400_0094;
        e.put_vtable(bhk_table, &bhk_slots);
        e.mem.set_u32(this.addr(), bhk_table);
        for i in 0..4 {
            e.mem.set_f32(this.addr() + 0x60 + i * 4, i as f32);
            e.mem.set_f32(requested + i * 4, (i + 10) as f32);
        }
        e.register_double(0x0400_0094, move |_, _| ret(world));
        e.register_double(0x00c9_6a20, move |e, a| {
            assert_eq!(a[0], world);
            assert_eq!(
                (0..4).map(|i| e.mem.f32(a[1] + i * 4)).collect::<Vec<_>>(),
                [10.0, 10.0, 10.0, 10.0]
            );
            for i in 0..4 {
                e.mem.set_f32(a[2] + i * 4, 2.0);
            }
            assert_eq!(a[3], 0);
            Ret::default()
        });
        e.call(0x00c6_6c00, &args![this, requested]);
        for i in 0..4 {
            assert_eq!(e.mem.f32(this.addr() + 0x60 + i * 4), i as f32 + 2.0);
        }
    }

    #[test]
    fn bhk_world_init_builds_and_attaches_havok_world_in_call_order() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let world = e.mem.alloc(0x360);
        let allocator = e.mem.alloc(4);
        let router = e.mem.alloc(0x14);
        let bhk_table = 0x0400_0000;
        let allocator_table = 0x0400_1000;
        let mut bhk_slots = vec![0; 0x30];
        bhk_slots[0x8c / 4] = 0x0400_008c;
        bhk_slots[0x94 / 4] = 0x0400_0094;
        bhk_slots[0xbc / 4] = 0x0400_00bc;
        e.put_vtable(bhk_table, &bhk_slots);
        e.put_vtable(allocator_table, &[0, 0x0400_0100]);
        e.mem.set_u32(this.addr(), bhk_table);
        e.mem.set_u32(allocator, allocator_table);
        e.mem.set_u32(router + 0x10, allocator);
        e.set(this, BhkWorld::pInfo, Ptr::new(0x0700_0000));
        e.set_global(0x0126_77a8, 0x0710_0000u32);
        e.register_double(0x00c8_5750, move |_, _| ret(router));
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a, [allocator, 0x360]);
            ret(world)
        });
        e.register_double(0x00c9_5a80, move |_, a| {
            assert_eq!(a, [world, 0x0700_0000, 0x111d4]);
            Ret::default()
        });
        e.register_double(0x00c9_77c0, move |_, a| {
            assert_eq!(a, [world, 0x0710_0000, 1, 0, 1]);
            Ret::default()
        });
        e.register_double(0x0400_008c, move |_, a| {
            assert_eq!(a, [this.addr(), world]);
            Ret::default()
        });
        e.register_double(0x00c9_05b0, move |_, a| {
            assert_eq!(a, [world]);
            Ret::default()
        });
        e.register_double(0x0400_00bc, move |_, a| {
            assert_eq!(a, [this.addr(), 0x0700_00f0]);
            Ret::default()
        });
        e.register_double(0x0400_0094, move |_, _| ret(world));
        e.register_double(0x00cb_fd20, move |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_6c90, &args![this, 0x0700_00f0u32]);
        let calls = e.call_log.as_ref().unwrap();
        let addresses: Vec<_> = calls.iter().map(|(address, _)| *address).collect();
        let constructor = addresses.iter().position(|a| *a == 0x00c9_5a80).unwrap();
        let filter = addresses.iter().position(|a| *a == 0x00c9_77c0).unwrap();
        let attach = addresses.iter().position(|a| *a == 0x0400_008c).unwrap();
        let setup = addresses.iter().position(|a| *a == 0x00c9_05b0).unwrap();
        let info = addresses.iter().position(|a| *a == 0x0400_00bc).unwrap();
        assert!(constructor < filter && filter < attach && attach < setup && setup < info);
        assert_eq!(e.mem.u32(world), HAVOK_WORLD_VTABLE);
        assert_eq!(e.mem.u32(world + 0x350), 0);
        assert_eq!(e.mem.u16(world + 4), 0x360);
    }

    #[test]
    fn link_sets_world_pointer_only_when_link_flag_is_set() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let world = e.mem.alloc(0x354);
        let table = 0x0400_0000;
        let mut slots = vec![0; 0x26];
        slots[0x94 / 4] = 0x0400_0094;
        e.put_vtable(table, &slots);
        e.mem.set_u32(this.addr(), table);
        e.register_double(0x0400_0094, move |_, _| ret(world));
        e.register_double(0x00c8_7c90, move |_, _| Ret::default());
        e.call(0x00c6_6de0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(world + 0x350), this.addr());
        e.call(0x00c6_6de0, &args![this, 0x100u32]);
        assert_eq!(e.mem.u32(world + 0x350), 0);
    }

    #[test]
    fn entity_listener_update_skips_zero_and_calls_each_non_null_entry() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let listeners = e.mem.alloc(12);
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        e.mem.set_u32(this.addr() + 8, 1);
        e.set(this, BhkWorld::entityListeners, Ptr::new(listeners));
        e.set(this, BhkWorld::entityListenerCount, 3);
        e.mem.set_u32(listeners, a);
        e.mem.set_u32(listeners + 4, 0);
        e.mem.set_u32(listeners + 8, b);
        let mut listener_slots = vec![0; 7];
        listener_slots[0x18 / 4] = 0x0400_0100;
        e.put_vtable(0x0400_0000, &listener_slots);
        e.mem.set_u32(a, 0x0400_0000);
        e.mem.set_u32(b, 0x0400_0000);
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a[1], 2.5f32.to_bits());
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c6_6e20, &args![this, 2.5f32]);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == 0x0400_0100)
                .count(),
            2
        );
    }

    #[test]
    fn kill_cinfo_clears_pointer_and_calls_deleting_destructor_when_requested() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let base = e.mem.alloc(4);
        e.set(this, BhkWorld::pInfo, Ptr::new(base + 0xf0));
        e.put_vtable(0x0400_0000, &[0x0400_0100]);
        e.mem.set_u32(base, 0x0400_0000);
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a, [base, 1]);
            Ret::default()
        });
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 3);
        e.call(0x00c6_6e70, &args![this, 1u32]);
        assert!(e.get(this, BhkWorld::pInfo).is_null());
        assert_eq!(e.mem.u32(tls + 0x2b4), 3);
    }

    #[test]
    fn collision_filter_reuses_existing_group_and_assigns_encoded_group() {
        let mut e = engine();
        let collidable = e.mem.alloc(0x30);
        let filter = e.mem.alloc(0x14);
        let world = e.mem.alloc(0x30);
        e.mem.set_u32(collidable + 8, world);
        e.mem.set_u32(world + 0x2c, 0x1234_0000);
        e.mem.set_u32(filter + 0x10, 77);
        e.register_double(0x00c8_5ce0, move |_, a| {
            assert!(a == [77] || a == [0x1234]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c6_6f10, &args![collidable, filter]).bool());
        assert_eq!(e.mem.u32(filter + 0x10), 77);
        e.mem.set_u32(filter + 0x10, 0);
        assert!(e.call(0x00c6_6f10, &args![collidable, filter]).bool());
        assert_eq!(e.mem.u32(filter + 0x10), 0x1234);
        let groups: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00c8_5ce0)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(groups, [vec![77]]);
    }

    #[test]
    fn has_active_items_reads_the_havok_world_count() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let world = e.mem.alloc(0x30);
        let mut slots = vec![0; 0x26];
        slots[0x94 / 4] = 0x0400_0094;
        e.put_vtable(0x0400_0000, &slots);
        e.mem.set_u32(this.addr(), 0x0400_0000);
        e.mem.set_u32(world + 0x2c, 3);
        e.register_double(0x0400_0094, move |_, _| ret(world));
        assert!(e.call(0x00c6_6f90, &args![this]).bool());
        e.mem.set_u32(world + 0x2c, 0);
        assert!(!e.call(0x00c6_6f90, &args![this]).bool());
    }

    #[test]
    fn pick_data_resolves_ni_object_only_for_a_root_collidable() {
        let mut e = engine();
        let this: Ptr<BhkPickData> = e.new_object();
        let collidable = e.mem.alloc(4);
        e.set(this, BhkPickData::rootCollidable, Ptr::new(collidable));
        e.register_double(0x00c7_fa90, move |_, a| {
            assert_eq!(a[0], collidable);
            ret(0x0700_1234)
        });
        assert_eq!(e.call(0x00c6_6fb0, &args![this]).u32(), 0x0700_1234);
        e.set(this, BhkPickData::rootCollidable, Ptr::NULL);
        assert_eq!(e.call(0x00c6_6fb0, &args![this]).u32(), 0);
    }

    #[test]
    fn pick_data_rigid_body_uses_signed_embedded_offset_and_type_byte() {
        let mut e = engine();
        let this: Ptr<BhkPickData> = e.new_object();
        let collidable = e.mem.alloc(0x20);
        e.set(this, BhkPickData::rootCollidable, Ptr::new(collidable));
        e.mem.set_u8(collidable + 0x18, 1);
        e.mem.set_i8(collidable + 0x10, 4);
        assert_eq!(e.call(0x00c6_6fd0, &args![this]).u32(), collidable + 4);
        e.mem.set_u8(collidable + 0x18, 0);
        assert_eq!(e.call(0x00c6_6fd0, &args![this]).u32(), 0);
    }

    #[test]
    fn collision_agent_empty_branch_clears_filter_active_bit_and_counts_contact() {
        let mut e = engine();
        let agent = e.mem.alloc(0x20);
        let filter = e.mem.alloc(0x20);
        e.mem.set_u32(agent + 0x10, 0);
        e.mem.set_u32(filter + 0xc, 5);
        e.mem.set_u32(filter + 0x14, 8);
        e.call(0x00c6_6ff0, &args![agent, filter]);
        assert_eq!(e.mem.u32(filter + 0xc), 4);
        assert_eq!(e.mem.u32(filter + 0x14), 9);
    }

    #[test]
    fn collision_agent_applies_world_flags_then_forwards_contact_and_activation() {
        let mut e = engine();
        let agent = e.mem.alloc(0x20);
        let filter = e.mem.alloc(0x20);
        let collidable = e.mem.alloc(0x20);
        let property = e.mem.alloc(0x10);
        let filter_object = e.mem.alloc(0x20);
        let data_owner = e.mem.alloc(4);
        let filter_world = 0x0700_1000;
        let collidable_world = 0x0700_2000;
        let agent_table = 0x0400_0000;
        let filter_table = 0x0400_1000;
        let collidable_table = 0x0400_2000;
        let data_table = 0x0400_3000;
        let mut slots = vec![0; 0x31];
        slots[0x3c / 4] = 0x0400_0100;
        slots[0xc0 / 4] = 0x0400_0114;
        e.put_vtable(agent_table, &slots);
        let mut slots = vec![0; 0x26];
        slots[0x94 / 4] = 0x0400_0104;
        e.put_vtable(filter_table, &slots);
        let mut slots = vec![0; 0x28];
        slots[0x94 / 4] = 0x0400_0108;
        slots[0x9c / 4] = 0x0400_010c;
        e.put_vtable(collidable_table, &slots);
        let mut slots = vec![0; 0x2f];
        slots[0xb8 / 4] = 0x0400_0110;
        e.put_vtable(data_table, &slots);
        e.mem.set_u32(agent, agent_table);
        e.mem.set_u32(agent + 8, data_owner);
        e.mem.set_u32(agent + 0x10, collidable);
        e.mem.set_u32(filter, filter_object);
        e.mem.set_u32(filter + 0xc, 1);
        e.mem.set_u32(filter + 0x14, 0);
        e.mem.set_u32(filter + 0x18, 1);
        e.mem.set_u32(collidable, collidable_table);
        e.mem.set_u32(filter_object, filter_table);
        e.mem.set_u32(data_owner, data_table);
        e.mem.set_u8(filter_object + 0x16, 1);
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a[0], agent);
            ret(property)
        });
        e.register_double(0x0400_0104, move |_, a| {
            assert_eq!(a[0], filter_object);
            ret(filter_world)
        });
        e.register_double(0x0400_0108, move |_, a| {
            assert_eq!(a[0], collidable);
            ret(collidable_world)
        });
        e.register_double(0x0400_010c, move |_, a| {
            assert_eq!(a, [collidable, filter_object]);
            Ret::default()
        });
        e.register_double(0x0400_0110, move |e, a| {
            assert_eq!(a[0], data_owner);
            assert_eq!(e.mem.f32(a[1]), 0.0);
            assert_eq!(e.mem.bytes(a[1] + 4, 5), [0; 5]);
            Ret::default()
        });
        e.register_double(0x0400_0114, move |_, a| {
            assert_eq!(a, [agent, 1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c6_6ff0, &args![agent, filter]);
        assert_eq!(e.mem.u16(property + 0xc), 0x100);
        assert_eq!(e.mem.u16(agent + 0xc), 0x4c);
        assert_eq!(e.mem.u32(filter + 0xc), 0);
        assert_eq!(e.mem.u32(filter + 0x14), 1);
        let calls: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|(address, _)| address)
            .collect();
        assert_eq!(
            calls,
            [
                0x00c6_6ff0,
                0x0400_0100,
                0x0400_0104,
                0x0400_0108,
                0x0400_010c,
                0x0400_0110,
                0x0400_0114
            ]
        );
    }

    #[test]
    fn shape_transform_early_return_when_no_shape_owner_exists() {
        let mut e = engine();
        let agent = e.mem.alloc(0x14);
        let transform = e.mem.alloc(0x10);
        let mut slots = vec![0; 0x10];
        slots[0x3c / 4] = 0x0400_0100;
        e.put_vtable(0x0400_0000, &slots);
        e.mem.set_u32(agent, 0x0400_0000);
        e.register_double(0x0400_0100, move |_, _| ret(0));
        e.call(0x00c6_7320, &args![agent, transform]);
        assert_eq!(e.mem.u32(agent + 0x10), 0);
    }

    #[test]
    fn shape_transform_scales_offset_and_calls_shape_get_then_set_in_order() {
        let mut e = engine();
        let agent = e.mem.alloc(0x14);
        let owner = e.mem.alloc(4);
        let transform = e.mem.alloc(0x10);
        let material = e.mem.alloc(0x20);
        let vector = e.mem.alloc(16);
        let shape = e.mem.alloc(4);
        let agent_table = 0x0400_0000;
        let owner_table = 0x0400_1000;
        let shape_table = 0x0400_2000;
        let mut slots = vec![0; 0x10];
        slots[0x3c / 4] = 0x0400_0100;
        e.put_vtable(agent_table, &slots);
        let mut slots = vec![0; 0x11];
        slots[0x40 / 4] = 0x0400_0104;
        e.put_vtable(owner_table, &slots);
        let mut slots = vec![0; 0x38];
        slots[0xd4 / 4] = 0x0400_0108;
        slots[0xdc / 4] = 0x0400_010c;
        e.put_vtable(shape_table, &slots);
        e.mem.set_u32(agent, agent_table);
        e.mem.set_u32(agent + 0x10, owner);
        e.mem.set_u32(owner, owner_table);
        e.mem.set_u32(shape, shape_table);
        e.mem.set_u32(transform + 0xc, vector);
        e.mem.set_f32(material + 0x18, 0.5);
        e.mem.set_f32(material + 0x1c, 2.0);
        e.mem.set_f32(vector, 1.0);
        e.mem.set_f32(vector + 4, 2.0);
        e.mem.set_f32(vector + 8, 3.0);
        e.mem.set_f32(vector + 12, 4.0);
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a[0], agent);
            ret(material)
        });
        e.register_double(0x0400_0104, move |_, a| {
            assert_eq!(a[0], owner);
            ret(shape)
        });
        e.register_double(0x0400_0108, move |e, a| {
            assert_eq!(a[0], shape);
            for (i, value) in [10.0f32, 20.0, 30.0, 40.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + i as u32 * 4, value);
            }
            Ret::default()
        });
        e.register_double(0x0400_010c, move |e, a| {
            assert_eq!(a[0], shape);
            assert_eq!(
                (0..4).map(|i| e.mem.f32(a[1] + i * 4)).collect::<Vec<_>>(),
                [12.0, 24.0, 36.0, 48.0]
            );
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c6_7320, &args![agent, transform]);
        let calls: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|(address, _)| address)
            .collect();
        assert_eq!(
            calls,
            [
                0x00c6_7320,
                0x0400_0100,
                0x0400_0104,
                0x0400_0108,
                0x0400_010c
            ]
        );
    }
}
