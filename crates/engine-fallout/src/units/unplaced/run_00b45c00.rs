//! `(unplaced)/run_00b45c00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// CRT `malloc` (`00ecd1c7`), `free` (`00ecd291`) and `memcpy` (`00ec44d0`),
/// all `cdecl`.
const MALLOC: u32 = 0x00ec_d1c7;
const FREE: u32 = 0x00ec_d291;
const MEMCPY: u32 = 0x00ec_44d0;

// Translated from 00b49020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the default allocator `00b43e70` installs
/// (`cdecl`, three words: an opaque word, a count and a size). With a
/// non-zero opaque word the count goes through `size - size + count`; then
/// `malloc(count * size)` is answered.
pub fn fn_00b49020(e: &mut Engine, opaque: u32, count: u32, size: u32) -> u32 {
    let mut count = count;
    if opaque != 0 {
        count = size.wrapping_sub(size).wrapping_add(count);
    }
    e.call(MALLOC, &args![count.wrapping_mul(size)]).u32()
}

// Translated from 00b49050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the default free (`cdecl`, two words: an
/// opaque word, not read, and the block): `free(block)`.
pub fn fn_00b49050(e: &mut Engine, _unused_opaque: u32, block: u32) {
    e.call(FREE, &args![block]);
}

// Translated from 00b45db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word: the stream record): takes
/// the stream down. Without a stream, a state (`+0x1c`) or a free function
/// (`+0x24`) answers -2. Frees the state's window (`+0x2c`) when there is
/// one, then the state when it was made by the set-up (`+0x34`), both
/// through the free function with the opaque word (`+0x28`); clears `+0x1c`
/// and answers 0.
pub fn fn_00b45db0(e: &mut Engine, stream: u32) -> i32 {
    if stream == 0 || e.mem.u32(stream + 0x1c) == 0 || e.mem.u32(stream + 0x24) == 0 {
        return -2;
    }
    let state = e.mem.u32(stream + 0x1c);
    let window = e.mem.u32(state + 0x2c);
    if window != 0 {
        let free = e.mem.u32(stream + 0x24);
        let opaque = e.mem.u32(stream + 0x28);
        e.call(free, &args![opaque, window]);
    }
    if e.mem.u32(stream + 0x34) != 0 {
        let free = e.mem.u32(stream + 0x24);
        let opaque = e.mem.u32(stream + 0x28);
        let state = e.mem.u32(stream + 0x1c);
        e.call(free, &args![opaque, state]);
    }
    e.mem.set_u32(stream + 0x1c, 0);
    0
}

