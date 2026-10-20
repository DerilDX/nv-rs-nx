//! `NiAnimation/run_00a276a0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a27b00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3656` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a27b00(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3656) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_3656, 0x1);
    e.mem.set_u32(0x011f_3dc8, 0xe7cd40);
    e.mem.set_u32(0x011f_3a68, 0xa29680);
    e.mem.set_u32(0x011f_39d8, 0xa29680);
    e.mem.set_u32(0x011f_3948, 0xe7cd40);
    e.mem.set_u32(0x011f_38b8, 0xa29680);
    e.mem.set_u8(0x011f_375e, 0x10);
    e.call(0x00a2_7aa0, &args![0x1u32, 0x0u32]);
    1
}

// Translated from 00a28680 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3668` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a28680(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3668) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_3668, 0x1);
    e.mem.set_u32(0x011f_3dd0, 0xa281e0);
    e.mem.set_u32(0x011f_3a70, 0xa28260);
    e.mem.set_u32(0x011f_39e0, 0xa27b60);
    e.mem.set_u32(0x011f_3950, 0xa27c40);
    e.mem.set_u32(0x011f_38c0, 0xa2a600);
    e.mem.set_u8(0x011f_3760, 0x40);
    e.call(0x00a2_8620, &args![0x1u32, 0x2u32]);
    1
}

// Translated from 00a27aa0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a27aa0(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa291e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa27760);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa29680);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0x0);
}

// Translated from 00a28620 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a28620(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa28060);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa27c80);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa27d60);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa27e40);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa27bd0);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa27ee0);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0xa282a0);
}

/// The Ni memory manager (`MemoryManager` singleton the key arrays are
/// allocated from).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `00aa3e40` (thiscall on the memory manager, byte count): allocates.
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `00a27430` (thiscall, stream): loads a key's time.
const KEY_LOAD_TIME: u32 = 0x00a2_7430;
/// `00a59be0` (`NiColor::LoadBinary` in the engine map, thiscall, stream):
/// reads three floats.
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `2.0`, `3.0` and `6.0` (`double`).
const TWO: u32 = 0x0101_1590;
const THREE: u32 = 0x0102_1928;
const SIX: u32 = 0x0103_4208;
/// The number of curvature samples per key (an `int` global).
const CURVATURE_SAMPLES: u32 = 0x011a_7ed4;
/// The time step between curvature samples (a `float` global).
const CURVATURE_STEP: u32 = 0x011f_3658;
/// `_CIsqrt` (`00ec6040`): square root of ST0.
const CI_SQRT: u32 = 0x00ec_6040;
/// The size of one of these position keys.
const KEY_SIZE: u32 = 0x40;

/// `FLD dword` of the float at `at`.
fn load(e: &Engine, at: u32) -> f64 {
    x87_load32(e.mem.u32(at))
}

/// A `float` memory operand at `at`.
fn operand(e: &Engine, at: u32) -> f64 {
    x87_m32(e.mem.u32(at))
}

/// `FSTP dword` of `value` to `at`.
fn store(e: &mut Engine, at: u32, value: f64) {
    e.mem.set_u32(at, x87_store32(value));
}

/// A float stored and loaded again (an `FSTP dword` into a temporary
/// followed by an `FLD dword` or a memory operand of it).
fn rounded(value: f64) -> f64 {
    x87_m32(x87_store32(value))
}

// Translated from 00a281e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in Ghidra (read from the bytes); the key creator this unit's
/// registration stores for key type 2 (`011f3dd0`): allocates `count` keys
/// of 0x40 bytes behind a count word (the byte count saturates to
/// 0xFFFFFFFF on overflow, as the compiler's array new does) from the
/// memory manager, and loads each key's time (`00a27430`) and the points at
/// +0x10 and +0x1C (`00a59be0`) from `stream`. Returns the keys (null when
/// the allocation fails; the loads still run then, as in the exe).
pub fn fn_00a281e0(e: &mut Engine, stream: Ptr, count: u32) -> u32 {
    let bytes = count.saturating_mul(KEY_SIZE);
    let bytes = bytes.saturating_add(4);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    let keys = if block != 0 {
        e.mem.set_u32(block, count);
        block + 4
    } else {
        0
    };
    let mut key = keys;
    for _ in 0..count {
        e.call(KEY_LOAD_TIME, &args![key, stream]);
        e.call(POINT_LOAD_BINARY, &args![key.wrapping_add(0x10), stream]);
        e.call(POINT_LOAD_BINARY, &args![key.wrapping_add(0x1c), stream]);
        key = key.wrapping_add(KEY_SIZE);
    }
    keys
}

