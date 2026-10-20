//! `NiMain/niskinpartition.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSkinPartition::ms_RTTI` (the class's `NiRTTI` record).
const NI_SKIN_PARTITION_RTTI: u32 = 0x011f_5b1c;

// Translated from 00a820d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_skin_partition_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SKIN_PARTITION_RTTI)
}

// Translated from 00a82c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a824d0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_skin_partition_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a8_24d0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

/// The Gamebryo allocator (`00aa13e0`, cdecl, the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiAlloc` (`00aa1070`, cdecl, the size).
const NI_ALLOC: u32 = 0x00aa_1070;
/// `NiObject`'s constructor and destructor body (`00a5d3a0`, `00a5d3d0`).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `NiObject::LoadBinary` (`00a5d610`) and `NiObject::RegisterStreamables`
/// (`00a5d2f0`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
/// The base `SaveBinary` this unit calls (`00a89af0`, a bare `RET 4`).
const NI_OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
/// The renderer singleton pointer (`011f4748`); its virtual `+0x100` is
/// called with the partition before it goes.
const RENDERER: u32 = 0x011f_4748;
/// `MemoryManager::Allocate` / `Deallocate` (`00aa3e40`, `00aa4060`) on the
/// singleton at `011f6238`.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `_vector_constructor_iterator` (`00401050`) and the `Partition`
/// constructor it runs (`00a81ba0`).
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
const PARTITION_CONSTRUCT: u32 = 0x00a8_1ba0;
/// The `NiSkinPartition` vtable (its RTTI locator names
/// `.?AVNiSkinPartition@@`).
const NI_SKIN_PARTITION_VTABLE: u32 = 0x0109_e47c;
/// `NiSkinPartition`: `m_uiPartitions` +8, `m_pkPartitions` +0xC (Xbox
/// PDB); a `Partition` is 0x2C bytes.
const PARTITIONS: u32 = 0x08;
const PARTITION_ARRAY: u32 = 0x0c;
const PARTITION_SIZE: u32 = 0x2c;
/// `NiSkinPartition::Partition` (Xbox PDB field names).
const PART_BONES: u32 = 0x04;
const PART_WEIGHTS: u32 = 0x08;
const PART_VERTEX_MAP: u32 = 0x0c;
const PART_BONE_PALETTE: u32 = 0x10;
const PART_TRI_LIST: u32 = 0x14;
const PART_STRIP_LENGTHS: u32 = 0x18;
const PART_VERTICES: u32 = 0x1c;
const PART_TRIANGLES: u32 = 0x1e;
const PART_BONE_COUNT: u32 = 0x20;
const PART_STRIPS: u32 = 0x22;
const PART_BONES_PER_VERTEX: u32 = 0x24;
/// The stream's version (+0xD8) and the first one with the has-array bytes
/// (`10.0.1.18`).
const STREAM_VERSION: u32 = 0xd8;
const VERSION_10_0_1_18: u32 = 0x0a00_0112;
/// `NiStream`'s input (+0x24C, read function +8) and output (+0x250,
/// write function +0xC); both move their position (+4) by what the
/// function answers.
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;

/// Reads `bytes` bytes of components of `size` bytes into `buffer`.
fn read(e: &mut Engine, stream: Ptr, buffer: u32, bytes: u32, size: u32) {
    transfer(e, stream, STREAM_INPUT, 8, buffer, bytes, size);
}

/// Writes `bytes` bytes of components of `size` bytes from `buffer`.
fn write(e: &mut Engine, stream: Ptr, buffer: u32, bytes: u32, size: u32) {
    transfer(e, stream, STREAM_OUTPUT, 0xc, buffer, bytes, size);
}

fn transfer(e: &mut Engine, stream: Ptr, side: u32, slot: u32, buffer: u32, bytes: u32, size: u32) {
    let file = e.mem.u32(stream.addr() + side);
    let function = e.mem.u32(file + slot);
    let count = e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), size);
        e.call(function, &args![file, buffer, bytes, sizes, 1u32])
            .u32()
    });
    let position = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, position.wrapping_add(count));
}

fn u16_at(e: &Engine, address: u32) -> u32 {
    u32::from(e.mem.u16(address))
}

// Translated from 00a81fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::CreateObject` (the class's stream factory; Ghidra made
/// no function of it): a new 0x10-byte object with the `NiObject`
/// constructor, the class's vtable and no partitions array (the count at
/// +8 is left as it is); null when the allocation fails.
pub fn ni_skin_partition_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_SKIN_PARTITION_VTABLE);
    e.mem.set_u32(object + PARTITION_ARRAY, 0);
    Ptr::new(object)
}

