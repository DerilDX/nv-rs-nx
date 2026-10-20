//! `NiMain/niadditionalgeometrydata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiAdditionalGeometryData` (Xbox PDB, 0x2c bytes; the PC code's
//! offsets): `m_usVertexCount` +0x0c (u16), `m_uiDataStreamCount` +0x10,
//! `m_pkDataStreamEntries` +0x14 (`NiAGDDataStream`, 0x1c bytes each),
//! `m_uiD3DStream` +0x18 and `m_aDataBlocks` +0x1c (an
//! `NiTPrimitiveArray<NiAGDDataBlock*>`: vtable, `m_pBase` +0x20,
//! `m_usMaxSize` +0x24, `m_usSize` +0x26, `m_usESize` +0x28, `m_usGrowBy`
//! +0x2a). `NiAGDDataStream`: `m_uFlags` +0 (a byte), `m_uiType` +4,
//! `m_uiUnitSize` +8, `m_uiTotalSize` +0xc, `m_uiStride` +0x10,
//! `m_uiBlockIndex` +0x14, `m_uiBlockOffset` +0x18. `NiAGDDataBlock` (0x10
//! bytes): vtable, `m_uiDataBlockSize` +4, `m_pucDataBlock` +8,
//! `m_bOwnData` +0xc, `m_bBSHeap` +0xd; its virtual slots +4
//! `AllocateBuffer`, +8 `ProtectBuffer`, +0x10 `FreeBuffer`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAdditionalGeometryData` fields.
const VERTEX_COUNT: u32 = 0x0c;
const STREAM_COUNT: u32 = 0x10;
const STREAMS: u32 = 0x14;
const D3D_STREAM: u32 = 0x18;
const BLOCKS: u32 = 0x1c;
const BLOCKS_BASE: u32 = 0x20;
const BLOCKS_MAX_SIZE: u32 = 0x24;
const BLOCKS_SIZE: u32 = 0x26;
const BLOCKS_E_SIZE: u32 = 0x28;
const BLOCKS_GROW_BY: u32 = 0x2a;
/// `NiAGDDataStream` fields and size.
const STREAM_FLAGS: u32 = 0x00;
const STREAM_TYPE: u32 = 0x04;
const STREAM_UNIT_SIZE: u32 = 0x08;
const STREAM_TOTAL_SIZE: u32 = 0x0c;
const STREAM_STRIDE: u32 = 0x10;
const STREAM_BLOCK_INDEX: u32 = 0x14;
const STREAM_BLOCK_OFFSET: u32 = 0x18;
const STREAM_SIZE: u32 = 0x1c;
/// `NiAGDDataBlock` fields, size and virtual slots.
const BLOCK_DATA_SIZE: u32 = 0x04;
const BLOCK_DATA: u32 = 0x08;
const BLOCK_OWN_DATA: u32 = 0x0c;
const BLOCK_BS_HEAP: u32 = 0x0d;
const BLOCK_OBJECT_SIZE: u32 = 0x10;
const BLOCK_SLOT_DELETE: u32 = 0x00;
const BLOCK_SLOT_ALLOCATE: u32 = 0x04;
const BLOCK_SLOT_PROTECT: u32 = 0x08;
const BLOCK_SLOT_FREE: u32 = 0x10;
/// The data object's virtual slot +0x90 (`CreateAGDDataBlock`, by the
/// function at that address).
const SLOT_CREATE_BLOCK: u32 = 0x90;
/// Vtables: `NiAdditionalGeometryData`, `NiAGDDataBlock`,
/// `NiTPrimitiveArray<NiAGDDataBlock*>` and its base
/// `NiTArray<NiAGDDataBlock*,NiTMallocInterface<..>>` (RTTI names).
const NI_ADDITIONAL_GEOMETRY_DATA_VTABLE: u32 = 0x0102_e274;
const NI_AGD_DATA_BLOCK_VTABLE: u32 = 0x0109_cf58;
const BLOCK_ARRAY_VTABLE: u32 = 0x0102_e474;
const BLOCK_ARRAY_BASE_VTABLE: u32 = 0x0102_e46c;
/// The memory manager (`011f6238`), its `Allocate` (`00aa3e40`, thiscall,
/// the size) and `Deallocate` (`00aa4060`, thiscall, the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// `NiNew`'s allocator (`00aa13e0`, cdecl) and the free of `NiTArray`
/// storage (`00aa10f0`, cdecl).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE: u32 = 0x00aa_10f0;
/// The `NiObject` constructor (`00a5d3a0`), destructor (`00a5d3d0`) and
/// `LoadBinary` (`00a5d610`, thiscall, the stream).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// The block array's `SetSize` (`00a90180`, thiscall, the size) and
/// `SetAt` (`00ba3aa0`, thiscall: index, the element's address).
const BLOCK_ARRAY_SET_SIZE: u32 = 0x00a9_0180;
const BLOCK_ARRAY_SET_AT: u32 = 0x00ba_3aa0;
/// The vector constructor iterator (`00401050`, stdcall: array, element
/// size, count, constructor) and the stream entry's constructor
/// (`00639940`).
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
const STREAM_CONSTRUCT: u32 = 0x0063_9940;
/// `_memcpy` (`00ec44d0`, cdecl).
const MEMCPY: u32 = 0x00ec_44d0;
/// The stream's file (`NiStream` +0x24c) and, in the file, its read
/// function (+8, cdecl: file, destination, size, element sizes, element
/// count; returns the bytes read) and its position (+4, raised by them).
const STREAM_FILE: u32 = 0x24c;
const FILE_READ: u32 = 0x08;
const FILE_POSITION: u32 = 0x04;

/// The stream file's read: `size` bytes into `dest`, the element sizes at
/// `sizes` (`count` of them); the bytes read are added to the file's
/// position.
fn file_read(e: &mut Engine, stream: u32, dest: u32, size: u32, sizes: u32, count: u32) {
    let file = e.mem.u32(stream.wrapping_add(STREAM_FILE));
    let read = e.mem.u32(file.wrapping_add(FILE_READ));
    let got = e.call(read, &args![file, dest, size, sizes, count]).u32();
    let position = e
        .mem
        .u32(file.wrapping_add(FILE_POSITION))
        .wrapping_add(got);
    e.mem.set_u32(file.wrapping_add(FILE_POSITION), position);
}

/// [`file_read`] of one element of `size` bytes, its size written to the
/// stack word `sizes` first.
fn file_read_one(e: &mut Engine, stream: u32, dest: u32, size: u32, sizes: u32) {
    e.mem.set_u32(sizes, size);
    file_read(e, stream, dest, size, sizes, 1);
}

/// `count * 0x1c`, or `0xffffffff` when the product overflows (`MUL`,
/// `SETO`, `NEG`, `OR`).
fn stream_array_bytes(count: u32) -> u32 {
    let product = u64::from(count) * u64::from(STREAM_SIZE);
    if product >> 32 != 0 {
        u32::MAX
    } else {
        product as u32
    }
}

