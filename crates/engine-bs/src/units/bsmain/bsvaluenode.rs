//! `BSMain/bsvaluenode.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

// Translated from 00a643b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStreamLoadBinary<unsigned char>` (engine map name
/// `NiStreamLoadBinary<unsigned_char>_ov2`, the second copy; `cdecl`, two stack words): reads one byte
/// from the stream's input stream into `destination`. Calls the input stream's read
/// function as `function(input, destination, 1, &count, 1)` with `count` a local set
/// to 1, and adds the byte count it returns to the input stream's position.
pub fn ni_stream_load_binary_unsigned_char_ov2(
    e: &mut Engine,
    stream: Ptr<NiStream>,
    destination: Ptr,
) {
    let input = e.get(stream, NiStream::m_pkIstr);
    let input = Ptr::<NiBinaryStream>::new(input);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        let done = e
            .call(function, &args![input, destination, 1u32, count, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
}

/// `00a5e9d0` (named `NiNode::LinkObject` in the engine map): the base body the wrapper
/// forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A5E9D0: u32 = 0x00a5_e9d0;

// Translated from 00c43900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSValueNode` vtable word 010c19ac, right after
/// `BSValueNode::LoadBinary` (010c19a8). Forwards `this` and the stream to `00a5e9d0`
/// (`NiNode::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c43900(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A5E9D0, &args![this, stream]);
}

// Translated from 00c43cf0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202de8 without reading the receiver. The map names this body `BSValueNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c43cf0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2de8
}

/// The vtable of `BSValueNode` (the constructor stores it).
const BS_VALUE_NODE_VTABLE: u32 = 0x010c_195c;
/// `BSValueNode` size on PC (the allocations and the sized delete).
const BS_VALUE_NODE_SIZE: u32 = 0xb8;
/// `BSValueNode` fields on PC (Xbox PDB names; the Xbox offsets are 0x24
/// higher): `m_uFlags` +0xac, `iValue` +0xb0, `iType` +0xb4.
const VALUE_NODE_FLAGS: u32 = 0xac;
const VALUE: u32 = 0xb0;
const VALUE_TYPE: u32 = 0xb4;
/// The world transform `UpdateWorldData` overwrites (`NiAVObject`, nine
/// words at +0x68) and the nine words it copies there (`011a9448`).
const WORLD_TRANSFORM: u32 = 0x68;
const FIXED_TRANSFORM: u32 = 0x011a_9448;

/// `00c5f570` (`BSNiNode`'s constructor body), `00c5fb60` (`thiscall`,
/// `(destination, cloning)`: its member copy) and `00c5faa0` (its
/// destructor body).
const BS_NI_NODE_CONSTRUCT: u32 = 0x00c5_f570;
const BS_NI_NODE_COPY_MEMBERS: u32 = 0x00c5_fb60;
const BS_NI_NODE_DESTRUCT: u32 = 0x00c5_faa0;
/// `NiNode::LoadBinary` (`00a5dd40`).
const NI_NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
/// `00a26df0` (unnamed in the engine map; `cdecl`, `(stream, &word)`):
/// reads 4 bytes.
const STREAM_READ_WORD: u32 = 0x00a2_6df0;
/// `NiAVObject::UpdateWorldData` (`00a68c60`).
const AV_OBJECT_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiStream`'s word at +4 that `LoadBinary` compares with 0x18 (the
/// file's user version, by its use; unconfirmed).
const STREAM_USER_VERSION: u32 = 0x04;

// Translated from 00c43a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c43a30 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e472, state variable) is not translated
/// Unnamed in the engine map (the `BSValueNode` factory, no arguments):
/// allocates 0xb8 bytes and constructs a node in it (`00c43660`) when that
/// succeeds. Returns it, or 0.
pub fn fn_00c43a30(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![BS_VALUE_NODE_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    bs_value_node_construct(e, Ptr::new(block))
}

// Translated from 00c43660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::BSValueNode` (Xbox PDB): `BSNiNode`'s constructor body
/// (`00c5f570`), the vtable, value 0 and type 0. Returns `this`.
pub fn bs_value_node_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BS_NI_NODE_CONSTRUCT, &args![this]);
    let node = this.addr();
    e.mem.set_u32(node, BS_VALUE_NODE_VTABLE);
    e.mem.set_u32(node.wrapping_add(VALUE), 0);
    e.mem.set_u32(node.wrapping_add(VALUE_TYPE), 0);
    this
}

// Translated from 00c438a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::LoadBinary` (Xbox PDB): `NiNode::LoadBinary`, the value (4
/// bytes through `00a26df0` into a local, then stored), and the flags byte
/// when the stream's word at +4 is above 0x18.
pub fn bs_value_node_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_NODE_LOAD_BINARY, &args![this, stream]);
    let node = this.addr();
    let value = e.with_stack(4, |e, local| {
        e.call(STREAM_READ_WORD, &args![stream, local]);
        e.mem.u32(local.addr())
    });
    e.mem.set_u32(node.wrapping_add(VALUE), value);
    if e.mem.u32(stream.addr().wrapping_add(STREAM_USER_VERSION)) > 0x18 {
        ni_stream_load_binary_unsigned_char_ov2(
            e,
            Ptr::new(stream.addr()),
            Ptr::new(node.wrapping_add(VALUE_NODE_FLAGS)),
        );
    }
}

