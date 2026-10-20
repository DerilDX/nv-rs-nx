//! `BSMain/bsstream.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The two functions are the `NiTLargeArray<NiPointer<NiObject>>` instance the
//! stream's object array uses (`NiStream::m_kObjects`): elements are counted
//! references to objects whose reference count is at +4 and whose virtual
//! slot at +4 deletes them.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::NiTLargeArray;

/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00c1d790`: `cdecl(count)`, allocates the storage of `count` pointers (the
/// count in front of the elements, which are null).
const ARRAY_ALLOCATE: u32 = 0x00c1_d790;
/// `00a47600`: `cdecl(storage)`, releases every reference in the storage
/// `ARRAY_ALLOCATE` made and frees it.
const ARRAY_FREE_STORAGE: u32 = 0x00a4_7600;
/// The virtual slot of an object that deletes it when its count reaches zero.
const SLOT_DELETE_THIS: u32 = 0x04;

/// `InterlockedDecrement` of the count at +4; the delete slot runs at zero.
/// (The code tests for null first where the element can be null.)
fn release(e: &mut Engine, object: u32) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if count == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// `InterlockedIncrement` of the count at +4.
fn add_ref(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

// Translated from 00a64fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<NiPointer<NiObject>,NiTNewInterface<NiPointer<NiObject>>>::SetSize`
/// (engine map; `thiscall`, one stack word): gives the array room for `new_max`
/// elements. Does nothing when that is the current capacity. When it is below
/// the used size, the elements from `new_max` on are released and cleared (the
/// element count `m_uiESize` goes down for each non-null one) and the used size
/// becomes `new_max`. Then a new storage block (`00c1d790`) takes the elements,
/// each one a new counted reference, the old storage is released and freed
/// (`00a47600`) and the capacity is `new_max` (the pointer is null for 0).
pub fn ni_t_large_array_set_size(e: &mut Engine, this: Ptr<NiTLargeArray>, new_max: u32) {
    if new_max == e.get(this, NiTLargeArray::m_uiMaxSize) {
        return;
    }
    if new_max < e.get(this, NiTLargeArray::m_uiSize) {
        let mut index = new_max;
        while index < e.get(this, NiTLargeArray::m_uiSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let entry = e.mem.u32(slot);
            if entry != 0 {
                release(e, entry);
                e.mem.set_u32(slot, 0);
                let present = e.get(this, NiTLargeArray::m_uiESize);
                e.set(this, NiTLargeArray::m_uiESize, present.wrapping_sub(1));
            }
            index = index.wrapping_add(1);
        }
        e.set(this, NiTLargeArray::m_uiSize, new_max);
    }
    let old = e.get(this, NiTLargeArray::m_pBase);
    e.set(this, NiTLargeArray::m_uiMaxSize, new_max);
    if new_max == 0 {
        e.set(this, NiTLargeArray::m_pBase, 0);
    } else {
        let storage = e.call(ARRAY_ALLOCATE, &args![new_max]).u32();
        e.set(this, NiTLargeArray::m_pBase, storage);
        let mut index = 0u32;
        while index < e.get(this, NiTLargeArray::m_uiSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let current = e.mem.u32(slot);
            let carried = e.mem.u32(old.wrapping_add(index.wrapping_mul(4)));
            if current != carried {
                if current != 0 {
                    release(e, current);
                }
                e.mem.set_u32(slot, carried);
                if carried != 0 {
                    add_ref(e, carried);
                }
            }
            index = index.wrapping_add(1);
        }
        let mut index = e.get(this, NiTLargeArray::m_uiSize);
        while index < e.get(this, NiTLargeArray::m_uiMaxSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let entry = e.mem.u32(slot);
            if entry != 0 {
                release(e, entry);
                e.mem.set_u32(slot, 0);
            }
            index = index.wrapping_add(1);
        }
    }
    e.call(ARRAY_FREE_STORAGE, &args![old]);
}