/// A new array of `count` stream entries from the memory manager, each
/// constructed (`00401050` with `00639940`); null when the allocation
/// fails.
fn new_stream_array(e: &mut Engine, count: u32) -> u32 {
    let array = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER, stream_array_bytes(count)],
        )
        .u32();
    if array == 0 {
        return 0;
    }
    e.call(
        VECTOR_CONSTRUCTOR,
        &args![array, STREAM_SIZE, count, STREAM_CONSTRUCT],
    );
    array
}

// Translated from 00a72510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::NiAGDDataBlock::AllocateBuffer` (engine map;
/// thiscall, `RET 4`, `this` unused): `size` bytes from the memory manager
/// (`00aa3e40`), or null for a zero size.
pub fn ni_agd_data_block_allocate_buffer(e: &mut Engine, _this: Ptr, size: u32) -> u32 {
    if size == 0 {
        return 0;
    }
    e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32()
}

// Translated from 00a72530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::NiAGDDataBlock::FreeBuffer` (engine map;
/// thiscall, `RET 4`): gives `buffer` back to the memory manager
/// (`00aa4060`); a null `buffer` means the block's own data when the block
/// owns it (`m_bOwnData`), else null is given back.
pub fn ni_agd_data_block_free_buffer(e: &mut Engine, this: Ptr, buffer: u32) {
    let mut block = buffer;
    if block == 0 && e.mem.u8(this.addr() + BLOCK_OWN_DATA) != 0 {
        block = e.mem.u32(this.addr() + BLOCK_DATA);
    }
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, block]);
}

// Translated from 00a72600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes data block `index` (thiscall, `RET 8`; unnamed in the engine
/// map): false when the index is past the array's size or the slot is
/// empty. Otherwise the block frees its buffer (slot +0x10 with null) and
/// is deleted (slot 0 with 1), the array's slot is set to null
/// (`00ba3aa0` with the address of a null word) and, when `clear_streams`
/// is non-zero, every stream entry of that block is cleared (its words at
/// +4 to +0x18). Returns true.
pub fn fn_00a72600(e: &mut Engine, this: Ptr, index: u32, clear_streams: u8) -> bool {
    let base = this.addr();
    if index >= u32::from(e.mem.u16(base + BLOCKS_SIZE)) {
        return false;
    }
    let blocks = e.mem.u32(base + BLOCKS_BASE);
    let block = e.mem.u32(blocks.wrapping_add(index.wrapping_mul(4)));
    if block == 0 {
        return false;
    }
    e.vcall(block, BLOCK_SLOT_FREE, &args![0u32]);
    e.vcall(block, BLOCK_SLOT_DELETE, &args![1u32]);
    e.with_stack(4, |e, null| {
        e.mem.set_u32(null.addr(), 0);
        e.call(BLOCK_ARRAY_SET_AT, &args![base + BLOCKS, index, null]);
    });
    if clear_streams != 0 {
        let mut count = e.mem.u32(base + STREAM_COUNT);
        let mut i = 0u32;
        let mut offset = 0u32;
        if count != 0 {
            loop {
                let streams = e.mem.u32(base + STREAMS);
                let entry = streams.wrapping_add(offset);
                if index == e.mem.u32(entry.wrapping_add(STREAM_BLOCK_INDEX))
                    && i < count
                    && streams != 0
                {
                    for field in [
                        STREAM_TYPE,
                        STREAM_TOTAL_SIZE,
                        STREAM_UNIT_SIZE,
                        STREAM_STRIDE,
                        STREAM_BLOCK_INDEX,
                        STREAM_BLOCK_OFFSET,
                    ] {
                        e.mem.set_u32(entry + field, 0);
                    }
                }
                count = e.mem.u32(base + STREAM_COUNT);
                i = i.wrapping_add(1);
                offset = offset.wrapping_add(STREAM_SIZE);
                if i >= count {
                    break;
                }
            }
        }
    }
    true
}

// Translated from 00a726a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::SetDataStream` (engine map; thiscall,
/// `RET 0x1c`): fills stream entry `stream` (type, block index and offset,
/// stride, unit size, total size `count * unit_size`). False when `count`
/// is not the vertex count, the stream is past the count or there are no
/// entries, or the block index is past the block array's size; else true.
pub fn ni_additional_geometry_data_set_data_stream(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
    block_index: u32,
    block_offset: u32,
    data_type: u32,
    count: u16,
    unit_size: u32,
    stride: u32,
) -> bool {
    let base = this.addr();
    if count != e.mem.u16(base + VERTEX_COUNT) {
        return false;
    }
    if stream >= e.mem.u32(base + STREAM_COUNT) {
        return false;
    }
    let streams = e.mem.u32(base + STREAMS);
    if streams == 0 {
        return false;
    }
    if block_index > u32::from(e.mem.u16(base + BLOCKS_SIZE)) {
        return false;
    }
    let entry = streams.wrapping_add(stream.wrapping_mul(STREAM_SIZE));
    e.mem.set_u32(entry + STREAM_TYPE, data_type);
    e.mem.set_u32(entry + STREAM_BLOCK_INDEX, block_index);
    e.mem.set_u32(entry + STREAM_BLOCK_OFFSET, block_offset);
    e.mem.set_u32(entry + STREAM_STRIDE, stride);
    e.mem.set_u32(entry + STREAM_UNIT_SIZE, unit_size);
    e.mem.set_u32(
        entry + STREAM_TOTAL_SIZE,
        u32::from(count).wrapping_mul(unit_size),
    );
    true
}

// Translated from 00a72720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads stream entry `stream` out (thiscall, `RET 0x1c`; unnamed in the
/// engine map): false when the stream is past the count or there are no
/// entries. The type is written always; with a non-zero type also the
/// vertex count (u16), total size, unit size and stride; then, when the
/// block index is not past the array's size (`<=`), the block exists and
/// its size is at least the offset, `*data` is the block's data plus the
/// offset and the answer true; otherwise false.
pub fn fn_00a72720(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
    data: Ptr,
    data_type: Ptr,
    count: Ptr,
    total_size: Ptr,
    unit_size: Ptr,
    stride: Ptr,
) -> bool {
    let base = this.addr();
    if stream >= e.mem.u32(base + STREAM_COUNT) {
        return false;
    }
    let streams = e.mem.u32(base + STREAMS);
    if streams == 0 {
        return false;
    }
    let entry = streams.wrapping_add(stream.wrapping_mul(STREAM_SIZE));
    let kind = e.mem.u32(entry.wrapping_add(STREAM_TYPE));
    e.mem.set_u32(data_type.addr(), kind);
    if kind == 0 {
        return false;
    }
    let vertices = e.mem.u16(base + VERTEX_COUNT);
    e.mem.set_u16(count.addr(), vertices);
    let total = e.mem.u32(entry + STREAM_TOTAL_SIZE);
    e.mem.set_u32(total_size.addr(), total);
    let unit = e.mem.u32(entry + STREAM_UNIT_SIZE);
    e.mem.set_u32(unit_size.addr(), unit);
    let step = e.mem.u32(entry + STREAM_STRIDE);
    e.mem.set_u32(stride.addr(), step);
    let block_index = e.mem.u32(entry + STREAM_BLOCK_INDEX);
    if block_index > u32::from(e.mem.u16(base + BLOCKS_SIZE)) {
        return false;
    }
    let blocks = e.mem.u32(base + BLOCKS_BASE);
    let block = e.mem.u32(blocks.wrapping_add(block_index.wrapping_mul(4)));
    if block == 0 {
        return false;
    }
    let offset = e.mem.u32(entry + STREAM_BLOCK_OFFSET);
    if e.mem.u32(block + BLOCK_DATA_SIZE) < offset {
        return false;
    }
    let pointer = e.mem.u32(block + BLOCK_DATA).wrapping_add(offset);
    e.mem.set_u32(data.addr(), pointer);
    true
}