// Translated from 00a81fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A jump to `NiObject::RegisterStreamables` (`00a5d2f0`) with the same
/// `this` and stream; answers what it answers.
pub fn fn_00a81fe0(e: &mut Engine, this: Ptr, stream: Ptr) -> u32 {
    e.call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .u32()
}

// Translated from 00a824d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiSkinPartition` destructor body: the vtable; the renderer, when
/// there is one, lets go of the partition (its virtual `+0x100`); the
/// partitions array goes through the first item's vector deleting
/// destructor (virtual `+0`, flag 3) when its count word is not 0, else
/// its block (count word included) back to the `MemoryManager`; then the
/// `NiObject` part.
pub fn fn_00a824d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_SKIN_PARTITION_VTABLE);
    let renderer = e.global::<u32>(RENDERER);
    if renderer != 0 {
        e.vcall(renderer, 0x100, &args![this]);
    }
    let parts = e.mem.u32(base + PARTITION_ARRAY);
    if parts != 0 {
        let block = parts.wrapping_sub(4);
        if e.mem.u32(block) != 0 {
            e.vcall(parts, 0, &args![3u32]);
        } else {
            e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, block]);
        }
    }
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a81c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sum of the partition's strip lengths (`m_pusStripLengths`, one
/// `u16` per strip; the exe adds them two by two, then an odd last one).
pub fn fn_00a81c20(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let strips = u16_at(e, base + PART_STRIPS);
    let mut odd = 0u32;
    let mut first = 0u32;
    let mut second = 0u32;
    let mut index = 0u32;
    if strips >= 2 {
        let lengths = e.mem.u32(base + PART_STRIP_LENGTHS);
        let pairs = ((strips - 2) >> 1) + 1;
        for pair in 0..pairs {
            first = first.wrapping_add(u16_at(e, lengths + 4 * pair));
            second = second.wrapping_add(u16_at(e, lengths + 4 * pair + 2));
        }
        index = pairs * 2;
    }
    if index < strips {
        let lengths = e.mem.u32(base + PART_STRIP_LENGTHS);
        odd = u16_at(e, lengths + 2 * index);
    }
    second.wrapping_add(first).wrapping_add(odd)
}

/// The triangle index count: three per triangle without strips, else the
/// strip lengths' sum.
fn tri_list_count(e: &mut Engine, this: Ptr, read_lengths: bool, stream: Ptr) -> u32 {
    let base = this.addr();
    let strips = e.mem.u16(base + PART_STRIPS);
    if strips == 0 {
        return u16_at(e, base + PART_TRIANGLES) * 3;
    }
    let bytes = u32::from(strips) * 2;
    if read_lengths {
        let lengths = e.call(NI_ALLOC, &args![bytes]).u32();
        e.mem.set_u32(base + PART_STRIP_LENGTHS, lengths);
        let bytes = u16_at(e, base + PART_STRIPS) * 2;
        read(e, stream, lengths, bytes, 2);
    } else {
        let lengths = e.mem.u32(base + PART_STRIP_LENGTHS);
        write(e, stream, lengths, bytes, 2);
    }
    fn_00a81c20(e, this)
}