// Translated from 00c437c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c437c0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e445, state variable) is not translated
/// `BSValueNode::CreateClone` (Xbox PDB): allocates 0xb8 bytes, constructs
/// a node in it when that succeeds, runs the member copy (`00c43850`) into
/// it (into address 0 when the allocation failed) and returns it.
pub fn bs_value_node_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![BS_VALUE_NODE_SIZE]).u32();
    let node = if block == 0 {
        Ptr::new(0)
    } else {
        bs_value_node_construct(e, Ptr::new(block))
    };
    bs_value_node_copy_members(e, this, node, cloning);
    node
}

// Translated from 00c43850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::CopyMembers` (Xbox PDB): `BSNiNode`'s member copy
/// (`00c5fb60`), then the value and the flags byte.
pub fn bs_value_node_copy_members(e: &mut Engine, this: Ptr, destination: Ptr, cloning: u32) {
    e.call(BS_NI_NODE_COPY_MEMBERS, &args![this, destination, cloning]);
    let value = e.mem.u32(this.addr().wrapping_add(VALUE));
    e.mem.set_u32(destination.addr().wrapping_add(VALUE), value);
    let flags = e.mem.u8(this.addr().wrapping_add(VALUE_NODE_FLAGS));
    e.mem
        .set_u8(destination.addr().wrapping_add(VALUE_NODE_FLAGS), flags);
}

// Translated from 00c43b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSValueNode::UpdateWorldData` (Xbox PDB): `NiAVObject::UpdateWorldData`,
/// then, with bit 0 of the flags byte, the nine words at `011a9448` copied
/// over the world transform (+0x68).
pub fn bs_value_node_update_world_data(e: &mut Engine, this: Ptr, update: u32) {
    e.call(AV_OBJECT_UPDATE_WORLD_DATA, &args![this, update]);
    let node = this.addr();
    if e.mem.u8(node.wrapping_add(VALUE_NODE_FLAGS)) & 1 != 0 {
        for k in 0..9 {
            let word = e.mem.u32(FIXED_TRANSFORM + 4 * k);
            e.mem
                .set_u32(node.wrapping_add(WORLD_TRANSFORM + 4 * k), word);
        }
    }
}