// Translated from 00a727c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::CreateAGDDataBlock` (engine map): a new
/// 0x10-byte `NiAGDDataBlock` from the memory manager (its vtable, no size,
/// no data, not owning; the byte at +0xd left as allocated), or null.
pub fn ni_additional_geometry_data_create_agd_data_block(e: &mut Engine) -> Ptr {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER, BLOCK_OBJECT_SIZE],
        )
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.mem.set_u32(block, NI_AGD_DATA_BLOCK_VTABLE);
    e.mem.set_u32(block + BLOCK_DATA_SIZE, 0);
    e.mem.set_u32(block + BLOCK_DATA, 0);
    e.mem.set_u8(block + BLOCK_OWN_DATA, 0);
    Ptr::new(block)
}

// Translated from 00a72bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::SetDataBlock` (engine map; thiscall,
/// `RET 0x10`): puts `size` bytes of `data` in block `index`. An existing
/// block of another size frees its buffer first (slot +0x10 with null, the
/// data pointer cleared); a missing one is created (slot +0x90; false when
/// that fails). A block without data gets, when `copy` is non-zero, its own
/// buffer (`m_bBSHeap` set when the vertex count is below 0x40; slot +4,
/// `data` copied in when non-null, slot +8 with the buffer and 0), else
/// `data` itself; and the size. The block array grows to `index + growBy`
/// when `index` is not below its maximum size, the block is stored at
/// `index` (`00ba3aa0`) and `m_bOwnData` becomes `copy`. Returns true.
pub fn ni_additional_geometry_data_set_data_block(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    data: u32,
    size: u32,
    copy: u8,
) -> bool {
    let base = this.addr();
    let mut block = 0u32;
    if index < u32::from(e.mem.u16(base + BLOCKS_SIZE)) {
        let blocks = e.mem.u32(base + BLOCKS_BASE);
        block = e.mem.u32(blocks.wrapping_add(index.wrapping_mul(4)));
    }
    if block == 0 {
        block = e.vcall(base, SLOT_CREATE_BLOCK, &args![]).u32();
        if block == 0 {
            return false;
        }
    } else if e.mem.u32(block + BLOCK_DATA_SIZE) != size {
        e.vcall(block, BLOCK_SLOT_FREE, &args![0u32]);
        e.mem.set_u32(block + BLOCK_DATA, 0);
    }
    if e.mem.u32(block + BLOCK_DATA) == 0 {
        let buffer = if copy != 0 {
            let small = e.mem.u16(base + VERTEX_COUNT) < 0x40;
            e.mem.set_u8(block + BLOCK_BS_HEAP, u8::from(small));
            let buffer = e.vcall(block, BLOCK_SLOT_ALLOCATE, &args![size]).u32();
            if data != 0 {
                e.call(MEMCPY, &args![buffer, data, size]);
            }
            e.vcall(block, BLOCK_SLOT_PROTECT, &args![buffer, 0u32]);
            buffer
        } else {
            data
        };
        e.mem.set_u32(block + BLOCK_DATA_SIZE, size);
        e.mem.set_u32(block + BLOCK_DATA, buffer);
    }
    if index >= u32::from(e.mem.u16(base + BLOCKS_MAX_SIZE)) {
        let grow_by = u32::from(e.mem.u16(base + BLOCKS_GROW_BY));
        e.call(
            BLOCK_ARRAY_SET_SIZE,
            &args![base + BLOCKS, grow_by.wrapping_add(index)],
        );
    }
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), block);
        e.call(BLOCK_ARRAY_SET_AT, &args![base + BLOCKS, index, element]);
    });
    e.mem.set_u8(block + BLOCK_OWN_DATA, copy);
    true
}

// Translated from 00a72ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::SetDataBlockCount` (engine map): the block
/// array's `SetSize(count)` (`00a90180` on +0x1c).
pub fn ni_additional_geometry_data_set_data_block_count(e: &mut Engine, this: Ptr, count: u32) {
    e.call(BLOCK_ARRAY_SET_SIZE, &args![this.addr() + BLOCKS, count]);
}

// Translated from 00a72cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a stream entry from `stream` (thiscall on the entry, `RET 4`;
/// unnamed in the engine map): the words at +4, +8, +0xc, +0x10, +0x14 and
/// +0x18, then the flag byte at +0, each one element read through the
/// stream file's read function (the element size in the argument's stack
/// word).
pub fn fn_00a72cb0(e: &mut Engine, this: Ptr, stream: u32) {
    let entry = this.addr();
    e.with_stack(4, |e, sizes| {
        for field in [
            STREAM_TYPE,
            STREAM_UNIT_SIZE,
            STREAM_TOTAL_SIZE,
            STREAM_STRIDE,
            STREAM_BLOCK_INDEX,
            STREAM_BLOCK_OFFSET,
        ] {
            file_read_one(e, stream, entry + field, 4, sizes.addr());
        }
        file_read_one(e, stream, entry + STREAM_FLAGS, 1, sizes.addr());
    });
}