// Translated from 00b45c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words: the stream record and the
/// output position): keeps the last output in the state's window. The window
/// (`+0x2c`, `1 << bits` bytes from the allocator with the opaque word) is
/// made on first use (none answers 1); its size (`+0x20`) set with the write
/// position (`+0x28`) and fill (`+0x24`) cleared. The new output is the
/// position minus the stream's `+0x10`: at least a window's worth copies the
/// window's size from the output's end, write position 0, fill the size;
/// less is copied at the write position up to the window's end and the rest
/// wraps to its start (then the write position is the rest and the fill the
/// size), or the write position moves on (wrapping at the size) and the fill
/// grows while below the size. Copies go through `memcpy`; answers 0.
pub fn fn_00b45c00(e: &mut Engine, stream: u32, position: u32) -> i32 {
    let state = e.mem.u32(stream + 0x1c);
    if e.mem.u32(state + 0x2c) == 0 {
        let allocate = e.mem.u32(stream + 0x20);
        let opaque = e.mem.u32(stream + 0x28);
        let size = 1u32.wrapping_shl(e.mem.u32(state + 0x1c));
        let window = e.call(allocate, &args![opaque, size, 1u32]).u32();
        e.mem.set_u32(state + 0x2c, window);
        if e.mem.u32(state + 0x2c) == 0 {
            return 1;
        }
    }
    if e.mem.u32(state + 0x20) == 0 {
        let size = 1u32.wrapping_shl(e.mem.u32(state + 0x1c));
        e.mem.set_u32(state + 0x20, size);
        e.mem.set_u32(state + 0x28, 0);
        e.mem.set_u32(state + 0x24, 0);
    }
    let copy = position.wrapping_sub(e.mem.u32(stream + 0x10));
    let size = e.mem.u32(state + 0x20);
    let window = e.mem.u32(state + 0x2c);
    let end = e.mem.u32(stream + 0xc);
    if copy >= size {
        e.call(MEMCPY, &args![window, end.wrapping_sub(size), size]);
        e.mem.set_u32(state + 0x28, 0);
        let size = e.mem.u32(state + 0x20);
        e.mem.set_u32(state + 0x24, size);
        return 0;
    }
    let write = e.mem.u32(state + 0x28);
    let mut part = size.wrapping_sub(write);
    if part > copy {
        part = copy;
    }
    e.call(
        MEMCPY,
        &args![window.wrapping_add(write), end.wrapping_sub(copy), part],
    );
    let rest = copy.wrapping_sub(part);
    if rest != 0 {
        let window = e.mem.u32(state + 0x2c);
        let end = e.mem.u32(stream + 0xc);
        e.call(MEMCPY, &args![window, end.wrapping_sub(rest), rest]);
        e.mem.set_u32(state + 0x28, rest);
        let size = e.mem.u32(state + 0x20);
        e.mem.set_u32(state + 0x24, size);
    } else {
        let write = e.mem.u32(state + 0x28).wrapping_add(part);
        e.mem.set_u32(state + 0x28, write);
        if e.mem.u32(state + 0x28) == e.mem.u32(state + 0x20) {
            e.mem.set_u32(state + 0x28, 0);
        }
        if e.mem.u32(state + 0x24) < e.mem.u32(state + 0x20) {
            let fill = e.mem.u32(state + 0x24).wrapping_add(part);
            e.mem.set_u32(state + 0x24, fill);
        }
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b4_9020, fn_00b49020(u32, u32, u32) -> u32),
        entry!(0x00b4_9050, fn_00b49050(u32, u32)),
        entry!(0x00b4_5db0, fn_00b45db0(u32) -> i32),
        entry!(0x00b4_5c00, fn_00b45c00(u32, u32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_allocator_and_free() {
        let mut e = Engine::new();
        e.register(MALLOC, |_, a| (a[0] + 0x1000).into_ret());
        e.register(FREE, |_, _| Ret::default());
        assert_eq!(e.call(0x00b4_9020, &args![0u32, 3u32, 4u32]).u32(), 0x100c);
        assert_eq!(e.call(0x00b4_9020, &args![1u32, 3u32, 4u32]).u32(), 0x100c);
        e.call_log = Some(vec![]);
        e.call(0x00b4_9050, &args![9u32, 0x500u32]);
        assert_eq!(e.call_log.take().unwrap()[1], (FREE, vec![0x500]));
    }

    const ALLOCATE: u32 = 0x013b_0000;
    const RELEASE: u32 = 0x013b_0010;

    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1] * a[2]).into_ret());
        e.register(RELEASE, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            a[0].into_ret()
        });
        let stream = e.mem.alloc(0x38);
        let state = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 0x1c, state);
        e.mem.set_u32(stream + 0x20, ALLOCATE);
        e.mem.set_u32(stream + 0x24, RELEASE);
        e.mem.set_u32(state + 0x1c, 3);
        (e, stream, state)
    }

    #[test]
    fn fn_00b45db0_frees_what_was_made() {
        let (mut e, stream, state) = engine();
        e.mem.set_u32(state + 0x2c, 0x7000);
        e.mem.set_u32(stream + 0x34, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_5db0, &args![stream]).i32(), 0);
        let frees: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == RELEASE)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(frees, vec![0x7000, state]);
        assert_eq!(e.mem.u32(stream + 0x1c), 0);
        assert_eq!(e.call(0x00b4_5db0, &args![stream]).i32(), -2);
    }

    #[test]
    fn fn_00b45c00_keeps_the_last_output() {
        let (mut e, stream, state) = engine();
        let output = e.mem.alloc(0x20);
        for i in 0..0x20 {
            e.mem.set_u8(output + i, i as u8);
        }
        // 5 bytes of output into an empty 8-byte window.
        e.mem.set_u32(stream + 0xc, output + 5);
        e.mem.set_u32(stream + 0x10, 0x100);
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0x105u32]).i32(), 0);
        let window = e.mem.u32(state + 0x2c);
        assert_eq!(e.mem.u32(state + 0x20), 8);
        assert_eq!((e.mem.u32(state + 0x28), e.mem.u32(state + 0x24)), (5, 5));
        assert_eq!(e.mem.u8(window + 4), 4);
        // 6 more: 3 fill the window, 3 wrap to its start.
        e.mem.set_u32(stream + 0xc, output + 11);
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0x106u32]).i32(), 0);
        assert_eq!((e.mem.u32(state + 0x28), e.mem.u32(state + 0x24)), (3, 8));
        assert_eq!(e.mem.u8(window + 7), 7);
        assert_eq!(e.mem.u8(window), 8);
        // A window's worth or more: the last 8 bytes, from the start.
        e.mem.set_u32(stream + 0xc, output + 0x20);
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0x120u32]).i32(), 0);
        assert_eq!(e.mem.u8(window), 0x18);
        assert_eq!((e.mem.u32(state + 0x28), e.mem.u32(state + 0x24)), (0, 8));
    }

    #[test]
    fn fn_00b45c00_fails_without_a_window() {
        let (mut e, stream, _) = engine();
        e.register(ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0u32]).i32(), 1);
    }
}