// Translated from 00a27c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: the key's cubic at `time`, per lane `value + (out +
/// (a + b * t) * t) * t` with value +4, out tangent +0x1C, a +0x28, b +0x34,
/// written to `out`. Every x87 operation and float temporary as the exe
/// does them. The third argument (the next key) is not read.
pub fn fn_00a27c80(e: &mut Engine, time: f32, key: Ptr, _unused_3: u32, out: Ptr) {
    let k = key.addr();
    let t = x87_load32(time.to_bits());
    // b * t, as floats.
    let b0 = rounded(x87_mul(load(e, k + 0x34), t));
    let b1 = rounded(x87_mul(load(e, k + 0x38), t));
    let b2 = rounded(x87_mul(load(e, k + 0x3c), t));
    let a0 = rounded(x87_add(load(e, k + 0x28), b0));
    let a1 = rounded(x87_add(load(e, k + 0x2c), b1));
    let a2 = rounded(x87_add(load(e, k + 0x30), b2));
    let c0 = rounded(x87_mul(a0, t));
    let c1 = rounded(x87_mul(a1, t));
    let c2 = rounded(x87_mul(a2, t));
    let d0 = rounded(x87_add(load(e, k + 0x1c), c0));
    let d1 = rounded(x87_add(load(e, k + 0x20), c1));
    let d2 = rounded(x87_add(load(e, k + 0x24), c2));
    let e0 = rounded(x87_mul(d0, t));
    let e1 = rounded(x87_mul(d1, t));
    let e2 = rounded(x87_mul(t, d2));
    let x = x87_add(load(e, k + 4), e0);
    let y = x87_add(load(e, k + 8), e1);
    let z = x87_add(load(e, k + 0xc), e2);
    store(e, out.addr(), x);
    store(e, out.addr() + 4, y);
    store(e, out.addr() + 8, z);
}

// Translated from 00a27d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: the cubic's first derivative at `time`, per lane `out
/// + (2a + 3t * b) * t` (fields as in `00a27c80`), written to `out`. The
/// third argument is not read.
pub fn fn_00a27d60(e: &mut Engine, time: f32, key: Ptr, _unused_3: u32, out: Ptr) {
    let k = key.addr();
    let t = x87_load32(time.to_bits());
    let three: f64 = e.global(THREE);
    let two: f64 = e.global(TWO);
    let three_t = rounded(x87_mul(x87_load64(three.to_bits()), t));
    let b0 = rounded(x87_mul(load(e, k + 0x34), three_t));
    let b1 = rounded(x87_mul(load(e, k + 0x38), three_t));
    let b2 = rounded(x87_mul(three_t, operand(e, k + 0x3c)));
    let two = x87_load64(two.to_bits());
    let a0 = rounded(x87_mul(load(e, k + 0x28), two));
    let a1 = rounded(x87_mul(load(e, k + 0x2c), two));
    let a2 = rounded(x87_mul(two, operand(e, k + 0x30)));
    let s0 = rounded(x87_add(a0, b0));
    let s1 = rounded(x87_add(a1, b1));
    let s2 = rounded(x87_add(a2, b2));
    let p0 = rounded(x87_mul(s0, t));
    let p1 = rounded(x87_mul(s1, t));
    let p2 = rounded(x87_mul(t, s2));
    let x = x87_add(load(e, k + 0x1c), p0);
    let y = x87_add(load(e, k + 0x20), p1);
    let z = x87_add(load(e, k + 0x24), p2);
    store(e, out.addr(), x);
    store(e, out.addr() + 4, y);
    store(e, out.addr() + 8, z);
}