// Translated from 00cc1a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<NiPointer<NiObject>,NiTNewInterface<NiPointer<NiObject>>>::SetAt`
/// (engine map; `thiscall`, two stack words): stores the reference that
/// `value` points at into element `index`. Keeps the used size (`index + 1`
/// when the index is at or beyond it) and the non-null count (`m_uiESize`) right,
/// and, when the element changes, releases the old object and counts a new
/// reference to the new one. The capacity is not checked.
pub fn ni_t_large_array_set_at(e: &mut Engine, this: Ptr<NiTLargeArray>, index: u32, value: Ptr) {
    let present = e.get(this, NiTLargeArray::m_uiESize);
    let wanted = e.mem.u32(value.addr());
    let base = e.get(this, NiTLargeArray::m_pBase);
    let slot = base.wrapping_add(index.wrapping_mul(4));
    if index < e.get(this, NiTLargeArray::m_uiSize) {
        let held = e.mem.u32(slot);
        if wanted == 0 {
            if held != 0 {
                e.set(this, NiTLargeArray::m_uiESize, present.wrapping_sub(1));
            }
        } else if held == 0 {
            e.set(this, NiTLargeArray::m_uiESize, present.wrapping_add(1));
        }
    } else {
        e.set(this, NiTLargeArray::m_uiSize, index.wrapping_add(1));
        if wanted != 0 {
            e.set(this, NiTLargeArray::m_uiESize, present.wrapping_add(1));
        }
    }
    let base = e.get(this, NiTLargeArray::m_pBase);
    let slot = base.wrapping_add(index.wrapping_mul(4));
    let held = e.mem.u32(slot);
    if held != e.mem.u32(value.addr()) {
        if held != 0 {
            release(e, held);
        }
        let new = e.mem.u32(value.addr());
        e.mem.set_u32(slot, new);
        if new != 0 {
            add_ref(e, new);
        }
    }
}

// Translated from 00c3b9e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043dc without reading the receiver. The map names this body `BSReference::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c3b9e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43dc
}

// Translated from 00c3ba70 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043e4 without reading the receiver. The map names this body `BSNodeReferences::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c3ba70(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43e4
}

/// `NiStream::RegisterLoader` (engine map, `00a64900`, cdecl `(name,
/// loader)`).
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// The class names and loader functions `BSStream::InitLoaders` registers,
/// in its order (the code's immediate operands: name string, loader).
const LOADERS: [(u32, u32); 25] = [
    (0x010c_1344, 0x00c6_4810),
    (0x010c_1330, 0x00c6_49f0),
    (0x010c_1318, 0x00c5_37f0),
    (0x010c_1310, 0x00c5_5010),
    (0x010c_1304, 0x00c4_3190),
    (0x010c_12e4, 0x00c6_4530),
    (0x010c_12d4, 0x00c4_6740),
    (0x010c_12b8, 0x00c6_3880),
    (0x010c_12a4, 0x00c5_4210),
    (0x010c_1290, 0x00c6_1e70),
    (0x010c_1284, 0x00c6_1510),
    (0x010c_1274, 0x00c4_b0c0),
    (0x010c_1268, 0x00c4_ac00),
    (0x010c_125c, 0x00c4_3a30),
    (0x010c_123c, 0x00c5_cbb0),
    (0x010c_1224, 0x00c5_e5a0),
    (0x010c_1010, 0x00c3_61a0),
    (0x010c_1210, 0x00c4_6df0),
    (0x010c_1048, 0x00c3_81a0),
    (0x010c_1020, 0x00c3_6500),
    (0x010c_11fc, 0x00c6_10a0),
    (0x010c_11e8, 0x00c4_fdb0),
    (0x010c_11cc, 0x00c6_07d0),
    (0x010c_11bc, 0x00c4_ae60),
    (0x010c_11a0, 0x00c5_fee0),
];
/// `NiFile::GetFile` (engine map, `00aa1a20`, cdecl `(path, mode, size)`).
const NI_FILE_GET_FILE: u32 = 0x00aa_1a20;
/// `strcpy_s` (`00ec65a6`, cdecl) and the path normalizer `00aa1380`
/// (cdecl, one word).
const STRCPY_S: u32 = 0x00ec_65a6;
const NORMALIZE_PATH: u32 = 0x00aa_1380;
/// `NiStream::Load` on a binary stream (`00a63330`, engine map `Load_ov2`).
const NI_STREAM_LOAD_FROM_STREAM: u32 = 0x00a6_3330;
/// `NiStream`'s file name buffer (`+0xE0`, 0x104 bytes).
const FILE_NAME_OFFSET: u32 = 0xe0;
/// `NiStream` fields `LoadStream` uses: the file version (`+0xD8`), the
/// object array and its count (`+0x1F0`, `+0x1F4`), the load state
/// (`+0x28C`, 2 is failure), the three link indices (`+0x294`, `+0x298`,
/// `+0x29C`) and the reference map of `BSStream` (`+0x5C4`).
const VERSION_OFFSET: u32 = 0xd8;
const OBJECTS_OFFSET: u32 = 0x1f0;
const OBJECT_COUNT_OFFSET: u32 = 0x1f4;
const LOAD_STATE_OFFSET: u32 = 0x28c;
const LOAD_INDEX_OFFSET: u32 = 0x294;
const LINK_INDEX_OFFSET: u32 = 0x298;
const POST_LINK_INDEX_OFFSET: u32 = 0x29c;
const REFERENCE_MAP_OFFSET: u32 = 0x5c4;
/// `NiObjectNET::ms_RTTI` and the version strings `"5.0.0.1"` and
/// `"5.0.0.6"`.
const NI_OBJECT_NET_RTTI: u32 = 0x011f_4304;
const VERSION_5_0_0_1: u32 = 0x010c_1358;
const VERSION_5_0_0_6: u32 = 0x010c_1350;

