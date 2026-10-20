//! `BSSystem/run_00b01510` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSWin32FileCache`'s vtable (its RTTI name).
const WIN32_FILE_CACHE_VTABLE: u32 = 0x010a_483c;
/// `memcpy` (`00ec44d0`, cdecl).
const MEMCPY: u32 = 0x00ec_44d0;
/// The direct read of a file object (`00afb7a0`, `thiscall`, three words:
/// destination, size, and a third the code passes on).
const READ_DIRECT: u32 = 0x00af_b7a0;
/// The read buffer of the file object: its capacity (`+0x10`), the bytes
/// in it (`+0x14`), the read position in it (`+0x18`) and its storage
/// (`+0x20`).
const BUFFER_CAPACITY: u32 = 0x10;
const BUFFER_FILLED: u32 = 0x14;
const BUFFER_POSITION: u32 = 0x18;
const BUFFER_STORAGE: u32 = 0x20;

fn field(e: &Engine, this: Ptr, offset: u32) -> u32 {
    e.mem.u32(this.addr().wrapping_add(offset))
}

fn set_field(e: &mut Engine, this: Ptr, offset: u32, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(offset), value);
}

// Translated from 00b01800 (decompiled, FalloutNV.exe 1.4.0.525)
/// A buffered read of `size` bytes into `destination`: served from the
/// buffer when it holds enough. Otherwise the rest of the buffer is copied
/// first, the buffer emptied, and either the remainder read directly
/// (`00afb7a0`; more than the capacity) or the buffer refilled with up to
/// its capacity and the remainder (limited to what came in) copied from
/// it. Returns the bytes delivered.
pub fn fn_00b01800(e: &mut Engine, this: Ptr, destination: u32, size: u32) -> u32 {
    let mut destination = destination;
    let mut size = size;
    let mut done = 0u32;
    let available = field(e, this, BUFFER_FILLED).wrapping_sub(field(e, this, BUFFER_POSITION));
    if size > available {
        if available > 0 {
            let source =
                field(e, this, BUFFER_STORAGE).wrapping_add(field(e, this, BUFFER_POSITION));
            e.call(MEMCPY, &args![destination, source, available]);
            destination = destination.wrapping_add(available);
            size = size.wrapping_sub(available);
            done = available;
        }
        set_field(e, this, BUFFER_FILLED, 0);
        set_field(e, this, BUFFER_POSITION, 0);
        if size > field(e, this, BUFFER_CAPACITY) {
            let read = e
                .call(READ_DIRECT, &args![this, destination, size, done])
                .u32();
            return read.wrapping_add(done);
        }
        let storage = field(e, this, BUFFER_STORAGE);
        let capacity = field(e, this, BUFFER_CAPACITY);
        let filled = e
            .call(READ_DIRECT, &args![this, storage, capacity, done])
            .u32();
        set_field(e, this, BUFFER_FILLED, filled);
        if field(e, this, BUFFER_FILLED) < size {
            size = field(e, this, BUFFER_FILLED);
        }
    }
    let source = field(e, this, BUFFER_STORAGE).wrapping_add(field(e, this, BUFFER_POSITION));
    e.call(MEMCPY, &args![destination, source, size]);
    let position = field(e, this, BUFFER_POSITION).wrapping_add(size);
    set_field(e, this, BUFFER_POSITION, position);
    done.wrapping_add(size)
}

// Translated from 00b01bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1 when bit 1 of `flags` is clear, else 0 (the second word and `this`
/// are not read).
pub fn fn_00b01bc0(_e: &mut Engine, _this: Ptr, flags: u32, _unused_2: u32) -> u32 {
    u32::from(flags & 2 == 0)
}

// Translated from 00b01be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1 when bit 2 of `flags` is clear, else 0 (the second word and `this`
/// are not read).
pub fn fn_00b01be0(_e: &mut Engine, _this: Ptr, flags: u32, _unused_2: u32) -> u32 {
    u32::from(flags & 4 == 0)
}

// Translated from 00b01c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache`'s constructor: the `BSFileCache` base (`00b00f20`),
/// then its vtable. Returns `this`.
pub fn fn_00b01c00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x00b0_0f20, &args![this]);
    e.mem.set_u32(this.addr(), WIN32_FILE_CACHE_VTABLE);
    this
}