// Translated from 00a72eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a data block from `stream` (thiscall on the block, `RET 0x14`;
/// unnamed in the engine map; the second to fourth words are not read):
/// its size, a buffer (`m_bBSHeap` set when `vertex_count` is below 0x40,
/// slot +4; `m_bOwnData` set), then a count of parts and their offsets;
/// for each part a count of element sizes and the sizes, and the part's
/// bytes (the sum of the sizes times `vertex_count`) read at the buffer
/// plus its offset with those element sizes. Finally slot +8 with (0, 0).
/// The offsets and sizes live in two 32-word stack arrays, as in the exe.
pub fn fn_00a72eb0(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    vertex_count: u16,
) {
    let block = this.addr();
    // The exe's frame from [esp+0x10] to [esp+0x128].
    e.with_stack(0x118, |e, frame| {
        let at = |offset: u32| frame.addr() + offset - 0x10;
        let index_word = at(0x10);
        let sizes_word = at(0x14);
        let size_count = at(0x18);
        let part_count = at(0x1c);
        let odd_sum = at(0x20);
        let sizes = at(0x28);
        let offsets = at(0xa8);
        file_read_one(e, stream, block + BLOCK_DATA_SIZE, 4, index_word);
        let small = vertex_count < 0x40;
        let data_size = e.mem.u32(block + BLOCK_DATA_SIZE);
        e.mem.set_u8(block + BLOCK_BS_HEAP, u8::from(small));
        let buffer = e.vcall(block, BLOCK_SLOT_ALLOCATE, &args![data_size]).u32();
        e.mem.set_u32(block + BLOCK_DATA, buffer);
        e.mem.set_u8(block + BLOCK_OWN_DATA, 1);
        file_read_one(e, stream, part_count, 4, index_word);
        let parts = e.mem.u32(part_count);
        e.mem.set_u32(index_word, 4);
        file_read(e, stream, offsets, parts.wrapping_mul(4), index_word, 1);
        e.mem.set_u32(index_word, 0);
        if e.mem.u32(part_count) != 0 {
            let multiplier = u32::from(vertex_count);
            loop {
                e.mem.set_u32(odd_sum, 0);
                file_read_one(e, stream, size_count, 4, sizes_word);
                let count = e.mem.u32(size_count);
                e.mem.set_u32(sizes_word, 4);
                file_read(e, stream, sizes, count.wrapping_mul(4), sizes_word, 1);
                let part = e.mem.u32(index_word);
                let offset = e.mem.u32(offsets.wrapping_add(part.wrapping_mul(4)));
                let destination = offset.wrapping_add(e.mem.u32(block + BLOCK_DATA));
                let count = e.mem.u32(size_count);
                let mut k = 0u32;
                let mut even_sum = 0u32;
                e.mem.set_u32(sizes_word, 0);
                if (count as i32) >= 2 {
                    loop {
                        let odd = e
                            .mem
                            .u32(sizes.wrapping_add(k.wrapping_add(1).wrapping_mul(4)));
                        let sum = e.mem.u32(odd_sum).wrapping_add(odd);
                        e.mem.set_u32(odd_sum, sum);
                        even_sum =
                            even_sum.wrapping_add(e.mem.u32(sizes.wrapping_add(k.wrapping_mul(4))));
                        k = k.wrapping_add(2);
                        if k >= count.wrapping_sub(1) {
                            break;
                        }
                    }
                }
                let tail = if k < count {
                    e.mem.u32(sizes.wrapping_add(k.wrapping_mul(4)))
                } else {
                    e.mem.u32(sizes_word)
                };
                let total = e
                    .mem
                    .u32(odd_sum)
                    .wrapping_add(even_sum)
                    .wrapping_add(tail)
                    .wrapping_mul(multiplier);
                file_read(e, stream, destination, total, sizes, count);
                let next = e.mem.u32(index_word).wrapping_add(1);
                e.mem.set_u32(index_word, next);
                if next >= e.mem.u32(part_count) {
                    break;
                }
            }
        }
    });
    e.vcall(block, BLOCK_SLOT_PROTECT, &args![0u32, 0u32]);
}

// Translated from 00a73080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData`'s destructor body (by the vtable it stores;
/// unnamed in the engine map): every block removed (`00a72600(i, 1)` for
/// each index below the array's size), the stream entries given back to the
/// memory manager when present, the block array's storage freed
/// (`00aa10f0`, after its base vtable), then `NiObject`'s destructor.
pub fn fn_00a73080(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_ADDITIONAL_GEOMETRY_DATA_VTABLE);
    if e.mem.u16(base + BLOCKS_SIZE) != 0 {
        let mut i = 0u32;
        loop {
            fn_00a72600(e, this, i, 1);
            i = i.wrapping_add(1);
            if i >= u32::from(e.mem.u16(base + BLOCKS_SIZE)) {
                break;
            }
        }
    }
    let streams = e.mem.u32(base + STREAMS);
    if streams != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, streams]);
    }
    let storage = e.mem.u32(base + BLOCKS_BASE);
    e.mem.set_u32(base + BLOCKS, BLOCK_ARRAY_BASE_VTABLE);
    e.call(NI_FREE, &args![storage]);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a730e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::SetDataStreamCount` (engine map; thiscall,
/// `RET 4`): a new array of `count` constructed stream entries (null when
/// the allocation fails), the old entries copied over (the old count of entries) and
/// given back, then the count.
pub fn ni_additional_geometry_data_set_data_stream_count(e: &mut Engine, this: Ptr, count: u32) {
    let base = this.addr();
    let old = e.mem.u32(base + STREAMS);
    let new = new_stream_array(e, count);
    e.mem.set_u32(base + STREAMS, new);
    if old != 0 {
        let bytes = e.mem.u32(base + STREAM_COUNT).wrapping_mul(STREAM_SIZE);
        e.call(MEMCPY, &args![new, old, bytes]);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, old]);
    }
    e.mem.set_u32(base + STREAM_COUNT, count);
}

// Translated from 00a73160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::LoadBinary` (engine map; thiscall, `RET 4`):
/// `NiObject::LoadBinary`, the vertex count (2 bytes) and the stream count
/// (4 bytes); with streams, a new array of entries each loaded
/// (`00a72cb0`); then the block count, the block array sized to it
/// (`00a90180`) and for each block a flag byte: a set flag creates a block
/// (slot +0x90) and loads it (`00a72eb0(stream, i, entries, stream count,
/// vertex count)`), a clear one stores null. Storing grows the array's size
/// to `i + 1` when needed and keeps its element count (`m_usESize`) in step
/// with the non-null slots.
pub fn ni_additional_geometry_data_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let base = this.addr();
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(0xc, |e, frame| {
        let flag_word = frame.addr();
        let block_count = frame.addr() + 4;
        let sizes = frame.addr() + 8;
        file_read_one(e, stream, base + VERTEX_COUNT, 2, sizes);
        file_read_one(e, stream, base + STREAM_COUNT, 4, sizes);
        let count = e.mem.u32(base + STREAM_COUNT);
        if count != 0 {
            let entries = new_stream_array(e, count);
            e.mem.set_u32(sizes, entries);
            e.mem.set_u32(base + STREAMS, entries);
            if e.mem.u32(base + STREAM_COUNT) != 0 {
                let mut j = 0u32;
                e.mem.set_u32(sizes, 0);
                loop {
                    let entry = e.mem.u32(base + STREAMS).wrapping_add(e.mem.u32(sizes));
                    fn_00a72cb0(e, Ptr::new(entry), stream);
                    let next = e.mem.u32(sizes).wrapping_add(STREAM_SIZE);
                    e.mem.set_u32(sizes, next);
                    j = j.wrapping_add(1);
                    if j >= e.mem.u32(base + STREAM_COUNT) {
                        break;
                    }
                }
            }
        }
        e.mem.set_u32(block_count, 0);
        file_read_one(e, stream, block_count, 4, sizes);
        let blocks = e.mem.u32(block_count);
        e.call(BLOCK_ARRAY_SET_SIZE, &args![base + BLOCKS, blocks]);
        let mut i = 0u32;
        while i < e.mem.u32(block_count) {
            // The flag is the top byte of the first frame word.
            file_read_one(e, stream, flag_word + 3, 1, sizes);
            let slot_of = |e: &Engine, i: u32| {
                e.mem
                    .u32(base + BLOCKS_BASE)
                    .wrapping_add(i.wrapping_mul(4))
            };
            let adjust_count = |e: &mut Engine, delta: u16| {
                let value = e.mem.u16(base + BLOCKS_E_SIZE).wrapping_add(delta);
                e.mem.set_u16(base + BLOCKS_E_SIZE, value);
            };
            if e.mem.u8(flag_word + 3) != 0 {
                let block = e.vcall(base, SLOT_CREATE_BLOCK, &args![]).u32();
                let streams = e.mem.u32(base + STREAMS);
                let stream_count = e.mem.u32(base + STREAM_COUNT);
                let vertices = e.mem.u16(base + VERTEX_COUNT);
                fn_00a72eb0(
                    e,
                    Ptr::new(block),
                    stream,
                    i,
                    streams,
                    stream_count,
                    vertices,
                );
                if i >= u32::from(e.mem.u16(base + BLOCKS_SIZE)) {
                    e.mem.set_u16(base + BLOCKS_SIZE, i.wrapping_add(1) as u16);
                    if block != 0 {
                        adjust_count(e, 1);
                    }
                } else if block != 0 {
                    if e.mem.u32(slot_of(e, i)) == 0 {
                        adjust_count(e, 1);
                    }
                } else if e.mem.u32(slot_of(e, i)) != 0 {
                    adjust_count(e, 0xffff);
                }
                let slot = slot_of(e, i);
                e.mem.set_u32(slot, block);
            } else {
                if i >= u32::from(e.mem.u16(base + BLOCKS_SIZE)) {
                    e.mem.set_u16(base + BLOCKS_SIZE, i.wrapping_add(1) as u16);
                } else if e.mem.u32(slot_of(e, i)) != 0 {
                    adjust_count(e, 0xffff);
                }
                let slot = slot_of(e, i);
                e.mem.set_u32(slot, 0);
            }
            i = i.wrapping_add(1);
        }
    });
}

