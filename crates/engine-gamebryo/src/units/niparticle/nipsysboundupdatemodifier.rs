//! `NiParticle/nipsysboundupdatemodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c31650`: the jump target of `00c2d6d0`.
const JUMP_TARGET_00C2D6D0: u32 = 0x00c3_1650;

// Translated from 00c2d6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoundUpdateModifier::LinkObject` (Xbox PDB): a 5-byte jump to `00c31650` (engine map: `NiPSysModifier::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysBoundUpdateModifier@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_bound_update_modifier_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C2D6D0, &args![this, stream]);
}

// Translated from 00c2d710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoundUpdateModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2788.
pub fn ni_p_sys_bound_update_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2788
}

use crate::units::niparticle::nipsysdata::NiPSysData;
use crate::units::niparticle::nipsysmodifier::{stream_read, STREAM_VERSION};
use crate::units::platform::MEMORY_MANAGER;

layout! {
    /// `NiPSysBoundUpdateModifier` (Xbox PDB), 0x20 bytes; the PC code uses
    /// the same offsets.
    pub struct NiPSysBoundUpdateModifier: 0x20 {
        /// `m_sUpdateSkip` (Xbox PDB).
        0x18 m_sUpdateSkip: i16,
        /// `m_usUpdateCount` (Xbox PDB).
        0x1A m_usUpdateCount: u16,
        /// `m_pkSkipBounds` (Xbox PDB): `NiBound*` (0x10 bytes each: the
        /// centre and the radius).
        0x1C m_pkSkipBounds: u32,
    }
}

/// The vtable of `NiPSysBoundUpdateModifier` (RTTI
/// `.?AVNiPSysBoundUpdateModifier@@`, `010c0134`).
const VTABLE: u32 = 0x010c_0134;
/// `NiPoint3::ZERO` (three `float`s at `011f426c`, copied by `MOV`).
const POINT_ZERO: u32 = 0x011f_426c;
/// `MemoryManager::Allocate` / `Deallocate` (`thiscall` on `011f6238`).
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;
/// `00aa13e0` (`cdecl`, one word) and `00aa1460` (`cdecl`, `(block,
/// size)`): the object allocation and sized free.
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiPSysModifier`'s constructor (`00c31720`), destructor (`00c31750`),
/// member copy (`00c31780`) and `LoadBinary` (`00c318b0`).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_DESTRUCTOR: u32 = 0x00c3_1750;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
/// `00c31640` (not a function in the Ghidra project; reached by a jump):
/// the `NiPSysModifier` system-pointer setter, `thiscall`, one word.
const MODIFIER_SET_SYSTEM_POINTER: u32 = 0x00c3_1640;
/// `NiParticleSystem`'s byte at +0xd9 (PC offset; set while a bound-update
/// modifier is attached, by this use).
const SYSTEM_HAS_BOUND_UPDATE: u32 = 0xd9;
/// `NiMeshPSysData::ms_RTTI` (`012027c0`) and `BSStripPSysData::ms_RTTI`
/// (`01202680`): data of these classes (or classes derived from them) goes
/// to the bound updates `00c2dce0` and `00c2d880` (`thiscall`, `(time,
/// data)`).
const MESH_PSYS_DATA_RTTI: u32 = 0x0120_27c0;
const STRIP_PSYS_DATA_RTTI: u32 = 0x0120_2680;
const MESH_BOUND_UPDATE: u32 = 0x00c2_dce0;
const STRIP_BOUND_UPDATE: u32 = 0x00c2_d880;
/// `NiBound::Merge` (`00a7f3f0`, `thiscall`, the other bound).
const BOUND_MERGE: u32 = 0x00a7_f3f0;
/// The square root (`00ec6040`, the `ST0` value as a leading `f64`).
const SQRT: u32 = 0x00ec_6040;
/// The `double` 0.5.
const HALF: u32 = 0x0101_1588;
/// Files up to 10.1.0.100 store the skip count without its own field
/// (`CMP [stream+0xd8], 0x0a010064; JA`).
const VERSION_SKIP_FIELD: u32 = 0x0a01_0064;

// Translated from 00c2d720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the update skip (`thiscall`, one word;
/// its low half is the `short`). The skip is stored, the old bound array
/// given back to the memory manager, and a new one of `max(skip, 1)`
/// bounds allocated (all ones when the size overflows) and cleared (centre
/// `NiPoint3::ZERO`, radius 0.0).
pub fn fn_00c2d720(e: &mut Engine, this: Ptr<NiPSysBoundUpdateModifier>, skip: u32) {
    let skip = skip as u16 as i16;
    let old = e.get(this, NiPSysBoundUpdateModifier::m_pkSkipBounds);
    e.set(this, NiPSysBoundUpdateModifier::m_sUpdateSkip, skip);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, old]);
    let count = if skip > 1 { i32::from(skip) } else { 1 };
    let bytes = (count as u32).checked_mul(0x10).unwrap_or(u32::MAX);
    let bounds = e.call(ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    e.set(this, NiPSysBoundUpdateModifier::m_pkSkipBounds, bounds);
    for i in 0..count as u32 {
        clear_bound(e, this, i);
    }
}

