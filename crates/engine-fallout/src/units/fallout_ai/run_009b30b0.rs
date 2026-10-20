//! `fallout/ai/run_009b30b0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The getter `fn_009b3100` reads through (`00560d30`): given the member at
/// `this + 0x10` and an index, it returns the address of a `float`.
const FLOAT_SLOT_GETTER: u32 = 0x0056_0d30;
/// `00622570`: called with the result object and a `bool` (one stack word).
const RESULT_FROM_BOOL: u32 = 0x0062_2570;
/// `00401030`: the memory release the scalar deleting destructors call
/// (cdecl, one word).
const RELEASE: u32 = 0x0040_1030;

// Translated from 009b30b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares the values `fn_009b3100` reads from `this` and from `other`:
/// sets up `result` through `00622570` with `other's > this's` (false when
/// either is a NaN: `FCOMP`, then `TEST AH,0x41`), and returns `result`.
/// Two stack words (`result`, `other`), `RET 8`.
pub fn fn_009b30b0(e: &mut Engine, this: Ptr, result: Ptr, other: Ptr) -> Ptr {
    let mine = f64::from(fn_009b3100(e, this));
    let theirs = f64::from(fn_009b3100(e, other));
    let greater = theirs > mine;
    e.call(RESULT_FROM_BOOL, &args![result, greater as u32]);
    result
}

// Translated from 009b3100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at the address `00560d30` gives for the member at
/// `this + 0x10` and index 3, loaded into `ST0`.
pub fn fn_009b3100(e: &mut Engine, this: Ptr) -> f32 {
    let slot = e
        .call(FLOAT_SLOT_GETTER, &args![this.byte_add(0x10), 3u32])
        .u32();
    x87_quiet(e.mem.f32(slot))
}

// Translated from 009b3120 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual that returns the constant 2 (`ECX` only, no stack words).
pub fn fn_009b3120(_e: &mut Engine, _this: Ptr) -> u32 {
    2
}

// Translated from 009b3130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: runs the destructor `009b3f80`, then
/// releases the object (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_009b3130(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x009b_3f80, &args![this]);
    if flags & 1 != 0 {
        e.call(RELEASE, &args![this]);
    }
    this
}

// Translated from 009b3490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: runs the destructor `009bb940`, then
/// releases the object (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_009b3490(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x009b_b940, &args![this]);
    if flags & 1 != 0 {
        e.call(RELEASE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009b_30b0, fn_009b30b0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x009b_3100, fn_009b3100(Ptr) -> f32),
        entry!(0x009b_3120, fn_009b3120(Ptr) -> u32),
        entry!(0x009b_3130, fn_009b3130(Ptr, u32) -> Ptr),
        entry!(0x009b_3490, fn_009b3490(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    /// The getter double: the float lives at `member + 0x40 + 4 * index`.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(FLOAT_SLOT_GETTER, |_, a| Ret {
            eax: a[0] + 0x40 + 4 * a[1],
            ..Ret::default()
        });
        e.register(RESULT_FROM_BOOL, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e
    }

    fn with_value(e: &mut Engine, value: f32) -> u32 {
        let object = e.mem.alloc(0x80);
        e.mem.set_f32(object + 0x10 + 0x40 + 12, value);
        object
    }

    #[test]
    fn fn_009b3100_reads_slot_3_of_the_member() {
        let mut e = engine();
        let object = with_value(&mut e, 4.5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009b_3100, &args![object]).f32(), 4.5);
        let log = e.call_log.take().unwrap();
        let getter: Vec<_> = log.iter().filter(|c| c.0 == FLOAT_SLOT_GETTER).collect();
        assert_eq!(getter[0].1, vec![object + 0x10, 3]);
    }

    #[test]
    fn fn_009b30b0_is_true_when_the_other_value_is_greater() {
        let mut e = engine();
        let result = e.mem.alloc(4);
        let low = with_value(&mut e, 1.0);
        let high = with_value(&mut e, 2.0);
        let nan = with_value(&mut e, f32::NAN);
        let back = e.call(0x009b_30b0, &args![low, result, high]).u32();
        assert_eq!(back, result);
        assert_eq!(e.mem.u32(result), 1);
        e.call(0x009b_30b0, &args![high, result, low]);
        assert_eq!(e.mem.u32(result), 0);
        e.call(0x009b_30b0, &args![low, result, low]);
        assert_eq!(e.mem.u32(result), 0);
        e.call(0x009b_30b0, &args![low, result, nan]);
        assert_eq!(e.mem.u32(result), 0);
    }

    #[test]
    fn fn_009b3120_returns_two() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009b_3120, &args![0x4000u32]).u32(), 2);
    }

    #[test]
    fn deleting_destructors_release_only_with_bit_0() {
        for (addr, dtor) in [(0x009b_3130, 0x009b_3f80), (0x009b_3490, 0x009b_b940)] {
            let mut e = Engine::new();
            e.register(dtor, |_, _| Ret::default());
            e.register(RELEASE, |_, _| Ret::default());
            for flags in [0u32, 1, 2, 3] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(addr, &args![0x5000u32, flags]).u32(), 0x5000);
                let log = e.call_log.take().unwrap();
                let dtors = log.iter().filter(|c| c.0 == dtor).count();
                let releases: Vec<_> = log.iter().filter(|c| c.0 == RELEASE).collect();
                assert_eq!(dtors, 1);
                assert_eq!(releases.len(), (flags & 1) as usize);
                if let Some(release) = releases.first() {
                    assert_eq!(release.1, vec![0x5000]);
                }
            }
        }
    }
}
