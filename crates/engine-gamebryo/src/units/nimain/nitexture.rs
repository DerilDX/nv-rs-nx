//! `NiMain/nitexture.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function pointer `00a62a10` calls through (`011a9600`; set to
/// `00a62690` on first use when null).
const BUILDER_POINTER: u32 = 0x011a_9600;
/// `00a62690` itself, the pointer's default.
const DEFAULT_BUILDER: u32 = 0x00a6_2690;
/// The ten cached objects (`011f4514`, one word each, indexed by the low
/// 31 bits of the request).
const CACHE: u32 = 0x011f_4514;
/// `00a62330` (cdecl: two four-`float` colours, the word table at
/// `011f4600`, a count; returns the new object) and the cache slot's
/// assignment (`00a5f860`, thiscall on the slot, the object).
const BUILD_00A62330: u32 = 0x00a6_2330;
const BUILD_TABLE: u32 = 0x011f_4600;
const SLOT_ASSIGN_00A5F860: u32 = 0x00a5_f860;

/// `1.0` and `0.0` as the code stores them (`FLD1`/`FLDZ`).
const ONE: u32 = 0x3f80_0000;
const ZERO: u32 = 0;

/// The two colours (the second and first arguments of `00a62330`, four
/// `float`s each) and the count for request `request`: the code's jump
/// table at `00a629ec` (requests `0x80000001` to `0x80000009`), any other
/// request taking the table's default block.
fn colours(request: u32) -> ([u32; 4], [u32; 4], u32) {
    match request {
        0x8000_0001 => ([ONE, ONE, ZERO, ONE], [ZERO, ZERO, ONE, ONE], 1),
        0x8000_0002 => ([ONE, ZERO, ZERO, ONE], [ZERO, ONE, ZERO, ONE], 6),
        0x8000_0003 => ([ONE, ONE, ZERO, ONE], [ZERO, ONE, ONE, ONE], 1),
        0x8000_0004 => ([ONE, ONE, ZERO, ONE], [ONE, ZERO, ZERO, ONE], 1),
        0x8000_0006 => ([ZERO, ONE, ONE, ONE], [ZERO, ONE, ONE, ONE], 1),
        0x8000_0007 => ([ONE, ONE, ZERO, ONE], [ZERO, ONE, ZERO, ONE], 6),
        0x8000_0008 => ([ONE, ZERO, ONE, ONE], [ZERO, ONE, ZERO, ONE], 6),
        0x8000_0009 => ([ZERO, ONE, ONE, ONE], [ZERO, ONE, ZERO, ONE], 6),
        // 0x80000005 and every request outside the table.
        _ => ([ONE, ZERO, ONE, ONE], [ONE, ZERO, ONE, ONE], 1),
    }
}

// Translated from 00a62690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cached object for `request` (cdecl, two words, the first unused;
/// unnamed in the engine map): 0 when the low 31 bits of the request are
/// 10 or more; otherwise the cache slot `011f4514[request & 0x7fffffff]`,
/// filled on first use with `00a62330(second colour, first colour,
/// 011f4600, count)` (the colours and count of the request, see
/// [`colours`]) through `00a5f860`. Returns the slot's object.
pub fn fn_00a62690(e: &mut Engine, _unused_1: u32, request: u32) -> u32 {
    let index = request & 0x7fff_ffff;
    if index >= 10 {
        return 0;
    }
    let slot = CACHE + index * 4;
    if e.mem.u32(slot) == 0 {
        let (first, second, count) = colours(request);
        e.with_stack(0x20, |e, frame| {
            let first_colour = frame.addr();
            let second_colour = frame.addr() + 0x10;
            for i in 0..4 {
                e.mem.set_u32(first_colour + i * 4, first[i as usize]);
                e.mem.set_u32(second_colour + i * 4, second[i as usize]);
            }
            let object = e
                .call(
                    BUILD_00A62330,
                    &args![second_colour, first_colour, BUILD_TABLE, count],
                )
                .u32();
            e.call(SLOT_ASSIGN_00A5F860, &args![slot, object]);
        });
    }
    e.mem.u32(slot)
}

