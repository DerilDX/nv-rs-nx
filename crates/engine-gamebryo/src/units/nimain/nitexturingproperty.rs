//! `NiMain/nitexturingproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a6ab90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a6a0b0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x30 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_texturing_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a6_a0b0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, set_literal_fixed_string, ALLOCATE};

/// `011f49ac`: the pointer `00a6b410` stores the object it builds in.
const TEXTURING_PROPERTY_SLOT: u32 = 0x011f_49ac;
/// `00a6aa40` (`thiscall`, unit `nitexturingproperty.cpp`): constructs the object in the
/// block and answers it.
const TEXTURING_PROPERTY_CONSTRUCTOR: u32 = 0x00a6_aa40;
/// The static fixed strings `00a6b410` sets: (global, address of the literal), in order.
const TEXTURE_SLOT_NAMES: [(u32, u32); 10] = [
    (0x011f_49b0, 0x0108_03dc),
    (0x011f_49b4, 0x0103_c800),
    (0x011f_49b8, 0x0109_c5e4),
    (0x011f_49bc, 0x0109_c5dc),
    (0x011f_49c0, 0x0109_c5d4),
    (0x011f_49c4, 0x0109_c5cc),
    (0x011f_49c8, 0x0101_c29c),
    (0x011f_49cc, 0x0109_c5c0),
    (0x011f_49d0, 0x0101_e47c),
    (0x011f_49d4, 0x0109_c5b8),
];

// Translated from 00a6b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::_SDMInit` (Xbox PDB; run by the start-up `00a89cc0`, no
/// arguments): allocates 0x30 bytes (`00aa13e0`), constructs a texturing property in them
/// with `00a6aa40` and stores the object it answers, counted, in `011f49ac`, releasing the
/// object held there before (a failed allocation stores null); then sets ten static fixed
/// strings from literals (`TEXTURE_SLOT_NAMES`), each only when the global does not hold
/// the literal's address already.
pub fn ni_texturing_property_sdm_init(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x30u32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(TEXTURING_PROPERTY_CONSTRUCTOR, &args![block]).u32()
    };
    assign_ref_counted(e, TEXTURING_PROPERTY_SLOT, object);
    for (slot, literal) in TEXTURE_SLOT_NAMES {
        set_literal_fixed_string(e, slot, literal);
    }
}

/// `0096ae90` (`thiscall`, `RET 8`, no unit in the engine map): called with the part at
/// `+0x1c`, an index and the address of a word holding 0.
const SET_ELEMENT: u32 = 0x0096_ae90;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;

// Translated from 00a6aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitexturingproperty.cpp`; the body of a constructor,
/// `thiscall`, no stack word): runs the base constructor `00a5c200`, sets the vtable
/// `0109c514` and the part at `this + 0x1c` (vtable `0109c394`, then `0109c5ac`, count
/// words 9, 0, 0 and 1, a 0x24-byte buffer from `00aa1070` at `+0x20`), clears the word at
/// `+0x2c`, calls `0096ae90` for the indexes 0 to 8 with a word holding 0, then sets the
/// flag word at `+0x18`: bit 0 when one of the entries of the buffer from index 1 up to
/// the count at `+0x26` is non-zero, bit 2 always, and bits 1 and 3 cleared.
pub fn fn_00a6aa40(e: &mut Engine, this: Ptr) -> Ptr {
    let object = this.addr();
    e.call(
        crate::units::nimain::nimainsdm::OBJECT_BASE_CONSTRUCTOR,
        &args![this],
    );
    e.mem.set_u32(object, 0x0109_c514);
    let part = object + 0x1c;
    e.mem.set_u16(object + 0x18, 0);
    e.mem.set_u16(part + 8, 9);
    e.mem.set_u32(part, 0x0109_c394);
    e.mem.set_u16(part + 0xe, 1);
    e.mem.set_u16(part + 0xa, 0);
    e.mem.set_u16(part + 0xc, 0);
    let buffer = e.call(ALLOCATE_BLOCK, &args![0x24u32]).u32();
    e.mem.set_u32(part + 4, buffer);
    e.mem.set_u32(part, 0x0109_c5ac);
    e.mem.set_u32(object + 0x2c, 0);
    for index in 0..9u32 {
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), 0);
            e.call(SET_ELEMENT, &args![part, index, local]);
        });
    }
    let mut flags = u32::from(e.mem.u16(object + 0x18) & 0xf00f);
    e.mem.set_u16(object + 0x18, flags as u16);
    flags = u32::from(e.mem.u16(object + 0x18));
    let count = u32::from(e.mem.u16(object + 0x26));
    let mut found = false;
    if count > 1 {
        let mut slot = e.mem.u32(object + 0x20).wrapping_add(4);
        let mut index = 1u32;
        loop {
            if e.mem.u32(slot) != 0 {
                found = true;
                break;
            }
            index += 1;
            slot = slot.wrapping_add(4);
            if index >= count {
                break;
            }
        }
    }
    if found {
        flags |= 1;
    } else {
        flags &= 0xfffe;
    }
    let word = (flags as u16 & 0xfff5) | 4;
    e.mem.set_u16(object + 0x18, flags as u16);
    e.mem.set_u16(object + 0x18, word);
    this
}

// ---- Batch b012: maps, cloning, loading and linking ------------------------------------

use crate::types::NiTArray;
use crate::units::nimain::nimainsdm::{release_fixed_string, GET_FIXED_STRING};

/// `NiTexturingProperty` (Xbox PDB) fields the code below uses: the flag word `m_uFlags`
/// (`+0x18`), the map array `m_kMaps` (an `NiTPrimitiveArray<Map *>`, `+0x1c`, its
/// elements at `+0x20`) and `m_pkShaderMaps` (a pointer to another such array, `+0x2c`).
const FLAGS: u32 = 0x18;
const MAPS: u32 = 0x1c;
const SHADER_MAPS: u32 = 0x2c;
/// The vtables: `NiTexturingProperty` (`0109c514`), its map array (`0109c394`, then
/// `0109c5ac`), a shader-map array (`0109c5b4`), `NiTexturingProperty::Map` (`0109c4c4`)
/// and the map kinds made for slot 5 (`0109c4d8`), slot 7 (`0109c500`) and the shader maps
/// (`0109c4ec`).
const TEXTURING_PROPERTY_VTABLE: u32 = 0x0109_c514;
const MAP_ARRAY_BASE_VTABLE: u32 = 0x0109_c394;
const MAP_ARRAY_VTABLE: u32 = 0x0109_c5ac;
const SHADER_MAP_ARRAY_VTABLE: u32 = 0x0109_c5b4;
const MAP_VTABLE: u32 = 0x0109_c4c4;
const SLOT_5_MAP_VTABLE: u32 = 0x0109_c4d8;
const SLOT_7_MAP_VTABLE: u32 = 0x0109_c500;
const SHADER_MAP_VTABLE: u32 = 0x0109_c4ec;
/// `NiTexturingProperty::GetRTTI` answers this record (`011f49a4`).
const TEXTURING_PROPERTY_RTTI: u32 = 0x011f_49a4;
/// The memory manager object (`011f6238`) and its `thiscall` allocate (`00aa3e40`, one
/// word) and free (`00aa4060`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `00aa10f0` (`cdecl`, one word): frees a block from `00aa1070`.
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// `00a5c240` (`thiscall`, unit `niobjectnet.cpp`): the `NiObjectNET` destructor body.
const OBJECT_NET_DESTRUCTOR: u32 = 0x00a5_c240;
/// `00a5d450` (`thiscall`, `RET 8`, unit `niobject.cpp`); `00a5d7a0` (`thiscall`, `RET 8`, a
/// jump to `00a5c290`): the member copies `CreateClone` and `00a6bb00` run first.
const OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
const OBJECT_NET_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `00a90180` (`thiscall`, `RET 4`; engine-map unit `nioldscreenspace.cpp`, a shared
/// body): sets the capacity of an `NiTPrimitiveArray`.
const ARRAY_SET_SIZE: u32 = 0x00a9_0180;
/// `NiSourceTexture::Create_ov2` (`00a5ff10`, `cdecl`, three words).
const SOURCE_TEXTURE_CREATE: u32 = 0x00a5_ff10;
/// `00a88eb0` (`thiscall`, `RET 0x14`): builds a texture transform from a translation, a
/// rotation (a float word), a scale, a centre and a method; `00a88f00` (`thiscall`,
/// `RET 4`): loads one from the stream.
const TEXTURE_TRANSFORM_CONSTRUCTOR: u32 = 0x00a8_8eb0;
const TEXTURE_TRANSFORM_LOAD_BINARY: u32 = 0x00a8_8f00;
/// The two floats `00a69b00` copies as a new transform's translation (`011f4980`) and the
/// float its centre starts at (`01016248`).
const ZERO_POINT_2: u32 = 0x011f_4980;
const TRANSFORM_CENTRE: u32 = 0x0101_6248;
/// `NiProperty::LoadBinary` (`00a5d7f0`, `RET 4`), `00a645b0` (`thiscall` on the stream),
/// `NiStream::SetNumberOfLinkIDs` (`00a64700`, engine-map name, `RET 4`),
/// `NiStream::GetNumberOfLinkIDs` (`00a63460`) and `NiStream::GetObjectFromLinkID`
/// (`00a63430`), both `thiscall` on the stream without a stack word.
const PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;
const STREAM_00A645B0: u32 = 0x00a6_45b0;
const SET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_4700;
const GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a47360` (`thiscall`, `RET 4`; engine map: `NiSequenceStreamHelper::LinkObject`, a body
/// shared by folding): the base link step.
const BASE_LINK_OBJECT: u32 = 0x00a4_7360;
/// Constructors in this unit outside the batch: the slot-5 map (`00a69fb0`, 0x28 bytes), the
/// slot-7 map (`00a6a010`, 0x14 bytes) and the shader-map array (`00a6a990`, `RET 8`:
/// capacity and grow step).
const SLOT_5_MAP_CONSTRUCTOR: u32 = 0x00a6_9fb0;
const SLOT_7_MAP_CONSTRUCTOR: u32 = 0x00a6_a010;
const SHADER_MAP_ARRAY_CONSTRUCTOR: u32 = 0x00a6_a990;
/// `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// `MemoryManager::Allocate` on the manager object (`00aa3e40`).
fn memory_allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32()
}