// Translated from 00a82d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::Partition::LoadBinary` (by its use): the five counts,
/// the bones (`NiAlloc`), then, each behind a has-array byte from version
/// 10.0.1.18 (always before), the vertex map, the weights (bones per
/// vertex times vertices floats), the strip lengths (when there are
/// strips) and the triangle list; the bone palette's has-byte is read in
/// every version.
pub fn fn_00a82d90(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    for offset in [
        PART_VERTICES,
        PART_TRIANGLES,
        PART_BONE_COUNT,
        PART_STRIPS,
        PART_BONES_PER_VERTEX,
    ] {
        read(e, stream, base + offset, 2, 2);
    }
    let bones = e
        .call(NI_ALLOC, &args![u16_at(e, base + PART_BONE_COUNT) * 2])
        .u32();
    let bytes = u16_at(e, base + PART_BONE_COUNT) * 2;
    e.mem.set_u32(base + PART_BONES, bones);
    read(e, stream, bones, bytes, 2);
    let tagged = e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_10_0_1_18;
    e.with_stack(4, |e, flags| {
        let flag = |e: &mut Engine, at: u32| {
            if tagged {
                read(e, stream, at, 1, 1);
            } else {
                e.mem.set_u8(at, 1);
            }
            e.mem.u8(at) != 0
        };
        let has = flags.addr();
        if flag(e, has) {
            let map = e
                .call(NI_ALLOC, &args![u16_at(e, base + PART_VERTICES) * 2])
                .u32();
            let bytes = u16_at(e, base + PART_VERTICES) * 2;
            e.mem.set_u32(base + PART_VERTEX_MAP, map);
            read(e, stream, map, bytes, 2);
        }
        let weight_count =
            u16_at(e, base + PART_BONES_PER_VERTEX).wrapping_mul(u16_at(e, base + PART_VERTICES));
        if flag(e, has + 1) {
            let bytes = weight_count.wrapping_mul(4);
            let weights = e.call(NI_ALLOC, &args![bytes]).u32();
            e.mem.set_u32(base + PART_WEIGHTS, weights);
            read(e, stream, weights, bytes, 4);
        }
        let count = tri_list_count(e, this, true, stream);
        if flag(e, has + 2) {
            let bytes = count.wrapping_mul(2);
            let list = e.call(NI_ALLOC, &args![bytes]).u32();
            e.mem.set_u32(base + PART_TRI_LIST, list);
            read(e, stream, list, bytes, 2);
        }
        read(e, stream, has + 3, 1, 1);
        if e.mem.u8(has + 3) != 0 {
            let palette = e.call(NI_ALLOC, &args![weight_count]).u32();
            e.mem.set_u32(base + PART_BONE_PALETTE, palette);
            read(e, stream, palette, weight_count, 1);
        }
    });
}

// Translated from 00a830d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::Partition::SaveBinary` (by its use): the five counts,
/// the bones, then for the vertex map, the weights, the triangle list and
/// the bone palette a has-array byte (whether the array is set) followed
/// by the array when set; the strip lengths go before the triangle list's
/// byte when there are strips.
pub fn fn_00a830d0(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    for offset in [
        PART_VERTICES,
        PART_TRIANGLES,
        PART_BONE_COUNT,
        PART_STRIPS,
        PART_BONES_PER_VERTEX,
    ] {
        write(e, stream, base + offset, 2, 2);
    }
    let bones = e.mem.u32(base + PART_BONES);
    write(e, stream, bones, u16_at(e, base + PART_BONE_COUNT) * 2, 2);
    e.with_stack(4, |e, flags| {
        let has = flags.addr();
        let flag = |e: &mut Engine, at: u32, field: u32| {
            let set = e.mem.u32(base + field) != 0;
            e.mem.set_u8(at, u8::from(set));
            write(e, stream, at, 1, 1);
            e.mem.u8(at) != 0
        };
        if flag(e, has, PART_VERTEX_MAP) {
            let map = e.mem.u32(base + PART_VERTEX_MAP);
            write(e, stream, map, u16_at(e, base + PART_VERTICES) * 2, 2);
        }
        let weight_count =
            u16_at(e, base + PART_BONES_PER_VERTEX).wrapping_mul(u16_at(e, base + PART_VERTICES));
        if flag(e, has + 1, PART_WEIGHTS) {
            let weights = e.mem.u32(base + PART_WEIGHTS);
            write(e, stream, weights, weight_count.wrapping_mul(4), 4);
        }
        let count = tri_list_count(e, this, false, stream);
        if flag(e, has + 2, PART_TRI_LIST) {
            let list = e.mem.u32(base + PART_TRI_LIST);
            write(e, stream, list, count.wrapping_mul(2), 2);
        }
        if flag(e, has + 3, PART_BONE_PALETTE) {
            let palette = e.mem.u32(base + PART_BONE_PALETTE);
            write(e, stream, palette, weight_count, 1);
        }
    });
}

