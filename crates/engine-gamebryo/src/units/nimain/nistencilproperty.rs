//! `NiMain/nistencilproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `011f4a14`: the pointer `00a6f450` stores the object it builds in.
const STENCIL_PROPERTY_SLOT: u32 = 0x011f_4a14;

// Translated from 00a6f450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nistencilproperty.cpp`; run by the start-up
/// `00a89cc0`, no arguments): builds a 0x24-byte object (allocation `00aa13e0`, base
/// constructor `00a5c200`, vtable `0101e07c`, the word at `+0x1c` set to 0, the word at
/// `+0x20` to -1, the word at `+0x18` to 0x4180) and stores it, counted, in `011f4a14`,
/// releasing the object held there before. A failed allocation stores null.
pub fn fn_00a6f450(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x24u32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, 0x0101_e07c);
        e.mem.set_u32(block + 0x1c, 0);
        e.mem.set_u32(block + 0x20, 0xffff_ffff);
        e.mem.set_u16(block + 0x18, 0x4180);
        block
    };
    assign_ref_counted(e, STENCIL_PROPERTY_SLOT, object);
}

use crate::units::nimain::nimainsdm::interlocked_decrement;

/// The vtable of `NiStencilProperty` (RTTI `.?AVNiStencilProperty@@`).
const NI_STENCIL_PROPERTY_VTABLE: u32 = 0x0101_e07c;
/// `NiStencilProperty` size (Xbox PDB and the allocations).
const NI_STENCIL_PROPERTY_SIZE: u32 = 0x24;
/// `NiObjectNET::m_spControllers` +0x0c, then `NiStencilProperty`'s
/// `m_uFlags` (16 bits) +0x18, `m_uiRef` +0x1c and `m_uiMask` +0x20 (Xbox
/// PDB names; PC offsets as the code uses them).
const CONTROLLERS: u32 = 0x0c;
const STENCIL_FLAGS: u32 = 0x18;
const STENCIL_REF: u32 = 0x1c;
const STENCIL_MASK: u32 = 0x20;
/// The flags, reference and mask a new property starts with.
const DEFAULT_FLAGS: u16 = 0x4180;
const DEFAULT_MASK: u32 = 0xffff_ffff;

/// `00a5d7a0` (a jump to `00a5c290`; `thiscall`, `(destination,
/// cloning)`): the member copy `CreateClone` runs.
const OBJECT_NET_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `00a5d450` (unit `niobject.cpp`, `thiscall`, `(object, cloning)`): what
/// `CreateClone` runs for a property without controllers, on the property
/// itself.
const OBJECT_SHARE_CLONE: u32 = 0x00a5_d450;
/// `NiProperty::LoadBinary` (`00a5d7f0`).
const PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;

/// `NiStream::m_uiNifFileVersion` +0xd8 and `m_pkIstr` +0x24c (Xbox PDB
/// names; PC offsets), and the input stream's position (+4) and read
/// function (+8, `cdecl` `(stream, buffer, size, &component size, component
/// count)`, returns the bytes read).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
/// The first file version that stores the flags word whole.
const VERSION_WITH_PACKED_FLAGS: u32 = 0x1401_0002;

/// One read through the stream's read function into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes read.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(slot, component);
    let bytes = e.call(read, &args![input, buffer, size, slot, 1u32]).u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// The constructor body the property's `CreateClone` and `CreateObject`
/// run on a new block: the `NiObjectNET` base constructor, the flags, the
/// vtable, reference 0 and mask -1.
fn construct_in(e: &mut Engine, object: u32) {
    e.call(OBJECT_BASE_CONSTRUCTOR, &args![object]);
    e.mem
        .set_u16(object.wrapping_add(STENCIL_FLAGS), DEFAULT_FLAGS);
    e.mem.set_u32(object, NI_STENCIL_PROPERTY_VTABLE);
    e.mem.set_u32(object.wrapping_add(STENCIL_REF), 0);
    e.mem
        .set_u32(object.wrapping_add(STENCIL_MASK), DEFAULT_MASK);
}