/// `InterlockedDecrement` of the count at `object + 4`; `DeleteThis` (slot `+4`) at zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).i32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

/// `NiPointer::operator=` on the holder at `holder`.
fn assign(e: &mut Engine, holder: u32, value: u32) {
    let old = e.mem.u32(holder);
    if old != value {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(holder, value);
        if value != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
        }
    }
}

/// One read through the stream's reader (`stream + 0x24c`): the function pointer at the
/// reader's `+8`, `cdecl (reader, buffer, size, count, 1)` with the word at `count` set to
/// `size` first; the answer is added to the reader's word at `+4`.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, count: u32) {
    let reader = e.mem.u32(stream + 0x24c);
    let read_function = e.mem.u32(reader + 8);
    e.mem.set_u32(count, size);
    let read = e
        .call(read_function, &args![reader, buffer, size, count, 1u32])
        .u32();
    let total = e.mem.u32(reader + 4).wrapping_add(read);
    e.mem.set_u32(reader + 4, total);
}

/// The inlined `NiTPrimitiveArray::SetAt` after its capacity check: past the size the size
/// becomes `index + 1` (and a non-null value is counted); inside it the count of non-null
/// elements follows the change; then the element is stored.
fn array_set_counted(e: &mut Engine, array: Ptr<NiTArray>, index: u32, value: u32) {
    let size = u32::from(e.get(array, NiTArray::m_usSize));
    let used = e.get(array, NiTArray::m_usESize);
    if index >= size {
        e.set(array, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if value != 0 {
            e.set(array, NiTArray::m_usESize, used.wrapping_add(1));
        }
    } else {
        let current = e.mem.u32(e.get(array, NiTArray::m_pBase) + 4 * index);
        if value != 0 {
            if current == 0 {
                e.set(array, NiTArray::m_usESize, used.wrapping_add(1));
            }
        } else if current != 0 {
            e.set(array, NiTArray::m_usESize, used.wrapping_add(0xffff));
        }
    }
    let base = e.get(array, NiTArray::m_pBase);
    e.mem.set_u32(base + 4 * index, value);
}

/// The capacity check before a `SetAt` at `index`: grown to `grow step + index` when
/// `index` is not below the capacity (`00a90180`).
fn array_reserve(e: &mut Engine, array: Ptr<NiTArray>, index: u32) {
    if index >= u32::from(e.get(array, NiTArray::m_usMaxSize)) {
        let step = u32::from(e.get(array, NiTArray::m_usGrowBy));
        e.call(ARRAY_SET_SIZE, &args![array, step.wrapping_add(index)]);
    }
}

/// The map count in bits 4..11 of the flag word, plus one.
fn count_one_more_map(e: &mut Engine, this: u32) {
    let flags = u32::from(e.mem.u16(this + FLAGS));
    let count = ((flags >> 4) & 0xff) + 1;
    e.mem
        .set_u16(this + FLAGS, ((count << 4) | (flags & 0xf00f)) as u16);
}

// Translated from 00a69dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtable (`0109c4c4`) the `NiTexturingProperty::Map`
/// default constructor (`ECX` only): flags cleared then set to `(flags & 0xf100) | 0x3100`,
/// no texture and no transform.
pub fn fn_00a69dd0(e: &mut Engine, this: Ptr) {
    let map = this.addr();
    e.mem.set_u16(map + 4, 0);
    e.mem.set_u32(map, MAP_VTABLE);
    e.mem.set_u32(map + 8, 0);
    let flags = e.mem.u16(map + 4);
    e.mem.set_u32(map + 0xc, 0);
    e.mem.set_u16(map + 4, (flags & 0xf100) | 0x3100);
}

// Translated from 00a69e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty::Map` constructor from its parts
/// (`thiscall`, `RET 0x14`): the texture (counted), then the flag word
/// `((((filter << 4) | uv_set) << 8) | (flags & 0xc0ff)) & 0xff00 | clamp` (16 bits; the
/// flags cleared before) and the transform. Answers `this`.
pub fn fn_00a69e00(
    e: &mut Engine,
    this: Ptr,
    texture: u32,
    clamp: u32,
    filter: u32,
    uv_set: u32,
    transform: u32,
) -> Ptr {
    let map = this.addr();
    e.mem.set_u16(map + 4, 0);
    e.mem.set_u32(map, MAP_VTABLE);
    e.mem.set_u32(map + 8, texture);
    if texture != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![texture + 4]);
    }
    let high = ((filter << 4) | uv_set) << 8;
    let old = e.mem.u16(map + 4);
    e.mem.set_u32(map + 0xc, transform);
    let flags = ((high as u16 | (old & 0xc0ff)) & 0xff00) | clamp as u16;
    e.mem.set_u16(map + 4, flags);
    this
}

// Translated from 00a69b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitexturingproperty.cpp`; `ECX` only, answers `this`);
/// its fields match `NiTextureTransform` (Xbox PDB; `m_eMethod` at `+0x44` on PC): the
/// translation from the two words at `011f4980`, rotation 0, scale 1 and 1, centre twice
/// the float at `01016248` (through the x87 stack), the dirty byte 1 and the method 2.
pub fn fn_00a69b00(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    let x = e.mem.u32(ZERO_POINT_2);
    e.mem.set_u32(at, x);
    let y = e.mem.u32(ZERO_POINT_2 + 4);
    e.mem.set_u32(at + 8, x87_store32(0.0));
    e.mem.set_u32(at + 4, y);
    e.mem.set_u32(at + 0xc, x87_store32(1.0));
    e.mem.set_u32(at + 0x10, x87_store32(1.0));
    let centre = x87_quiet_bits(e.mem.u32(TRANSFORM_CENTRE));
    e.mem.set_u32(at + 0x14, centre);
    e.mem.set_u32(at + 0x18, centre);
    e.mem.set_u8(at + 0x1c, 1);
    e.mem.set_u32(at + 0x44, 2);
    this
}

// Translated from 00a6a000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual slot (referenced only from the vtables at
/// `010973f0`, `01097bd0` and `01099658`; `this` in `ECX`, not read) that answers 3.
pub fn fn_00a6a000(_e: &mut Engine, _this: Ptr) -> u32 {
    3
}

// Translated from 00a6a140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::GetRTTI` (Xbox PDB): the record at `011f49a4`.
pub fn ni_texturing_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(TEXTURING_PROPERTY_RTTI)
}

// Translated from 00a6ad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in Ghidra's project (read from the bytes; the class factory, `cdecl`, no
/// argument): allocates 0x30 bytes (`00aa13e0`) and runs the constructor `00a6aa40` on
/// them (a jump: its answer is the answer); a failed allocation answers null.
pub fn fn_00a6ad40(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x30u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00a6aa40(e, Ptr::new(block))
}

// Translated from 00a6a0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty` destructor body (`ECX` only): sets
/// the vtable, deletes every map of the map array (their slot 0 with 1; the count at `+0x26`
/// read again each time), then, with a shader-map array, every shader map and the array
/// itself (slot 0 with 1); the map array's vtable becomes `0109c394`, its elements are freed
/// (`00aa10f0`) and the `NiObjectNET` destructor body (`00a5c240`) ends it.
pub fn fn_00a6a0b0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, TEXTURING_PROPERTY_VTABLE);
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(at + 0x26)) {
        let map = e.mem.u32(e.mem.u32(at + 0x20) + 4 * index);
        if map != 0 {
            e.vcall(map, 0, &args![1u32]);
        }
        index += 1;
    }
    if e.mem.u32(at + SHADER_MAPS) != 0 {
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(e.mem.u32(at + SHADER_MAPS) + 0xa)) {
            let base = e.mem.u32(e.mem.u32(at + SHADER_MAPS) + 4);
            let map = e.mem.u32(base + 4 * index);
            if map != 0 {
                e.vcall(map, 0, &args![1u32]);
            }
            index += 1;
        }
        let array = e.mem.u32(at + SHADER_MAPS);
        if array != 0 {
            e.vcall(array, 0, &args![1u32]);
        }
    }
    let elements = e.mem.u32(at + 0x20);
    e.mem.set_u32(at + MAPS, MAP_ARRAY_BASE_VTABLE);
    e.call(FREE_BLOCK, &args![elements]);
    e.call(OBJECT_NET_DESTRUCTOR, &args![this]);
}