// Translated from 00a83c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::LoadBinary` (Xbox PDB): the `NiObject` part, the
/// partition count, the partitions array (a `MemoryManager` block with the
/// count word in front, the `Partition`s built by the vector constructor;
/// all ones on overflow) and each partition's `LoadBinary`.
pub fn ni_skin_partition_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    read(e, stream, base + PARTITIONS, 4, 4);
    let count = e.mem.u32(base + PARTITIONS);
    let bytes = count
        .checked_mul(PARTITION_SIZE)
        .unwrap_or(0xffff_ffff)
        .checked_add(4)
        .unwrap_or(0xffff_ffff);
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32();
    let parts = if block != 0 {
        e.mem.set_u32(block, count);
        e.call(
            VECTOR_CONSTRUCTOR,
            &args![block + 4, PARTITION_SIZE, count, PARTITION_CONSTRUCT],
        );
        block + 4
    } else {
        0
    };
    e.mem.set_u32(base + PARTITION_ARRAY, parts);
    let mut index = 0u32;
    while index < e.mem.u32(base + PARTITIONS) {
        let part = e
            .mem
            .u32(base + PARTITION_ARRAY)
            .wrapping_add(index.wrapping_mul(PARTITION_SIZE));
        fn_00a82d90(e, Ptr::new(part), stream);
        index += 1;
    }
}

