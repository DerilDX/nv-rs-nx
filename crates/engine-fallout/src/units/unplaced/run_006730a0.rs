//! `(unplaced)/run_006730a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0067fcb0`: constructor of the 8-byte value at `TESPackage + 0x38`.
const VALUE_CONSTRUCT: u32 = 0x0067_fcb0;
/// `0067fd40`: fills that value from a source object.
const VALUE_FILL: u32 = 0x0067_fd40;
/// `00483710` (`tesform.cpp`): the destructor run on the stack value.
const VALUE_DESTRUCT: u32 = 0x0048_3710;

// Translated from 00673180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00673180 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// Fills the value at `this + 0x38` from `source` (one stack word) through
/// `0067fd40`. When `source` is null it constructs an 8-byte value on the
/// stack (`0067fcb0`), fills from that one's address instead, and destroys
/// it (`00483710`).
pub fn fn_00673180(e: &mut Engine, this: Ptr, source: Ptr) {
    let member = this.byte_add(0x38);
    if !source.is_null() {
        e.call(VALUE_FILL, &args![member, source]);
        return;
    }
    e.with_stack(8, |e, temporary| {
        e.call(VALUE_CONSTRUCT, &args![temporary]);
        e.call(VALUE_FILL, &args![member, temporary]);
        e.call(VALUE_DESTRUCT, &args![temporary]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0067_3180, fn_00673180(Ptr, Ptr))]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_00673180_fills_from_the_source_or_a_temporary() {
        let mut e = Engine::new();
        for callee in [VALUE_CONSTRUCT, VALUE_FILL, VALUE_DESTRUCT] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0067_3180, &args![0x4000u32, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(VALUE_FILL, vec![0x4038, 0x7000])]);

        e.call_log = Some(vec![]);
        e.call(0x0067_3180, &args![0x4000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 4);
        let temporary = log[1].1[0];
        assert_eq!(log[1], (VALUE_CONSTRUCT, vec![temporary]));
        assert_eq!(log[2], (VALUE_FILL, vec![0x4038, temporary]));
        assert_eq!(log[3], (VALUE_DESTRUCT, vec![temporary]));
    }
}
