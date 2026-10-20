//! `fallout shared/pathfinding/run_006b5560` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006b5a90` calls on the member (`0044ddc0`).
const CALLEE_0044DDC0: u32 = 0x0044_ddc0;

// Translated from 006b5a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0044ddc0` with `ECX` = `this + 0x58` and passes on its result.
pub fn fn_006b5a90(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0044DDC0, &args![this.byte_add(0x58)]).u32()
}

// Translated from 006b5ab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b5ab0 - the exception-unwinding frame (FS:[0] chain, handler 00f009ab, state variable) is not translated.
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`): copies this object into
/// `destination`. The words at +0x0 to +0x1c and +0x54 are copied directly; the three
/// arrays at +0x24, +0x34 and +0x44 through `006e5330` (`BSSimpleArray<unsigned int>`'s
/// `CopyTo`: the source array as `this`, the destination array as the argument). For the
/// object at +0x58: when this one has one, the destination gets one if it has none (0x38
/// bytes from `00401000`, constructed by `0069bcf0`, or 0 when the allocation failed) and
/// `0069cc80` copies this one's into it (`ECX` this one's, the argument the
/// destination's); when this one has none, the destination's (if any) is destroyed and
/// freed through `006b4850` with 1 and the pointer cleared.
pub fn fn_006b5ab0(e: &mut Engine, this: Ptr, destination: Ptr) {
    let source = this.addr();
    let target = destination.addr();
    for offset in [0x0u32, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x54] {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(target + offset, word);
    }
    for offset in [0x24u32, 0x34, 0x44] {
        e.call(0x006e_5330, &args![source + offset, target + offset]);
    }
    let own = e.mem.u32(source + 0x58);
    if own != 0 {
        if e.mem.u32(target + 0x58) == 0 {
            let block = e.call(0x0040_1000, &args![0x38u32]).u32();
            let created = if block == 0 {
                0
            } else {
                e.call(0x0069_bcf0, &args![block]).u32()
            };
            e.mem.set_u32(target + 0x58, created);
        }
        let own = e.mem.u32(source + 0x58);
        let theirs = e.mem.u32(target + 0x58);
        e.call(0x0069_cc80, &args![own, theirs]);
    } else {
        let theirs = e.mem.u32(target + 0x58);
        if theirs != 0 {
            e.call(0x006b_4850, &args![theirs, 1u32]);
        }
        e.mem.set_u32(target + 0x58, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_5a90, fn_006b5a90(Ptr) -> u32),
        entry!(0x006b_5ab0, fn_006b5ab0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006b5a90_passes_this_plus_58_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0044DDC0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_5a90, &args![0x4100u32]).u32(), 0x140d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0044DDC0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x58]);
    }

    /// A source and a destination of 0x5c bytes, the source's words numbered, and doubles
    /// for the callees (the allocator gives `block`, the constructor returns its `this`).
    fn copy_case(e: &mut Engine, own: u32, theirs: u32, block: u32) -> (u32, u32) {
        let source = e.mem.alloc(0x5c);
        let target = e.mem.alloc(0x5c);
        for index in 0..0x16u32 {
            e.mem.set_u32(source + 4 * index, 0x100 + index);
        }
        e.mem.set_u32(source + 0x58, own);
        e.mem.set_u32(target + 0x58, theirs);
        for address in [0x006e_5330u32, 0x0069_cc80, 0x006b_4850] {
            e.register(address, |_, _| Ret::default());
        }
        e.register_double(0x0040_1000, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(0x0069_bcf0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        (source, target)
    }

    #[test]
    fn fn_006b5ab0_copies_the_words_and_arrays() {
        let mut e = Engine::new();
        let (source, target) = copy_case(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x006b_5ab0, &args![source, target]);
        for offset in [0x0u32, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x54] {
            assert_eq!(e.mem.u32(target + offset), 0x100 + offset / 4);
        }
        assert_eq!(e.mem.u32(target + 0x20), 0);
        assert_eq!(e.mem.u32(target + 0x24), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x006e_5330, vec![source + 0x24, target + 0x24]),
                (0x006e_5330, vec![source + 0x34, target + 0x34]),
                (0x006e_5330, vec![source + 0x44, target + 0x44]),
            ]
        );
    }

    #[test]
    fn fn_006b5ab0_creates_or_reuses_the_destination_object() {
        // (destination's object, allocation result, the object the copy goes into)
        for (theirs, block, expected) in
            [(0x7000u32, 0u32, 0x7000u32), (0, 0x8000, 0x8000), (0, 0, 0)]
        {
            let mut e = Engine::new();
            let (source, target) = copy_case(&mut e, 0x6000, theirs, block);
            e.call_log = Some(vec![]);
            e.call(0x006b_5ab0, &args![source, target]);
            assert_eq!(e.mem.u32(target + 0x58), expected);
            let log = e.call_log.take().unwrap();
            assert_eq!(log.last().unwrap(), &(0x0069_cc80, vec![0x6000, expected]));
            let allocations: Vec<_> = log.iter().filter(|call| call.0 == 0x0040_1000).collect();
            assert_eq!(allocations.len(), usize::from(theirs == 0));
            if theirs == 0 {
                assert_eq!(allocations[0].1, vec![0x38]);
            }
            assert_eq!(
                log.iter().filter(|call| call.0 == 0x0069_bcf0).count(),
                usize::from(block != 0)
            );
        }
    }

    #[test]
    fn fn_006b5ab0_drops_the_destination_object_when_the_source_has_none() {
        for theirs in [0u32, 0x7000] {
            let mut e = Engine::new();
            let (source, target) = copy_case(&mut e, 0, theirs, 0);
            e.call_log = Some(vec![]);
            e.call(0x006b_5ab0, &args![source, target]);
            assert_eq!(e.mem.u32(target + 0x58), 0);
            let log = e.call_log.take().unwrap();
            let freed: Vec<_> = log.iter().filter(|call| call.0 == 0x006b_4850).collect();
            if theirs == 0 {
                assert!(freed.is_empty());
            } else {
                assert_eq!(freed[0].1, vec![theirs, 1]);
            }
        }
    }
}