// Translated from 00a83ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::SaveBinary` (Xbox PDB): the base part, the partition
/// count, then each partition's `SaveBinary`.
pub fn ni_skin_partition_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(NI_OBJECT_SAVE_BINARY, &args![this, stream]);
    write(e, stream, base + PARTITIONS, 4, 4);
    let mut index = 0u32;
    while index < e.mem.u32(base + PARTITIONS) {
        let part = e
            .mem
            .u32(base + PARTITION_ARRAY)
            .wrapping_add(index.wrapping_mul(PARTITION_SIZE));
        fn_00a830d0(e, Ptr::new(part), stream);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a820d0, ni_skin_partition_get_rtti() -> Ptr),
        entry!(0x00a82c70, ni_skin_partition_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a81fb0, ni_skin_partition_create_object() -> Ptr),
        entry!(0x00a81fe0, fn_00a81fe0(Ptr, Ptr) -> u32),
        entry!(0x00a824d0, fn_00a824d0(Ptr)),
        entry!(0x00a81c20, fn_00a81c20(Ptr) -> u32),
        entry!(0x00a82d90, fn_00a82d90(Ptr, Ptr)),
        entry!(0x00a830d0, fn_00a830d0(Ptr, Ptr)),
        entry!(0x00a83c30, ni_skin_partition_load_binary(Ptr, Ptr)),
        entry!(0x00a83ce0, ni_skin_partition_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_skin_partition_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a820d0, &args![]).u32(), NI_SKIN_PARTITION_RTTI);
    }

    #[test]
    fn ni_skin_partition_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a8_24d0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a82c70, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a82c70, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00a8_24d0), vec![vec![this]; 2]);
    }

    fn calls_to_partition(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing_partition(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    type Bytes = std::rc::Rc<std::cell::RefCell<Vec<u8>>>;

    /// A stream whose input reads from `bytes` and whose output appends to
    /// the answered buffer.
    fn stream_with(e: &mut Engine, bytes: Vec<u8>, version: u32) -> (u32, Bytes) {
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 8, 0x0640_0000);
        let source = std::rc::Rc::new(std::cell::RefCell::new(bytes));
        e.register_double(0x0640_0000, move |e, a| {
            let taken: Vec<u8> = source.borrow_mut().drain(..a[2] as usize).collect();
            e.mem.write(a[1], &taken);
            a[2].into_ret()
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, 0x0640_0010);
        let written: Bytes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = written.clone();
        e.register_double(0x0640_0010, move |e, a| {
            sink.borrow_mut().extend(e.mem.bytes(a[1], a[2]));
            a[2].into_ret()
        });
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        (stream, written)
    }

    #[test]
    fn create_object_and_register_streamables() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_OBJECT_CONSTRUCT, nothing_partition);
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 0x101u32.into_ret());
        let object = e.call(0x00a8_1fb0, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_SKIN_PARTITION_VTABLE);
        assert_eq!(e.mem.u32(object + PARTITION_ARRAY), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_1fe0, &args![object, 0x55u32]).u32(), 0x101);
        assert_eq!(
            calls_to_partition(&e, NI_OBJECT_REGISTER_STREAMABLES),
            vec![vec![object, 0x55]]
        );
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_1fb0, &args![]).u32(), 0);
    }

    #[test]
    fn destructor_body_releases_the_partitions() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCT, nothing_partition);
        e.register(MEMORY_MANAGER_DEALLOCATE, nothing_partition);
        e.register(0x0640_0100, nothing_partition);
        e.register(0x0640_0200, nothing_partition);
        e.map(RENDERER, 4);
        let this = e.mem.alloc(0x10);
        let block = e.mem.alloc(4 + 0x2c);
        e.mem.set_u32(this + PARTITION_ARRAY, block + 4);
        e.call_log = Some(vec![]);
        e.call(0x00a8_24d0, &args![this]);
        assert_eq!(e.mem.u32(this), NI_SKIN_PARTITION_VTABLE);
        assert_eq!(
            calls_to_partition(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, block]]
        );
        assert_eq!(calls_to_partition(&e, NI_OBJECT_DESTRUCT), vec![vec![this]]);
        // With a count and a renderer: the vector deleting destructor.
        let renderer_vtable = e.mem.alloc(0x104);
        e.mem.set_u32(renderer_vtable + 0x100, 0x0640_0100);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, renderer_vtable);
        e.set_global(RENDERER, renderer);
        let part_vtable = e.mem.alloc(4);
        e.mem.set_u32(part_vtable, 0x0640_0200);
        e.mem.set_u32(block, 1);
        e.mem.set_u32(block + 4, part_vtable);
        e.call(0x00a8_24d0, &args![this]);
        assert_eq!(
            calls_to_partition(&e, 0x0640_0100),
            vec![vec![renderer, this]]
        );
        assert_eq!(
            calls_to_partition(&e, 0x0640_0200),
            vec![vec![block + 4, 3]]
        );
        assert_eq!(calls_to_partition(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
        assert_eq!(calls_to_partition(&e, NI_OBJECT_DESTRUCT).len(), 2);
    }

    #[test]
    fn strip_lengths_sum() {
        let mut e = Engine::new();
        let part = e.mem.alloc(0x2c);
        let lengths = e.mem.alloc(10);
        for (i, v) in [3u16, 4, 5, 6, 7].iter().enumerate() {
            e.mem.set_u16(lengths + 2 * i as u32, *v);
        }
        e.mem.set_u32(part + PART_STRIP_LENGTHS, lengths);
        for (strips, sum) in [(0u16, 0u32), (1, 3), (2, 7), (3, 12), (5, 25)] {
            e.mem.set_u16(part + PART_STRIPS, strips);
            assert_eq!(e.call(0x00a8_1c20, &args![part]).u32(), sum);
        }
    }

    fn partition_bytes(with_flags: bool) -> Vec<u8> {
        let mut bytes = Vec::new();
        // 2 vertices, 1 triangle, 1 bone, 0 strips, 1 bone per vertex.
        for count in [2u16, 1, 1, 0, 1] {
            bytes.extend_from_slice(&count.to_le_bytes());
        }
        bytes.extend_from_slice(&9u16.to_le_bytes());
        if with_flags {
            bytes.push(1);
        }
        bytes.extend_from_slice(&[5, 0, 6, 0]);
        if with_flags {
            bytes.push(1);
        }
        bytes.extend_from_slice(&1.0f32.to_le_bytes());
        bytes.extend_from_slice(&0.5f32.to_le_bytes());
        if with_flags {
            bytes.push(1);
        }
        bytes.extend_from_slice(&[0, 0, 1, 0, 1, 0]);
        bytes.push(1);
        bytes.extend_from_slice(&[0, 0]);
        bytes
    }

    #[test]
    fn partition_load_binary_reads_every_array() {
        for (version, with_flags) in [(VERSION_10_0_1_18, true), (0x0a00_0111, false)] {
            let mut e = Engine::new();
            e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
            let (stream, _) = stream_with(&mut e, partition_bytes(with_flags), version);
            let part = e.mem.alloc(0x2c);
            e.call(0x00a8_2d90, &args![part, stream]);
            assert_eq!(e.mem.u16(part + PART_VERTICES), 2);
            assert_eq!(e.mem.u16(e.mem.u32(part + PART_BONES)), 9);
            assert_eq!(e.mem.u16(e.mem.u32(part + PART_VERTEX_MAP) + 2), 6);
            assert_eq!(e.mem.f32(e.mem.u32(part + PART_WEIGHTS) + 4), 0.5);
            assert_eq!(e.mem.u16(e.mem.u32(part + PART_TRI_LIST) + 4), 1);
            assert_ne!(e.mem.u32(part + PART_BONE_PALETTE), 0);
            assert_eq!(e.mem.u32(part + PART_STRIP_LENGTHS), 0);
            let input = e.mem.u32(stream + STREAM_INPUT);
            assert_eq!(
                e.mem.u32(input + 4) as usize,
                partition_bytes(with_flags).len()
            );
        }
    }

    #[test]
    fn partition_load_binary_with_strips_and_no_arrays() {
        let mut e = Engine::new();
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
        let mut bytes = Vec::new();
        for count in [3u16, 0, 0, 2, 0] {
            bytes.extend_from_slice(&count.to_le_bytes());
        }
        bytes.extend_from_slice(&[0, 0]);
        bytes.extend_from_slice(&[3, 0, 1, 0]);
        bytes.extend_from_slice(&[1, 7, 0, 8, 0, 9, 0, 1, 0]);
        bytes.push(0);
        let (stream, _) = stream_with(&mut e, bytes, VERSION_10_0_1_18);
        let part = e.mem.alloc(0x2c);
        e.call_log = Some(vec![]);
        e.call(0x00a8_2d90, &args![part, stream]);
        assert_eq!(e.mem.u32(part + PART_VERTEX_MAP), 0);
        assert_eq!(e.mem.u32(part + PART_WEIGHTS), 0);
        let lengths = e.mem.u32(part + PART_STRIP_LENGTHS);
        assert_eq!(e.mem.u16(lengths), 3);
        assert_eq!(e.mem.u16(e.mem.u32(part + PART_TRI_LIST) + 6), 1);
        assert_eq!(e.mem.u32(part + PART_BONE_PALETTE), 0);
        let allocated: Vec<u32> = calls_to_partition(&e, NI_ALLOC)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(allocated, vec![0, 4, 8]);
    }

    #[test]
    fn partition_save_binary_writes_flags_and_arrays() {
        let mut e = Engine::new();
        let (stream, written) = stream_with(&mut e, vec![], VERSION_10_0_1_18);
        let part = e.mem.alloc(0x2c);
        for (offset, count) in [
            (PART_VERTICES, 2u16),
            (PART_TRIANGLES, 1),
            (PART_BONE_COUNT, 1),
            (PART_STRIPS, 0),
            (PART_BONES_PER_VERTEX, 1),
        ] {
            e.mem.set_u16(part + offset, count);
        }
        let bones = e.mem.alloc(2);
        e.mem.set_u16(bones, 9);
        e.mem.set_u32(part + PART_BONES, bones);
        let weights = e.mem.alloc(8);
        e.mem.set_f32(weights, 1.0);
        e.mem.set_f32(weights + 4, 0.5);
        e.mem.set_u32(part + PART_WEIGHTS, weights);
        let list = e.mem.alloc(6);
        e.mem.set_u16(list + 4, 1);
        e.mem.set_u32(part + PART_TRI_LIST, list);
        e.call(0x00a8_30d0, &args![part, stream]);
        let mut expected = Vec::new();
        for count in [2u16, 1, 1, 0, 1] {
            expected.extend_from_slice(&count.to_le_bytes());
        }
        expected.extend_from_slice(&9u16.to_le_bytes());
        expected.push(0);
        expected.push(1);
        expected.extend_from_slice(&1.0f32.to_le_bytes());
        expected.extend_from_slice(&0.5f32.to_le_bytes());
        expected.push(1);
        expected.extend_from_slice(&[0, 0, 0, 0, 1, 0]);
        expected.push(0);
        assert_eq!(*written.borrow(), expected);
    }

    #[test]
    fn skin_partition_load_and_save() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_LOAD_BINARY, nothing_partition);
        e.register(NI_OBJECT_SAVE_BINARY, nothing_partition);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCTOR, nothing_partition);
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
        let mut bytes = vec![1, 0, 0, 0];
        bytes.extend(partition_bytes(true));
        let (stream, written) = stream_with(&mut e, bytes, VERSION_10_0_1_18);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00a8_3c30, &args![this, stream]);
        let parts = e.mem.u32(this + PARTITION_ARRAY);
        assert_eq!(e.mem.u32(parts - 4), 1);
        assert_eq!(
            calls_to_partition(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x30]]
        );
        assert_eq!(
            calls_to_partition(&e, VECTOR_CONSTRUCTOR),
            vec![vec![parts, 0x2c, 1, PARTITION_CONSTRUCT]]
        );
        assert_eq!(e.mem.u16(parts + PART_VERTICES), 2);
        e.call(0x00a8_3ce0, &args![this, stream]);
        assert_eq!(
            calls_to_partition(&e, NI_OBJECT_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        let written = written.borrow();
        assert_eq!(written[..4], [1, 0, 0, 0]);
        assert_eq!(written.len(), 4 + partition_bytes(true).len());
    }
}