// Translated from 00a734c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData`'s constructor (by the vtable it stores;
/// unnamed in the engine map): `NiObject`'s constructor, the vtable, an
/// empty block array (its vtable, no storage, sizes 0, grow-by 1; the
/// exe's clearing loop over the just-zeroed size runs no time), no revision
/// id, streams or D3D stream and a zero vertex count. Returns `this`.
pub fn fn_00a734c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, NI_ADDITIONAL_GEOMETRY_DATA_VTABLE);
    e.mem.set_u16(base + BLOCKS_GROW_BY, 1);
    e.mem.set_u16(base + BLOCKS_MAX_SIZE, 0);
    e.mem.set_u16(base + BLOCKS_SIZE, 0);
    e.mem.set_u16(base + BLOCKS_E_SIZE, 0);
    e.mem.set_u32(base + BLOCKS_BASE, 0);
    e.mem.set_u32(base + BLOCKS, BLOCK_ARRAY_VTABLE);
    e.mem.set_u32(base + 0x08, 0);
    e.mem.set_u32(base + STREAM_COUNT, 0);
    e.mem.set_u32(base + STREAMS, 0);
    e.mem.set_u16(base + VERTEX_COUNT, 0);
    let mut i: u16 = 0;
    while i < e.mem.u16(base + BLOCKS_SIZE) {
        let storage = e.mem.u32(base + BLOCKS_BASE);
        e.mem.set_u32(storage.wrapping_add(u32::from(i) * 4), 0);
        i = i.wrapping_add(1);
    }
    e.mem.set_u16(base + BLOCKS_SIZE, 0);
    e.mem.set_u16(base + BLOCKS_E_SIZE, 0);
    e.mem.set_u32(base + D3D_STREAM, 0);
    this
}

