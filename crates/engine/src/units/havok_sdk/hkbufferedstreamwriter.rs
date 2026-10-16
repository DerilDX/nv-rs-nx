//! `Havok SDK/hkbufferedstreamwriter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d27570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d2_7520`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d27570(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d2_7520, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `memset` of the CRT behind the jump at `00cb1280` (cdecl `(destination, value, bytes)`).
const MEM_SET: u32 = 0x00cb_1280;

// Translated from 00d274c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::hkBufferedStreamWriter_ov2` (Xbox PDB name from the exe's symbol
/// table; the vtable `010ce448` it stores has the RTTI name `hkBufferedStreamWriter`).
/// `this` is ECX; `buffer` and `size` describe the byte buffer; `clear` is the low byte of
/// the third stack word (the upper bytes are not read). Sets the 16-bit word at +6 to 1, the
/// vtable, the buffer at +0xc, zeros at +8 and +0x10 and the byte at +0x18, and the limit at
/// +0x14 to `size - 1` when `clear` is non-zero (the buffer is zeroed with `memset`) or `size`
/// when it is zero. Returns `this`.
pub fn hk_buffered_stream_writer_hk_buffered_stream_writer_ov2(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
    clear: u8,
) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t, 0x010c_e448);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0xc, buffer);
    e.mem.set_u32(t + 0x10, 0);
    let limit = if clear != 0 {
        size.wrapping_sub(1)
    } else {
        size
    };
    e.mem.set_u32(t + 0x14, limit);
    e.mem.set_u8(t + 0x18, 0);
    if clear != 0 {
        e.call(MEM_SET, &args![buffer, 0u32, size]);
    }
    this
}

// Translated from 00d27560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_89e4.
pub fn hk_buffered_stream_writer_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_89e4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_7570, deleting_destructor_00d27570(Ptr, u8) -> Ptr),
        entry!(0x00d2_74c0, hk_buffered_stream_writer_hk_buffered_stream_writer_ov2(Ptr, u32, u32, u8) -> Ptr),
        entry!(0x00d27560, hk_buffered_stream_writer_get_class_type() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d27570_frees_only_when_bit_zero_is_set() {
        check(0x00d2_7570, 0x00d2_7520);
    }

    #[test]
    fn hk_buffered_stream_writer_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d2_7560, &args![]).u32(), 0x0126_89e4);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    #[test]
    fn construct_sets_the_fields_and_clears_the_buffer_when_asked() {
        let mut e = test_engine();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(MEM_SET, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.mem.set_u8(this + 0x18, 9);
        let got = e.call(0x00d2_74c0, &args![this, 0x4000u32, 0x100u32, 1u32]);
        assert_eq!(got.u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010c_e448);
        assert_eq!(e.mem.u32(this + 0xc), 0x4000);
        assert_eq!(e.mem.u32(this + 0x14), 0xff);
        assert_eq!(e.mem.u8(this + 0x18), 0);
        assert_eq!(*log.borrow(), vec![vec![0x4000, 0, 0x100]]);
        // Without the flag the limit is the whole size and nothing is cleared; only the low
        // byte of the word counts.
        log.borrow_mut().clear();
        e.call(
            0x00d2_74c0,
            &args![this, 0x4000u32, 0x100u32, 0x1234_5600u32],
        );
        assert_eq!(e.mem.u32(this + 0x14), 0x100);
        assert!(log.borrow().is_empty());
    }
}