// Translated from 00a6a6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitexturingproperty.cpp`; the `_SDMShutdown` of
/// `00a6b410`, no argument): releases the default texturing property held at `011f49ac`
/// and clears it, then each of the ten static fixed strings that is not null gets the
/// handle `00a5b690(0)` and its old handle is released (unless it is the empty string's).
pub fn fn_00a6a6f0(e: &mut Engine) {
    let object = e.mem.u32(TEXTURING_PROPERTY_SLOT);
    if object != 0 {
        release(e, object);
        e.mem.set_u32(TEXTURING_PROPERTY_SLOT, 0);
    }
    for (slot, _) in TEXTURE_SLOT_NAMES {
        let old = e.mem.u32(slot);
        if old != 0 {
            let handle = e.call(GET_FIXED_STRING, &args![0u32]).u32();
            e.mem.set_u32(slot, handle);
            release_fixed_string(e, old);
        }
    }
}

// Translated from 00a6a8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty::Map` copy constructor (`thiscall`,
/// `RET 4`): the vtable, the flag word and the texture (counted) of `other`; with a
/// transform in `other` a 0x48-byte block from the memory manager is built by `00a88eb0`
/// from a copy of its translation, rotation (through the x87 stack), scale, centre and
/// method, and becomes the transform (null when the allocation fails or `other` has none).
/// Answers `this`.
pub fn fn_00a6a8b0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let (map, source) = (this.addr(), other.addr());
    e.mem.set_u32(map, MAP_VTABLE);
    let flags = e.mem.u16(source + 4);
    e.mem.set_u16(map + 4, flags);
    let texture = e.mem.u32(source + 8);
    e.mem.set_u32(map + 8, texture);
    if texture != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![texture + 4]);
    }
    if e.mem.u32(source + 0xc) == 0 {
        e.mem.set_u32(map + 0xc, 0);
        return this;
    }
    let block = memory_allocate(e, 0x48);
    if block == 0 {
        e.mem.set_u32(map + 0xc, 0);
        return this;
    }
    let transform = e.mem.u32(source + 0xc);
    let made = e.with_stack(0x18, |e, locals| {
        // The centre, the scale and the translation, copied as words.
        let (centre, scale, translate) = (locals.addr(), locals.addr() + 8, locals.addr() + 0x10);
        for (to, from) in [(centre, 0x14), (scale, 0xc), (translate, 0)] {
            let first = e.mem.u32(transform + from);
            let second = e.mem.u32(transform + from + 4);
            e.mem.set_u32(to, first);
            e.mem.set_u32(to + 4, second);
        }
        let method = e.mem.u32(transform + 0x44);
        let rotation = x87_quiet_bits(e.mem.u32(transform + 8));
        e.call(
            TEXTURE_TRANSFORM_CONSTRUCTOR,
            &args![block, translate, rotation, scale, centre, method],
        )
        .u32()
    });
    e.mem.set_u32(map + 0xc, made);
    this
}

// Translated from 00a6a9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty::Map` scalar deleting destructor
/// (`thiscall`, `RET 4`): the vtable, the transform freed through the memory manager
/// (`00aa4060`), the texture released, then the map itself freed the same way when bit 0
/// of `flags` is set. Answers `this`.
pub fn fn_00a6a9f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let map = this.addr();
    let transform = e.mem.u32(map + 0xc);
    e.mem.set_u32(map, MAP_VTABLE);
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, transform]);
    let texture = e.mem.u32(map + 8);
    if texture != 0 {
        release(e, texture);
    }
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a6abb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty` constructor from a texture file
/// (`thiscall`, `RET 0xc`): the same set-up as `00a6aa40`, then the texture
/// `NiSourceTexture::Create_ov2(a1, *a2, a3)` (`00a5ff10`) in a new map (0x10 bytes from the
/// memory manager, `00a69e00` with clamp 0, filter 3, UV set 1 and no transform; null when
/// the allocation fails) at index 0 and nothing at the indexes 1 to 8 (`0096ae90`), then
/// the flag word as `00a6aa40` leaves it. Answers `this`.
pub fn fn_00a6abb0(e: &mut Engine, this: Ptr, a1: u32, a2: Ptr, a3: u32) -> Ptr {
    let object = this.addr();
    e.call(
        crate::units::nimain::nimainsdm::OBJECT_BASE_CONSTRUCTOR,
        &args![this],
    );
    e.mem.set_u32(object, TEXTURING_PROPERTY_VTABLE);
    let part = object + MAPS;
    e.mem.set_u16(object + FLAGS, 0);
    e.mem.set_u16(part + 8, 9);
    e.mem.set_u32(part, MAP_ARRAY_BASE_VTABLE);
    e.mem.set_u16(part + 0xe, 1);
    e.mem.set_u16(part + 0xa, 0);
    e.mem.set_u16(part + 0xc, 0);
    let buffer = e.call(ALLOCATE_BLOCK, &args![0x24u32]).u32();
    e.mem.set_u32(part + 4, buffer);
    e.mem.set_u32(part, MAP_ARRAY_VTABLE);
    e.mem.set_u32(object + SHADER_MAPS, 0);
    let name = e.mem.u32(a2.addr());
    let texture = e.call(SOURCE_TEXTURE_CREATE, &args![a1, name, a3]).u32();
    let block = memory_allocate(e, 0x10);
    let map = if block == 0 {
        0
    } else {
        fn_00a69e00(e, Ptr::new(block), texture, 0, 3, 1, 0).addr()
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), map);
        e.call(SET_ELEMENT, &args![part, 0u32, slot]);
        for index in 1..9u32 {
            e.mem.set_u32(slot.addr(), 0);
            e.call(SET_ELEMENT, &args![part, index, slot]);
        }
    });
    let count = u32::from(e.mem.u16(object + 0x26));
    let masked = e.mem.u16(object + FLAGS) & 0xf00f;
    e.mem.set_u16(object + FLAGS, masked);
    let mut flags = e.mem.u16(object + FLAGS);
    let mut found = false;
    let mut index = 1u32;
    while index < count {
        if e.mem.u32(e.mem.u32(object + 0x20) + 4 * index) != 0 {
            found = true;
            break;
        }
        index += 1;
    }
    if found {
        flags |= 1;
    } else {
        flags &= 0xfffe;
    }
    e.mem.set_u16(object + FLAGS, flags);
    e.mem.set_u16(object + FLAGS, (flags & 0xfff5) | 4);
    this
}

// Translated from 00a6c610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::CreateClone` (Xbox PDB; `thiscall`, `RET 4`): without controllers
/// (`+0x0c` null) the property is its own clone: `00a5d450(this, this, cloning)` and `this`
/// answered. Otherwise a new property (0x30 bytes from `00aa13e0`, `00a6aa40`) gets the
/// members (`00a6bb00`, also run with a null clone when the allocation fails) and is
/// answered.
pub fn ni_texturing_property_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    if e.mem.u32(this.addr() + 0xc) == 0 {
        e.call(OBJECT_COPY_MEMBERS, &args![this, this, cloning]);
        return this;
    }
    let block = e.call(ALLOCATE, &args![0x30u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        fn_00a6aa40(e, Ptr::new(block))
    };
    fn_00a6bb00(e, this, clone, cloning);
    clone
}