// Translated from 00c43cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsvaluenode.obj`): the `BSValueNode`
/// scalar deleting destructor. `BSNiNode`'s destructor body (`00c5faa0`),
/// then the sized delete of 0xb8 bytes when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00c43cb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BS_NI_NODE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, BS_VALUE_NODE_SIZE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_3cf0, fn_00c43cf0(Ptr) -> u32),
        entry!(
            0x00a6_43b0,
            ni_stream_load_binary_unsigned_char_ov2(Ptr<NiStream>, Ptr)
        ),
        entry!(0x00c4_3900, fn_00c43900(Ptr, Ptr)),
        entry!(0x00c4_3a30, fn_00c43a30() -> Ptr),
        entry!(0x00c4_3660, bs_value_node_construct(Ptr) -> Ptr),
        entry!(0x00c4_38a0, bs_value_node_load_binary(Ptr, Ptr)),
        entry!(0x00c4_37c0, bs_value_node_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00c4_3850, bs_value_node_copy_members(Ptr, Ptr, u32)),
        entry!(0x00c4_3b90, bs_value_node_update_world_data(Ptr, u32)),
        entry!(0x00c4_3cb0, fn_00c43cb0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c43900_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A5E9D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_3900, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A5E9D0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn load_binary_unsigned_char_reads_one_byte_through_the_input_stream() {
        const READ: u32 = 0x00aa_2e60;
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.mem.set_u32(stream.addr() + 0x24c, input.addr());
        e.mem.set_u32(input.addr() + 4, 7);
        e.mem.set_u32(input.addr() + 8, READ);
        let destination = e.mem.alloc(8);
        e.register(READ, |e, a| {
            // (input, destination, 1, &count, 1) with count set to 1
            assert_eq!(a[2], 1);
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), 1);
            e.mem.set_u8(a[1], 0x5a);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00a6_43b0, &args![stream, destination]);
        let log = e.call_log.take().unwrap();
        let reads: Vec<_> = log.iter().filter(|call| call.0 == READ).collect();
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0].1[..3], [input.addr(), destination, 1]);
        assert_eq!(e.mem.u8(destination), 0x5a);
        assert_eq!(e.mem.u32(input.addr() + 4), 8);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c43cf0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_3cf0, &args![0x1000u32]).u32(), 0x0120_2de8);
        assert_eq!(e.call(0x00c4_3cf0, &args![0u32]).u32(), 0x0120_2de8);
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    #[test]
    fn factory_and_constructor_build_a_value_node() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0xb8);
        e.mem.write(block, &[0xaa; 0xb8]);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, BS_NI_NODE_CONSTRUCT, 0);
        assert_eq!(e.call(0x00c4_3a30, &args![]).u32(), block);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0xb8]),
                (BS_NI_NODE_CONSTRUCT, vec![block])
            ]
        );
        assert_eq!(e.mem.u32(block), BS_VALUE_NODE_VTABLE);
        assert_eq!(e.mem.u32(block + VALUE), 0);
        assert_eq!(e.mem.u32(block + VALUE_TYPE), 0);
        assert_eq!(e.mem.u8(block + VALUE_NODE_FLAGS), 0xaa);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        assert_eq!(e.call(0x00c4_3a30, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_reads_the_value_and_the_flags_of_newer_files() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_NODE_LOAD_BINARY, 0);
        let l = log.clone();
        e.register_double(STREAM_READ_WORD, move |e, a| {
            l.borrow_mut().push((STREAM_READ_WORD, vec![a[0]]));
            e.mem.set_u32(a[1], 0x1234);
            0u32.into_ret()
        });
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        let read = 0x00ee_0400u32;
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, read);
        e.register_double(read, |e, a| {
            e.mem.set_u8(a[1], 0x5);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(stream + STREAM_USER_VERSION, 0x18);
        e.call(0x00c4_38a0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + VALUE), 0x1234);
        assert_eq!(e.mem.u8(this + VALUE_NODE_FLAGS), 0);
        e.mem.set_u32(stream + STREAM_USER_VERSION, 0x19);
        e.call(0x00c4_38a0, &args![this, stream]);
        assert_eq!(e.mem.u8(this + VALUE_NODE_FLAGS), 5);
        assert_eq!(e.mem.u32(input + 4), 1);
        assert_eq!(log.borrow()[0], (NI_NODE_LOAD_BINARY, vec![this, stream]));
        assert_eq!(log.borrow()[1], (STREAM_READ_WORD, vec![stream]));
    }

    #[test]
    fn create_clone_copies_value_and_flags() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0xb8);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, BS_NI_NODE_CONSTRUCT, 0);
        logged(&mut e, &log, BS_NI_NODE_COPY_MEMBERS, 0);
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this + VALUE, 77);
        e.mem.set_u8(this + VALUE_NODE_FLAGS, 3);
        e.mem.set_u32(this + VALUE_TYPE, 9);
        assert_eq!(e.call(0x00c4_37c0, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(e.mem.u32(block + VALUE), 77);
        assert_eq!(e.mem.u8(block + VALUE_NODE_FLAGS), 3);
        assert_eq!(e.mem.u32(block + VALUE_TYPE), 0);
        assert_eq!(
            log.borrow().last().unwrap(),
            &(BS_NI_NODE_COPY_MEMBERS, vec![this, block, 0x9000])
        );
        e.map(0, 0x100);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        assert_eq!(e.call(0x00c4_37c0, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(e.mem.u32(VALUE), 77);
    }

    #[test]
    fn update_world_data_fixes_the_transform_when_flagged() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, AV_OBJECT_UPDATE_WORLD_DATA, 0);
        e.map(FIXED_TRANSFORM, 0x24);
        for k in 0..9 {
            e.mem.set_u32(FIXED_TRANSFORM + 4 * k, 0x100 + k);
        }
        let this = e.mem.alloc(0xb8);
        e.call(0x00c4_3b90, &args![this, 0x77u32]);
        assert_eq!(
            *log.borrow(),
            [(AV_OBJECT_UPDATE_WORLD_DATA, vec![this, 0x77])]
        );
        assert_eq!(e.mem.u32(this + WORLD_TRANSFORM), 0);
        e.mem.set_u8(this + VALUE_NODE_FLAGS, 1);
        e.call(0x00c4_3b90, &args![this, 0x77u32]);
        assert_eq!(e.mem.u32(this + WORLD_TRANSFORM), 0x100);
        assert_eq!(e.mem.u32(this + WORLD_TRANSFORM + 0x20), 0x108);
        assert_eq!(e.mem.u32(this + WORLD_TRANSFORM + 0x24), 0);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, BS_NI_NODE_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        assert_eq!(e.call(0x00c4_3cb0, &args![0x5000u32, 1u32]).u32(), 0x5000);
        e.call(0x00c4_3cb0, &args![0x5000u32, 0u32]);
        assert_eq!(
            *log.borrow(),
            [
                (BS_NI_NODE_DESTRUCT, vec![0x5000]),
                (NI_SIZED_DELETE, vec![0x5000, 0xb8]),
                (BS_NI_NODE_DESTRUCT, vec![0x5000]),
            ]
        );
    }
}