/// Bound `i` of the array (re-read from `this`): centre `NiPoint3::ZERO`,
/// radius 0.0.
fn clear_bound(e: &mut Engine, this: Ptr<NiPSysBoundUpdateModifier>, i: u32) {
    let bound = e.get(this, NiPSysBoundUpdateModifier::m_pkSkipBounds) + 0x10 * i;
    for word in 0..3 {
        let value = e.global::<u32>(POINT_ZERO + 4 * word);
        e.mem.set_u32(bound + 4 * word, value);
    }
    e.mem.set_u32(bound + 0xc, 0.0f32.to_bits());
}

// Translated from 00c2dc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoundUpdateModifier::SetSystemPointer` (Xbox PDB): the old
/// system's byte at +0xd9 is cleared and the new one's set (each when not
/// null), then the `NiPSysModifier` setter `00c31640` (a tail jump).
pub fn ni_p_sys_bound_update_modifier_set_system_pointer(
    e: &mut Engine,
    this: Ptr<NiPSysBoundUpdateModifier>,
    system: u32,
) {
    let old = e.mem.u32(this.addr() + 0x10);
    if old != 0 {
        e.mem.set_u8(old + SYSTEM_HAS_BOUND_UPDATE, 0);
    }
    if system != 0 {
        e.mem.set_u8(system + SYSTEM_HAS_BOUND_UPDATE, 1);
    }
    e.call(MODIFIER_SET_SYSTEM_POINTER, &args![this, system]);
}

// Translated from 00c2dca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoundUpdateModifier::_scalar_deleting_destructor_` (Xbox PDB): the
/// vtable back to the class's, the bound array given back to the memory
/// manager, `NiPSysModifier`'s destructor, then the sized delete of 0x20
/// bytes when bit 0 of `flags` is set. Returns `this`.
pub fn ni_p_sys_bound_update_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiPSysBoundUpdateModifier>,
    flags: u32,
) -> Ptr<NiPSysBoundUpdateModifier> {
    let bounds = e.get(this, NiPSysBoundUpdateModifier::m_pkSkipBounds);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, bounds]);
    e.call(MODIFIER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x20u32]);
    }
    this
}

// Translated from 00c2dec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoundUpdateModifier::CreateClone` (Xbox PDB): a 0x20-byte object
/// from `00aa13e0` (constructed: the `NiPSysModifier` constructor, the
/// vtable, skip and count 0, no bounds; 0 when the allocation fails), the
/// `NiPSysModifier` member copy into it (it still runs into a null clone),
/// then the skip set on it (`00c2d720`). Returns the clone.
pub fn ni_p_sys_bound_update_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysBoundUpdateModifier>,
    process: Ptr,
) -> Ptr<NiPSysBoundUpdateModifier> {
    let clone = Ptr::<NiPSysBoundUpdateModifier>::new(e.call(NI_NEW, &args![0x20u32]).u32());
    if !clone.is_null() {
        e.call(MODIFIER_CONSTRUCTOR, &args![clone]);
        e.mem.set_u32(clone.addr(), VTABLE);
        e.set(clone, NiPSysBoundUpdateModifier::m_sUpdateSkip, 0);
        e.set(clone, NiPSysBoundUpdateModifier::m_usUpdateCount, 0);
        e.set(clone, NiPSysBoundUpdateModifier::m_pkSkipBounds, 0);
    }
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let skip = e.get(this, NiPSysBoundUpdateModifier::m_sUpdateSkip) as u16;
    fn_00c2d720(e, clone, u32::from(skip));
    clone
}