// Translated from 00c3a6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream::InitLoaders` (Xbox PDB, engine map): registers the 25 class
/// loaders of [`LOADERS`] with `NiStream::RegisterLoader`, in order.
pub fn bs_stream_init_loaders(e: &mut Engine) {
    for (name, loader) in LOADERS {
        e.call(REGISTER_LOADER, &args![name, loader]);
    }
}

// Translated from 00c3a8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream::Load` (Xbox PDB, engine map; vtable): opens `path` as a file
/// (`NiFile::GetFile` with mode 0 and 0x2800 bytes) when no `file` is given,
/// keeps the path (`strcpy_s` into `+0xE0`, then `00aa1380` on it), loads
/// from the file (`NiStream::Load`) and deletes the file (its vtable +0
/// with 1). Returns the load's result, false without a file.
pub fn bs_stream_load(e: &mut Engine, this: Ptr, path: u32, file: u32) -> u8 {
    let mut file = file;
    if file == 0 {
        file = e
            .call(NI_FILE_GET_FILE, &args![path, 0u32, 0x2800u32])
            .u32();
    }
    let mut loaded = 0u8;
    if file != 0 {
        let name = this.addr().wrapping_add(FILE_NAME_OFFSET);
        e.call(STRCPY_S, &args![name, 0x104u32, path]);
        e.call(NORMALIZE_PATH, &args![name]);
        loaded = e.call(NI_STREAM_LOAD_FROM_STREAM, &args![this, file]).u8();
        if file != 0 {
            e.vcall(file, 0, &args![1u32]);
        }
    }
    loaded
}

