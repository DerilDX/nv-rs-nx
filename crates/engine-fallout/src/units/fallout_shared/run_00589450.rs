//! `fallout shared/run_00589450` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A constructor and a destructor body the engine map gives no name. The
//! offsets they set match `AudioMarkerData` in the Xbox PDB
//! (`LocationName` at +0, the layer percents at +0xc and +0x10, `Location`
//! at +0x18, `pController` at +0x24, `lBuoyMarkers` at +0x2c); the class is
//! not confirmed from the PC code.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00402d00` (`ECX` = object): the base at +0.
const BASE_CONSTRUCT: u32 = 0x0040_2d00;
/// `006815c0` (`ECX` = the point at +0x18).
const POINT_PREPARE: u32 = 0x0068_15c0;
/// `0096a2d0` (`ECX` = the list at +0x2c).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `0046ffb0` (`ECX` = the list at +0x2c).
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `00591300` (`ECX` = object): the base at +0.
const BASE_DESTRUCT: u32 = 0x0059_1300;
/// The floats stored at +0xc and +0x10 (`01026994`, `010320e4`).
const FIRST_FLOAT: u32 = 0x0102_6994;
const SECOND_FLOAT: u32 = 0x0103_20e4;
/// The point stored at +0x18 (`011f426c`, three floats).
const ORIGIN: u32 = 0x011f_426c;

// Translated from 00589450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00589450 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs the object: the base (`00402d00`), `006815c0` on the point
/// at +0x18, the list at +0x2c (`0096a2d0`), the floats at `01026994` and
/// `010320e4` into +0xc and +0x10 (`FLD`/`FSTP`, a signalling NaN quieted),
/// the point at `011f426c` copied to +0x18, and +0x24 zero. Returns
/// `this`.
pub fn fn_00589450(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![this]);
    e.call(POINT_PREPARE, &args![base + 0x18]);
    e.call(LIST_CONSTRUCT, &args![base + 0x2c]);
    let first = x87_quiet_bits(e.mem.u32(FIRST_FLOAT));
    e.mem.set_u32(base + 0xc, first);
    let second = x87_quiet_bits(e.mem.u32(SECOND_FLOAT));
    e.mem.set_u32(base + 0x10, second);
    for i in 0..3 {
        let word = e.mem.u32(ORIGIN + 4 * i);
        e.mem.set_u32(base + 0x18 + 4 * i, word);
    }
    e.mem.set_u32(base + 0x24, 0);
    this
}

// Translated from 00589500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00589500 - the exception-unwinding frame (FS:[0]) is not translated.
/// The matching destructor body: the list at +0x2c (`0046ffb0`), then the
/// base (`00591300`).
pub fn fn_00589500(e: &mut Engine, this: Ptr) {
    e.call(LIST_DESTRUCT, &args![this.addr() + 0x2c]);
    e.call(BASE_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_9450, fn_00589450(Ptr) -> Ptr),
        entry!(0x0058_9500, fn_00589500(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    #[test]
    fn at_00589450_constructs_the_parts_and_sets_the_fields() {
        let mut e = Engine::new();
        for addr in [BASE_CONSTRUCT, POINT_PREPARE, LIST_CONSTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        e.map(FIRST_FLOAT, 4);
        e.map(SECOND_FLOAT, 4);
        e.map(ORIGIN, 0xc);
        e.mem.set_u32(FIRST_FLOAT, 0.33f32.to_bits());
        e.mem.set_u32(SECOND_FLOAT, 0xff80_0002);
        for i in 0..3 {
            e.mem.set_u32(ORIGIN + 4 * i, 0x100 + i);
        }
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x24, 0xffff_ffff);
        let (result, log) = run(&mut e, 0x0058_9450, &[this]);
        assert_eq!(result.u32(), this);
        assert_eq!(
            log,
            vec![
                (BASE_CONSTRUCT, vec![this]),
                (POINT_PREPARE, vec![this + 0x18]),
                (LIST_CONSTRUCT, vec![this + 0x2c]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0.33f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x10), 0xffc0_0002);
        let point: Vec<u32> = (0..3).map(|i| e.mem.u32(this + 0x18 + 4 * i)).collect();
        assert_eq!(point, vec![0x100, 0x101, 0x102]);
        assert_eq!(e.mem.u32(this + 0x24), 0);
    }

    #[test]
    fn at_00589500_destroys_the_list_then_the_base() {
        let mut e = Engine::new();
        for addr in [LIST_DESTRUCT, BASE_DESTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        let (_, log) = run(&mut e, 0x0058_9500, &[0x4000]);
        assert_eq!(
            log,
            vec![(LIST_DESTRUCT, vec![0x402c]), (BASE_DESTRUCT, vec![0x4000])]
        );
    }
}