// Translated from 00c2df20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoundUpdateModifier::Update` (Xbox PDB): `thiscall`, `RET 8`
/// (time, particle data). Without active particles the data's bound is
/// cleared (centre `NiPoint3::ZERO`, radius 0) and the update count reset.
/// Mesh and strip particle data (by their `NiRTTI` chains, data vtable slot
/// +8) go to `00c2dce0` / `00c2d880`. Otherwise, with a skip of -1 the skip
/// becomes `capacity / 50 + 1`; the step is the skip, at most
/// `count / 50 + 1`, at least 1. From the update count, every step-th
/// particle widens a box by its radius (`size * radius`) on each axis (a
/// minimum is taken when not above, a maximum when not below, NaN
/// included); the box's centre and half diagonal become the bound at the
/// update count; the bounds from the step up to the skip are cleared; the
/// bound at the update count is merged with the others of the first `step`
/// that have a radius (`NiBound::Merge`) into the data's bound, whose
/// radius falls back to the first particle's `radius * size` when zero.
/// The update count then advances, wrapping at the step. The x87
/// operations are done instruction by instruction; the minima stay on the
/// x87 stack as in the exe.
pub fn ni_p_sys_bound_update_modifier_update(
    e: &mut Engine,
    this: Ptr<NiPSysBoundUpdateModifier>,
    time: f32,
    data: Ptr<NiPSysData>,
) {
    let count = e.get(data, NiPSysData::m_usActive);
    if count == 0 {
        for word in 0..3 {
            let value = e.global::<u32>(POINT_ZERO + 4 * word);
            e.mem
                .set_u32(data.addr() + NiPSysData::m_kBound.off + 4 * word, value);
        }
        e.set(data, NiPSysData::m_kBoundRadius, 0.0f32.to_bits());
        e.set(this, NiPSysBoundUpdateModifier::m_usUpdateCount, 0);
        return;
    }
    for (rtti, update) in [
        (MESH_PSYS_DATA_RTTI, MESH_BOUND_UPDATE),
        (STRIP_PSYS_DATA_RTTI, STRIP_BOUND_UPDATE),
    ] {
        let mut class = e.vcall(data.addr(), 8, &args![]).u32();
        while class != 0 {
            if class == rtti {
                let now = x87_quiet_bits(time.to_bits());
                e.call(update, &args![this, now, data]);
                return;
            }
            class = e.mem.u32(class + 4);
        }
    }
    if e.get(this, NiPSysBoundUpdateModifier::m_sUpdateSkip) == -1 {
        let capacity = i32::from(e.get(data, NiPSysData::m_usVertices));
        fn_00c2d720(e, this, (capacity / 50 + 1) as u32);
    }
    let count = i32::from(count);
    let limit = count / 50 + 1;
    let mut step = i32::from(e.get(this, NiPSysBoundUpdateModifier::m_sUpdateSkip));
    if step >= limit {
        step = limit;
    }
    if step <= 1 {
        step = 1;
    }
    let vertices = e.get(data, NiPSysData::m_pkVertex);
    let sizes = e.get(data, NiPSysData::m_pfSizes);
    let radii = e.get(data, NiPSysData::m_pfRadii);
    // The first vertex starts both corners; the minima are x87 registers.
    let first = [0, 1, 2].map(|k| x87_quiet_bits(e.mem.u32(vertices + 4 * k)));
    let mut low = first.map(x87_load32);
    let mut high = first;
    let start = u32::from(e.get(this, NiPSysBoundUpdateModifier::m_usUpdateCount));
    let mut i = start as i32;
    while i < count {
        let k = i as u32;
        let vertex = vertices + 0xc * k;
        let r = x87_load32(x87_store32(x87_mul(
            x87_load32(e.mem.u32(sizes + 4 * k)),
            x87_m32(e.mem.u32(radii + 4 * k)),
        )));
        for axis in 0..3u32 {
            let v = e.mem.u32(vertex + 4 * axis);
            let a = axis as usize;
            let below = x87_load32(x87_store32(x87_sub(x87_load32(v), r)));
            if !(below > low[a]) {
                low[a] = below;
            }
            let above = if axis == 2 {
                x87_store32(x87_add(r, x87_m32(v)))
            } else {
                x87_store32(x87_add(x87_load32(v), r))
            };
            if !(x87_load32(above) < x87_load32(high[a])) {
                high[a] = x87_quiet_bits(above);
            }
        }
        i += step;
    }
    let sums = [0, 1, 2].map(|a| x87_store32(x87_add(x87_load32(high[a]), low[a])));
    let half: f64 = e.global(HALF);
    let bound = e.get(this, NiPSysBoundUpdateModifier::m_pkSkipBounds) + 0x10 * start;
    let center = [
        x87_store32(x87_mul(x87_load32(sums[0]), half)),
        x87_store32(x87_mul(x87_load32(sums[1]), half)),
        x87_store32(x87_mul(half, x87_m32(sums[2]))),
    ];
    for (a, value) in center.iter().enumerate() {
        e.mem.set_u32(bound + 4 * a as u32, *value);
    }
    let extent = [0, 1, 2].map(|a| x87_store32(x87_sub(x87_load32(high[a]), low[a])));
    let (ex, ey, ez) = (
        x87_load32(extent[0]),
        x87_load32(extent[1]),
        x87_load32(extent[2]),
    );
    let squared = x87_store32(x87_add(
        x87_add(x87_mul(ey, ey), x87_mul(ex, ex)),
        x87_mul(ez, ez),
    ));
    let root = e.call(SQRT, &args![x87_load32(squared)]).f64();
    let radius = x87_store32(x87_mul(x87_load32(x87_store32(root)), half));
    let index = u32::from(e.get(this, NiPSysBoundUpdateModifier::m_usUpdateCount));
    let bounds = e.get(this, NiPSysBoundUpdateModifier::m_pkSkipBounds);
    e.mem
        .set_u32(bounds + 0x10 * index + 0xc, x87_quiet_bits(radius));
    let mut j = step;
    while j < i32::from(e.get(this, NiPSysBoundUpdateModifier::m_sUpdateSkip)) {
        clear_bound(e, this, j as u32);
        j += 1;
    }
    e.with_stack(0x10, |e, merged| {
        let index = u32::from(e.get(this, NiPSysBoundUpdateModifier::m_usUpdateCount));
        let source = e.get(this, NiPSysBoundUpdateModifier::m_pkSkipBounds) + 0x10 * index;
        for word in 0..3 {
            let value = e.mem.u32(source + 4 * word);
            e.mem.set_u32(merged.addr() + 4 * word, value);
        }
        let r = x87_quiet_bits(e.mem.u32(source + 0xc));
        e.mem.set_u32(merged.addr() + 0xc, r);
        for other in 1..step as u32 {
            let bound = e.get(this, NiPSysBoundUpdateModifier::m_pkSkipBounds) + 0x10 * other;
            // `FLDZ; FCOM radius; TEST AH,0x44; JNP`: bounds with a zero
            // radius are left out.
            if !(0.0 == x87_m32(e.mem.u32(bound + 0xc))) {
                e.call(BOUND_MERGE, &args![merged, bound]);
            }
        }
        let radius = x87_load32(e.mem.u32(merged.addr() + 0xc));
        e.set(data, NiPSysData::m_kBoundRadius, x87_store32(radius));
        for word in 0..3 {
            let value = e.mem.u32(merged.addr() + 4 * word);
            e.mem
                .set_u32(data.addr() + NiPSysData::m_kBound.off + 4 * word, value);
        }
        if radius == 0.0 {
            let radii = e.get(data, NiPSysData::m_pfRadii);
            let sizes = e.get(data, NiPSysData::m_pfSizes);
            let fallback = x87_store32(x87_mul(
                x87_load32(e.mem.u32(radii)),
                x87_m32(e.mem.u32(sizes)),
            ));
            e.set(data, NiPSysData::m_kBoundRadius, fallback);
        }
    });
    let next = e
        .get(this, NiPSysBoundUpdateModifier::m_usUpdateCount)
        .wrapping_add(1);
    e.set(this, NiPSysBoundUpdateModifier::m_usUpdateCount, next);
    if i32::from(next) >= step {
        e.set(this, NiPSysBoundUpdateModifier::m_usUpdateCount, 0);
    }
}