// Translated from 00a6bb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty` member copy (`thiscall`, `RET 8`):
/// the `NiObjectNET` copy (`00a5d7a0`), the flag word, the capacity of the map array of
/// `dest` set to the count of `this` (`00a90180`), then for each non-null map of `this`
/// a copy (`00a6a8b0`) set into `dest` at the same index (`0096ae90`, after growing the
/// array when the index is not below its capacity): slot 5 as a 0x28-byte map (vtable
/// `0109c4d8`, six more floats at `+0x10..+0x24` through the x87 stack), slot 7 as a
/// 0x14-byte map (vtable `0109c500`, one float at `+0x10`), the others as plain 0x10-byte
/// maps (all from the memory manager; a failed allocation sets null). With shader maps in
/// `this`, `dest` gets an empty shader-map array when it has none, its capacity set to the
/// count of `this`, and a 0x14-byte copy (vtable `0109c4ec`, the word at `+0x10`) of each
/// non-null shader map set at the same index (grown first, counted inline).
pub fn fn_00a6bb00(e: &mut Engine, this: Ptr, dest: Ptr, cloning: u32) {
    let (from, to) = (this.addr(), dest.addr());
    e.call(OBJECT_NET_COPY_MEMBERS, &args![this, dest, cloning]);
    let flags = e.mem.u16(from + FLAGS);
    e.mem.set_u16(to.wrapping_add(FLAGS), flags);
    let count = u32::from(e.mem.u16(from + 0x26));
    let array: Ptr<NiTArray> = Ptr::new(to.wrapping_add(MAPS));
    e.call(ARRAY_SET_SIZE, &args![array, count]);
    e.with_stack(8, |e, slots| {
        // The local the slot-5 and slot-7 copies go through, and the stack slot of the
        // `cloning` argument, which holds the index and the plain copies.
        let (local, index_slot) = (slots.addr(), slots.addr() + 4);
        e.mem.set_u32(index_slot, 0);
        let mut index = 0u32;
        while index < count {
            if e.mem.u32(e.mem.u32(from + 0x20) + 4 * index) != 0 {
                match index {
                    5 => {
                        let block = memory_allocate(e, 0x28);
                        if block == 0 {
                            e.mem.set_u32(local, 0);
                        } else {
                            let source = e.mem.u32(e.mem.u32(from + 0x20) + 0x14);
                            fn_00a6a8b0(e, Ptr::new(block), Ptr::new(source));
                            e.mem.set_u32(block, SLOT_5_MAP_VTABLE);
                            for at in (0x10..0x28).step_by(4) {
                                let value = x87_quiet_bits(e.mem.u32(source + at));
                                e.mem.set_u32(block + at, value);
                                if at == 0x10 {
                                    e.mem.set_u32(local, block);
                                }
                            }
                        }
                        array_reserve(e, array, 5);
                        e.call(SET_ELEMENT, &args![array, 5u32, local]);
                    }
                    7 => {
                        let block = memory_allocate(e, 0x14);
                        if block == 0 {
                            e.mem.set_u32(local, 0);
                        } else {
                            let source = e.mem.u32(e.mem.u32(from + 0x20) + 0x1c);
                            fn_00a6a8b0(e, Ptr::new(block), Ptr::new(source));
                            e.mem.set_u32(block, SLOT_7_MAP_VTABLE);
                            let value = x87_quiet_bits(e.mem.u32(source + 0x10));
                            e.mem.set_u32(block + 0x10, value);
                            e.mem.set_u32(local, block);
                        }
                        array_reserve(e, array, 7);
                        e.call(SET_ELEMENT, &args![array, 7u32, local]);
                    }
                    _ => {
                        let block = memory_allocate(e, 0x10);
                        let copy = if block == 0 {
                            0
                        } else {
                            let source = e.mem.u32(e.mem.u32(from + 0x20) + 4 * index);
                            fn_00a6a8b0(e, Ptr::new(block), Ptr::new(source)).addr()
                        };
                        e.mem.set_u32(index_slot, copy);
                        array_reserve(e, array, index);
                        e.call(SET_ELEMENT, &args![array, index, index_slot]);
                    }
                }
            }
            index += 1;
            e.mem.set_u32(index_slot, index);
        }
    });
    if e.mem.u32(from + SHADER_MAPS) == 0 {
        return;
    }
    if e.mem.u32(to.wrapping_add(SHADER_MAPS)) == 0 {
        let block = memory_allocate(e, 0x10);
        if block != 0 {
            e.mem.set_u16(block + 0xe, 1);
            e.mem.set_u16(block + 8, 0);
            e.mem.set_u16(block + 0xa, 0);
            e.mem.set_u16(block + 0xc, 0);
            e.mem.set_u32(block + 4, 0);
            e.mem.set_u32(block, SHADER_MAP_ARRAY_VTABLE);
        }
        e.mem.set_u32(to.wrapping_add(SHADER_MAPS), block);
    }
    let count = u32::from(e.mem.u16(e.mem.u32(from + SHADER_MAPS) + 0xa));
    let target = e.mem.u32(to.wrapping_add(SHADER_MAPS));
    e.call(ARRAY_SET_SIZE, &args![target, count]);
    for index in 0..count {
        let base = e.mem.u32(e.mem.u32(from + SHADER_MAPS) + 4);
        if e.mem.u32(base + 4 * index) == 0 {
            continue;
        }
        let block = memory_allocate(e, 0x14);
        let copy = if block == 0 {
            0
        } else {
            let base = e.mem.u32(e.mem.u32(from + SHADER_MAPS) + 4);
            let source = e.mem.u32(base + 4 * index);
            fn_00a6a8b0(e, Ptr::new(block), Ptr::new(source));
            e.mem.set_u32(block, SHADER_MAP_VTABLE);
            let word = e.mem.u32(source + 0x10);
            e.mem.set_u32(block + 0x10, word);
            block
        };
        let target: Ptr<NiTArray> = Ptr::new(e.mem.u32(to.wrapping_add(SHADER_MAPS)));
        array_reserve(e, target, index);
        array_set_counted(e, target, index, copy);
    }
}

// Translated from 00a6b640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::Map::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`): `00a645b0` on
/// the stream, then by stream version (`+0xd8`): before `0x14010002` three 4-byte words
/// whose low bits become the flag word's bits 12-13 (`<< 12`, mask `0xcfff`), 8-11
/// (`<< 8`, mask `0xf0ff`) and 0-7 (mask `0xff00`); from it the flag word itself (2 bytes).
/// Before `0x0a030004` two 2-byte words and before `0x04010010` one more are read and
/// dropped. From `0x0a00010a` the transform is cleared and one byte read: when it is 1 a
/// transform (0x48 bytes from the memory manager, `00a69b00`; null when that fails) is made
/// and loaded (`00a88f00`). Every read goes through the reader's function pointer; the
/// stack slot of the stream argument serves as count or buffer as in the exe.
pub fn ni_texturing_property_map_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (map, source) = (this.addr(), stream.addr());
    e.call(STREAM_00A645B0, &args![stream]);
    let version = |e: &Engine| e.mem.u32(source + 0xd8);
    e.with_stack(0xc, |e, frame| {
        // Two locals and the stack slot of the stream argument (holding the pointer).
        let (first, second, argument) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        e.mem.set_u32(argument, source);
        if version(e) < 0x1401_0002 {
            stream_read(e, source, first, 4, argument);
            let value = e.mem.u32(first);
            let flags = (e.mem.u16(map + 4) & 0xcfff) | (value << 12) as u16;
            e.mem.set_u16(map + 4, flags);
            stream_read(e, source, first, 4, argument);
            let value = e.mem.u32(first);
            let flags = (e.mem.u16(map + 4) & 0xf0ff) | (value << 8) as u16;
            e.mem.set_u16(map + 4, flags);
            stream_read(e, source, second, 4, argument);
            let flags = (e.mem.u16(map + 4) & 0xff00) | e.mem.u16(second);
            e.mem.set_u16(map + 4, flags);
        } else {
            stream_read(e, source, map + 4, 2, argument);
        }
        if version(e) < 0x0a03_0004 {
            stream_read(e, source, argument, 2, second);
            stream_read(e, source, argument, 2, second);
        }
        if version(e) < 0x0401_0010 {
            stream_read(e, source, argument, 2, second);
        }
        if version(e) >= 0x0a00_010a {
            e.mem.set_u32(map + 0xc, 0);
            stream_read(e, source, argument, 1, second);
            if e.mem.u8(argument) == 1 {
                let block = memory_allocate(e, 0x48);
                let transform = if block == 0 {
                    0
                } else {
                    fn_00a69b00(e, Ptr::new(block)).addr()
                };
                e.mem.set_u32(map + 0xc, transform);
                e.call(TEXTURE_TRANSFORM_LOAD_BINARY, &args![transform, stream]);
            }
        }
    });
}

// Translated from 00a6a240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::LinkObject` (Xbox PDB; `thiscall`, `RET 4`): the base link step
/// (`00a47360`), then the map count (flag bits 4-11) cleared; for each of the stream's link
/// ids (`GetNumberOfLinkIDs`, asked again each time; nothing for an id when it answers 0)
/// the map at that index gets the linked object as its texture (`GetObjectFromLinkID`,
/// counted), and from index 8 on the map count goes up by one. Streams before
/// `0x14020005` then move the old slot-6 and slot-7 maps behind the others: with either
/// present and a map count, the maps from the last one down to index 8 move up by the
/// number present (`0096ae90` with the address of the element, after growing the array);
/// the slot-6 map goes to index 8 (index 6 cleared, map count + 1) or index 8 is cleared;
/// the slot-7 map goes to index 9 (index 7 cleared, map count + 1), or index 9 is cleared
/// when two were present (which cannot happen without a slot-7 map; kept as in the exe).
/// Streams from `0x05000011` link the shader maps' textures the same way.
pub fn ni_texturing_property_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let at = this.addr();
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    let links = e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    let masked = e.mem.u16(at + FLAGS) & 0xf00f;
    e.mem.set_u16(at + FLAGS, masked);
    for index in 0..links {
        if e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32() == 0 {
            continue;
        }
        let slot = e.mem.u32(at + 0x20).wrapping_add(4 * index);
        let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        let map = e.mem.u32(slot);
        assign(e, map + 8, object);
        if index >= 8 {
            count_one_more_map(e, at);
        }
    }
    if e.mem.u32(stream.addr() + 0xd8) < 0x1402_0005 {
        e.with_stack(0xc, |e, frame| {
            let (sixth, seventh, cleared) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
            let size = e.mem.u16(at + 0x26);
            e.mem.set_u32(sixth, 0);
            e.mem.set_u32(seventh, 0);
            if size > 6 {
                let map = e.mem.u32(e.mem.u32(at + 0x20) + 0x18);
                e.mem.set_u32(sixth, map);
            }
            if size > 7 {
                let map = e.mem.u32(e.mem.u32(at + 0x20) + 0x1c);
                e.mem.set_u32(seventh, map);
            }
            let (has_sixth, has_seventh) = (e.mem.u32(sixth) != 0, e.mem.u32(seventh) != 0);
            if !has_sixth && !has_seventh {
                return;
            }
            let present = u32::from(has_sixth) + u32::from(has_seventh);
            let array: Ptr<NiTArray> = Ptr::new(at + MAPS);
            if e.mem.u16(at + FLAGS) & 0xff0 != 0 && present != 0 {
                let mut from = u32::from(size).wrapping_sub(1);
                if from >= 8 {
                    let mut to = from.wrapping_add(present);
                    loop {
                        let element = e.mem.u32(at + 0x20).wrapping_add(4 * from);
                        array_reserve(e, array, to);
                        e.call(SET_ELEMENT, &args![array, to, element]);
                        from = from.wrapping_sub(1);
                        to = to.wrapping_sub(1);
                        if from < 8 {
                            break;
                        }
                    }
                }
            }
            if has_sixth {
                array_reserve(e, array, 8);
                e.call(SET_ELEMENT, &args![array, 8u32, sixth]);
                e.mem.set_u32(cleared, 0);
                e.call(SET_ELEMENT, &args![array, 6u32, cleared]);
                count_one_more_map(e, at);
            } else {
                e.mem.set_u32(cleared, 0);
                e.call(SET_ELEMENT, &args![array, 8u32, cleared]);
            }
            if has_seventh {
                array_reserve(e, array, 9);
                e.call(SET_ELEMENT, &args![array, 9u32, seventh]);
                e.mem.set_u32(cleared, 0);
                e.call(SET_ELEMENT, &args![array, 7u32, cleared]);
                count_one_more_map(e, at);
            } else if present == 2 {
                e.mem.set_u32(cleared, 0);
                e.call(SET_ELEMENT, &args![array, 9u32, cleared]);
            }
        });
    }
    if e.mem.u32(stream.addr() + 0xd8) >= 0x0500_0011 {
        let links = e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
        for index in 0..links {
            if e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32() == 0 {
                continue;
            }
            let base = e.mem.u32(e.mem.u32(at + SHADER_MAPS) + 4);
            let slot = base.wrapping_add(4 * index);
            let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            let map = e.mem.u32(slot);
            assign(e, map + 8, object);
        }
    }
}