// Translated from 00a6f390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::CreateClone` (Xbox PDB). With controllers: a new
/// 0x24-byte property (null when the allocation fails), the member copy
/// (`00a5d7a0`) into it, then the flags, reference and mask copied; returns
/// it. Without controllers the property is its own clone: `00a5d450` runs
/// with the property and `cloning`, and the property is returned.
pub fn ni_stencil_property_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let this = this.addr();
    if e.mem.u32(this.wrapping_add(CONTROLLERS)) == 0 {
        e.call(OBJECT_SHARE_CLONE, &args![this, this, cloning]);
        return Ptr::new(this);
    }
    let block = e.call(ALLOCATE, &args![NI_STENCIL_PROPERTY_SIZE]).u32();
    let object = if block == 0 {
        0
    } else {
        construct_in(e, block);
        block
    };
    e.call(OBJECT_NET_COPY_MEMBERS, &args![this, object, cloning]);
    let flags = e.mem.u16(this.wrapping_add(STENCIL_FLAGS));
    e.mem.set_u16(object.wrapping_add(STENCIL_FLAGS), flags);
    let reference = e.mem.u32(this.wrapping_add(STENCIL_REF));
    e.mem.set_u32(object.wrapping_add(STENCIL_REF), reference);
    let mask = e.mem.u32(this.wrapping_add(STENCIL_MASK));
    e.mem.set_u32(object.wrapping_add(STENCIL_MASK), mask);
    Ptr::new(object)
}