// Translated from 00a27e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: the cubic's second derivative at `time`, per lane `2a
/// + 6t * b` (fields as in `00a27c80`), written to `out`. The third
/// argument is not read.
pub fn fn_00a27e40(e: &mut Engine, time: f32, key: Ptr, _unused_3: u32, out: Ptr) {
    let k = key.addr();
    let six: f64 = e.global(SIX);
    let two: f64 = e.global(TWO);
    let six_t = rounded(x87_mul(x87_load32(time.to_bits()), six));
    let b0 = rounded(x87_mul(load(e, k + 0x34), six_t));
    let b1 = rounded(x87_mul(load(e, k + 0x38), six_t));
    let b2 = rounded(x87_mul(six_t, operand(e, k + 0x3c)));
    let two = x87_load64(two.to_bits());
    let a0 = rounded(x87_mul(load(e, k + 0x28), two));
    let a1 = rounded(x87_mul(load(e, k + 0x2c), two));
    let a2 = rounded(x87_mul(two, operand(e, k + 0x30)));
    let x = x87_add(a0, b0);
    store(e, out.addr(), x);
    let y = x87_add(b1, a1);
    store(e, out.addr() + 4, y);
    let z = x87_add(a2, b2);
    store(e, out.addr() + 8, z);
}

// Translated from 00a27ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: fills the cubic coefficients of the first `count - 1`
/// keys from each key and the next: with `d = next.value - value`,
/// `a = 3d - (next.in + 2 * out)` (+0x28) and `b = (next.in + out) - 2d`
/// (+0x34), in-tangent +0x10, out-tangent +0x1C. Every x87 operation and
/// float temporary as the exe does them.
pub fn fn_00a27ee0(e: &mut Engine, keys: Ptr, count: u32) {
    let mut remaining = count.wrapping_sub(1);
    if remaining == 0 {
        return;
    }
    let two = x87_load64(e.global::<f64>(TWO).to_bits());
    let three = x87_load64(e.global::<f64>(THREE).to_bits());
    let mut key = keys.addr();
    loop {
        let next = key.wrapping_add(KEY_SIZE);
        let mut out2 = [0f64; 3];
        let mut delta3 = [0f64; 3];
        for lane in 0..3u32 {
            out2[lane as usize] = rounded(x87_mul(load(e, key + 0x1c + 4 * lane), two));
        }
        let mut sum = [0f64; 3];
        for lane in 0..3u32 {
            sum[lane as usize] = rounded(x87_add(
                load(e, next + 0x10 + 4 * lane),
                out2[lane as usize],
            ));
        }
        let mut delta = [0f64; 3];
        for lane in 0..3u32 {
            delta[lane as usize] = rounded(x87_sub(
                load(e, next + 4 + 4 * lane),
                operand(e, key + 4 + 4 * lane),
            ));
        }
        for lane in 0..3 {
            delta3[lane] = rounded(x87_mul(delta[lane], three));
        }
        for lane in 0..3u32 {
            let a = x87_sub(delta3[lane as usize], sum[lane as usize]);
            store(e, key + 0x28 + 4 * lane, a);
        }
        let mut delta_again = [0f64; 3];
        for lane in 0..3u32 {
            delta_again[lane as usize] = rounded(x87_sub(
                load(e, next + 4 + 4 * lane),
                operand(e, key + 4 + 4 * lane),
            ));
        }
        let mut delta2 = [0f64; 3];
        for lane in 0..3 {
            delta2[lane] = rounded(x87_mul(delta_again[lane], two));
        }
        let mut tangents = [0f64; 3];
        for lane in 0..3u32 {
            tangents[lane as usize] = rounded(x87_add(
                load(e, next + 0x10 + 4 * lane),
                operand(e, key + 0x1c + 4 * lane),
            ));
        }
        for lane in 0..3u32 {
            let b = x87_sub(tangents[lane as usize], delta2[lane as usize]);
            store(e, key + 0x34 + 4 * lane, b);
        }
        key = next;
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            break;
        }
    }
}