// Translated from 00a6ad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`):
/// `NiProperty::LoadBinary` (`00a5d7f0`); the flag word (before stream version
/// `0x14010002` a 4-byte word put in bits 1-3, `(flags & 0xfff1) | word * 2`; from it the
/// 2-byte word itself, then masked with `0xf00f`); the map count (4 bytes), the map array's
/// capacity set to it (plus 2 for streams before `0x14020005` with more than 6) and
/// `SetNumberOfLinkIDs(count)` (`+1` before `0x0303000d`). For each map a presence flag (a
/// 4-byte word tested for non-zero before `0x04010000`, a byte from it) goes to
/// `SetNumberOfLinkIDs`; a present map is made and loaded (its slot `+4` with the stream; a
/// failed allocation still calls through null, as the exe does). Before `0x0303000d` the
/// maps from index 5 on are stored one index higher, after a `SetNumberOfLinkIDs(0)`.
/// From `0x0303000d` index 5 is a 0x28-byte map (`00a69fb0`) and, from `0x14020005`, index 7
/// a 0x14-byte map (`00a6a010`), both set with `0096ae90`; every other index is a plain
/// 0x10-byte map (initialized inline as `00a69dd0` does) stored inline (capacity checked,
/// counted). From `0x05000011` the shader maps follow: their count (4 bytes, to
/// `SetNumberOfLinkIDs`), a new shader-map array (`00a6a990(count, 1)`, null when the
/// allocation fails) and for each a presence byte and a 0x14-byte shader map (vtable
/// `0109c4ec`) loaded the same way and stored inline (no capacity check). Last, bit 0 of
/// the flag word tells whether a map from index 1 on is present.
pub fn ni_texturing_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (at, source) = (this.addr(), stream.addr());
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    let version = |e: &Engine| e.mem.u32(source + 0xd8);
    let flags_at = at + FLAGS;
    let array: Ptr<NiTArray> = Ptr::new(at + MAPS);
    e.with_stack(0x14, |e, frame| {
        // The locals of the exe's frame: the map being made, the count, a scratch word and
        // a count slot, then the stack slot of the stream argument (holding the pointer).
        let made = frame.addr();
        let count_word = frame.addr() + 4;
        let scratch = frame.addr() + 8;
        let count_slot = frame.addr() + 0xc;
        let argument = frame.addr() + 0x10;
        e.mem.set_u32(argument, source);
        if version(e) < 0x1401_0002 {
            stream_read(e, source, scratch, 4, argument);
            let value = e.mem.u32(scratch);
            let flags = (e.mem.u16(flags_at) & 0xfff1) | value.wrapping_add(value) as u16;
            e.mem.set_u16(flags_at, flags);
        } else {
            stream_read(e, source, flags_at, 2, argument);
            let flags = e.mem.u16(flags_at) & 0xf00f;
            e.mem.set_u16(flags_at, flags);
        }
        stream_read(e, source, count_word, 4, argument);
        let count = e.mem.u32(count_word);
        let capacity = if version(e) < 0x1402_0005 && count > 6 {
            count.wrapping_add(2)
        } else {
            count
        };
        e.call(ARRAY_SET_SIZE, &args![array, capacity]);
        let links = if version(e) < 0x0303_000d {
            e.mem.u32(count_word).wrapping_add(1)
        } else {
            e.mem.u32(count_word)
        };
        e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, links]);
        let mut index = 0u32;
        while index < e.mem.u32(count_word) {
            let mut slot = index;
            e.mem.set_u32(made, 0);
            let present = if version(e) < 0x0401_0000 {
                stream_read(e, source, made, 4, count_slot);
                let present = u8::from(e.mem.u32(made) != 0);
                e.mem.set_u8(argument, present);
                present
            } else {
                stream_read(e, source, argument, 1, count_slot);
                e.mem.u8(argument)
            };
            let current = version(e);
            let mut plain = true;
            if current < 0x0303_000d {
                if index >= 5 {
                    e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, 0u32]);
                    slot = index + 1;
                }
            } else if index == 5 || (index == 7 && current >= 0x1402_0005) {
                plain = false;
                e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, u32::from(present)]);
                if e.mem.u8(argument) != 0 {
                    let (size, constructor) = if index == 5 {
                        (0x28, SLOT_5_MAP_CONSTRUCTOR)
                    } else {
                        (0x14, SLOT_7_MAP_CONSTRUCTOR)
                    };
                    let block = memory_allocate(e, size);
                    let map = if block == 0 {
                        0
                    } else {
                        e.call(constructor, &args![block]).u32()
                    };
                    e.mem.set_u32(made, map);
                    e.vcall(map, 4, &args![stream]);
                }
                e.call(SET_ELEMENT, &args![array, index, made]);
            }
            if plain {
                e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, u32::from(present)]);
                if e.mem.u8(argument) != 0 {
                    let block = memory_allocate(e, 0x10);
                    if block != 0 {
                        e.mem.set_u16(block + 4, 0);
                        e.mem.set_u32(block, MAP_VTABLE);
                        e.mem.set_u32(block + 8, 0);
                        e.mem.set_u32(block + 0xc, 0);
                        let flags = (e.mem.u16(block + 4) & 0xf100) | 0x3100;
                        e.mem.set_u16(block + 4, flags);
                    }
                    e.mem.set_u32(made, block);
                    e.vcall(block, 4, &args![stream]);
                }
                array_reserve(e, array, slot);
                let map = e.mem.u32(made);
                array_set_counted(e, array, slot, map);
            }
            index += 1;
        }
        if version(e) >= 0x0500_0011 {
            stream_read(e, source, count_word, 4, argument);
            let count = e.mem.u32(count_word);
            e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, count]);
            if count != 0 {
                let block = memory_allocate(e, 0x10);
                let shader_maps = if block == 0 {
                    0
                } else {
                    e.call(SHADER_MAP_ARRAY_CONSTRUCTOR, &args![block, count, 1u32])
                        .u32()
                };
                e.mem.set_u32(at + SHADER_MAPS, shader_maps);
                let mut index = 0u32;
                while index < e.mem.u32(count_word) {
                    stream_read(e, source, argument, 1, count_slot);
                    let present = u32::from(e.mem.u8(argument));
                    e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, present]);
                    let mut map = 0u32;
                    if e.mem.u8(argument) != 0 {
                        let block = memory_allocate(e, 0x14);
                        if block != 0 {
                            e.mem.set_u16(block + 4, 0);
                            e.mem.set_u32(block + 8, 0);
                            let flags = (e.mem.u16(block + 4) & 0xf100) | 0x3100;
                            e.mem.set_u32(block + 0xc, 0);
                            e.mem.set_u16(block + 4, flags);
                            e.mem.set_u32(block, SHADER_MAP_VTABLE);
                            e.mem.set_u32(block + 0x10, 0);
                        }
                        map = block;
                        e.vcall(map, 4, &args![stream]);
                    }
                    let target: Ptr<NiTArray> = Ptr::new(e.mem.u32(at + SHADER_MAPS));
                    array_set_counted(e, target, index, map);
                    index += 1;
                }
            }
        }
    });
    let count = u32::from(e.mem.u16(at + 0x26));
    let mut index = 1u32;
    while index < count {
        if e.mem.u32(e.mem.u32(at + 0x20) + 4 * index) != 0 {
            let flags = e.mem.u16(flags_at) | 1;
            e.mem.set_u16(flags_at, flags);
            return;
        }
        index += 1;
    }
    let flags = e.mem.u16(flags_at) & 0xfffe;
    e.mem.set_u16(flags_at, flags);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6ab90, ni_texturing_property_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a6_b410, ni_texturing_property_sdm_init()),
        entry!(0x00a6_aa40, fn_00a6aa40(Ptr) -> Ptr),
        entry!(0x00a6_9dd0, fn_00a69dd0(Ptr)),
        entry!(0x00a6_9e00, fn_00a69e00(Ptr, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x00a6_9b00, fn_00a69b00(Ptr) -> Ptr),
        entry!(0x00a6_a000, fn_00a6a000(Ptr) -> u32),
        entry!(0x00a6_a140, ni_texturing_property_get_rtti() -> Ptr),
        entry!(0x00a6_ad40, fn_00a6ad40() -> Ptr),
        entry!(0x00a6_a0b0, fn_00a6a0b0(Ptr)),
        entry!(0x00a6_a6f0, fn_00a6a6f0()),
        entry!(0x00a6_a8b0, fn_00a6a8b0(Ptr, Ptr) -> Ptr),
        entry!(0x00a6_a9f0, fn_00a6a9f0(Ptr, u32) -> Ptr),
        entry!(0x00a6_abb0, fn_00a6abb0(Ptr, u32, Ptr, u32) -> Ptr),
        entry!(0x00a6_c610, ni_texturing_property_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00a6_bb00, fn_00a6bb00(Ptr, Ptr, u32)),
        entry!(0x00a6_b640, ni_texturing_property_map_load_binary(Ptr, Ptr)),
        entry!(0x00a6_a240, ni_texturing_property_link_object(Ptr, Ptr)),
        entry!(0x00a6_ad60, ni_texturing_property_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_texturing_property_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a6_a0b0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a6ab90, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a6ab90, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x30]]);
        assert_eq!(calls_to(&e, 0x00a6_a0b0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_and_sets_the_ten_static_strings() {
        let run = run_default_instance(0x00a6_b410, TEXTURING_PROPERTY_SLOT, |e| {
            e.register(TEXTURING_PROPERTY_CONSTRUCTOR, |_, a| a[0].into_ret());
            e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
            let empty = e.mem.u32(EMPTY_FIXED_STRING);
            for (slot, _) in TEXTURE_SLOT_NAMES {
                e.mem.set_u32(slot, empty);
            }
            // The fourth global already holds its literal's address.
            e.mem.set_u32(0x011f_49bc, 0x0109_c5dc);
        });
        assert_ne!(run.object, 0);
        assert_eq!(run.engine.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
        for (slot, literal) in TEXTURE_SLOT_NAMES {
            let expected = if slot == 0x011f_49bc {
                literal
            } else {
                literal + 0x1000
            };
            assert_eq!(run.engine.mem.u32(slot), expected);
        }
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a6_b410, TEXTURING_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
            e.register(GET_FIXED_STRING, |_, a| a[0].into_ret());
            let empty = e.mem.u32(EMPTY_FIXED_STRING);
            for (slot, _) in TEXTURE_SLOT_NAMES {
                e.mem.set_u32(slot, empty);
            }
        });
        assert_eq!(run.object, 0);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    /// Runs the constructor with `0096ae90` setting the count word to `count` and the
    /// buffer entries to `entries` (from index 1).
    fn construct(count: u16, entries: &[u32]) -> (Engine, u32, Vec<Vec<u32>>) {
        let mut e = engine_with_counting();
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xcc; 0x40]);
        let entries = entries.to_vec();
        e.register_double(SET_ELEMENT, move |e, a| {
            if a[1] == 8 {
                // The last call: the count and entries become visible.
                e.mem.set_u16(a[0] + 0xa, count);
                let buffer = e.mem.u32(a[0] + 4);
                for (index, value) in entries.iter().enumerate() {
                    e.mem.set_u32(buffer + 4 * (index as u32 + 1), *value);
                }
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_aa40, &args![this]).u32(), this);
        let calls = calls_to(&e, SET_ELEMENT);
        (e, this, calls)
    }

    #[test]
    fn sets_the_part_and_calls_the_element_setter_for_nine_indexes() {
        let (e, this, calls) = construct(1, &[]);
        assert_eq!(calls_to(&e, OBJECT_BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x0109_c514);
        assert_eq!(e.mem.u32(this + 0x1c), 0x0109_c5ac);
        assert_eq!(e.mem.u16(this + 0x1c + 8), 9);
        assert_eq!(e.mem.u16(this + 0x1c + 0xe), 1);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        let indexes: Vec<u32> = calls.iter().map(|c| c[1]).collect();
        assert_eq!(indexes, (0..9).collect::<Vec<u32>>());
        assert!(calls.iter().all(|c| c[0] == this + 0x1c));
    }

    #[test]
    fn flag_word_follows_the_buffer_entries() {
        // count 1: nothing to scan: bit 0 cleared, bit 2 set.
        let (e, this, _) = construct(1, &[]);
        assert_eq!(e.mem.u16(this + 0x18), 0x0004);
        // count 3, all entries zero: same.
        let (e, this, _) = construct(3, &[0, 0]);
        assert_eq!(e.mem.u16(this + 0x18), 0x0004);
        // count 3, the second entry set: bit 0 set.
        let (e, this, _) = construct(3, &[0, 7]);
        assert_eq!(e.mem.u16(this + 0x18), 0x0005);
    }
}

#[cfg(test)]
mod batch_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    const READ: u32 = 0x0300_0000;
    const MAP_LOAD: u32 = 0x0300_1000;
    const DELETE: u32 = 0x0300_2000;
    const TEST_VTABLE: u32 = 0x0300_3000;

    /// The engine with the memory manager, the element setter (it stores the word its third
    /// argument points to) and the other callees doubled.
    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x0101_6000, 0x1000);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SET_ELEMENT, |e, a| {
            let value = e.mem.u32(a[2]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + 4 * a[1], value);
            Ret::default()
        });
        for address in [
            MEMORY_FREE,
            FREE_BLOCK,
            OBJECT_NET_DESTRUCTOR,
            OBJECT_COPY_MEMBERS,
            OBJECT_NET_COPY_MEMBERS,
            ARRAY_SET_SIZE,
            TEXTURE_TRANSFORM_LOAD_BINARY,
            PROPERTY_LOAD_BINARY,
            STREAM_00A645B0,
            SET_NUMBER_OF_LINK_IDS,
            BASE_LINK_OBJECT,
            MAP_LOAD,
            DELETE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(TEXTURE_TRANSFORM_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(SOURCE_TEXTURE_CREATE, |_, _| 0x4400u32.into_ret());
        e.put_vtable(TEST_VTABLE, &[DELETE, DELETE]);
        e.put_vtable(MAP_VTABLE, &[DELETE, MAP_LOAD]);
        e.put_vtable(SHADER_MAP_VTABLE, &[DELETE, MAP_LOAD]);
        e.call_log = Some(vec![]);
        e
    }

    /// A counted object (vtable `TEST_VTABLE`, count `count`).
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let at = e.mem.alloc(0x10);
        e.mem.set_u32(at, TEST_VTABLE);
        e.mem.set_u32(at + 4, count);
        at
    }

    /// A stream of `version` whose reader hands out `words` in order (each read stores the
    /// first `size` bytes of the next word and answers `size`).
    fn stream(e: &mut Engine, version: u32, words: &[u32]) -> u32 {
        let queue = Rc::new(RefCell::new(
            words.iter().copied().collect::<VecDeque<u32>>(),
        ));
        e.register_double(READ, move |e, a| {
            let word = queue.borrow_mut().pop_front().expect("a word to read");
            let bytes = word.to_le_bytes();
            for (offset, byte) in bytes.iter().take(a[2] as usize).enumerate() {
                e.mem.set_u8(a[1] + offset as u32, *byte);
            }
            a[2].into_ret()
        });
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0xd8, version);
        e.mem.set_u32(stream + 0x24c, reader);
        stream
    }

    /// A texturing property with a map array of capacity `capacity` holding `maps`.
    fn property(e: &mut Engine, capacity: u16, maps: &[u32]) -> u32 {
        let this = e.mem.alloc(0x30);
        let elements = e.mem.alloc(4 * u32::from(capacity).max(1));
        for (index, map) in maps.iter().enumerate() {
            e.mem.set_u32(elements + 4 * index as u32, *map);
        }
        e.mem.set_u32(this + MAPS, MAP_ARRAY_VTABLE);
        e.mem.set_u32(this + 0x20, elements);
        e.mem.set_u16(this + 0x24, capacity);
        e.mem.set_u16(this + 0x26, maps.len() as u16);
        e.mem
            .set_u16(this + 0x28, maps.iter().filter(|m| **m != 0).count() as u16);
        e.mem.set_u16(this + 0x2a, 1);
        this
    }

    fn map(e: &mut Engine, texture: u32, transform: u32, size: u32) -> u32 {
        let at = e.mem.alloc(size);
        e.mem.set_u32(at, MAP_VTABLE);
        e.mem.set_u16(at + 4, 0x1234);
        e.mem.set_u32(at + 8, texture);
        e.mem.set_u32(at + 0xc, transform);
        at
    }

    /// The link-id count double: 9 for the first ten questions (the map loop), then 0.
    fn nine_link_ids_then_none(e: &mut Engine) {
        let asked = Rc::new(std::cell::Cell::new(0u32));
        e.register_double(GET_NUMBER_OF_LINK_IDS, move |_, _| {
            asked.set(asked.get() + 1);
            if asked.get() <= 10 { 9u32 } else { 0u32 }.into_ret()
        });
    }

    /// The array set-size double: fresh zeroed elements of the asked capacity.
    fn growing_arrays(e: &mut Engine) {
        e.register(ARRAY_SET_SIZE, |e, a| {
            let elements = e.mem.alloc(4 * a[1].max(1));
            e.mem.set_u32(a[0] + 4, elements);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
    }

    #[test]
    fn the_map_constructors_set_the_flag_word() {
        let mut e = engine();
        let at = e.mem.alloc(0x10);
        e.mem.write(at, &[0xff; 0x10]);
        e.call(0x00a6_9dd0, &args![at]);
        assert_eq!(e.mem.u32(at), MAP_VTABLE);
        assert_eq!(e.mem.u16(at + 4), 0x3100);
        assert_eq!((e.mem.u32(at + 8), e.mem.u32(at + 0xc)), (0, 0));
        let texture = counted(&mut e, 1);
        let answer = e
            .call(0x00a6_9e00, &args![at, texture, 2u32, 3u32, 1u32, 0x77u32])
            .u32();
        assert_eq!(answer, at);
        assert_eq!(e.mem.u16(at + 4), 0x3102);
        assert_eq!(e.mem.u32(at + 8), texture);
        assert_eq!(e.mem.u32(texture + 4), 2);
        assert_eq!(e.mem.u32(at + 0xc), 0x77);
        e.call(0x00a6_9e00, &args![at, 0u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!((e.mem.u32(at + 8), e.mem.u16(at + 4)), (0, 0));
    }

    #[test]
    fn a_new_transform_has_its_defaults() {
        let mut e = engine();
        e.mem.set_u32(ZERO_POINT_2, 0x11);
        e.mem.set_u32(ZERO_POINT_2 + 4, 0x22);
        e.mem.set_f32(TRANSFORM_CENTRE, 0.5);
        let at = e.mem.alloc(0x48);
        assert_eq!(e.call(0x00a6_9b00, &args![at]).u32(), at);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (0x11, 0x22));
        let floats: Vec<f32> = (2..7).map(|i| e.mem.f32(at + 4 * i)).collect();
        assert_eq!(floats, vec![0.0, 1.0, 1.0, 0.5, 0.5]);
        assert_eq!(e.mem.u8(at + 0x1c), 1);
        assert_eq!(e.mem.u32(at + 0x44), 2);
    }

    #[test]
    fn the_constant_slots() {
        let mut e = engine();
        assert_eq!(e.call(0x00a6_a000, &args![0x5000u32]).u32(), 3);
        assert_eq!(e.call(0x00a6_a140, &args![]).u32(), TEXTURING_PROPERTY_RTTI);
    }

    #[test]
    fn the_factory_constructs_a_property() {
        let mut e = engine();
        let object = e.call(0x00a6_ad40, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), TEXTURING_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(object + FLAGS), 4);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a6_ad40, &args![]).u32(), 0);
    }

    #[test]
    fn the_destructor_deletes_the_maps_and_the_shader_maps() {
        let mut e = engine();
        let first = map(&mut e, 0, 0, 0x10);
        let second = map(&mut e, 0, 0, 0x10);
        let this = property(&mut e, 4, &[first, 0, second]);
        let shader = map(&mut e, 0, 0, 0x14);
        let shader_array = e.mem.alloc(0x10);
        let shader_elements = e.mem.alloc(8);
        e.mem.set_u32(shader_array, TEST_VTABLE);
        e.mem.set_u32(shader_array + 4, shader_elements);
        e.mem.set_u16(shader_array + 0xa, 2);
        e.mem.set_u32(shader_elements + 4, shader);
        e.mem.set_u32(this + SHADER_MAPS, shader_array);
        let elements = e.mem.u32(this + 0x20);
        e.call(0x00a6_a0b0, &args![this]);
        let deleted: Vec<u32> = calls_to(&e, DELETE).iter().map(|c| c[0]).collect();
        assert_eq!(deleted, vec![first, second, shader, shader_array]);
        assert!(calls_to(&e, DELETE).iter().all(|c| c[1] == 1));
        assert_eq!(e.mem.u32(this), TEXTURING_PROPERTY_VTABLE);
        assert_eq!(e.mem.u32(this + MAPS), MAP_ARRAY_BASE_VTABLE);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![elements]]);
        assert_eq!(calls_to(&e, OBJECT_NET_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn the_shutdown_releases_the_default_and_the_strings() {
        let mut e = engine();
        e.register(GET_FIXED_STRING, |_, _| 0x0109_b228u32.into_ret());
        let object = counted(&mut e, 1);
        e.mem.set_u32(TEXTURING_PROPERTY_SLOT, object);
        let handle = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(handle - 8, 30);
        for (index, (slot, _)) in TEXTURE_SLOT_NAMES.iter().enumerate() {
            let value = if index == 2 { 0 } else { handle };
            e.mem.set_u32(*slot, value);
        }
        e.call(0x00a6_a6f0, &args![]);
        assert_eq!(e.mem.u32(TEXTURING_PROPERTY_SLOT), 0);
        assert_eq!(calls_to(&e, DELETE), vec![vec![object]]);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0]; 9]);
        assert_eq!(e.mem.u32(TEXTURE_SLOT_NAMES[2].0), 0);
        assert_eq!(e.mem.u32(TEXTURE_SLOT_NAMES[0].0), 0x0109_b228);
        assert_eq!(e.mem.u32(handle - 8), 21);
    }

    #[test]
    fn the_map_copy_copies_the_transform_through_its_constructor() {
        let mut e = engine();
        let texture = counted(&mut e, 1);
        let transform = e.mem.alloc(0x48);
        for word in 0..0x12 {
            e.mem.set_u32(transform + 4 * word, 0x100 + word);
        }
        e.mem.set_f32(transform + 8, 0.25);
        let source = map(&mut e, texture, transform, 0x10);
        let this = e.mem.alloc(0x10);
        e.register(TEXTURE_TRANSFORM_CONSTRUCTOR, |e, a| {
            // translate, rotation, scale, centre, method
            assert_eq!((e.mem.u32(a[1]), e.mem.u32(a[1] + 4)), (0x100, 0x101));
            assert_eq!(f32::from_bits(a[2]), 0.25);
            assert_eq!((e.mem.u32(a[3]), e.mem.u32(a[3] + 4)), (0x103, 0x104));
            assert_eq!((e.mem.u32(a[4]), e.mem.u32(a[4] + 4)), (0x105, 0x106));
            assert_eq!(a[5], 0x111);
            0x9900u32.into_ret()
        });
        assert_eq!(e.call(0x00a6_a8b0, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u32(this), MAP_VTABLE);
        assert_eq!(e.mem.u16(this + 4), 0x1234);
        assert_eq!(e.mem.u32(this + 8), texture);
        assert_eq!(e.mem.u32(texture + 4), 2);
        assert_eq!(e.mem.u32(this + 0xc), 0x9900);
        // Without a transform, or when the allocation fails: none.
        let plain = map(&mut e, 0, 0, 0x10);
        e.call(0x00a6_a8b0, &args![this, plain]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00a6_a8b0, &args![this, source]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn the_map_deleting_destructor_frees_through_the_memory_manager() {
        let mut e = engine();
        let texture = counted(&mut e, 1);
        let this = map(&mut e, texture, 0x6600, 0x10);
        assert_eq!(e.call(0x00a6_a9f0, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls_to(&e, MEMORY_FREE),
            vec![vec![MEMORY_MANAGER, 0x6600], vec![MEMORY_MANAGER, this]]
        );
        assert_eq!(calls_to(&e, DELETE), vec![vec![texture]]);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00a6_a9f0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE).len(), 3);
    }

    #[test]
    fn the_file_constructor_puts_the_texture_in_map_0() {
        let mut e = engine();
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x5150);
        let this = e.mem.alloc(0x30);
        let texture = counted(&mut e, 1);
        e.register_double(SOURCE_TEXTURE_CREATE, move |_, _| texture.into_ret());
        // The setter also leaves the size at 9 as the real one would.
        e.register(SET_ELEMENT, |e, a| {
            let value = e.mem.u32(a[2]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, 9);
            Ret::default()
        });
        let answer = e.call(0x00a6_abb0, &args![this, 7u32, name, 9u32]).u32();
        assert_eq!(answer, this);
        assert_eq!(
            calls_to(&e, SOURCE_TEXTURE_CREATE),
            vec![vec![7, 0x5150, 9]]
        );
        let first = e.mem.u32(e.mem.u32(this + 0x20));
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(first + 8), texture);
        assert_eq!(e.mem.u16(first + 4), 0x3100);
        let indexes: Vec<u32> = calls_to(&e, SET_ELEMENT).iter().map(|c| c[1]).collect();
        assert_eq!(indexes, (0..9).collect::<Vec<u32>>());
        // Only map 0 is set: bit 0 stays clear.
        assert_eq!(e.mem.u16(this + FLAGS), 4);
        assert_eq!(e.mem.u32(this), TEXTURING_PROPERTY_VTABLE);
    }

    #[test]
    fn a_property_without_controllers_is_its_own_clone() {
        let mut e = engine();
        let this = property(&mut e, 2, &[]);
        e.mem.set_u32(this + 0xc, 0);
        assert_eq!(e.call(0x00a6_c610, &args![this, 3u32]).u32(), this);
        assert_eq!(calls_to(&e, OBJECT_COPY_MEMBERS), vec![vec![this, this, 3]]);
    }

    #[test]
    fn a_clone_copies_the_maps_by_kind() {
        let mut e = engine();
        growing_arrays(&mut e);
        let texture = counted(&mut e, 1);
        let plain = map(&mut e, texture, 0, 0x10);
        let fifth = map(&mut e, 0, 0, 0x28);
        for at in (0x10..0x28).step_by(4) {
            e.mem.set_f32(fifth + at, at as f32);
        }
        let seventh = map(&mut e, 0, 0, 0x14);
        e.mem.set_f32(seventh + 0x10, 7.5);
        let this = property(&mut e, 8, &[plain, 0, 0, 0, 0, fifth, 0, seventh]);
        e.mem.set_u32(this + 0xc, 0x1111);
        let shader = map(&mut e, 0, 0, 0x14);
        e.mem.set_u32(shader + 0x10, 0xabc);
        let shader_array = e.mem.alloc(0x10);
        let shader_elements = e.mem.alloc(8);
        e.mem.set_u32(shader_array + 4, shader_elements);
        e.mem.set_u16(shader_array + 0xa, 2);
        e.mem.set_u32(shader_elements + 4, shader);
        e.mem.set_u32(this + SHADER_MAPS, shader_array);
        let clone = e.call(0x00a6_c610, &args![this, 3u32]).u32();
        assert_ne!(clone, this);
        assert_eq!(e.mem.u32(clone), TEXTURING_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(clone + FLAGS), e.mem.u16(this + FLAGS));
        let elements = e.mem.u32(clone + 0x20);
        let copies: Vec<u32> = (0..8).map(|i| e.mem.u32(elements + 4 * i)).collect();
        assert_eq!(e.mem.u32(copies[0] + 8), texture);
        assert_eq!(e.mem.u32(copies[0]), MAP_VTABLE);
        assert_eq!(e.mem.u32(copies[5]), SLOT_5_MAP_VTABLE);
        assert_eq!(e.mem.f32(copies[5] + 0x24), 36.0);
        assert_eq!(e.mem.u32(copies[7]), SLOT_7_MAP_VTABLE);
        assert_eq!(e.mem.f32(copies[7] + 0x10), 7.5);
        assert_eq!(copies[1], 0);
        let shader_copy_array = e.mem.u32(clone + SHADER_MAPS);
        assert_ne!(shader_copy_array, 0);
        assert_eq!(e.mem.u16(shader_copy_array + 0xa), 2);
        assert_eq!(e.mem.u16(shader_copy_array + 0xc), 1);
        let copy = e.mem.u32(e.mem.u32(shader_copy_array + 4) + 4);
        assert_eq!(e.mem.u32(copy), SHADER_MAP_VTABLE);
        assert_eq!(e.mem.u32(copy + 0x10), 0xabc);
    }

    #[test]
    fn a_map_from_an_old_stream_builds_its_flags_from_three_words() {
        let mut e = engine();
        let at = map(&mut e, 0, 0x55, 0x10);
        e.mem.set_u16(at + 4, 0xffff);
        let s = stream(&mut e, 0x0400_0000, &[0x1, 0x7, 0x42, 0, 0, 0]);
        e.call(0x00a6_b640, &args![at, s]);
        assert_eq!(e.mem.u16(at + 4), 0xd742);
        assert_eq!(e.mem.u32(at + 0xc), 0x55);
        assert_eq!(calls_to(&e, READ).len(), 6);
    }

    #[test]
    fn a_map_from_a_new_stream_reads_its_flags_and_maybe_a_transform() {
        let mut e = engine();
        let at = map(&mut e, 0, 0x55, 0x10);
        let s = stream(&mut e, 0x1402_0007, &[0x2233, 1]);
        e.call(0x00a6_b640, &args![at, s]);
        assert_eq!(e.mem.u16(at + 4), 0x2233);
        let transform = e.mem.u32(at + 0xc);
        assert_ne!(transform, 0);
        assert_eq!(e.mem.u32(transform + 0x44), 2);
        assert_eq!(
            calls_to(&e, TEXTURE_TRANSFORM_LOAD_BINARY),
            vec![vec![transform, s]]
        );
        let s = stream(&mut e, 0x1402_0007, &[0x2233, 0]);
        e.call(0x00a6_b640, &args![at, s]);
        assert_eq!(e.mem.u32(at + 0xc), 0);
    }

    #[test]
    fn linking_gives_each_map_its_texture_and_counts_the_extra_maps() {
        let mut e = engine();
        let textures: Vec<u32> = (0..9).map(|_| counted(&mut e, 1)).collect();
        let maps: Vec<u32> = (0..9).map(|_| map(&mut e, 0, 0, 0x10)).collect();
        let this = property(&mut e, 9, &maps);
        e.mem.set_u16(this + FLAGS, 0xffff);
        let s = stream(&mut e, 0x1402_0005, &[]);
        let next = Rc::new(RefCell::new(textures.clone().into_iter()));
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| {
            next.borrow_mut().next().unwrap().into_ret()
        });
        nine_link_ids_then_none(&mut e);
        e.call(0x00a6_a240, &args![this, s]);
        for (map, texture) in maps.iter().zip(&textures) {
            assert_eq!(e.mem.u32(map + 8), *texture);
            assert_eq!(e.mem.u32(texture + 4), 2);
        }
        // Index 8 counted once in bits 4-11.
        assert_eq!(e.mem.u16(this + FLAGS), 0xf01f);
    }

    #[test]
    fn linking_an_old_stream_moves_the_slot_6_and_7_maps() {
        let mut e = engine();
        let maps: Vec<u32> = (0..9).map(|_| map(&mut e, 0, 0, 0x10)).collect();
        let this = property(&mut e, 12, &maps);
        let s = stream(&mut e, 0x1402_0004, &[]);
        // Nine link ids, all null textures: index 8 counts one extra map.
        nine_link_ids_then_none(&mut e);
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| 0u32.into_ret());
        e.call(0x00a6_a240, &args![this, s]);
        let sets: Vec<u32> = calls_to(&e, SET_ELEMENT).iter().map(|c| c[1]).collect();
        // Map 8 moves up by two (to 10), map 6 goes to 8 (6 cleared) and map 7 to 9
        // (7 cleared).
        assert_eq!(sets, vec![10, 8, 6, 9, 7]);
        let moved = calls_to(&e, SET_ELEMENT)[0][2];
        assert_eq!(moved, e.mem.u32(this + 0x20) + 0x20);
        let elements = e.mem.u32(this + 0x20);
        assert_eq!(e.mem.u32(elements + 0x28), maps[8]);
        assert_eq!(e.mem.u32(elements + 0x20), maps[6]);
        assert_eq!(e.mem.u32(elements + 0x24), maps[7]);
        assert_eq!(e.mem.u32(elements + 0x18), 0);
        assert_eq!(e.mem.u16(this + FLAGS), 0x0030);
    }

    #[test]
    fn loading_reads_the_maps_and_the_shader_maps() {
        let mut e = engine();
        growing_arrays(&mut e);
        let this = property(&mut e, 0, &[]);
        e.register(SHADER_MAP_ARRAY_CONSTRUCTOR, |e, a| {
            let elements = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 4, elements);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            a[0].into_ret()
        });
        // Flags, two maps (the first absent, the second present), two shader maps (the
        // second present).
        let s = stream(&mut e, 0x1402_0007, &[0xff7f, 2, 0, 1, 2, 0, 1]);
        e.call(0x00a6_ad60, &args![this, s]);
        assert_eq!(calls_to(&e, PROPERTY_LOAD_BINARY), vec![vec![this, s]]);
        let links: Vec<u32> = calls_to(&e, SET_NUMBER_OF_LINK_IDS)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(links, vec![2, 0, 1, 2, 0, 1]);
        let elements = e.mem.u32(this + 0x20);
        let second = e.mem.u32(elements + 4);
        assert_eq!(e.mem.u32(elements), 0);
        assert_eq!(e.mem.u32(second), MAP_VTABLE);
        assert_eq!(e.mem.u16(this + 0x26), 2);
        assert_eq!(e.mem.u16(this + 0x28), 1);
        let shader_array = e.mem.u32(this + SHADER_MAPS);
        assert_eq!(e.mem.u16(shader_array + 0xa), 2);
        assert_eq!(e.mem.u16(shader_array + 0xc), 1);
        let shader = e.mem.u32(e.mem.u32(shader_array + 4) + 4);
        assert_eq!(e.mem.u32(shader), SHADER_MAP_VTABLE);
        let loads: Vec<u32> = calls_to(&e, MAP_LOAD).iter().map(|c| c[0]).collect();
        assert_eq!(loads, vec![second, shader]);
        // Flags masked with 0xf00f, then bit 0 set for the map at index 1.
        assert_eq!(e.mem.u16(this + FLAGS), 0xf00f);
    }

    #[test]
    fn loading_an_old_stream_puts_the_flag_word_in_bits_1_to_3() {
        let mut e = engine();
        growing_arrays(&mut e);
        let this = property(&mut e, 0, &[]);
        e.mem.set_u16(this + FLAGS, 0xffff);
        // Flags 5, one map (absent, a 4-byte word in a stream before 0x04010000).
        let s = stream(&mut e, 0x0303_0000, &[5, 1, 0]);
        e.call(0x00a6_ad60, &args![this, s]);
        let links: Vec<u32> = calls_to(&e, SET_NUMBER_OF_LINK_IDS)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(links, vec![2, 0]);
        assert_eq!(e.mem.u16(this + FLAGS), 0xfffa);
        assert_eq!(e.mem.u16(this + 0x26), 1);
    }
}