// Translated from 00a6f410 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::CreateObject` (registered through
/// `NiStream::RegisterLoader` for `"NiStencilProperty"` at `00a8a094`;
/// Ghidra has no function here): allocates 0x24 bytes and, when that
/// succeeds, constructs a default property in it (base constructor, flags
/// 0x4180, the vtable, reference 0, mask -1). Returns it, or 0.
pub fn ni_stencil_property_create_object(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![NI_STENCIL_PROPERTY_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    construct_in(e, block);
    Ptr::new(block)
}

// Translated from 00a6f4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nistencilproperty.cpp`): releases the
/// default property `00a6f450` stores in `011f4a14` (`DeleteThis` when the
/// count reaches 0) and clears the pointer; nothing when it is null.
pub fn fn_00a6f4d0(e: &mut Engine) {
    let object = e.mem.u32(STENCIL_PROPERTY_SLOT);
    if object == 0 {
        return;
    }
    if interlocked_decrement(e, object.wrapping_add(4)) == 0 {
        e.vcall(object, 4, &args![]);
    }
    e.mem.set_u32(STENCIL_PROPERTY_SLOT, 0);
}

// Translated from 00a6f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary`,
/// then from file version `0x14010002` on the flags word (2 bytes), the
/// reference and the mask (4 bytes each). Older files store the fields
/// one by one and the flags word is packed from them: an enable byte (bit
/// 0), the function (bits 12-15), the reference and mask, the fail action
/// (bits 1-3), the z-fail action (bits 4-6), the pass action (bits 7-9) and
/// the draw mode (bits 10-11), each a 4-byte value shifted into place and
/// masked only by the 16-bit word.
///
/// The component sizes and buffers are the exe's stack words: the
/// argument's own word and two locals (`L0` at -8 and `L1` at -4 from the
/// return address), as the disassembly uses them.
pub fn ni_stencil_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    let this = this.addr();
    let stream = stream.addr();
    // Frame: L0 +0, L1 +4, the argument word +8.
    e.with_stack(12, |e, frame| {
        let l0 = frame.addr();
        let l1 = l0 + 4;
        let argument = l0 + 8;
        e.mem.set_u32(argument, stream);
        let flags_at = this.wrapping_add(STENCIL_FLAGS);
        if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) >= VERSION_WITH_PACKED_FLAGS {
            stream_read(e, stream, flags_at, 2, 2, argument);
            stream_read(e, stream, this.wrapping_add(STENCIL_REF), 4, 4, argument);
            stream_read(e, stream, this.wrapping_add(STENCIL_MASK), 4, 4, argument);
            return;
        }
        stream_read(e, stream, argument, 1, 1, l0);
        let flags = e.mem.u16(flags_at);
        if e.mem.u8(argument) != 0 {
            e.mem.set_u16(flags_at, flags | 1);
        } else {
            e.mem.set_u16(flags_at, flags & 0xfffe);
        }
        stream_read(e, stream, l1, 4, 4, l0);
        let packed = (e.mem.u16(flags_at) & 0x0fff) | (e.mem.u32(l1) << 12) as u16;
        e.mem.set_u16(flags_at, packed);
        stream_read(e, stream, this.wrapping_add(STENCIL_REF), 4, 4, l1);
        stream_read(e, stream, this.wrapping_add(STENCIL_MASK), 4, 4, l1);
        for (shift, keep) in [(1u32, 0xfff1u16), (4, 0xff8f), (7, 0xfc7f), (10, 0xf3ff)] {
            stream_read(e, stream, l0, 4, 4, l1);
            let value = e.mem.u32(l0).wrapping_shl(shift) as u16;
            let packed = (e.mem.u16(flags_at) & keep) | value;
            e.mem.set_u16(flags_at, packed);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_f450, fn_00a6f450()),
        entry!(0x00a6_f390, ni_stencil_property_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00a6_f410, ni_stencil_property_create_object() -> Ptr),
        entry!(0x00a6_f4d0, fn_00a6f4d0()),
        entry!(0x00a6_f500, ni_stencil_property_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_and_replaces_the_held_one() {
        let run = run_default_instance(0x00a6_f450, STENCIL_PROPERTY_SLOT, |_| {});
        let e = &run.engine;
        assert_eq!(e.mem.u32(run.object), 0x0101_e07c);
        assert_eq!(e.mem.u32(run.object + 0x1c), 0);
        assert_eq!(e.mem.u32(run.object + 0x20), 0xffff_ffff);
        assert_eq!(e.mem.u16(run.object + 0x18), 0x4180);
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a6_f450, STENCIL_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const READ: u32 = 0x00ee_0400;
    const DELETE_THIS: u32 = 0x00ee_0500;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    #[test]
    fn create_object_builds_a_default_property() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x24);
        logged(&mut e, &log, ALLOCATE, block);
        logged(&mut e, &log, OBJECT_BASE_CONSTRUCTOR, block);
        assert_eq!(e.call(0x00a6_f410, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_STENCIL_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(block + STENCIL_FLAGS), 0x4180);
        assert_eq!(e.mem.u32(block + STENCIL_REF), 0);
        assert_eq!(e.mem.u32(block + STENCIL_MASK), 0xffff_ffff);
        logged(&mut e, &log, ALLOCATE, 0);
        assert_eq!(e.call(0x00a6_f410, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_a_controlled_property_and_shares_a_plain_one() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x24);
        logged(&mut e, &log, ALLOCATE, block);
        logged(&mut e, &log, OBJECT_BASE_CONSTRUCTOR, block);
        logged(&mut e, &log, OBJECT_NET_COPY_MEMBERS, 0);
        logged(&mut e, &log, OBJECT_SHARE_CLONE, 0);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + CONTROLLERS, 0x7000);
        e.mem.set_u16(this + STENCIL_FLAGS, 0x1234);
        e.mem.set_u32(this + STENCIL_REF, 5);
        e.mem.set_u32(this + STENCIL_MASK, 0xff);
        assert_eq!(e.call(0x00a6_f390, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(
            log.borrow()[2],
            (OBJECT_NET_COPY_MEMBERS, vec![this, block, 0x9000])
        );
        assert_eq!(e.mem.u32(block), NI_STENCIL_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(block + STENCIL_FLAGS), 0x1234);
        assert_eq!(e.mem.u32(block + STENCIL_REF), 5);
        assert_eq!(e.mem.u32(block + STENCIL_MASK), 0xff);
        // A failed allocation copies into address 0.
        e.map(0, 0x100);
        logged(&mut e, &log, ALLOCATE, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a6_f390, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(
            log.borrow()[1],
            (OBJECT_NET_COPY_MEMBERS, vec![this, 0, 0x9000])
        );
        assert_eq!(e.mem.u32(STENCIL_MASK), 0xff);
        // No controllers: the property is its own clone.
        e.mem.set_u32(this + CONTROLLERS, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a6_f390, &args![this, 0x9000u32]).u32(), this);
        assert_eq!(
            *log.borrow(),
            [(OBJECT_SHARE_CLONE, vec![this, this, 0x9000])]
        );
    }

    #[test]
    fn fn_00a6f4d0_releases_the_default_property() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, DELETE_THIS, 0);
        e.register(0x00fd_f1c4, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        e.map(STENCIL_PROPERTY_SLOT, 4);
        e.call(0x00a6_f4d0, &args![]);
        assert!(log.borrow().is_empty());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let object = e.mem.alloc(0x24);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, 2);
        e.mem.set_u32(STENCIL_PROPERTY_SLOT, object);
        e.call(0x00a6_f4d0, &args![]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(object + 4), 1);
        assert_eq!(e.mem.u32(STENCIL_PROPERTY_SLOT), 0);
        e.mem.set_u32(STENCIL_PROPERTY_SLOT, object);
        e.call(0x00a6_f4d0, &args![]);
        assert_eq!(*log.borrow(), [(DELETE_THIS, vec![object])]);
    }

    /// `LoadBinary` on a stream of file `version` whose reads hand out
    /// `words` in turn (each read writes its size in bytes of the word).
    fn load(version: u32, words: &'static [u32], flags: u16) -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, PROPERTY_LOAD_BINARY, 0);
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ);
        let l = log.clone();
        let mut next = words.iter();
        e.register_double(READ, move |e, a| {
            l.borrow_mut()
                .push((READ, vec![a[1], a[2], e.mem.u32(a[3]), a[4]]));
            let word = *next.next().unwrap();
            e.mem.write(a[1], &word.to_le_bytes()[..a[2] as usize]);
            ret(a[2])
        });
        let this = e.mem.alloc(0x24);
        e.mem.set_u16(this + STENCIL_FLAGS, flags);
        e.call(0x00a6_f500, &args![this, stream]);
        (e, log, this, stream)
    }

    #[test]
    fn load_binary_reads_the_packed_flags_of_new_files() {
        let (e, log, this, stream) = load(0x1401_0002, &[0xbeef, 3, 0x7f], 0);
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u16(this + STENCIL_FLAGS), 0xbeef);
        assert_eq!(e.mem.u32(this + STENCIL_REF), 3);
        assert_eq!(e.mem.u32(this + STENCIL_MASK), 0x7f);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 10);
        let reads: Vec<Vec<u32>> = log.borrow()[1..].iter().map(|c| c.1.clone()).collect();
        assert_eq!(
            reads,
            [
                vec![this + STENCIL_FLAGS, 2, 2, 1],
                vec![this + STENCIL_REF, 4, 4, 1],
                vec![this + STENCIL_MASK, 4, 4, 1],
            ]
        );
    }

    #[test]
    fn load_binary_packs_the_fields_of_old_files() {
        // enable, function 5, ref 2, mask 0xff, fail 3, z-fail 6, pass 1,
        // draw mode 2.
        let (e, log, this, _) = load(0x1401_0001, &[1, 5, 2, 0xff, 3, 6, 1, 2], 0x0000);
        let expected = 1 | (5 << 12) | (3 << 1) | (6 << 4) | (1 << 7) | (2 << 10);
        assert_eq!(e.mem.u16(this + STENCIL_FLAGS), expected);
        assert_eq!(e.mem.u32(this + STENCIL_REF), 2);
        assert_eq!(e.mem.u32(this + STENCIL_MASK), 0xff);
        assert_eq!(log.borrow().len(), 9);
        assert_eq!(log.borrow()[1].1[1..], [1, 1, 1]);
        // A zero enable byte clears bit 0; values wider than their field
        // spill into the bits above, as the exe's shifts and ORs do.
        let (e, _, this, _) = load(0x0400_0002, &[0, 0, 0, 0, 0, 0, 0, 0x3f], 0xffff);
        assert_eq!(e.mem.u16(this + STENCIL_FLAGS), 0xfc00);
    }
}
