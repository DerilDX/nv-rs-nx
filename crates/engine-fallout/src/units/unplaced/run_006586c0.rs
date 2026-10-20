//! `(unplaced)/run_006586c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00658930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the 16-bit field at `+0xa` of `this` (zero-extended into `EAX`).
pub fn fn_00658930(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0xa)
}

/// The matrix's row count (+0x18) and column count (+0x1C).
const ROWS: u32 = 0x18;
const COLUMNS: u32 = 0x1c;
/// The row access `0060b370` (`thiscall (matrix, &result, row)`, `RET 8`:
/// the row's first element and the column count into the 8-byte result,
/// answered) and the address of a column in it (`00629ab0`, `thiscall (row,
/// column)`, `RET 4`).
const ELEMENT_AT: u32 = 0x0060_b370;
const ELEMENT_ADDRESS: u32 = 0x0062_9ab0;
/// The per-element value `006591f0` (`cdecl`, one float, result in ST0).
const ELEMENT_VALUE: u32 = 0x0065_91f0;
/// The matrix constructor (`00659000`, `thiscall (matrix, rows, columns)`),
/// copy into the result (`00658f30`, `thiscall (result, &matrix)`) and
/// destructor (`00449680`, `thiscall`).
const MATRIX_CONSTRUCT: u32 = 0x0065_9000;
const MATRIX_COPY: u32 = 0x0065_8f30;
const MATRIX_DESTRUCT: u32 = 0x0044_9680;

/// The address of element (`row`, `column`) of the matrix at `matrix`.
fn element(e: &mut Engine, matrix: u32, row: u32, column: u32) -> u32 {
    e.with_stack(8, |e, result| {
        let at = e.call(ELEMENT_AT, &args![matrix, result, row]).u32();
        e.call(ELEMENT_ADDRESS, &args![at, column]).u32()
    })
}

// Translated from 006588a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the sum over every element (rows, then
/// columns) of `006591f0` of its value, kept as a float (each addition
/// `value + sum`).
pub fn fn_006588a0(e: &mut Engine, this: Ptr) -> f64 {
    let mut sum = x87_store32(0.0);
    let mut row = 0u32;
    while row < e.mem.u32(this.addr().wrapping_add(ROWS)) {
        let mut column = 0u32;
        while column < e.mem.u32(this.addr().wrapping_add(COLUMNS)) {
            let at = element(e, this.addr(), row, column);
            let value = x87_quiet_bits(e.mem.u32(at));
            let mapped = e.call(ELEMENT_VALUE, &args![value]).f64();
            sum = x87_store32(x87_add(mapped, x87_m32(sum)));
            column += 1;
        }
        row += 1;
    }
    x87_load32(sum)
}

// Translated from 006587a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006587a0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map: the transpose: a local matrix of `columns x
/// rows` (`00659000`) gets element (`column`, `row`) from each (`row`,
/// `column`), is copied into `result` (`00658f30`) and destroyed
/// (`00449680`). Answers `result`. The exception-unwinding frame is not
/// translated.
pub fn fn_006587a0(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    e.with_stack(0x20, |e, local| {
        let rows = e.mem.u32(this.addr().wrapping_add(ROWS));
        let columns = e.mem.u32(this.addr().wrapping_add(COLUMNS));
        e.call(MATRIX_CONSTRUCT, &args![local, columns, rows]);
        let mut row = 0u32;
        while row < e.mem.u32(this.addr().wrapping_add(ROWS)) {
            let mut column = 0u32;
            while column < e.mem.u32(this.addr().wrapping_add(COLUMNS)) {
                let from = element(e, this.addr(), row, column);
                let to = element(e, local.addr(), column, row);
                let value = x87_quiet_bits(e.mem.u32(from));
                e.mem.set_u32(to, value);
                column += 1;
            }
            row += 1;
        }
        e.call(MATRIX_COPY, &args![result, local]);
        e.call(MATRIX_DESTRUCT, &args![local]);
    });
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00658930, fn_00658930(Ptr) -> u16),
        entry!(0x006588a0, fn_006588a0(Ptr) -> f64),
        entry!(0x006587a0, fn_006587a0(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_field_at_offset_0xa() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u16(object.addr() + 0xa, 0x1234);
        assert_eq!(e.call(0x0065_8930, &args![object]).u16(), 0x1234);
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;

    /// A 2 x 3 matrix whose element (r, c) is 10 r + c; the row access
    /// answers (row start, columns).
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(ELEMENT_AT, |e, a| {
            let data = e.mem.u32(a[0] + 0x10);
            let columns = e.mem.u32(a[0] + COLUMNS);
            e.mem.set_u32(a[1], data + 4 * a[2] * columns);
            e.mem.set_u32(a[1] + 4, columns);
            a[1].into_ret()
        });
        e.register(ELEMENT_ADDRESS, |e, a| {
            (e.mem.u32(a[0]) + 4 * a[1]).into_ret()
        });
        let this = e.mem.alloc(0x24);
        let data = e.mem.alloc(4 * 6);
        e.mem.set_u32(this + ROWS, 2);
        e.mem.set_u32(this + COLUMNS, 3);
        e.mem.set_u32(this + 0x10, data);
        for r in 0..2u32 {
            for c in 0..3u32 {
                e.mem.set_f32(data + 4 * (3 * r + c), (10 * r + c) as f32);
            }
        }
        (e, this)
    }

    #[test]
    fn sums_the_mapped_elements() {
        let (mut e, this) = engine();
        e.register(ELEMENT_VALUE, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) * 2.0,
            ..Ret::default()
        });
        // 2 * (0 + 1 + 2 + 10 + 11 + 12) = 72.
        assert_eq!(e.call(0x006588a0, &args![this]).f64(), 72.0);
    }

    #[test]
    fn transposes_into_the_result() {
        let (mut e, this) = engine();
        e.register(MATRIX_CONSTRUCT, |e, a| {
            let data = e.mem.alloc(4 * a[1] * a[2]);
            e.mem.set_u32(a[0] + ROWS, a[1]);
            e.mem.set_u32(a[0] + COLUMNS, a[2]);
            e.mem.set_u32(a[0] + 0x10, data);
            a[0].into_ret()
        });
        e.map(0x7300_0000, 0x40);
        e.register(MATRIX_COPY, |e, a| {
            let data = e.mem.u32(a[1] + 0x10);
            for i in 0..6 {
                let v = e.mem.u32(data + 4 * i);
                e.mem.set_u32(0x7300_0000 + 4 * i, v);
            }
            Ret::default()
        });
        e.register(MATRIX_DESTRUCT, |_, _| Ret::default());
        let result = e.mem.alloc(0x24);
        assert_eq!(e.call(0x006587a0, &args![this, result]).u32(), result);
        let got: Vec<f32> = (0..6).map(|i| e.mem.f32(0x7300_0000 + 4 * i)).collect();
        assert_eq!(got, [0.0, 10.0, 1.0, 11.0, 2.0, 12.0]);
    }
}