// Translated from 00a73e20 (disassembled, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `NiAdditionalGeometryData::CreateObject` (by the constructor it runs;
/// the code is not a Ghidra function): 0x2c bytes (`00aa13e0`), constructed
/// (`00a734c0`); null when the allocation fails.
pub fn ni_additional_geometry_data_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x2cu32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    fn_00a734c0(e, Ptr::new(object))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a7_2510,
            ni_agd_data_block_allocate_buffer(Ptr, u32) -> u32
        ),
        entry!(0x00a7_2530, ni_agd_data_block_free_buffer(Ptr, u32)),
        entry!(0x00a7_2600, fn_00a72600(Ptr, u32, u8) -> bool),
        entry!(
            0x00a7_26a0,
            ni_additional_geometry_data_set_data_stream(Ptr, u32, u32, u32, u32, u16, u32, u32) -> bool
        ),
        entry!(
            0x00a7_2720,
            fn_00a72720(Ptr, u32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a7_27c0,
            ni_additional_geometry_data_create_agd_data_block() -> Ptr
        ),
        entry!(
            0x00a7_2bb0,
            ni_additional_geometry_data_set_data_block(Ptr, u32, u32, u32, u8) -> bool
        ),
        entry!(
            0x00a7_2ca0,
            ni_additional_geometry_data_set_data_block_count(Ptr, u32)
        ),
        entry!(0x00a7_2cb0, fn_00a72cb0(Ptr, u32)),
        entry!(0x00a7_2eb0, fn_00a72eb0(Ptr, u32, u32, u32, u32, u16)),
        entry!(0x00a7_3080, fn_00a73080(Ptr)),
        entry!(
            0x00a7_30e0,
            ni_additional_geometry_data_set_data_stream_count(Ptr, u32)
        ),
        entry!(
            0x00a7_3160,
            ni_additional_geometry_data_load_binary(Ptr, u32)
        ),
        entry!(0x00a7_34c0, fn_00a734c0(Ptr) -> Ptr),
        entry!(
            0x00a7_3e20,
            ni_additional_geometry_data_create_object() -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The test file's read function (registered at this address): copies
    /// `size` bytes from the file's source (+0xc) at its position (+4) and
    /// answers `size`; the translation moves the position.
    const READ: u32 = 0x005a_0000;
    /// Vtables of the test objects: the data object (slot +0x90 creates a
    /// block) and the blocks (slots 0, +4, +8, +0x10).
    const DATA_VTABLE: u32 = 0x0714_0000;
    const BLOCK_VTABLE: u32 = 0x0715_0000;
    const CREATE_BLOCK: u32 = 0x005b_0000;
    const BLOCK_DELETE: u32 = 0x005b_1000;
    const BLOCK_ALLOCATE: u32 = 0x005b_2000;
    const BLOCK_PROTECT: u32 = 0x005b_3000;
    const BLOCK_FREE: u32 = 0x005b_4000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            assert_eq!(a[0], MEMORY_MANAGER);
            e.mem.alloc(a[1]).into_ret()
        });
        quiet(
            &mut e,
            &[
                MEMORY_MANAGER_FREE,
                VECTOR_CONSTRUCTOR,
                BLOCK_DELETE,
                BLOCK_PROTECT,
                BLOCK_FREE,
                NI_FREE,
                NI_OBJECT_DESTRUCT,
                NI_OBJECT_LOAD_BINARY,
                BLOCK_ARRAY_SET_SIZE,
            ],
        );
        e.register(BLOCK_ALLOCATE, |e, a| e.mem.alloc(a[1].max(4)).into_ret());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        // `SetAt(index, &element)` on the array at +0x1c.
        e.register(BLOCK_ARRAY_SET_AT, |e, a| {
            let storage = e.mem.u32(a[0] + 4);
            let element = e.mem.u32(a[2]);
            e.mem.set_u32(storage + a[1] * 4, element);
            Ret::default()
        });
        e.register(READ, |e, a| {
            let source = e.mem.u32(a[0] + 0xc) + e.mem.u32(a[0] + 4);
            let bytes = e.mem.bytes(source, a[2]);
            e.mem.write(a[1], &bytes);
            a[2].into_ret()
        });
        let mut data_slots = vec![0u32; 0x25];
        data_slots[0x24] = CREATE_BLOCK;
        e.put_vtable(DATA_VTABLE, &data_slots);
        e.put_vtable(
            BLOCK_VTABLE,
            &[BLOCK_DELETE, BLOCK_ALLOCATE, BLOCK_PROTECT, 0, BLOCK_FREE],
        );
        e.register(CREATE_BLOCK, |e, _| {
            let block = e.mem.alloc(0x10);
            e.mem.set_u32(block, BLOCK_VTABLE);
            block.into_ret()
        });
        e
    }

    /// A data object with `blocks` slots (array capacity 8) and `streams`
    /// stream entries.
    fn data(e: &mut Engine, blocks: &[u32], streams: u32) -> u32 {
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this, DATA_VTABLE);
        let storage = e.mem.alloc(8 * 4);
        for (i, block) in blocks.iter().enumerate() {
            e.mem.set_u32(storage + 4 * i as u32, *block);
        }
        e.mem.set_u32(this + BLOCKS_BASE, storage);
        e.mem.set_u16(this + BLOCKS_MAX_SIZE, 8);
        e.mem.set_u16(this + BLOCKS_SIZE, blocks.len() as u16);
        e.mem.set_u16(this + BLOCKS_GROW_BY, 1);
        if streams != 0 {
            let entries = e.mem.alloc(streams * STREAM_SIZE);
            e.mem.set_u32(this + STREAMS, entries);
            e.mem.set_u32(this + STREAM_COUNT, streams);
        }
        this
    }

    fn block(e: &mut Engine, size: u32, data: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, BLOCK_VTABLE);
        e.mem.set_u32(block + BLOCK_DATA_SIZE, size);
        e.mem.set_u32(block + BLOCK_DATA, data);
        block
    }

    /// A stream (`NiStream`) whose file reads `bytes`.
    fn stream(e: &mut Engine, bytes: &[u8]) -> u32 {
        let source = e.mem.alloc(bytes.len().max(4) as u32);
        e.mem.write(source, bytes);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + FILE_READ, READ);
        e.mem.set_u32(file + 0xc, source);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + STREAM_FILE, file);
        stream
    }

    fn file_position(e: &Engine, stream: u32) -> u32 {
        e.mem.u32(e.mem.u32(stream + STREAM_FILE) + FILE_POSITION)
    }

    #[test]
    fn allocate_buffer_asks_the_memory_manager_unless_empty() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_2510, &args![0u32, 0u32]).u32(), 0);
        assert!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        assert_ne!(e.call(0x00a7_2510, &args![0u32, 0x20u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x20]]
        );
    }

    #[test]
    fn free_buffer_frees_the_given_or_the_owned_buffer() {
        let mut e = engine();
        let owned = block(&mut e, 4, 0x0abc_0000);
        e.mem.set_u8(owned + BLOCK_OWN_DATA, 1);
        let not_owned = block(&mut e, 4, 0x0abc_0000);
        e.call_log = Some(vec![]);
        e.call(0x00a7_2530, &args![owned, 0x0123_0000u32]);
        e.call(0x00a7_2530, &args![owned, 0u32]);
        e.call(0x00a7_2530, &args![not_owned, 0u32]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![
                vec![MEMORY_MANAGER, 0x0123_0000],
                vec![MEMORY_MANAGER, 0x0abc_0000],
                vec![MEMORY_MANAGER, 0]
            ]
        );
    }

    #[test]
    fn remove_block_deletes_it_and_clears_its_streams_on_request() {
        let mut e = engine();
        let first = block(&mut e, 4, 0);
        let second = block(&mut e, 4, 0);
        let this = data(&mut e, &[first, second], 3);
        let entries = e.mem.u32(this + STREAMS);
        for (i, block_index) in [1u32, 0, 1].iter().enumerate() {
            let entry = entries + i as u32 * STREAM_SIZE;
            for field in (4..0x1c).step_by(4) {
                e.mem.set_u32(entry + field, 0x99);
            }
            e.mem.set_u32(entry + STREAM_BLOCK_INDEX, *block_index);
        }
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a7_2600, &args![this, 1u32, 1u32]).u32() & 0xff,
            1
        );
        assert_eq!(calls_to(&e, BLOCK_FREE), vec![vec![second, 0]]);
        assert_eq!(calls_to(&e, BLOCK_DELETE), vec![vec![second, 1]]);
        assert_eq!(e.mem.u32(e.mem.u32(this + BLOCKS_BASE) + 4), 0);
        for (i, cleared) in [true, false, true].iter().enumerate() {
            let entry = entries + i as u32 * STREAM_SIZE;
            let expected = if *cleared { 0 } else { 0x99 };
            assert_eq!(e.mem.u32(entry + STREAM_TYPE), expected, "entry {i}");
            assert_eq!(e.mem.u32(entry + STREAM_BLOCK_OFFSET), expected);
        }
        // The slot is empty now; past the size; no stream clearing.
        assert_eq!(
            e.call(0x00a7_2600, &args![this, 1u32, 1u32]).u32() & 0xff,
            0
        );
        assert_eq!(
            e.call(0x00a7_2600, &args![this, 2u32, 1u32]).u32() & 0xff,
            0
        );
        assert_eq!(
            e.call(0x00a7_2600, &args![this, 0u32, 0u32]).u32() & 0xff,
            1
        );
        assert_eq!(e.mem.u32(entries + STREAM_SIZE + STREAM_TYPE), 0x99);
    }

    #[test]
    fn set_data_stream_fills_the_entry_or_refuses() {
        let mut e = engine();
        let this = data(&mut e, &[0, 0], 2);
        e.mem.set_u16(this + VERTEX_COUNT, 10);
        let set = |e: &mut Engine, stream: u32, block: u32, count: u32| {
            e.call(
                0x00a7_26a0,
                &args![this, stream, block, 0x30u32, 7u32, count, 12u32, 16u32],
            )
            .u32()
                & 0xff
        };
        assert_eq!(set(&mut e, 1, 2, 10), 1);
        let entry = e.mem.u32(this + STREAMS) + STREAM_SIZE;
        let words: Vec<u32> = (1..7).map(|i| e.mem.u32(entry + 4 * i)).collect();
        assert_eq!(words, vec![7, 12, 120, 16, 2, 0x30]);
        assert_eq!(set(&mut e, 1, 3, 10), 0);
        assert_eq!(set(&mut e, 2, 0, 10), 0);
        assert_eq!(set(&mut e, 0, 0, 9), 0);
        e.mem.set_u32(this + STREAMS, 0);
        assert_eq!(set(&mut e, 0, 0, 10), 0);
    }

    #[test]
    fn get_data_stream_reports_the_entry_and_its_data() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x40);
        let the_block = block(&mut e, 0x40, buffer);
        let this = data(&mut e, &[the_block], 1);
        e.mem.set_u16(this + VERTEX_COUNT, 5);
        let entry = e.mem.u32(this + STREAMS);
        for (field, value) in [
            (STREAM_TYPE, 3u32),
            (STREAM_UNIT_SIZE, 8),
            (STREAM_TOTAL_SIZE, 40),
            (STREAM_STRIDE, 12),
            (STREAM_BLOCK_INDEX, 0),
            (STREAM_BLOCK_OFFSET, 0x10),
        ] {
            e.mem.set_u32(entry + field, value);
        }
        let outs = e.mem.alloc(0x20);
        let get = |e: &mut Engine, stream: u32| {
            e.call(
                0x00a7_2720,
                &args![
                    this,
                    stream,
                    outs,
                    outs + 4,
                    outs + 8,
                    outs + 0xc,
                    outs + 0x10,
                    outs + 0x14
                ],
            )
            .u32()
                & 0xff
        };
        assert_eq!(get(&mut e, 0), 1);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(outs + 4 * i)).collect();
        assert_eq!(words, vec![buffer + 0x10, 3, 5, 40, 8, 12]);
        // Past the block's size, past the array's size (+1 allowed), no
        // type, past the count.
        e.mem.set_u32(entry + STREAM_BLOCK_OFFSET, 0x41);
        assert_eq!(get(&mut e, 0), 0);
        e.mem.set_u32(entry + STREAM_BLOCK_INDEX, 2);
        assert_eq!(get(&mut e, 0), 0);
        e.mem.set_u32(entry + STREAM_TYPE, 0);
        e.mem.set_u32(outs + 8, 0x7777);
        assert_eq!(get(&mut e, 0), 0);
        assert_eq!(e.mem.u32(outs + 4), 0);
        assert_eq!(e.mem.u32(outs + 8), 0x7777);
        assert_eq!(get(&mut e, 1), 0);
    }

    #[test]
    fn create_agd_data_block_makes_an_empty_block() {
        let mut e = engine();
        let block = e.call(0x00a7_27c0, &args![]).u32();
        assert_eq!(e.mem.u32(block), NI_AGD_DATA_BLOCK_VTABLE);
        assert_eq!(e.mem.u32(block + BLOCK_DATA_SIZE), 0);
        assert_eq!(e.mem.u32(block + BLOCK_DATA), 0);
        assert_eq!(e.mem.u8(block + BLOCK_OWN_DATA), 0);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a7_27c0, &args![]).u32(), 0);
    }

    #[test]
    fn set_data_block_copies_into_a_new_block_and_grows_the_array() {
        let mut e = engine();
        let this = data(&mut e, &[], 0);
        e.mem.set_u16(this + VERTEX_COUNT, 0x3f);
        e.mem.set_u16(this + BLOCKS_MAX_SIZE, 0);
        let source = e.mem.alloc(8);
        e.mem.write(source, b"abcdefgh");
        e.call_log = Some(vec![]);
        let ok = e
            .call(0x00a7_2bb0, &args![this, 0u32, source, 8u32, 1u32])
            .u32();
        assert_eq!(ok & 0xff, 1);
        let block = e.mem.u32(e.mem.u32(this + BLOCKS_BASE));
        let buffer = e.mem.u32(block + BLOCK_DATA);
        assert_ne!(buffer, source);
        assert_eq!(e.mem.bytes(buffer, 8), b"abcdefgh");
        assert_eq!(e.mem.u32(block + BLOCK_DATA_SIZE), 8);
        assert_eq!(e.mem.u8(block + BLOCK_BS_HEAP), 1);
        assert_eq!(e.mem.u8(block + BLOCK_OWN_DATA), 1);
        assert_eq!(calls_to(&e, BLOCK_PROTECT), vec![vec![block, buffer, 0]]);
        assert_eq!(
            calls_to(&e, BLOCK_ARRAY_SET_SIZE),
            vec![vec![this + BLOCKS, 1]]
        );
    }

    #[test]
    fn set_data_block_reuses_or_resizes_an_existing_block() {
        let mut e = engine();
        let kept = block(&mut e, 8, 0x0abc_0000);
        let this = data(&mut e, &[kept], 0);
        e.call_log = Some(vec![]);
        // Same size, data present: only stored again, not owning now.
        e.call(0x00a7_2bb0, &args![this, 0u32, 0x0555_0000u32, 8u32, 0u32]);
        assert_eq!(e.mem.u32(kept + BLOCK_DATA), 0x0abc_0000);
        assert_eq!(e.mem.u8(kept + BLOCK_OWN_DATA), 0);
        assert!(calls_to(&e, BLOCK_FREE).is_empty());
        assert!(calls_to(&e, BLOCK_ARRAY_SET_SIZE).is_empty());
        // Another size without copying: the buffer is freed and the given
        // data used.
        e.call(0x00a7_2bb0, &args![this, 0u32, 0x0555_0000u32, 4u32, 0u32]);
        assert_eq!(calls_to(&e, BLOCK_FREE), vec![vec![kept, 0]]);
        assert_eq!(e.mem.u32(kept + BLOCK_DATA), 0x0555_0000);
        assert_eq!(e.mem.u32(kept + BLOCK_DATA_SIZE), 4);
        // A failed creation answers false.
        e.register(CREATE_BLOCK, |_, _| Ret::default());
        assert_eq!(
            e.call(0x00a7_2bb0, &args![this, 3u32, 0u32, 4u32, 0u32])
                .u32()
                & 0xff,
            0
        );
    }

    #[test]
    fn set_data_block_count_sizes_the_block_array() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x00a7_2ca0, &args![0x5000u32, 6u32]);
        assert_eq!(calls_to(&e, BLOCK_ARRAY_SET_SIZE), vec![vec![0x501c, 6]]);
    }

    #[test]
    fn stream_entry_load_reads_six_words_then_the_flag() {
        let mut e = engine();
        let mut bytes = vec![];
        for word in 1u32..=6 {
            bytes.extend(word.to_le_bytes());
        }
        bytes.push(0x80);
        let stream = stream(&mut e, &bytes);
        let entry = e.mem.alloc(STREAM_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x00a7_2cb0, &args![entry, stream]);
        let words: Vec<u32> = (1..7).map(|i| e.mem.u32(entry + 4 * i)).collect();
        assert_eq!(words, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(e.mem.u8(entry), 0x80);
        assert_eq!(file_position(&e, stream), 25);
        let reads = calls_to(&e, READ);
        assert_eq!(reads.len(), 7);
        assert!(reads.iter().all(|r| r[4] == 1));
        assert_eq!(reads[6][2], 1);
    }

    #[test]
    fn data_block_load_reads_its_parts_into_the_buffer() {
        let mut e = engine();
        let mut bytes = vec![];
        bytes.extend(16u32.to_le_bytes()); // block size
        bytes.extend(2u32.to_le_bytes()); // two parts
        bytes.extend(0u32.to_le_bytes()); // offsets
        bytes.extend(8u32.to_le_bytes());
        // Part 0: three element sizes (1, 2, 1) for 2 vertices: 8 bytes.
        bytes.extend(3u32.to_le_bytes());
        for size in [1u32, 2, 1] {
            bytes.extend(size.to_le_bytes());
        }
        bytes.extend(*b"ABCDEFGH");
        // Part 1: one element size (2): 4 bytes.
        bytes.extend(1u32.to_le_bytes());
        bytes.extend(2u32.to_le_bytes());
        bytes.extend(*b"wxyz");
        let stream = stream(&mut e, &bytes);
        let the_block = block(&mut e, 0, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x00a7_2eb0,
            &args![the_block, stream, 9u32, 9u32, 9u32, 2u32],
        );
        let buffer = e.mem.u32(the_block + BLOCK_DATA);
        assert_eq!(e.mem.bytes(buffer, 12), b"ABCDEFGHwxyz");
        assert_eq!(e.mem.u32(the_block + BLOCK_DATA_SIZE), 16);
        assert_eq!(e.mem.u8(the_block + BLOCK_OWN_DATA), 1);
        assert_eq!(e.mem.u8(the_block + BLOCK_BS_HEAP), 1);
        assert_eq!(calls_to(&e, BLOCK_ALLOCATE), vec![vec![the_block, 16]]);
        assert_eq!(calls_to(&e, BLOCK_PROTECT), vec![vec![the_block, 0, 0]]);
        assert_eq!(file_position(&e, stream), bytes.len() as u32);
        // The part reads carry their element sizes.
        let reads = calls_to(&e, READ);
        let part_0 = &reads[5];
        assert_eq!((part_0[2], part_0[4]), (8, 3));
        let part_1 = &reads[8];
        assert_eq!((part_1[2], part_1[4]), (4, 1));
    }

    #[test]
    fn destructor_removes_every_block_and_frees_the_storage() {
        let mut e = engine();
        let first = block(&mut e, 4, 0);
        let second = block(&mut e, 4, 0);
        let this = data(&mut e, &[first, second], 1);
        let entries = e.mem.u32(this + STREAMS);
        let storage = e.mem.u32(this + BLOCKS_BASE);
        e.call_log = Some(vec![]);
        e.call(0x00a7_3080, &args![this]);
        assert_eq!(e.mem.u32(this), NI_ADDITIONAL_GEOMETRY_DATA_VTABLE);
        assert_eq!(
            calls_to(&e, BLOCK_DELETE),
            vec![vec![first, 1], vec![second, 1]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, entries]]
        );
        assert_eq!(e.mem.u32(this + BLOCKS), BLOCK_ARRAY_BASE_VTABLE);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![storage]]);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn set_data_stream_count_moves_the_entries() {
        let mut e = engine();
        let this = data(&mut e, &[], 1);
        let old = e.mem.u32(this + STREAMS);
        e.mem.set_u32(old + STREAM_TYPE, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x00a7_30e0, &args![this, 3u32]);
        let new = e.mem.u32(this + STREAMS);
        assert_ne!(new, old);
        assert_eq!(e.mem.u32(new + STREAM_TYPE), 0x42);
        assert_eq!(e.mem.u32(this + STREAM_COUNT), 3);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 3 * STREAM_SIZE]]
        );
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR),
            vec![vec![new, STREAM_SIZE, 3, STREAM_CONSTRUCT]]
        );
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![new, old, STREAM_SIZE]]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, old]]
        );
        assert_eq!(stream_array_bytes(0x1000_0000), u32::MAX);
    }

    #[test]
    fn load_binary_reads_streams_and_blocks() {
        let mut e = engine();
        let mut bytes = vec![];
        bytes.extend(3u16.to_le_bytes()); // vertex count
        bytes.extend(1u32.to_le_bytes()); // one stream entry
        for word in 1u32..=6 {
            bytes.extend(word.to_le_bytes());
        }
        bytes.push(0);
        bytes.extend(2u32.to_le_bytes()); // two blocks
        bytes.push(1); // block 0 present: size 0, no parts
        bytes.extend(0u32.to_le_bytes());
        bytes.extend(0u32.to_le_bytes());
        bytes.push(0); // block 1 absent
        let stream = stream(&mut e, &bytes);
        let this = data(&mut e, &[], 0);
        e.call_log = Some(vec![]);
        e.call(0x00a7_3160, &args![this, stream]);
        assert_eq!(e.mem.u16(this + VERTEX_COUNT), 3);
        assert_eq!(e.mem.u32(this + STREAM_COUNT), 1);
        let entry = e.mem.u32(this + STREAMS);
        assert_eq!(e.mem.u32(entry + STREAM_BLOCK_OFFSET), 6);
        assert_eq!(
            calls_to(&e, BLOCK_ARRAY_SET_SIZE),
            vec![vec![this + BLOCKS, 2]]
        );
        let storage = e.mem.u32(this + BLOCKS_BASE);
        assert_ne!(e.mem.u32(storage), 0);
        assert_eq!(e.mem.u32(storage + 4), 0);
        assert_eq!(e.mem.u16(this + BLOCKS_SIZE), 2);
        assert_eq!(e.mem.u16(this + BLOCKS_E_SIZE), 1);
        assert_eq!(file_position(&e, stream), bytes.len() as u32);
        assert_eq!(
            calls_to(&e, NI_OBJECT_LOAD_BINARY),
            vec![vec![this, stream]]
        );
    }

    #[test]
    fn constructor_and_create_object_make_an_empty_object() {
        let mut e = engine();
        e.register(NI_OBJECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in (0..a[0]).step_by(4) {
                e.mem.set_u32(block + offset, 0xdead_beef);
            }
            block.into_ret()
        });
        e.call_log = Some(vec![]);
        let object = e.call(0x00a7_3e20, &args![]).u32();
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x2c]]);
        assert_eq!(e.mem.u32(object), NI_ADDITIONAL_GEOMETRY_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + BLOCKS), BLOCK_ARRAY_VTABLE);
        for offset in [0x08, STREAM_COUNT, STREAMS, D3D_STREAM, BLOCKS_BASE] {
            assert_eq!(e.mem.u32(object + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u16(object + VERTEX_COUNT), 0);
        assert_eq!(e.mem.u32(object + BLOCKS_MAX_SIZE), 0);
        assert_eq!(e.mem.u16(object + BLOCKS_E_SIZE), 0);
        assert_eq!(e.mem.u16(object + BLOCKS_GROW_BY), 1);
        // The constructor alone returns `this`; no allocation, no object.
        let this = e.mem.alloc(0x2c);
        assert_eq!(e.call(0x00a7_34c0, &args![this]).u32(), this);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a7_3e20, &args![]).u32(), 0);
    }
}