// Translated from 00c2e370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoundUpdateModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, then two bytes of skip count: into a local (whose upper
/// half still holds `this`'s upper half from the `PUSH ECX` that made it)
/// for files up to 10.1.0.100, into `m_sUpdateSkip` from 10.1.0.101; the
/// skip is then set (`00c2d720`) with that word. The component-size word is
/// the stream argument's stack word.
pub fn ni_p_sys_bound_update_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysBoundUpdateModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_LOAD_BINARY, &args![this, s]);
    let version = e.mem.u32(s + STREAM_VERSION);
    let skip = e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument_slot = frame.addr() + 4;
        e.mem.set_u32(local, this.addr());
        if version <= VERSION_SKIP_FIELD {
            stream_read(e, s, local, 2, 2, argument_slot);
            e.mem.u32(local)
        } else {
            let field = this.addr() + NiPSysBoundUpdateModifier::m_sUpdateSkip.off;
            stream_read(e, s, field, 2, 2, argument_slot);
            u32::from(e.mem.u16(field))
        }
    });
    fn_00c2d720(e, this, skip);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_d720,
            fn_00c2d720(Ptr<NiPSysBoundUpdateModifier>, u32)
        ),
        entry!(
            0x00c2_dc70,
            ni_p_sys_bound_update_modifier_set_system_pointer(Ptr<NiPSysBoundUpdateModifier>, u32)
        ),
        entry!(
            0x00c2_dca0,
            ni_p_sys_bound_update_modifier_scalar_deleting_destructor(
                Ptr<NiPSysBoundUpdateModifier>,
                u32,
            ) -> Ptr<
                NiPSysBoundUpdateModifier,
            >
        ),
        entry!(
            0x00c2_dec0,
            ni_p_sys_bound_update_modifier_create_clone(
                Ptr<NiPSysBoundUpdateModifier>,
                Ptr,
            )
                -> Ptr<NiPSysBoundUpdateModifier>
        ),
        entry!(
            0x00c2_df20,
            ni_p_sys_bound_update_modifier_update(
                Ptr<NiPSysBoundUpdateModifier>,
                f32,
                Ptr<NiPSysData>,
            )
        ),
        entry!(
            0x00c2_e370,
            ni_p_sys_bound_update_modifier_load_binary(Ptr<NiPSysBoundUpdateModifier>, Ptr)
        ),
        entry!(0x00c2d710, ni_p_sys_bound_update_modifier_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_d6d0,
            ni_p_sys_bound_update_modifier_link_object(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c2d6d0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_d6d0;
        const TARGET: u32 = 0x00c3_1650;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn ni_p_sys_bound_update_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_d710, &args![]).u32(), 0x0120_2788);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

    fn log_of(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(POINT_ZERO & !0xfff, 0x1000);
        e.map(HALF & !0xfff, 0x1000);
        e.set_global(HALF, 0.5f64);
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1].max(4)),
            ..Ret::default()
        });
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        e
    }

    #[test]
    fn set_update_skip_makes_cleared_bounds() {
        let mut e = engine();
        let this = e.new_object::<NiPSysBoundUpdateModifier>();
        e.set(this, NiPSysBoundUpdateModifier::m_pkSkipBounds, 0x999);
        e.call_log = Some(vec![]);
        e.call(0x00c2_d720, &args![this, 0xabcd_0003u32]);
        assert_eq!(e.get(this, NiPSysBoundUpdateModifier::m_sUpdateSkip), 3);
        assert_eq!(log_of(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x999]]);
        assert_eq!(log_of(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 0x30]]);
        let bounds = e.get(this, NiPSysBoundUpdateModifier::m_pkSkipBounds);
        assert_eq!(e.mem.bytes(bounds, 0x30), vec![0; 0x30]);
        // A skip of 1 or less still gets one bound.
        e.call_log = Some(vec![]);
        e.call(0x00c2_d720, &args![this, 0xffffu32]);
        assert_eq!(e.get(this, NiPSysBoundUpdateModifier::m_sUpdateSkip), -1);
        assert_eq!(log_of(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 0x10]]);
    }

    #[test]
    fn set_system_pointer_moves_the_flag() {
        let mut e = engine();
        e.register(MODIFIER_SET_SYSTEM_POINTER, |_, _| Ret::default());
        let this = e.new_object::<NiPSysBoundUpdateModifier>();
        let old = e.mem.alloc(0x100);
        let new = e.mem.alloc(0x100);
        e.mem.set_u8(old + 0xd9, 1);
        e.mem.set_u32(this.addr() + 0x10, old);
        e.call_log = Some(vec![]);
        e.call(0x00c2_dc70, &args![this, new]);
        assert_eq!(e.mem.u8(old + 0xd9), 0);
        assert_eq!(e.mem.u8(new + 0xd9), 1);
        assert_eq!(
            log_of(&e, MODIFIER_SET_SYSTEM_POINTER),
            vec![vec![this.addr(), new]]
        );
    }

    #[test]
    fn scalar_deleting_destructor_frees_the_bounds() {
        let mut e = engine();
        e.register(MODIFIER_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        let this = e.new_object::<NiPSysBoundUpdateModifier>();
        e.set(this, NiPSysBoundUpdateModifier::m_pkSkipBounds, 0x777);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_dca0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(log_of(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x777]]);
        assert_eq!(log_of(&e, NI_DELETE), vec![vec![this.addr(), 0x20]]);
    }

    #[test]
    fn create_clone_sets_the_skip_on_the_clone() {
        let mut e = engine();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiPSysBoundUpdateModifier>();
        e.set(this, NiPSysBoundUpdateModifier::m_sUpdateSkip, 4);
        let clone = e
            .call(0x00c2_dec0, &args![this, 0x4000u32])
            .ptr::<NiPSysBoundUpdateModifier>();
        assert_eq!(e.mem.u32(clone.addr()), VTABLE);
        assert_eq!(e.get(clone, NiPSysBoundUpdateModifier::m_sUpdateSkip), 4);
        assert_ne!(e.get(clone, NiPSysBoundUpdateModifier::m_pkSkipBounds), 0);
    }

    const RTTI_FN: u32 = 0x0f70_0000;

    /// Particle data whose `GetRTTI` answers a chain of two records ending
    /// in `base`; particles at `positions` with size 1 and radius `r`.
    fn particles(e: &mut Engine, positions: &[[f32; 3]], r: f32, base: u32) -> Ptr<NiPSysData> {
        let data = e.new_object::<NiPSysData>();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, RTTI_FN);
        e.mem.set_u32(data.addr(), vtable);
        let own = e.mem.alloc(8);
        e.mem.set_u32(own + 4, base);
        e.mem.set_u32(data.addr() + 0x78, own);
        e.register(RTTI_FN, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x78),
            ..Ret::default()
        });
        let n = positions.len() as u32;
        let vertices = e.mem.alloc(12 * n);
        let sizes = e.mem.alloc(4 * n);
        let radii = e.mem.alloc(4 * n);
        for (i, p) in positions.iter().enumerate() {
            let i = i as u32;
            for (k, v) in p.iter().enumerate() {
                e.mem.set_f32(vertices + 12 * i + 4 * k as u32, *v);
            }
            e.mem.set_f32(sizes + 4 * i, 1.0);
            e.mem.set_f32(radii + 4 * i, r);
        }
        e.set(data, NiPSysData::m_pkVertex, vertices);
        e.set(data, NiPSysData::m_pfSizes, sizes);
        e.set(data, NiPSysData::m_pfRadii, radii);
        e.set(data, NiPSysData::m_usActive, n as u16);
        e.set(data, NiPSysData::m_usVertices, n as u16);
        data
    }

    #[test]
    fn update_without_particles_clears_the_bound() {
        let mut e = engine();
        let this = e.new_object::<NiPSysBoundUpdateModifier>();
        e.set(this, NiPSysBoundUpdateModifier::m_usUpdateCount, 3);
        let data = e.new_object::<NiPSysData>();
        e.set(data, NiPSysData::m_kBoundRadius, 5.0f32.to_bits());
        e.call(0x00c2_df20, &args![this, 1.0f32, data]);
        assert_eq!(e.get(data, NiPSysData::m_kBoundRadius), 0);
        assert_eq!(e.get(this, NiPSysBoundUpdateModifier::m_usUpdateCount), 0);
    }

    #[test]
    fn update_hands_mesh_and_strip_data_on() {
        for (rtti, target) in [
            (MESH_PSYS_DATA_RTTI, MESH_BOUND_UPDATE),
            (STRIP_PSYS_DATA_RTTI, STRIP_BOUND_UPDATE),
        ] {
            let mut e = engine();
            e.register(MESH_BOUND_UPDATE, |_, _| Ret::default());
            e.register(STRIP_BOUND_UPDATE, |_, _| Ret::default());
            e.map(MESH_PSYS_DATA_RTTI & !0xfff, 0x1000);
            let this = e.new_object::<NiPSysBoundUpdateModifier>();
            let data = particles(&mut e, &[[0.0; 3]], 1.0, rtti);
            e.call_log = Some(vec![]);
            e.call(0x00c2_df20, &args![this, 2.0f32, data]);
            assert_eq!(
                log_of(&e, target),
                vec![vec![this.addr(), 2.0f32.to_bits(), data.addr()]]
            );
        }
    }

    #[test]
    fn update_bounds_the_particles() {
        let mut e = engine();
        let this = e.new_object::<NiPSysBoundUpdateModifier>();
        e.call(0x00c2_d720, &args![this, 0xffffu32]);
        let data = particles(&mut e, &[[0.0, 0.0, 0.0], [4.0, 2.0, 6.0]], 1.0, 0);
        e.call(0x00c2_df20, &args![this, 1.0f32, data]);
        // The skip of -1 became capacity / 50 + 1 = 1.
        assert_eq!(e.get(this, NiPSysBoundUpdateModifier::m_sUpdateSkip), 1);
        let center = [0, 4, 8].map(|o| e.mem.f32(data.addr() + 0x10 + o));
        assert_eq!(center, [2.0, 1.0, 3.0]);
        let radius = ((6.0f64 * 6.0 + 4.0 * 4.0 + 8.0 * 8.0).sqrt() as f32) as f64 * 0.5;
        assert_eq!(
            f32::from_bits(e.get(data, NiPSysData::m_kBoundRadius)),
            radius as f32
        );
        assert_eq!(e.get(this, NiPSysBoundUpdateModifier::m_usUpdateCount), 0);
    }

    #[test]
    fn update_zero_radius_falls_back_to_the_first_particle() {
        let mut e = engine();
        let this = e.new_object::<NiPSysBoundUpdateModifier>();
        e.call(0x00c2_d720, &args![this, 1u32]);
        // A radius of -0: the box is a point, its radius +0, and the
        // fallback `radius * size` gives -0.
        let data = particles(&mut e, &[[1.0, 1.0, 1.0]], -0.0, 0);
        e.call(0x00c2_df20, &args![this, 1.0f32, data]);
        assert_eq!(e.get(data, NiPSysData::m_kBoundRadius), 0x8000_0000);
        assert_eq!(e.mem.f32(data.addr() + 0x10), 1.0);
    }

    #[test]
    fn update_steps_through_the_particles_and_merges_the_bounds() {
        let mut e = engine();
        e.register(BOUND_MERGE, |_, _| Ret::default());
        let this = e.new_object::<NiPSysBoundUpdateModifier>();
        e.call(0x00c2_d720, &args![this, 2u32]);
        let positions: Vec<[f32; 3]> = (0..50).map(|i| [i as f32, 0.0, 0.0]).collect();
        let data = particles(&mut e, &positions, 1.0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_df20, &args![this, 1.0f32, data]);
        // Step 2 from 0: particles 0..48; the second bound is still empty.
        let bounds = e.get(this, NiPSysBoundUpdateModifier::m_pkSkipBounds);
        assert_eq!(e.mem.f32(bounds), 24.0);
        assert!(log_of(&e, BOUND_MERGE).is_empty());
        assert_eq!(e.get(this, NiPSysBoundUpdateModifier::m_usUpdateCount), 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_df20, &args![this, 1.0f32, data]);
        assert_eq!(e.mem.f32(bounds + 0x10), 25.0);
        let merges = log_of(&e, BOUND_MERGE);
        assert_eq!(merges.len(), 1);
        assert_eq!(merges[0][1], bounds + 0x10);
        assert_eq!(e.get(this, NiPSysBoundUpdateModifier::m_usUpdateCount), 0);
    }

    #[test]
    fn load_binary_reads_the_skip_by_version() {
        for version in [0x0a01_0064u32, 0x0a01_0065] {
            let mut e = engine();
            e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
            let stream = test_stream::new(&mut e, version, &[3, 0]);
            let this = e.new_object::<NiPSysBoundUpdateModifier>();
            e.call_log = Some(vec![]);
            e.call(0x00c2_e370, &args![this, stream]);
            assert_eq!(e.get(this, NiPSysBoundUpdateModifier::m_sUpdateSkip), 3);
            assert_eq!(log_of(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 0x30]]);
        }
    }
}
