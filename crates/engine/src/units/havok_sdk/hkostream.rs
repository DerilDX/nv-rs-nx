//! `Havok SDK/hkostream.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cc07f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cc_0780`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cc07f0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cc_0780, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkString::strLen` (`00cb10d0`, cdecl, one argument).
const STRING_LENGTH: u32 = 0x00cb_10d0;
/// `"(null)"`.
const NULL_TEXT: u32 = 0x010c_9308;

// Translated from 00cc0080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a C string to a stream through the stream's vtable slot 0x10 (`write(buffer,
/// length)`), or `"(null)"` (6 bytes) when `text` is null. The exe takes the stream in EDI and
/// the text in ESI, so the parameters are in that order; the result is whatever the write
/// call left in EAX. The identity of the stream class is unconfirmed.
pub fn fn_00cc0080(e: &mut Engine, stream: Ptr, text: Ptr) -> u32 {
    if text.addr() != 0 {
        let length = e.call(STRING_LENGTH, &args![text]).u32();
        e.vcall(stream.addr(), 0x10, &args![text, length]).u32()
    } else {
        e.vcall(stream.addr(), 0x10, &args![NULL_TEXT, 6u32]).u32()
    }
}

// Translated from 00cc01c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `text` through the writer held at `this + 8` (`00cc0080`) and returns `this`. The
/// class is unconfirmed (an `hkOstream` has its `hkStreamWriter*` at +8).
pub fn fn_00cc01c0(e: &mut Engine, this: Ptr, text: Ptr) -> Ptr {
    let writer = e.mem.u32(this.addr() + 8);
    fn_00cc0080(e, Ptr::new(writer), text);
    this
}

// Translated from 00cc04c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_8560.
pub fn hk_ostream_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_8560
}

// Translated from 00cc0770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArrayStreamWriter::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_8ad8.
pub fn hk_array_stream_writer_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_8ad8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc_0080, fn_00cc0080(Ptr, Ptr) -> u32),
        entry!(0x00cc_01c0, fn_00cc01c0(Ptr, Ptr) -> Ptr),
        entry!(0x00cc_07f0, deleting_destructor_00cc07f0(Ptr, u8) -> Ptr),
        entry!(0x00cc04c0, hk_ostream_get_class_type() -> u32),
        entry!(0x00cc0770, hk_array_stream_writer_get_class_type() -> u32),
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

    type WriteLog = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// A stream whose vtable slot 0x10 is a double logging `(stream, buffer, length)`, and a
    /// `strLen` double answering 5.
    fn stream_rig(e: &mut Engine) -> (u32, WriteLog) {
        const WRITE: u32 = 0x0900_0022;
        let log: WriteLog = Rc::new(RefCell::new(Vec::new()));
        let write_log = Rc::clone(&log);
        e.register_double(WRITE, move |_, a| {
            write_log.borrow_mut().push((a[0], a[1], a[2]));
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        e.register(STRING_LENGTH, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, WRITE);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream, vtable);
        (stream, log)
    }

    #[test]
    fn fn_00cc0080_writes_the_text_or_the_null_marker() {
        let mut e = engine();
        let (stream, log) = stream_rig(&mut e);
        let text = e.mem.alloc(8);
        assert_eq!(e.call(0x00cc_0080, &args![stream, text]).u32(), 0x77);
        assert_eq!(e.call(0x00cc_0080, &args![stream, 0u32]).u32(), 0x77);
        assert_eq!(
            *log.borrow(),
            vec![(stream, text, 5), (stream, NULL_TEXT, 6)]
        );
    }

    #[test]
    fn fn_00cc01c0_writes_through_the_writer_and_returns_this() {
        let mut e = engine();
        let (stream, log) = stream_rig(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, stream);
        let text = e.mem.alloc(8);
        assert_eq!(e.call(0x00cc_01c0, &args![this, text]).u32(), this);
        assert_eq!(*log.borrow(), vec![(stream, text, 5)]);
    }

    #[test]
    fn deleting_destructor_00cc07f0_frees_only_when_bit_zero_is_set() {
        check(0x00cc_07f0, 0x00cc_0780);
    }

    #[test]
    fn hk_ostream_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_04c0, &args![]).u32(), 0x0126_8560);
    }

    #[test]
    fn hk_array_stream_writer_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_0770, &args![]).u32(), 0x0126_8ad8);
    }
}