/// The object at `index` of the stream's object array when it is an
/// `NiObjectNET` (its RTTI chain from vtable +8 reaches
/// `NiObjectNET::ms_RTTI`), else null; and the object itself.
fn object_net(e: &mut Engine, this: Ptr, index: u32) -> (u32, u32) {
    let objects = e.mem.u32(this.addr().wrapping_add(OBJECTS_OFFSET));
    let object = e.mem.u32(objects.wrapping_add(index.wrapping_mul(4)));
    if object == 0 {
        return (object, 0);
    }
    let mut rtti = e.vcall(object, 8, &[]).u32();
    let is_net = loop {
        if rtti == 0 {
            break false;
        }
        if rtti == NI_OBJECT_NET_RTTI {
            break true;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    };
    (object, if is_net { object } else { 0 })
}

/// Whether the reference map (`+0x5C4`, `00853130`) already resolved the
/// named object (an `NiObjectNET` whose name is at `+8`).
fn resolved_by_map(e: &mut Engine, this: Ptr, net: u32) -> bool {
    if net == 0 {
        return false;
    }
    let name = e.mem.u32(net.wrapping_add(8));
    e.with_stack(4, |e, found| {
        let map = e.mem.u32(this.addr().wrapping_add(REFERENCE_MAP_OFFSET));
        if map != 0 && name != 0 {
            e.call(0x0085_3130, &args![map, name, found]);
        }
        e.mem.u32(found.addr()) != 0
    })
}

fn stream_field(e: &Engine, this: Ptr, offset: u32) -> u32 {
    e.mem.u32(this.addr().wrapping_add(offset))
}

fn set_stream_field(e: &mut Engine, this: Ptr, offset: u32, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(offset), value);
}

