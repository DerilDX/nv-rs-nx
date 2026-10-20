//! `NiAnimation/run_00a289b0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a28cb0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3669` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a28cb0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3669) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d50, 0x0);
    e.mem.set_u32(0x011f_3c30, 0x0);
    e.mem.set_u32(0x011f_3ba0, 0x0);
    e.mem.set_u32(0x011f_3840, 0x0);
    e.mem.set_u8(0x011f_3669, 0x1);
    e.mem.set_u32(0x011f_3de0, 0xe7cd40);
    e.mem.set_u32(0x011f_3a80, 0xa29680);
    e.mem.set_u32(0x011f_39f0, 0xa29680);
    e.mem.set_u32(0x011f_3960, 0xe7cd40);
    e.mem.set_u32(0x011f_38d0, 0xa29680);
    e.mem.set_u8(0x011f_3764, 0x14);
    e.mem.set_u32(0x011f_3cc0, 0xa286e0);
    e.mem.set_u32(0x011f_3b10, 0xa28a80);
    e.mem.set_u32(0x011f_37b0, 0xa28ab0);
    1
}

// Translated from 00a29110 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f367c` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a29110(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_367c) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d58, 0x0);
    e.mem.set_u32(0x011f_3c38, 0x0);
    e.mem.set_u32(0x011f_3ba8, 0x0);
    e.mem.set_u8(0x011f_367c, 0x1);
    e.mem.set_u32(0x011f_3de8, 0xa290a0);
    e.mem.set_u32(0x011f_3a88, 0xa28ed0);
    e.mem.set_u32(0x011f_39f8, 0xa28d40);
    e.mem.set_u32(0x011f_3968, 0xa28f00);
    e.mem.set_u32(0x011f_38d8, 0xa2a600);
    e.mem.set_u8(0x011f_3766, 0x24);
    e.mem.set_u32(0x011f_3cc8, 0xa28d90);
    e.mem.set_u32(0x011f_3b18, 0xa28ea0);
    e.mem.set_u32(0x011f_37b8, 0xa28dc0);
    e.mem.set_u32(0x011f_3848, 0xa28f50);
    1
}

// Translated from 00a291f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a291f0(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa291e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa291d0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa291d0);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa291d0);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa291a0);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa29680);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0x0);
}

/// `NiQuaternion::operator-` (Xbox PDB, unary, returned by value), `00a6dbd0`.
const QUATERNION_NEGATE: u32 = 0x00a6_dbd0;
/// `float` 0.0 (`01011d78`), the sign test of the dot product.
const ZERO_FLOAT: u32 = 0x0101_1d78;
/// `float` -1.0 (`01012054`), the lower clamp of `W`.
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;

/// Clamps the `float` at `at` as the exe's clamp blocks do: the value
/// (through `FLD`/`FSTP`/`FLD` of a local) below `low` stores `low`,
/// otherwise above 1 stores 1; NaN stores nothing.
fn clamp_w(e: &mut Engine, at: u32, low: f64) {
    let v = x87_load32(x87_store32(x87_load32(e.mem.u32(at))));
    if v < low {
        e.mem.set_u32(at, x87_store32(low));
    } else if v > 1.0 {
        e.mem.set_u32(at, x87_store32(1.0));
    }
}