// Translated from 00b01c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWin32FileCache`'s destructor: its vtable, then the `BSFileCache`
/// destructor body (`00b01020`).
pub fn fn_00b01c20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), WIN32_FILE_CACHE_VTABLE);
    e.call(0x00b0_1020, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_1800, fn_00b01800(Ptr, u32, u32) -> u32),
        entry!(0x00b0_1bc0, fn_00b01bc0(Ptr, u32, u32) -> u32),
        entry!(0x00b0_1be0, fn_00b01be0(Ptr, u32, u32) -> u32),
        entry!(0x00b0_1c00, fn_00b01c00(Ptr) -> Ptr),
        entry!(0x00b0_1c20, fn_00b01c20(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A file object with a 16-byte buffer holding `filled`
    /// bytes read up to `position`.
    fn file(e: &mut Engine, filled: u32, position: u32) -> (u32, u32) {
        let this = e.mem.alloc(0x40);
        let storage = e.mem.alloc(0x10);
        e.mem.set_u32(this + BUFFER_CAPACITY, 0x10);
        e.mem.set_u32(this + BUFFER_FILLED, filled);
        e.mem.set_u32(this + BUFFER_POSITION, position);
        e.mem.set_u32(this + BUFFER_STORAGE, storage);
        (this, storage)
    }

    #[test]
    fn reads_from_the_buffer_when_it_holds_enough() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        let (this, storage) = file(&mut e, 0x10, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1800, &args![this, 0x9000u32, 8u32]).u32(), 8);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, storage + 4, 8]]);
        assert_eq!(e.mem.u32(this + BUFFER_POSITION), 12);
    }

    #[test]
    fn large_reads_go_direct_and_small_ones_refill() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(READ_DIRECT, |_, a| (a[2] / 2).into_ret());
        let (this, storage) = file(&mut e, 0x10, 0xc);
        e.call_log = Some(vec![]);
        // 4 buffered bytes, then 0x20 more read directly (half arrive).
        assert_eq!(
            e.call(0x00b0_1800, &args![this, 0x9000u32, 0x24u32]).u32(),
            4 + 0x10
        );
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, storage + 0xc, 4]]);
        assert_eq!(calls_to(&e, READ_DIRECT), vec![vec![this, 0x9004, 0x20, 4]]);
        assert_eq!(e.mem.u32(this + BUFFER_FILLED), 0);
        // Empty buffer, 12 wanted: a refill brings 8, so 8 are delivered.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1800, &args![this, 0x9000u32, 12u32]).u32(), 8);
        assert_eq!(
            calls_to(&e, READ_DIRECT),
            vec![vec![this, storage, 0x10, 0]]
        );
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x9000, storage, 8]]);
        assert_eq!(e.mem.u32(this + BUFFER_FILLED), 8);
        assert_eq!(e.mem.u32(this + BUFFER_POSITION), 8);
    }

    #[test]
    fn flag_tests() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b0_1bc0, &args![0u32, 2u32, 9u32]).u32(), 0);
        assert_eq!(e.call(0x00b0_1bc0, &args![0u32, 4u32, 9u32]).u32(), 1);
        assert_eq!(e.call(0x00b0_1be0, &args![0u32, 4u32, 9u32]).u32(), 0);
        assert_eq!(e.call(0x00b0_1be0, &args![0u32, 2u32, 9u32]).u32(), 1);
    }

    #[test]
    fn win32_file_cache_wraps_the_base() {
        let mut e = Engine::new();
        e.register(0x00b0_0f20, |_, a| a[0].into_ret());
        e.register(0x00b0_1020, |_, _| Ret::default());
        let cache = e.mem.alloc(0x64);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_1c00, &args![cache]).u32(), cache);
        assert_eq!(e.mem.u32(cache), WIN32_FILE_CACHE_VTABLE);
        e.mem.set_u32(cache, 0);
        e.call(0x00b0_1c20, &args![cache]);
        assert_eq!(e.mem.u32(cache), WIN32_FILE_CACHE_VTABLE);
        assert_eq!(calls_to(&e, 0x00b0_0f20), vec![vec![cache]]);
        assert_eq!(calls_to(&e, 0x00b0_1020), vec![vec![cache]]);
    }
}
