//! `NiXenonRenderer/run_00e76c30` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_memcpy` (`00ec44d0`) and `_memset` (`00ec61c0`), `cdecl`, three words each.
const MEMCPY: u32 = 0x00ec_44d0;
const MEMSET: u32 = 0x00ec_61c0;

// Translated from 00e783b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word: a stream-copy descriptor; plain `RET`). Copies `count`
/// (`u16` at +8) elements into the destination (+0x2c), stepping it by the destination
/// stride (+0x20), and returns the sum of the element size (+0x1c) over the elements.
///
/// - No source (+0x10 null): each destination element is cleared with `memset(dst, 0,
///   size)`, the size re-read after each one.
/// - With a source: each element copies `n` bytes, `n` being the `u16` at +4 minus 0x18
///   (as a `u16`) when that is at most 4, else 4, then clears the rest of the first four
///   bytes (`memset(dst + n, 0, 4 - n)`) when `n < 4`. Without an index array (+0xc null)
///   the source steps by its stride (+0x18); with one, element `i` is read at
///   `source + index[i] * stride` (`u16` indices).
pub fn fn_00e783b0(e: &mut Engine, descriptor: Ptr) -> u32 {
    let d = descriptor.addr();
    let source = e.mem.u32(d.wrapping_add(0x10));
    let index = e.mem.u32(d.wrapping_add(0xc));
    let mut destination = e.mem.u32(d.wrapping_add(0x2c));
    let mut total = 0u32;
    let count = |e: &Engine| e.mem.u16(d.wrapping_add(8));
    if source == 0 {
        let mut i = 0u16;
        while i < count(e) {
            let size = e.mem.u32(d.wrapping_add(0x1c));
            e.call(MEMSET, &args![destination, 0u32, size]);
            let size = e.mem.u32(d.wrapping_add(0x1c));
            destination = destination.wrapping_add(e.mem.u32(d.wrapping_add(0x20)));
            total = total.wrapping_add(size);
            i = i.wrapping_add(1);
        }
        return total;
    }
    let kind = e.mem.u16(d.wrapping_add(4)).wrapping_sub(0x18);
    let n: u16 = if kind > 4 { 4 } else { kind };
    let pad = |e: &mut Engine, destination: u32| {
        if n < 4 {
            e.call(
                MEMSET,
                &args![
                    destination.wrapping_add(u32::from(n)),
                    0u32,
                    u32::from(4 - n)
                ],
            );
        }
    };
    let mut i = 0u16;
    if index != 0 {
        while i < count(e) {
            let entry = u32::from(e.mem.u16(index.wrapping_add(u32::from(i) * 2)));
            let offset = entry.wrapping_mul(e.mem.u32(d.wrapping_add(0x18)));
            e.call(
                MEMCPY,
                &args![destination, source.wrapping_add(offset), u32::from(n)],
            );
            pad(e, destination);
            total = total.wrapping_add(e.mem.u32(d.wrapping_add(0x1c)));
            destination = destination.wrapping_add(e.mem.u32(d.wrapping_add(0x20)));
            i = i.wrapping_add(1);
        }
    } else {
        let mut source = source;
        while i < count(e) {
            e.call(MEMCPY, &args![destination, source, u32::from(n)]);
            pad(e, destination);
            let size = e.mem.u32(d.wrapping_add(0x1c));
            source = source.wrapping_add(e.mem.u32(d.wrapping_add(0x18)));
            destination = destination.wrapping_add(e.mem.u32(d.wrapping_add(0x20)));
            total = total.wrapping_add(size);
            i = i.wrapping_add(1);
        }
    }
    total
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e7_83b0, fn_00e783b0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, a| a[0].into_ret());
        e.register(MEMSET, |_, a| a[0].into_ret());
        let d = e.mem.alloc(0x30);
        e.mem.set_u16(d + 8, 2);
        e.mem.set_u32(d + 0x1c, 6);
        e.mem.set_u32(d + 0x20, 0x10);
        e.mem.set_u32(d + 0x18, 0x20);
        e.mem.set_u32(d + 0x2c, 0x5000);
        e.call_log = Some(vec![]);
        (e, d)
    }

    #[test]
    fn without_a_source_clears_each_element() {
        let (mut e, d) = rig();
        assert_eq!(e.call(0x00e7_83b0, &args![d]).u32(), 12);
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![vec![0x5000, 0, 6], vec![0x5010, 0, 6]]
        );
        e.mem.set_u16(d + 8, 0);
        assert_eq!(e.call(0x00e7_83b0, &args![d]).u32(), 0);
    }

    #[test]
    fn with_a_source_copies_and_pads_to_four_bytes() {
        let (mut e, d) = rig();
        e.mem.set_u32(d + 0x10, 0x8000);
        e.mem.set_u16(d + 4, 0x1a);
        assert_eq!(e.call(0x00e7_83b0, &args![d]).u32(), 12);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![0x5000, 0x8000, 2], vec![0x5010, 0x8020, 2]]
        );
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![vec![0x5002, 0, 2], vec![0x5012, 0, 2]]
        );
    }

    #[test]
    fn large_kinds_copy_four_bytes_without_padding() {
        let (mut e, d) = rig();
        e.mem.set_u32(d + 0x10, 0x8000);
        // 0x10 - 0x18 wraps to a large `u16`: four bytes.
        e.mem.set_u16(d + 4, 0x10);
        e.call(0x00e7_83b0, &args![d]);
        assert_eq!(calls_to(&e, MEMCPY)[0], vec![0x5000, 0x8000, 4]);
        assert!(calls_to(&e, MEMSET).is_empty());
    }

    #[test]
    fn with_an_index_array_reads_the_indexed_elements() {
        let (mut e, d) = rig();
        e.mem.set_u32(d + 0x10, 0x8000);
        e.mem.set_u16(d + 4, 0x1c);
        let index = e.mem.alloc(4);
        e.mem.set_u16(index, 3);
        e.mem.set_u16(index + 2, 1);
        e.mem.set_u32(d + 0xc, index);
        assert_eq!(e.call(0x00e7_83b0, &args![d]).u32(), 12);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![0x5000, 0x8060, 4], vec![0x5010, 0x8020, 4]]
        );
    }
}