// Translated from 00a28ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Prepares an array of `count` rotation keys of `stride` bytes (`cdecl`;
/// no Xbox PDB name; the quaternion at +4 of each key): first each key's
/// quaternion whose dot product with the previous key's is negative is
/// negated (through `NiQuaternion::operator-`, `00a6dbd0`), so neighbours
/// lie in one hemisphere; then every key's `W` is clamped to `[-1, 1]`,
/// four keys at a time and then the rest. As in the exe, the first loop
/// runs `count - 1` times without a guard for `count == 0`.
pub fn fn_00a28ab0(e: &mut Engine, keys: Ptr, count: u32, stride: u8) {
    let base = keys.addr();
    let stride = u32::from(stride);
    let mut remaining = count.wrapping_sub(1);
    let mut next = base.wrapping_add(stride).wrapping_add(4);
    let mut prev = base.wrapping_add(0xc);
    let zero = x87_m32(e.mem.u32(ZERO_FLOAT));
    while remaining != 0 {
        let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
        let m = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
        let a = x87_mul(ld(e, next.wrapping_add(4)), m(e, prev.wrapping_sub(4)));
        let b = x87_mul(ld(e, prev.wrapping_sub(8)), m(e, next));
        let sum = x87_add(a, b);
        let c = x87_mul(ld(e, next.wrapping_add(8)), m(e, prev));
        let sum = x87_add(sum, c);
        let d = x87_mul(ld(e, next.wrapping_add(0xc)), m(e, prev.wrapping_add(4)));
        let dot = x87_load32(x87_store32(x87_add(sum, d)));
        // FCOMP 0.0; TEST AH,0x5; JP: only "below" negates.
        if dot < zero {
            e.with_stack(0x10, |e, tmp| {
                let r = e.call(QUATERNION_NEGATE, &args![next, tmp]).u32();
                for i in 0..4 {
                    let w = e.mem.u32(r.wrapping_add(4 * i));
                    e.mem.set_u32(next.wrapping_add(4 * i), w);
                }
            });
        }
        next = next.wrapping_add(stride);
        prev = prev.wrapping_add(stride);
        remaining = remaining.wrapping_sub(1);
    }
    let low = x87_load32(e.mem.u32(MINUS_ONE_FLOAT));
    let mut done = 0u32;
    if count as i32 >= 4 {
        let mut groups = (count.wrapping_sub(4) >> 2).wrapping_add(1);
        done = groups.wrapping_mul(4);
        let step = stride.wrapping_mul(4);
        let mut p0 = base.wrapping_add(4);
        let mut p1 = base.wrapping_add(stride).wrapping_add(4);
        let mut p2 = base.wrapping_add(stride.wrapping_mul(2)).wrapping_add(4);
        let mut p3 = base.wrapping_add(stride.wrapping_mul(3)).wrapping_add(4);
        while groups != 0 {
            clamp_w(e, p0, low);
            clamp_w(e, p1, low);
            clamp_w(e, p2, low);
            clamp_w(e, p3, low);
            p0 = p0.wrapping_add(step);
            p1 = p1.wrapping_add(step);
            p2 = p2.wrapping_add(step);
            p3 = p3.wrapping_add(step);
            groups = groups.wrapping_sub(1);
        }
    }
    if done < count {
        let mut p = base.wrapping_add(stride.wrapping_mul(done)).wrapping_add(4);
        let mut left = count - done;
        while left != 0 {
            clamp_w(e, p, low);
            p = p.wrapping_add(stride);
            left -= 1;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a28cb0, fn_00a28cb0() -> u32),
        entry!(0x00a29110, fn_00a29110() -> u32),
        entry!(0x00a291f0, fn_00a291f0(u32, u32)),
        entry!(0x00a28ab0, fn_00a28ab0(Ptr, u32, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `count` keys of `stride` bytes, each with the given quaternion at +4.
    fn keys(e: &mut Engine, stride: u32, quats: &[[f32; 4]]) -> Ptr {
        let base = e.mem.alloc(stride * quats.len() as u32 + 0x10);
        for (i, q) in quats.iter().enumerate() {
            for (j, v) in q.iter().enumerate() {
                e.mem
                    .set_f32(base + stride * i as u32 + 4 + 4 * j as u32, *v);
            }
        }
        Ptr::new(base)
    }

    fn quat(e: &Engine, keys: Ptr, stride: u32, i: u32) -> [f32; 4] {
        [0, 4, 8, 12].map(|o| e.mem.f32(keys.addr() + stride * i + 4 + o))
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x2000);
        e.set_global(ZERO_FLOAT, 0.0f32);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.register(QUATERNION_NEGATE, |e, a| {
            for i in 0..4 {
                let v = e.mem.f32(a[0] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, -v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e
    }

    #[test]
    fn keys_opposite_their_neighbour_are_negated() {
        let mut e = engine();
        let k = keys(
            &mut e,
            0x14,
            &[
                [1.0, 0.0, 0.0, 0.0],
                [-0.5, 0.5, 0.0, 0.0],
                [0.5, 0.5, 0.0, 0.0],
            ],
        );
        e.call(0x00a2_8ab0, &args![k, 3u32, 0x14u32]);
        assert_eq!(quat(&e, k, 0x14, 1), [0.5, -0.5, -0.0, -0.0]);
        // The third is compared with the negated second: dot 0, kept.
        assert_eq!(quat(&e, k, 0x14, 2), [0.5, 0.5, 0.0, 0.0]);
    }

    #[test]
    fn every_w_is_clamped_in_groups_of_four_and_then_the_rest() {
        let mut e = engine();
        let w = [2.0f32, -3.0, 0.5, 1.5, -1.5, 0.25];
        // X = 10 keeps every dot product positive: nothing is negated.
        let quats: Vec<[f32; 4]> = w.iter().map(|w| [*w, 10.0, 0.0, 0.0]).collect();
        let k = keys(&mut e, 0x10, &quats);
        e.mem.set_u32(k.addr() + 0x10 * 5 + 4, 0x7fc0_0000);
        e.register(QUATERNION_NEGATE, |_, _| panic!("no key is negated"));
        e.call(0x00a2_8ab0, &args![k, 6u32, 0x10u32]);
        let got: Vec<u32> = (0..6).map(|i| e.mem.u32(k.addr() + 0x10 * i + 4)).collect();
        assert_eq!(
            got,
            vec![
                1.0f32.to_bits(),
                (-1.0f32).to_bits(),
                0.5f32.to_bits(),
                1.0f32.to_bits(),
                (-1.0f32).to_bits(),
                0x7fc0_0000,
            ]
        );
    }

    #[test]
    fn fewer_than_four_keys_go_through_the_tail_loop() {
        let mut e = engine();
        let k = keys(&mut e, 0x10, &[[5.0, 0.0, 0.0, 0.0]]);
        e.call(0x00a2_8ab0, &args![k, 1u32, 0x10u32]);
        assert_eq!(quat(&e, k, 0x10, 0)[0], 1.0);
    }
}