// Translated from 00a28060 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: the largest curvature measure over the first
/// `count - 1` keys: for each key, at the times `i * step` (`011f3658`) for
/// `i` in 0..=`011a7ed4`, the first (`00a27d60`) and second (`00a27e40`)
/// derivatives d1 and d2 give `|d1 x d2| / |d1|^2` (the cross product's
/// length through `_CIsqrt`); the maximum (0 to start) is returned in ST0.
/// Every x87 operation and float or double temporary as the exe does them.
pub fn fn_00a28060(e: &mut Engine, keys: Ptr, count: u32) -> f32 {
    let mut maximum = x87_store32(0.0);
    let mut remaining = count.wrapping_sub(1);
    if remaining != 0 {
        let mut key = keys.addr();
        e.with_stack(0x18, |e, locals| {
            let first = locals.addr();
            let second = first + 0xc;
            loop {
                let next = key.wrapping_add(KEY_SIZE);
                let mut index = 0i32;
                if e.global::<i32>(CURVATURE_SAMPLES) >= 0 {
                    loop {
                        let step = operand(e, CURVATURE_STEP);
                        let time = x87_store32(x87_mul(f64::from(index), step));
                        let time = x87_quiet_bits(time);
                        fn_00a27d60(
                            e,
                            f32::from_bits(time),
                            Ptr::new(key),
                            next,
                            Ptr::new(first),
                        );
                        fn_00a27e40(
                            e,
                            f32::from_bits(time),
                            Ptr::new(key),
                            next,
                            Ptr::new(second),
                        );
                        let (d1x, d1y, d1z) =
                            (load(e, first), load(e, first + 4), load(e, first + 8));
                        let (d2x, d2y, d2z) =
                            (load(e, second), load(e, second + 4), load(e, second + 8));
                        let cross_x = rounded(x87_sub(x87_mul(d1y, d2z), x87_mul(d1z, d2y)));
                        let cross_y = rounded(x87_sub(x87_mul(d1z, d2x), x87_mul(d2z, d1x)));
                        let cross_z = rounded(x87_sub(x87_mul(d2y, d1x), x87_mul(d2x, d1y)));
                        let length2 = x87_add(
                            x87_mul(cross_z, cross_z),
                            x87_add(x87_mul(cross_x, cross_x), x87_mul(cross_y, cross_y)),
                        );
                        let length2 = rounded(length2);
                        let root = e.call(CI_SQRT, &args![length2]).st0;
                        let root = rounded(root);
                        let speed2 = x87_add(
                            x87_add(x87_mul(d1x, d1x), x87_mul(d1y, d1y)),
                            x87_mul(d1z, d1z),
                        );
                        let speed2 = rounded(speed2);
                        let curvature = rounded(x87_div(root, speed2));
                        if x87_load32(maximum) < curvature {
                            maximum = x87_store32(curvature);
                        }
                        index += 1;
                        if index > e.global::<i32>(CURVATURE_SAMPLES) {
                            break;
                        }
                    }
                }
                remaining = remaining.wrapping_sub(1);
                key = next;
                if remaining == 0 {
                    break;
                }
            }
        });
    }
    f32::from_bits(x87_quiet_bits(maximum))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a27b00, fn_00a27b00() -> u32),
        entry!(0x00a28680, fn_00a28680() -> u32),
        entry!(0x00a27aa0, fn_00a27aa0(u32, u32)),
        entry!(0x00a28620, fn_00a28620(u32, u32)),
        entry!(0x00a2_81e0, fn_00a281e0(Ptr, u32) -> u32),
        entry!(0x00a2_7c80, fn_00a27c80(f32, Ptr, u32, Ptr)),
        entry!(0x00a2_7d60, fn_00a27d60(f32, Ptr, u32, Ptr)),
        entry!(0x00a2_7e40, fn_00a27e40(f32, Ptr, u32, Ptr)),
        entry!(0x00a2_7ee0, fn_00a27ee0(Ptr, u32)),
        entry!(0x00a2_8060, fn_00a28060(Ptr, u32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_registration_runs_once_and_stores_through_its_helper() {
        let mut e = Engine::new();
        e.map(0x011f_3600, 0x900);
        assert_eq!(e.call(0x00a27b00, &[]).u32(), 1);
        assert_eq!(e.mem.u8(0x011f_3656), 1);
        assert_eq!(e.mem.u32(0x011f_3dc8), 0x00e7_cd40);
        assert_eq!(e.mem.u8(0x011f_375e), 0x10);
        // The helper with row 1, column 0: (1 * 6 + 0) * 4 = 0x18 into
        // each of its tables.
        assert_eq!(e.mem.u32(0x011f_3d20 + 0x18), 0x00a2_91e0);
        assert_eq!(e.mem.u32(0x011f_3810 + 0x18), 0);
        // Once only: the guard is set, later calls answer 0 and store nothing.
        e.mem.set_u32(0x011f_3dc8, 0);
        assert_eq!(e.call(0x00a27b00, &[]).u32(), 0);
        assert_eq!(e.mem.u32(0x011f_3dc8), 0);
    }

    #[test]
    fn the_helper_indexes_by_row_and_column() {
        let mut e = Engine::new();
        e.map(0x011f_3600, 0x900);
        e.mem.set_u32(0x011f_3810 + 0x2c, 0x5555);
        e.call(0x00a27aa0, &[1, 5]);
        // (1 * 6 + 5) * 4 = 0x2c.
        assert_eq!(e.mem.u32(0x011f_3c90 + 0x2c), 0x00a2_7470);
        assert_eq!(e.mem.u32(0x011f_3810 + 0x2c), 0);
    }

    fn constants(e: &mut Engine) {
        e.map(TWO, 8);
        e.map(THREE, 8);
        e.map(SIX, 8);
        e.set_global(TWO, 2.0f64);
        e.set_global(THREE, 3.0f64);
        e.set_global(SIX, 6.0f64);
    }

    /// A key with value v (+4), in-tangent i (+0x10), out-tangent o
    /// (+0x1C), a (+0x28), b (+0x34), each the same in all lanes but scaled
    /// by the lane number plus 1.
    fn key_at(e: &mut Engine, key: u32, v: f32, i: f32, o: f32, a: f32, b: f32) {
        for lane in 0..3u32 {
            let s = (lane + 1) as f32;
            e.mem.set_f32(key + 4 + 4 * lane, v * s);
            e.mem.set_f32(key + 0x10 + 4 * lane, i * s);
            e.mem.set_f32(key + 0x1c + 4 * lane, o * s);
            e.mem.set_f32(key + 0x28 + 4 * lane, a * s);
            e.mem.set_f32(key + 0x34 + 4 * lane, b * s);
        }
    }

    fn lanes(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    #[test]
    fn fn_00a281e0_allocates_and_loads_the_keys() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(KEY_LOAD_TIME, |_, _| Ret::default());
        e.register(POINT_LOAD_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let keys = e.call(0x00a2_81e0, &args![0x5000u32, 2u32]).u32();
        assert_eq!(e.mem.u32(keys - 4), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MEMORY_ALLOCATE, vec![MEMORY_MANAGER, 0x84]));
        let loads: Vec<_> = log.into_iter().skip(2).collect();
        assert_eq!(
            loads,
            vec![
                (KEY_LOAD_TIME, vec![keys, 0x5000]),
                (POINT_LOAD_BINARY, vec![keys + 0x10, 0x5000]),
                (POINT_LOAD_BINARY, vec![keys + 0x1c, 0x5000]),
                (KEY_LOAD_TIME, vec![keys + 0x40, 0x5000]),
                (POINT_LOAD_BINARY, vec![keys + 0x50, 0x5000]),
                (POINT_LOAD_BINARY, vec![keys + 0x5c, 0x5000]),
            ]
        );
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a2_81e0, &args![0x5000u32, 0u32]).u32(), 0);
    }

    #[test]
    fn the_cubic_and_its_derivatives() {
        let mut e = Engine::new();
        constants(&mut e);
        let key = e.mem.alloc(0x40);
        // value 1, out 2, a 3, b 4.
        key_at(&mut e, key, 1.0, 0.0, 2.0, 3.0, 4.0);
        let out = e.mem.alloc(0xc);
        e.call(0x00a2_7c80, &args![2.0f32, key, 0u32, out]);
        // 1 + (2 + (3 + 4t)t)t at t = 2: 1 + (2 + 22)2 = 49.
        assert_eq!(lanes(&e, out), [49.0, 98.0, 147.0]);
        e.call(0x00a2_7d60, &args![2.0f32, key, 0u32, out]);
        // 2 + (6 + 24)2 = 62.
        assert_eq!(lanes(&e, out), [62.0, 124.0, 186.0]);
        e.call(0x00a2_7e40, &args![2.0f32, key, 0u32, out]);
        // 6 + 48 = 54.
        assert_eq!(lanes(&e, out), [54.0, 108.0, 162.0]);
    }

    #[test]
    fn fn_00a27ee0_fills_the_coefficients_of_all_but_the_last_key() {
        let mut e = Engine::new();
        constants(&mut e);
        let keys = e.mem.alloc(0xc0);
        key_at(&mut e, keys, 1.0, 0.0, 2.0, 0.0, 0.0);
        key_at(&mut e, keys + 0x40, 5.0, 3.0, 0.0, 9.0, 9.0);
        key_at(&mut e, keys + 0x80, 0.0, 0.0, 0.0, 7.0, 7.0);
        e.call(0x00a2_7ee0, &args![keys, 1u32]);
        assert_eq!(lanes(&e, keys + 0x28), [0.0; 3], "one key: nothing");
        e.call(0x00a2_7ee0, &args![keys, 2u32]);
        // d = 4: a = 12 - (3 + 4) = 5, b = (3 + 2) - 8 = -3.
        assert_eq!(lanes(&e, keys + 0x28), [5.0, 10.0, 15.0]);
        assert_eq!(lanes(&e, keys + 0x34), [-3.0, -6.0, -9.0]);
        assert_eq!(lanes(&e, keys + 0x68), [9.0, 18.0, 27.0], "last key kept");
    }

    #[test]
    fn fn_00a28060_takes_the_largest_curvature() {
        let mut e = Engine::new();
        constants(&mut e);
        e.map(CURVATURE_SAMPLES, 4);
        e.map(CURVATURE_STEP, 4);
        e.set_global(CURVATURE_SAMPLES, 1i32);
        e.set_global(CURVATURE_STEP, 1.0f32);
        e.register(CI_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).sqrt(),
            ..Ret::default()
        });
        let keys = e.mem.alloc(0x80);
        // d1 = out + (2a + 3t b)t, d2 = 2a + 6t b with per-lane x, 0, 0 by
        // giving the y and z lanes separate values.
        for lane in 0..3u32 {
            e.mem
                .set_f32(keys + 0x1c + 4 * lane, [1.0, 0.0, 0.0][lane as usize]);
            e.mem
                .set_f32(keys + 0x28 + 4 * lane, [0.0, 1.0, 0.0][lane as usize]);
        }
        // t = 0: d1 = (1, 0, 0), d2 = (0, 2, 0): |d1 x d2| / |d1|^2 = 2.
        // t = 1: d1 = (1, 2, 0), d2 = (0, 2, 0): 2 / 5.
        assert_eq!(e.call(0x00a2_8060, &args![keys, 2u32]).f32(), 2.0);
        assert_eq!(e.call(0x00a2_8060, &args![keys, 1u32]).f32(), 0.0);
        e.set_global(CURVATURE_SAMPLES, -1i32);
        assert_eq!(e.call(0x00a2_8060, &args![keys, 2u32]).f32(), 0.0);
    }
}