// Translated from 00c3ac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream::LoadStream` (Xbox PDB, engine map; vtable): without a
/// reference map (`+0x5C4`) it is `NiStream::LoadStream` (`00a65c60`).
/// Otherwise: the header (vtable +0x34) must load; the objects are removed
/// (`NiStream::RemoveAllObjects`); from version 5.0.0.1 on the RTTI table
/// must load (`NiStream::LoadRTTI`), and from 5.0.0.6 on `00a647f0` runs.
/// Each object is then loaded (its vtable +0x4C with the stream from
/// 5.0.0.1 on, else the stream's +0x50 must succeed), the top objects read
/// (vtable +0x48), and each object linked (+0x50) and post-linked (+0x6C)
/// unless it is a named `NiObjectNET` the reference map resolves. A load
/// state of 2 stops with false. Ends with `00a63d10` and
/// `NiStream::FreeLoadData`; returns true.
pub fn bs_stream_load_stream(e: &mut Engine, this: Ptr) -> u8 {
    if stream_field(e, this, REFERENCE_MAP_OFFSET) == 0 {
        return e.call(0x00a6_5c60, &args![this]).u8();
    }
    if e.vcall(this.addr(), 0x34, &[]).u8() == 0 {
        return 0;
    }
    set_stream_field(e, this, POST_LINK_INDEX_OFFSET, 0);
    set_stream_field(e, this, LINK_INDEX_OFFSET, 0);
    set_stream_field(e, this, LOAD_INDEX_OFFSET, 0);
    e.call(0x00a6_44f0, &args![this]);
    let version = stream_field(e, this, VERSION_OFFSET);
    let rtti_version = e.call(0x00a6_3630, &args![VERSION_5_0_0_1]).u32();
    let newer = version >= rtti_version;
    if newer && e.call(0x00a6_5800, &args![this]).u8() == 0 {
        return 0;
    }
    let version = stream_field(e, this, VERSION_OFFSET);
    if version >= e.call(0x00a6_3630, &args![VERSION_5_0_0_6]).u32() {
        e.call(0x00a6_47f0, &args![this]);
    }
    let count = stream_field(e, this, OBJECT_COUNT_OFFSET);
    while stream_field(e, this, LOAD_INDEX_OFFSET) < count {
        if stream_field(e, this, LOAD_STATE_OFFSET) == 2 {
            return 0;
        }
        if newer {
            let index = stream_field(e, this, LOAD_INDEX_OFFSET);
            let objects = stream_field(e, this, OBJECTS_OFFSET);
            let object = e.mem.u32(objects.wrapping_add(index.wrapping_mul(4)));
            e.vcall(object, 0x4c, &args![this]);
        } else if e.vcall(this.addr(), 0x50, &[]).u8() == 0 {
            return 0;
        }
        let index = stream_field(e, this, LOAD_INDEX_OFFSET);
        set_stream_field(e, this, LOAD_INDEX_OFFSET, index.wrapping_add(1));
    }
    e.vcall(this.addr(), 0x48, &[]);
    for (index_offset, slot) in [(LINK_INDEX_OFFSET, 0x50), (POST_LINK_INDEX_OFFSET, 0x6c)] {
        while stream_field(e, this, index_offset) < count {
            if stream_field(e, this, LOAD_STATE_OFFSET) == 2 {
                return 0;
            }
            let index = stream_field(e, this, index_offset);
            let (object, net) = object_net(e, this, index);
            if !resolved_by_map(e, this, net) {
                e.vcall(object, slot, &args![this]);
            }
            let index = stream_field(e, this, index_offset);
            set_stream_field(e, this, index_offset, index.wrapping_add(1));
        }
    }
    e.call(0x00a6_3d10, &args![this]);
    e.call(0x00a6_4ca0, &args![this]);
    1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_a6d0, bs_stream_init_loaders()),
        entry!(0x00c3_a8a0, bs_stream_load(Ptr, u32, u32) -> u8),
        entry!(0x00c3_ac10, bs_stream_load_stream(Ptr) -> u8),
        entry!(0x00c3_b9e0, fn_00c3b9e0(Ptr) -> u32),
        entry!(0x00c3_ba70, fn_00c3ba70(Ptr) -> u32),
        entry!(
            0x00a6_4fe0,
            ni_t_large_array_set_size(Ptr<NiTLargeArray>, u32)
        ),
        entry!(
            0x00cc_1a10,
            ni_t_large_array_set_at(Ptr<NiTLargeArray>, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const DELETE_FUNCTION: u32 = 0x0040_0010;
    type Deleted = Rc<RefCell<Vec<u32>>>;

    struct Fixture {
        e: Engine,
        array: Ptr<NiTLargeArray>,
        vtable: u32,
        deleted: Deleted,
        freed: Rc<RefCell<Vec<u32>>>,
    }

    impl Fixture {
        fn new() -> Fixture {
            let mut e = Engine::new();
            e.register(INTERLOCKED_INCREMENT, |e, a| {
                let count = e.mem.u32(a[0]).wrapping_add(1);
                e.mem.set_u32(a[0], count);
                count.into_ret()
            });
            e.register(INTERLOCKED_DECREMENT, |e, a| {
                let count = e.mem.u32(a[0]).wrapping_sub(1);
                e.mem.set_u32(a[0], count);
                count.into_ret()
            });
            e.register(ARRAY_ALLOCATE, |e, a| e.mem.alloc(a[0] * 4 + 4).into_ret());
            let deleted: Deleted = Rc::default();
            let log = deleted.clone();
            e.register_double(DELETE_FUNCTION, move |_, a| {
                log.borrow_mut().push(a[0]);
                Ret::default()
            });
            let freed: Rc<RefCell<Vec<u32>>> = Rc::default();
            let log = freed.clone();
            e.register_double(ARRAY_FREE_STORAGE, move |_, a| {
                log.borrow_mut().push(a[0]);
                Ret::default()
            });
            let vtable = e.mem.alloc(8);
            e.mem.set_u32(vtable + 4, DELETE_FUNCTION);
            let array = e.new_object::<NiTLargeArray>();
            Fixture {
                e,
                array,
                vtable,
                deleted,
                freed,
            }
        }

        fn object(&mut self, references: u32) -> u32 {
            let object = self.e.mem.alloc(8);
            self.e.mem.set_u32(object, self.vtable);
            self.e.mem.set_u32(object + 4, references);
            object
        }

        /// Gives the array storage for `max` elements holding `items`.
        fn fill(&mut self, max: u32, items: &[u32]) -> u32 {
            let base = self.e.mem.alloc(max * 4 + 4);
            for (i, item) in items.iter().enumerate() {
                self.e.mem.set_u32(base + i as u32 * 4, *item);
            }
            let array = self.array;
            self.e.set(array, NiTLargeArray::m_pBase, base);
            self.e.set(array, NiTLargeArray::m_uiMaxSize, max);
            self.e
                .set(array, NiTLargeArray::m_uiSize, items.len() as u32);
            let present = items.iter().filter(|item| **item != 0).count() as u32;
            self.e.set(array, NiTLargeArray::m_uiESize, present);
            base
        }

        fn state(&self) -> (u32, u32, u32) {
            (
                self.e.get(self.array, NiTLargeArray::m_uiMaxSize),
                self.e.get(self.array, NiTLargeArray::m_uiSize),
                self.e.get(self.array, NiTLargeArray::m_uiESize),
            )
        }

        fn element(&self, index: u32) -> u32 {
            let base = self.e.get(self.array, NiTLargeArray::m_pBase);
            self.e.mem.u32(base + index * 4)
        }

        fn references(&self, object: u32) -> u32 {
            self.e.mem.u32(object + 4)
        }
    }

    #[test]
    fn set_size_to_the_current_capacity_does_nothing() {
        let mut fx = Fixture::new();
        let a = fx.object(1);
        let base = fx.fill(3, &[a]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 3u32]);
        assert_eq!(fx.e.get(fx.array, NiTLargeArray::m_pBase), base);
        assert!(fx.freed.borrow().is_empty());
        assert_eq!(fx.references(a), 1);
    }

    #[test]
    fn set_size_grows_into_new_storage_and_counts_a_reference_per_element() {
        let mut fx = Fixture::new();
        let (a, b) = (fx.object(1), fx.object(1));
        let old = fx.fill(4, &[a, 0, b]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 8u32]);
        assert_eq!(fx.state(), (8, 3, 2));
        assert_eq!(*fx.freed.borrow(), vec![old]);
        assert_eq!([fx.element(0), fx.element(1), fx.element(2)], [a, 0, b]);
        assert_eq!(fx.element(5), 0);
        assert_eq!((fx.references(a), fx.references(b)), (2, 2));
        assert!(fx.deleted.borrow().is_empty());
    }

    #[test]
    fn set_size_below_the_used_size_releases_the_cut_elements() {
        let mut fx = Fixture::new();
        let (a, b, c) = (fx.object(1), fx.object(2), fx.object(1));
        let old = fx.fill(3, &[a, b, c]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 1u32]);
        assert_eq!(fx.state(), (1, 1, 1));
        // b only loses the array's reference, c is deleted.
        assert_eq!((fx.references(b), fx.references(c)), (1, 0));
        assert_eq!(*fx.deleted.borrow(), vec![c]);
        // The cut slots of the old storage were cleared before the copy.
        assert_eq!(fx.e.mem.u32(old + 4), 0);
        assert_eq!(fx.e.mem.u32(old + 8), 0);
        assert_eq!(fx.element(0), a);
        assert_eq!(fx.references(a), 2);
        assert_eq!(*fx.freed.borrow(), vec![old]);
    }

    #[test]
    fn set_size_to_zero_clears_the_pointer() {
        let mut fx = Fixture::new();
        let a = fx.object(1);
        let old = fx.fill(2, &[a]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 0u32]);
        assert_eq!(fx.state(), (0, 0, 0));
        assert_eq!(fx.e.get(fx.array, NiTLargeArray::m_pBase), 0);
        assert_eq!(*fx.deleted.borrow(), vec![a]);
        assert_eq!(*fx.freed.borrow(), vec![old]);
    }

    fn holder(fx: &mut Fixture, object: u32) -> u32 {
        let holder = fx.e.mem.alloc(4);
        fx.e.mem.set_u32(holder, object);
        holder
    }

    #[test]
    fn set_at_stores_a_new_reference_in_an_empty_slot() {
        let mut fx = Fixture::new();
        fx.fill(4, &[0, 0]);
        let a = fx.object(1);
        let value = holder(&mut fx, a);
        fx.e.call(0x00cc_1a10, &args![fx.array, 1u32, value]);
        assert_eq!(fx.element(1), a);
        assert_eq!(fx.references(a), 2);
        assert_eq!(fx.state(), (4, 2, 1));
    }

    #[test]
    fn set_at_beyond_the_used_size_extends_it() {
        let mut fx = Fixture::new();
        fx.fill(8, &[0]);
        let a = fx.object(1);
        let value = holder(&mut fx, a);
        fx.e.call(0x00cc_1a10, &args![fx.array, 5u32, value]);
        assert_eq!(fx.element(5), a);
        assert_eq!(fx.state(), (8, 6, 1));
        let nothing = holder(&mut fx, 0);
        fx.e.call(0x00cc_1a10, &args![fx.array, 7u32, nothing]);
        assert_eq!(fx.state(), (8, 8, 1));
    }

    #[test]
    fn set_at_replaces_and_releases_the_old_object() {
        let mut fx = Fixture::new();
        let (a, b) = (fx.object(1), fx.object(1));
        fx.fill(4, &[a]);
        let value = holder(&mut fx, b);
        fx.e.call(0x00cc_1a10, &args![fx.array, 0u32, value]);
        assert_eq!(fx.element(0), b);
        assert_eq!((fx.references(a), fx.references(b)), (0, 2));
        assert_eq!(*fx.deleted.borrow(), vec![a]);
        assert_eq!(fx.state(), (4, 1, 1));
    }

    #[test]
    fn set_at_with_null_clears_the_slot_and_the_count() {
        let mut fx = Fixture::new();
        let a = fx.object(2);
        fx.fill(4, &[a]);
        let value = holder(&mut fx, 0);
        fx.e.call(0x00cc_1a10, &args![fx.array, 0u32, value]);
        assert_eq!(fx.element(0), 0);
        assert_eq!(fx.references(a), 1);
        assert_eq!(fx.state(), (4, 1, 0));
        assert!(fx.deleted.borrow().is_empty());
    }

    #[test]
    fn set_at_with_the_same_object_changes_no_count() {
        let mut fx = Fixture::new();
        let a = fx.object(1);
        fx.fill(4, &[a]);
        let value = holder(&mut fx, a);
        fx.e.call(0x00cc_1a10, &args![fx.array, 0u32, value]);
        assert_eq!(fx.references(a), 1);
        assert_eq!(fx.state(), (4, 1, 1));
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c3b9e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_b9e0, &args![0x1000u32]).u32(), 0x0120_43dc);
        assert_eq!(e.call(0x00c3_b9e0, &args![0u32]).u32(), 0x0120_43dc);
    }

    #[test]
    fn fn_00c3ba70_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_ba70, &args![0x1000u32]).u32(), 0x0120_43e4);
        assert_eq!(e.call(0x00c3_ba70, &args![0u32]).u32(), 0x0120_43e4);
    }
}

