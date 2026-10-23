//! `NiCollision/run_00eab9d0` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSphereBV`'s vtable (`010f1854`); its slot 0 is `NiSphereBV::LoadBinary` (`00eab860`).
const NI_SPHERE_BV_VTABLE: u32 = 0x010f_1854;
/// The memory manager instance (`this` of its allocate).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (`00aa3e40`, thiscall on [`MEMORY_MANAGER`], size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The three words at `011f426c` a new sphere volume takes as its center.
const DEFAULT_CENTER_011F426C: u32 = 0x011f_426c;

fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// A stored float loaded back (`FSTP` then `FLD`).
fn round(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// `b - a` per component of two 3-float vectors, each stored.
fn difference(e: &Engine, b: u32, a: u32) -> [f64; 3] {
    [0u32, 4, 8].map(|offset| {
        round(x87_sub(
            load(e, b.wrapping_add(offset)),
            operand(e, a.wrapping_add(offset)),
        ))
    })
}

// Translated from 00eab9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00eab9d0`, cdecl): whether two moving spheres (sphere volume layout: center at
/// +4, radius at +0x10) touch within `time`. With the relative velocity `v = velocity1 -
/// velocity0` and offset `d = center1 - center0` (each component stored), `a = |v|^2` and
/// `c = |d|^2` (stored) and `r2 = (radius0 + radius1)^2` (stored): when `a > 0` and the spheres
/// approach (`b = v . d`, stored, at or below zero) the closest approach is `c * a - b^2`
/// against `a * r2` when it falls within the time (`-time * a <= b`), else the distance at the
/// end `(time * a + 2b) * time + c` against `r2`; otherwise `c` against `r2`. Answers whether
/// the distance is at most `r2`.
pub fn fn_00eab9d0(
    e: &mut Engine,
    time: f32,
    sphere0: Ptr,
    velocity0: Ptr,
    sphere1: Ptr,
    velocity1: Ptr,
) -> bool {
    let first = sphere0.addr();
    let second = sphere1.addr();
    let [vx, vy, vz] = difference(e, velocity1.addr(), velocity0.addr());
    let a = round(x87_add(
        x87_add(x87_mul(vy, vy), x87_mul(vx, vx)),
        x87_mul(vz, vz),
    ));
    let [dx, dy, dz] = difference(e, second.wrapping_add(4), first.wrapping_add(4));
    let c = x87_store32(x87_add(
        x87_add(x87_mul(dy, dy), x87_mul(dx, dx)),
        x87_mul(dz, dz),
    ));
    let reach = round(x87_add(
        load(e, first.wrapping_add(0x10)),
        operand(e, second.wrapping_add(0x10)),
    ));
    let r2 = x87_store32(x87_mul(reach, reach));
    if a > 0.0 {
        let b = round(x87_add(
            x87_add(x87_mul(vx, dx), x87_mul(vy, dy)),
            x87_mul(vz, dz),
        ));
        if b <= 0.0 {
            let t = x87_load32(time.to_bits());
            if x87_mul(-t, a) <= b {
                let closest = x87_sub(x87_mul(x87_load32(c), a), x87_mul(b, b));
                return x87_mul(a, x87_m32(r2)) >= closest;
            }
            let end = x87_add(
                x87_mul(x87_add(x87_mul(t, a), x87_add(b, b)), t),
                x87_m32(c),
            );
            return x87_load32(r2) >= end;
        }
    }
    x87_load32(r2) >= x87_load32(c)
}

// Translated from 00eac170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00eac170`, cdecl): makes a sphere volume and loads it from `stream`. Allocates
/// 0x14 bytes from the memory manager; a block gets the `NiSphereBV` vtable, the center from the
/// three words at `011f426c` and the radius 1.0. Then calls the volume's vtable slot 0
/// (`LoadBinary`) with `stream`, also when the allocation failed (the exe then reads the vtable
/// at address 0). Answers the volume.
pub fn fn_00eac170(e: &mut Engine, stream: Ptr) -> Ptr {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
        .u32();
    if block != 0 {
        e.mem.set_u32(block, NI_SPHERE_BV_VTABLE);
        for offset in [0u32, 4] {
            let word = e.mem.u32(DEFAULT_CENTER_011F426C + offset);
            e.mem.set_u32(block + 4 + offset, word);
        }
        e.mem.set_u32(block + 0x10, 1.0f32.to_bits());
        let z = e.mem.u32(DEFAULT_CENTER_011F426C + 8);
        e.mem.set_u32(block + 0xc, z);
    }
    e.vcall(block, 0, &args![stream]);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ea_b9d0,
            fn_00eab9d0(f32, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00ea_c170, fn_00eac170(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        block
    }

    #[test]
    fn moving_spheres_touch_within_the_time() {
        let mut e = Engine::new();
        let first = floats(&mut e, &[0.0, 0.0, 0.0, 0.0, 1.0]);
        let second = floats(&mut e, &[0.0, 10.0, 0.0, 0.0, 1.0]);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let toward = floats(&mut e, &[-4.0, 0.0, 0.0]);
        let away = floats(&mut e, &[4.0, 0.0, 0.0]);
        let mut test = |time: f32, v1: u32| {
            e.call(0x00ea_b9d0, &args![time, first, still, second, v1])
                .bool()
        };
        // Apart and still, moving away, too slow, then fast enough (the end and the middle).
        assert!(!test(1.0, still));
        assert!(!test(5.0, away));
        assert!(!test(1.0, toward));
        assert!(test(2.0, toward));
        assert!(test(5.0, toward));
        // Touching at rest.
        let near = floats(&mut e, &[0.0, 2.0, 0.0, 0.0, 1.0]);
        let words = args![0.0f32, first, still, near, still];
        assert!(e.call(0x00ea_b9d0, &words).bool());
    }

    #[test]
    fn create_from_stream_loads_the_new_sphere() {
        let mut e = Engine::new();
        e.map(0x010f_1000, 0x1000);
        e.map(0x011f_4000, 0x3000);
        e.mem.set_u32(NI_SPHERE_BV_VTABLE, 0x00ab_5030);
        for (axis, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem
                .set_f32(DEFAULT_CENTER_011F426C + 4 * axis as u32, *value);
        }
        e.register(MEMORY_ALLOCATE, |e, _| e.mem.alloc(0x14).into_ret());
        e.register(0x00ab_5030, |e, a| {
            assert_eq!(e.mem.f32(a[0] + 0x10), 1.0);
            assert_eq!(a[1], 0x77);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00ea_c170, &args![0x77u32]).u32();
        assert_eq!(e.mem.u32(made), NI_SPHERE_BV_VTABLE);
        let center: Vec<f32> = (0..3).map(|i| e.mem.f32(made + 4 + 4 * i)).collect();
        assert_eq!(center, vec![1.0, 2.0, 3.0]);
        let loads = e
            .call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(addr, _)| *addr == 0x00ab_5030)
            .count();
        assert_eq!(loads, 1);
    }
}
