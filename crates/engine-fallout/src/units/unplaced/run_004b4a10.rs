//! `(unplaced)/run_004b4a10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004b4ad0`: thiscall with two stack words (the body keeps the stack
/// 16-byte aligned itself and returns nothing).
const IMPLEMENTATION: u32 = 0x004b_4ad0;

// Translated from 004b4ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall wrapper (`RET 8`): forwards `this` and both stack
/// words, in order, to `004b4ad0`.
pub fn fn_004b4ab0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(IMPLEMENTATION, &args![this, first, second]);
}

/// The function `fn_004b4f20` calls on the member (`006815c0`).
const CALLEE_006815C0: u32 = 0x0068_15c0;

// Translated from 004b4f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `006815c0` with `ECX` = `this + 0x10` and passes on its pointer result.
pub fn fn_004b4f20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_006815C0, &args![this.byte_add(0x10)]).ptr()
}

/// One lane of an SSE arithmetic instruction (`MULPS`, `ADDPS`): a NaN in the destination
/// operand wins (quieted), then a NaN in the source operand, otherwise the result of `op`.
fn sse_lane(dest: u32, source: u32, op: fn(f32, f32) -> f32) -> u32 {
    let quiet = 0x0040_0000;
    if f32::from_bits(dest).is_nan() {
        dest | quiet
    } else if f32::from_bits(source).is_nan() {
        source | quiet
    } else {
        op(f32::from_bits(dest), f32::from_bits(source)).to_bits()
    }
}

/// `MULPS dest, source` on the four lanes (an XMM register as four float bit patterns).
fn mulps(dest: [u32; 4], source: [u32; 4]) -> [u32; 4] {
    let mul: fn(f32, f32) -> f32 = |a, b| a * b;
    [0, 1, 2, 3].map(|lane| sse_lane(dest[lane], source[lane], mul))
}

/// `ADDPS dest, source` on the four lanes.
fn addps(dest: [u32; 4], source: [u32; 4]) -> [u32; 4] {
    let add: fn(f32, f32) -> f32 = |a, b| a + b;
    [0, 1, 2, 3].map(|lane| sse_lane(dest[lane], source[lane], add))
}

/// `MOVAPS xmm, [address]`: four words. (The instruction faults on an address that is not
/// 16-byte aligned; the engine's heap is only 8-byte aligned, so the fault is not modelled.)
fn load_lanes(e: &Engine, address: u32) -> [u32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.u32(address.wrapping_add(offset)))
}

// Translated from 004b4cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall with one stack word: the address of element `index` of an array of
/// 16-byte rows starting at `this` (`SHL EAX, 4` then `ADD EAX, this`, 32-bit wrap-around). It
/// reads no memory.
pub fn fn_004b4cf0(_e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    Ptr::new((index << 4).wrapping_add(this.addr()))
}