#[cfg(test)]
mod stream_tests {
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

    /// An object whose vtable slot `slot` points at `base + slot` (doubles
    /// returning 0 unless replaced).
    fn object(e: &mut Engine, base: u32) -> u32 {
        let table = e.mem.alloc(0x80);
        for slot in (0..0x80).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
            e.register(base + slot, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x600);
        e.mem.set_u32(this, table);
        this
    }

    #[test]
    fn init_loaders_registers_every_class() {
        let mut e = Engine::new();
        e.register(REGISTER_LOADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_a6d0, &[]);
        let registered = calls_to(&e, REGISTER_LOADER);
        assert_eq!(registered.len(), 25);
        assert_eq!(registered[0], vec![0x010c_1344, 0x00c6_4810]);
        assert_eq!(registered[24], vec![0x010c_11a0, 0x00c5_fee0]);
    }

    #[test]
    fn load_opens_a_file_when_none_is_given_and_deletes_it() {
        let mut e = Engine::new();
        let stream = e.mem.alloc(0x200);
        let file = object(&mut e, 0x7100_0000);
        e.register_double(NI_FILE_GET_FILE, move |_, _| file.into_ret());
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(NORMALIZE_PATH, |_, _| Ret::default());
        e.register(NI_STREAM_LOAD_FROM_STREAM, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_a8a0, &args![stream, 0x5000u32, 0u32]).u8(), 1);
        assert_eq!(
            calls_to(&e, NI_FILE_GET_FILE),
            vec![vec![0x5000, 0, 0x2800]]
        );
        assert_eq!(
            calls_to(&e, STRCPY_S),
            vec![vec![stream + 0xe0, 0x104, 0x5000]]
        );
        assert_eq!(
            calls_to(&e, NI_STREAM_LOAD_FROM_STREAM),
            vec![vec![stream, file]]
        );
        assert_eq!(calls_to(&e, 0x7100_0000), vec![vec![file, 1]]);
        // A given file is used (and deleted) as it is; no file: false.
        e.call_log = Some(vec![]);
        e.call(0x00c3_a8a0, &args![stream, 0x5000u32, file]);
        assert!(calls_to(&e, NI_FILE_GET_FILE).is_empty());
        e.register(NI_FILE_GET_FILE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c3_a8a0, &args![stream, 0x5000u32, 0u32]).u8(), 0);
    }

    #[test]
    fn load_stream_without_a_map_is_the_base_load() {
        let mut e = Engine::new();
        let stream = object(&mut e, 0x7000_0000);
        e.register(0x00a6_5c60, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00c3_ac10, &args![stream]).u8(), 1);
        // With a map, a header that fails to load: false.
        e.mem.set_u32(stream + 0x5c4, 0x4444);
        assert_eq!(e.call(0x00c3_ac10, &args![stream]).u8(), 0);
    }

    #[test]
    fn load_stream_loads_links_and_skips_mapped_objects() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        let stream = object(&mut e, 0x7000_0000);
        e.register(0x7000_0034, |_, _| 1u32.into_ret());
        e.mem.set_u32(stream + 0x5c4, 0x4444);
        e.mem.set_u32(stream + 0xd8, 20);
        // Versions: 5.0.0.1 is 10 and 5.0.0.6 is 30.
        e.register(0x00a6_3630, |_, a| {
            if a[0] == VERSION_5_0_0_1 { 10u32 } else { 30 }.into_ret()
        });
        for target in [0x00a6_44f0, 0x00a6_47f0, 0x00a6_3d10, 0x00a6_4ca0] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(0x00a6_5800, |_, _| 1u32.into_ret());
        // Two objects: a plain one and an `NiObjectNET` named 0x99 that the
        // map resolves.
        let plain = object(&mut e, 0x7200_0000);
        let named = object(&mut e, 0x7300_0000);
        e.mem.set_u32(named + 8, 0x99);
        let parent = e.mem.alloc(8);
        e.mem.set_u32(parent + 4, NI_OBJECT_NET_RTTI);
        e.register_double(0x7300_0008, move |_, _| parent.into_ret());
        e.register(0x0085_3130, |e, a| {
            e.mem.set_u32(a[2], u32::from(a[1] == 0x99));
            Ret::default()
        });
        let objects = e.mem.alloc(8);
        e.mem.set_u32(objects, plain);
        e.mem.set_u32(objects + 4, named);
        e.mem.set_u32(stream + 0x1f0, objects);
        e.mem.set_u32(stream + 0x1f4, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_ac10, &args![stream]).u8(), 1);
        assert_eq!(calls_to(&e, 0x7200_004c), vec![vec![plain, stream]]);
        assert_eq!(calls_to(&e, 0x7300_004c), vec![vec![named, stream]]);
        assert_eq!(calls_to(&e, 0x7200_0050), vec![vec![plain, stream]]);
        assert_eq!(calls_to(&e, 0x7200_006c), vec![vec![plain, stream]]);
        assert!(calls_to(&e, 0x7300_0050).is_empty());
        assert!(calls_to(&e, 0x7300_006c).is_empty());
        assert_eq!(calls_to(&e, 0x7000_0048).len(), 1);
        assert!(calls_to(&e, 0x00a6_47f0).is_empty());
        assert_eq!(e.mem.u32(stream + 0x294), 2);
        assert_eq!(e.mem.u32(stream + 0x29c), 2);
        // An old file loads each object through the stream (+0x50), which
        // may fail; a failed load state stops too.
        e.mem.set_u32(stream + 0xd8, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_ac10, &args![stream]).u8(), 0);
        assert_eq!(calls_to(&e, 0x7000_0050).len(), 1);
        e.mem.set_u32(stream + 0xd8, 20);
        e.mem.set_u32(stream + 0x28c, 2);
        assert_eq!(e.call(0x00c3_ac10, &args![stream]).u8(), 0);
    }
}
