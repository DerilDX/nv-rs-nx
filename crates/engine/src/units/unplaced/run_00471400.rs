//! `(unplaced)/run_00471400` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_byteswap_uint64` (`00ec3bf0`, cdecl: the value as two words, low
/// first; the result in `EDX:EAX`).
const BYTESWAP_UINT64: u32 = 0x00ec_3bf0;

// Translated from 004717f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the eight bytes at `value` in place (`_byteswap_uint64`).
/// cdecl with two words, as its callers push them (`005e2870`, `00471400`):
/// the address and a flag byte the body never reads.
pub fn fn_004717f0(e: &mut Engine, value: Ptr, _unused_1: u32) {
    let low = e.mem.u32(value.addr());
    let high = e.mem.u32(value.addr() + 4);
    let swapped = e.call(BYTESWAP_UINT64, &args![low, high]);
    e.mem.set_u32(value.addr(), swapped.eax);
    e.mem.set_u32(value.addr() + 4, swapped.edx);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004717f0, fn_004717f0(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_swapped_edx_eax() {
        let mut e = Engine::new();
        e.register(BYTESWAP_UINT64, |_, a| Ret {
            eax: a[0] ^ 0x1111_1111,
            edx: a[1] ^ 0x2222_2222,
            ..Ret::default()
        });
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x0102_0304);
        e.mem.set_u32(value + 4, 0x0506_0708);
        e.call_log = Some(vec![]);
        e.call(0x0047_17f0, &[value, 0]);
        assert_eq!(e.mem.u32(value), 0x0102_0304 ^ 0x1111_1111);
        assert_eq!(e.mem.u32(value + 4), 0x0506_0708 ^ 0x2222_2222);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(BYTESWAP_UINT64, vec![0x0102_0304, 0x0506_0708])]
        );
    }
}