// Translated from 004b4ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004b4ad0 - the /GS stack cookie check (00ec408c) at the end is not translated.
/// No map name. Thiscall with two stack words (`RET 8`; `fn_004b4ab0` is its wrapper): `this` is a
/// 16-byte destination, `matrix` points at three 16-byte rows (`fn_004b4cf0` gives each row's
/// address) and `vector` at 16 bytes whose first three floats are used. It multiplies the vector
/// by the matrix's rows the way a 3x3 matrix times a vector does: the rows' lanes 0..2 are
/// transposed into columns by `UNPCKLPS`/`UNPCKHPS`/`SHUFPS`/`MOVLHPS`/`MOVHLPS`, each column is
/// multiplied (`MULPS`) by the vector's x, y or z broadcast to all lanes, and the three products
/// are added, `(x column + y column) + z column` (`ADDPS`), then stored with `MOVAPS` to `this`.
/// Lanes 0..2 are the three dot products; lane 3 is `row2[1] * x + row2[3] * y + row2[3] * z`,
/// whatever those words hold (the columns' fourth lanes come from the shuffles, as in the exe).
pub fn fn_004b4ad0(e: &mut Engine, this: Ptr, matrix: Ptr, vector: Ptr) {
    let row0_address = fn_004b4cf0(e, matrix, 0);
    let row0 = load_lanes(e, row0_address.addr());
    let row1_address = fn_004b4cf0(e, matrix, 1);
    let row1 = load_lanes(e, row1_address.addr());
    let row2_address = fn_004b4cf0(e, matrix, 2);
    let row2 = load_lanes(e, row2_address.addr());
    // UNPCKLPS / UNPCKHPS / SHUFPS 0xe4 / MOVLHPS / MOVHLPS / SHUFPS 0xd4, in the exe's order.
    let low_pairs = [row0[0], row1[0], row0[1], row1[1]];
    let high_pairs = [row0[2], row1[2], row0[3], row1[3]];
    let column2 = [high_pairs[0], high_pairs[1], row2[2], row2[3]];
    let column0 = [low_pairs[0], low_pairs[1], row2[0], row2[1]];
    let mixed = [low_pairs[2], low_pairs[3], column0[2], column0[3]];
    let column1 = [mixed[0], mixed[1], row2[1], row2[3]];
    let v = load_lanes(e, vector.addr());
    let x = mulps(column0, [v[0]; 4]);
    let y = mulps(column1, [v[1]; 4]);
    let z = mulps(column2, [v[2]; 4]);
    let sum = addps(addps(x, y), z);
    for (lane, word) in sum.iter().enumerate() {
        e.mem
            .set_u32(this.addr().wrapping_add(4 * lane as u32), *word);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b4ab0, fn_004b4ab0(Ptr, u32, u32)),
        entry!(0x004b_4f20, fn_004b4f20(Ptr) -> Ptr),
        entry!(0x004b_4ad0, fn_004b4ad0(Ptr, Ptr, Ptr)),
        entry!(0x004b_4cf0, fn_004b4cf0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_this_and_both_words() {
        let mut e = Engine::new();
        e.register(IMPLEMENTATION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004b_4ab0, &args![Ptr::<()>::new(0x100), 7u32, 9u32]);
        let log = e.call_log.take().unwrap();
        let forwarded = log
            .iter()
            .find(|(addr, _)| *addr == IMPLEMENTATION)
            .unwrap();
        assert_eq!(forwarded.1, vec![0x100, 7, 9]);
    }

    #[test]
    fn l68d_004b4f20_passes_this_plus_10_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_006815C0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004b_4f20, &args![0x4100u32]).ptr::<()>().addr(),
            0x1445
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_006815C0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x10]);
    }
}

#[cfg(test)]
mod physics_step_tests {
    use super::*;