// Translated from 00a62a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives 0 for a non-negative `request` (cdecl, two words; unnamed in the
/// engine map); otherwise calls the function at `011a9600` (set to
/// `00a62690` first when null) with both words and returns its result.
pub fn fn_00a62a10(e: &mut Engine, first: u32, request: u32) -> u32 {
    if (request as i32) >= 0 {
        return 0;
    }
    let mut builder = e.global::<u32>(BUILDER_POINTER);
    if builder == 0 {
        builder = DEFAULT_BUILDER;
        e.set_global(BUILDER_POINTER, builder);
    }
    e.call(builder, &args![first, request]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_2690, fn_00a62690(u32, u32) -> u32),
        entry!(0x00a6_2a10, fn_00a62a10(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Where the build double records the colours it is given.
    const SEEN: u32 = 0x0343_0000;

    fn setup(e: &mut Engine) {
        e.map(CACHE, 0x28);
        e.map(BUILDER_POINTER, 4);
        e.map(SEEN, 0x40);
        e.register(BUILD_00A62330, |e, a| {
            assert_eq!(a[2], BUILD_TABLE);
            let first = e.mem.bytes(a[0], 16);
            let second = e.mem.bytes(a[1], 16);
            e.mem.write(SEEN, &first);
            e.mem.write(SEEN + 0x10, &second);
            e.mem.set_u32(SEEN + 0x20, a[3]);
            0x0abc_0000u32.into_ret()
        });
        e.register(SLOT_ASSIGN_00A5F860, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
    }

    fn words(e: &Engine, address: u32) -> Vec<u32> {
        (0..4).map(|i| e.mem.u32(address + i * 4)).collect()
    }

    #[test]
    fn builds_a_cache_slot_once_with_the_requests_colours() {
        let mut e = Engine::new();
        setup(&mut e);
        assert_eq!(
            e.call(0x00a6_2690, &args![0u32, 0x8000_0002u32]).u32(),
            0x0abc_0000
        );
        assert_eq!(e.mem.u32(CACHE + 8), 0x0abc_0000);
        assert_eq!(words(&e, SEEN), vec![ZERO, ONE, ZERO, ONE]);
        assert_eq!(words(&e, SEEN + 0x10), vec![ONE, ZERO, ZERO, ONE]);
        assert_eq!(e.mem.u32(SEEN + 0x20), 6);
        // Cached: no second build.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a6_2690, &args![0u32, 0x8000_0002u32]).u32(),
            0x0abc_0000
        );
        assert!(!e
            .call_log
            .iter()
            .flatten()
            .any(|(a, _)| *a == BUILD_00A62330));
        // A request outside the table takes the default colours.
        e.call(0x00a6_2690, &args![0u32, 0x8000_0000u32]);
        assert_eq!(words(&e, SEEN), vec![ONE, ZERO, ONE, ONE]);
        assert_eq!(e.mem.u32(SEEN + 0x20), 1);
        // Out of range.
        assert_eq!(e.call(0x00a6_2690, &args![0u32, 0x8000_000au32]).u32(), 0);
    }

    #[test]
    fn negative_requests_go_through_the_pointer_set_on_first_use() {
        let mut e = Engine::new();
        setup(&mut e);
        assert_eq!(e.call(0x00a6_2a10, &args![0u32, 5u32]).u32(), 0);
        assert_eq!(e.mem.u32(BUILDER_POINTER), 0);
        assert_eq!(
            e.call(0x00a6_2a10, &args![7u32, 0x8000_0009u32]).u32(),
            0x0abc_0000
        );
        assert_eq!(e.mem.u32(BUILDER_POINTER), DEFAULT_BUILDER);
        // A pointer already set is used as it is.
        e.register(0x0055_0000, |_, a| (a[0] + a[1]).into_ret());
        e.mem.set_u32(BUILDER_POINTER, 0x0055_0000);
        assert_eq!(
            e.call(0x00a6_2a10, &args![1u32, 0x8000_0001u32]).u32(),
            0x8000_0002
        );
    }
}