    /// A 16-byte aligned block of `words`.
    fn block(e: &mut Engine, words: &[u32]) -> u32 {
        let raw = e.mem.alloc(4 * words.len() as u32 + 16);
        let aligned = (raw + 15) & !15;
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(aligned + 4 * i as u32, *w);
        }
        aligned
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    fn lanes(e: &Engine, at: u32) -> Vec<u32> {
        (0..4).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    #[test]
    fn row_address_is_this_plus_sixteen_times_the_index() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x004b_4cf0, &args![0x2000u32, 0u32]).u32(), 0x2000);
        assert_eq!(e.call(0x004b_4cf0, &args![0x2000u32, 1u32]).u32(), 0x2010);
        assert_eq!(e.call(0x004b_4cf0, &args![0x2000u32, 2u32]).u32(), 0x2020);
        assert_eq!(
            e.call(0x004b_4cf0, &args![0x10u32, 0x1000_0001u32]).u32(),
            0x20,
            "the shift drops the high bits: 32-bit arithmetic"
        );
        assert_eq!(
            e.call(0x004b_4cf0, &args![0xffff_fff0u32, 2u32]).u32(),
            0x10
        );
    }

    #[test]
    fn matrix_times_vector_gives_the_three_dot_products_and_the_exe_fourth_lane() {
        let mut e = Engine::new();
        // Three rows of four floats; lane 3 of rows 0 and 1 is never used, row 2 supplies lane 3.
        let rows = [
            1.5f32, 2.0, 3.0, 99.0, //
            4.0, 5.0, 6.0, 98.0, //
            7.0, 8.0, 9.0, 0.5,
        ];
        let matrix = block(&mut e, &bits(&rows));
        let vector = block(&mut e, &bits(&[10.0, 100.0, 1000.0, 7777.0]));
        let this = block(&mut e, &[0xdead_beef; 4]);
        e.call(0x004b_4ad0, &args![this, matrix, vector]);
        let expected = [
            1.5f32 * 10.0 + 2.0 * 100.0 + 3.0 * 1000.0,
            4.0 * 10.0 + 5.0 * 100.0 + 6.0 * 1000.0,
            7.0 * 10.0 + 8.0 * 100.0 + 9.0 * 1000.0,
            // lane 3: row 2's lane 1 times x, lane 3 times y and z
            8.0 * 10.0 + 0.5 * 100.0 + 0.5 * 1000.0,
        ];
        assert_eq!(lanes(&e, this), bits(&expected));
    }

    #[test]
    fn wrapper_and_the_function_agree_through_the_registry() {
        let mut e = Engine::new();
        let matrix = block(
            &mut e,
            &bits(&[
                0.0, 1.0, 0.0, 0.0, // x row picks y
                0.0, 0.0, 1.0, 0.0, // y row picks z
                1.0, 0.0, 0.0, 0.0, // z row picks x
            ]),
        );
        let vector = block(&mut e, &bits(&[2.0, 3.0, 4.0, 5.0]));
        let this = block(&mut e, &[0; 4]);
        e.call(0x004b_4ab0, &args![this, matrix, vector]);
        let out: Vec<f32> = lanes(&e, this).iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(&out[..3], &[3.0, 4.0, 2.0]);
        // lane 3: row 2 lane 1 (0.0) * 2 + row 2 lane 3 (0.0) * 3 + 0.0 * 4
        assert_eq!(out[3], 0.0);
    }

    #[test]
    fn a_nan_in_the_matrix_column_wins_over_the_vector_nan_and_is_quieted() {
        let mut e = Engine::new();
        let snan = 0x7f80_0001u32; // a signalling NaN
        let vector_nan = 0x7fc0_0042u32;
        let mut rows = bits(&[
            1.0, 2.0, 3.0, 0.0, //
            4.0, 5.0, 6.0, 0.0, //
            7.0, 8.0, 9.0, 0.0,
        ]);
        rows[0] = snan; // row 0, lane 0 -> column 0, lane 0
        let matrix = block(&mut e, &rows);
        let vector = block(&mut e, &[vector_nan, 1.0f32.to_bits(), 1.0f32.to_bits(), 0]);
        let this = block(&mut e, &[0; 4]);
        e.call(0x004b_4ad0, &args![this, matrix, vector]);
        let out = lanes(&e, this);
        // x product lane 0: destination (matrix) NaN, quieted; the later adds keep a NaN dest.
        assert_eq!(out[0], snan | 0x0040_0000);
        // lanes 1 and 2: column 0 is finite, the vector's NaN x is the source: it wins, quieted.
        assert_eq!(out[1], vector_nan);
        assert_eq!(out[2], vector_nan);
    }

    #[test]
    fn only_this_is_written() {
        let mut e = Engine::new();
        let matrix = block(&mut e, &bits(&[1.0; 12]));
        let vector = block(&mut e, &bits(&[1.0; 4]));
        let this = block(&mut e, &[0x1111_1111; 8]);
        e.call(0x004b_4ad0, &args![this, matrix, vector]);
        assert_eq!(e.mem.u32(this + 16), 0x1111_1111);
        assert_eq!(lanes(&e, matrix)[0], 1.0f32.to_bits());
        assert_eq!(lanes(&e, this)[..3], bits(&[3.0, 3.0, 3.0])[..]);
    }
}
