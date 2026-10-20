//! `fallout shared/ragdolldata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `RagDollData`: the saved bone poses of a reference's ragdoll (`XRGD`,
//! 0x1c bytes per bone) and the `Bip01` rotation (`XRGB`), and their
//! application back to the reference's 3D.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `RagDollData` (Xbox PDB), 0x14 bytes (the same on PC).
    pub struct RagDollData: 0x14 {
        /// `iBoneCount` (Xbox PDB).
        0x00 iBoneCount: u8,
        /// `pBoneArray` (Xbox PDB): `RagDollBone*`, 0x1c bytes each
        /// (`iBoneID` at +0, `BonePos` at +4, `BoneRot` at +0x10).
        0x04 pBoneArray: u32,
        /// `Bip01Rot` (Xbox PDB): `NiPoint3`, x at +8.
        0x08 Bip01Rot: f32,
    }
}

/// The size of a `RagDollBone` (Xbox PDB and PC).
const BONE_SIZE: u32 = 0x1c;
/// `006815c0` (`ECX` = point): run on a point before it is set.
const POINT_PREPARE: u32 = 0x0068_15c0;
/// The point `Bip01Rot` starts at (`011f426c`, three floats).
const ORIGIN: u32 = 0x011f_426c;
/// `00401000` (cdecl: size): allocates.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (cdecl: block): frees it.
const FREE: u32 = 0x0040_1030;
/// `00401050` (stdcall: array, element size, count, constructor): runs the
/// constructor on each element.
const CONSTRUCT_ARRAY: u32 = 0x0040_1050;
/// `0055a400`, the per-bone constructor `00401050` is given.
const BONE_CONSTRUCT: u32 = 0x0055_a400;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`; `ECX` =
/// file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401680` (`ECX` = file): whether the data read needs its byte order
/// swapped.
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// `004d9280` (`ECX` = bone): the bone's byte swap.
const BONE_SWAP: u32 = 0x004d_9280;
/// `00401080` (cdecl: the float, a byte): a float's byte swap.
const FLOAT_SWAP: u32 = 0x0040_1080;
/// The chunk tags.
const CHUNK_XRGD: u32 = 0x4447_5258;
const CHUNK_XRGB: u32 = 0x4247_5258;
/// `0043fcd0` (`ECX` = reference): its 3D root.
const REFERENCE_ROOT: u32 = 0x0043_fcd0;
/// `004da230` (`ECX` = ragdoll data, the reference; `RET 4`).
const ACTOR_PREPARE: u32 = 0x004d_a230;
/// `0043c490` (`ECX` = node).
const NODE_43C490: u32 = 0x0043_c490;
/// `00439090` (`ECX` = `0043c490`'s result, a word; `RET 4`): a byte.
const COMPARE_439090: u32 = 0x0043_9090;
/// `TESObjectREFR::GetOrientation` (Xbox PDB name of `0056fa00`; `ECX` =
/// reference, the matrix to fill; `RET 4`): returns the matrix.
const GET_ORIENTATION: u32 = 0x0056_fa00;
/// `006a9540` (`ECX` = node): its rotation matrix.
const NODE_ROTATION: u32 = 0x006a_9540;
/// `00440460` (`ECX` = node, a point; `RET 4`).
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
/// `0043fa80` (`ECX` = node, a matrix; `RET 4`).
const NODE_SET_ROTATE: u32 = 0x0043_fa80;
/// `bhkNiCollisionObject::ResetSim` (Xbox PDB name of `00c6bd00`; cdecl:
/// node, a word).
const RESET_SIM: u32 = 0x00c6_bd00;
/// `0043d410` (`ECX` = update data; a float, two words; `RET 0xc`).
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// `00a59c60` (`ECX` = node, the update data; `RET 4`).
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `bhkWorld::SetMotion` (Xbox PDB name of `00c6a350`; cdecl, five words).
const SET_MOTION: u32 = 0x00c6_a350;
/// `004a8b00` (cdecl: node).
const NODE_4A8B00: u32 = 0x004a_8b00;
/// `006fa820` (`ECX` = `004a8b00`'s result).
const OBJECT_6FA820: u32 = 0x006f_a820;
/// `0043b4f0` (`ECX` = `006fa820`'s result, a word; `RET 4`).
const OBJECT_43B4F0: u32 = 0x0043_b4f0;
/// `NiMatrix3::FromEulerAnglesXYZ` (Xbox PDB name of `00a59540`; `ECX` =
/// matrix; three floats; `RET 0xc`).
const FROM_EULER: u32 = 0x00a5_9540;
/// `0043b480` (`ECX` = node): its child count.
const CHILD_COUNT: u32 = 0x0043_b480;
/// `0043b4a0` (`ECX` = node, an index; `RET 4`): a child.
const CHILD_AT: u32 = 0x0043_b4a0;

// Translated from 004d9330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RagDollData::RagDollData` (Xbox PDB): runs `006815c0` on `Bip01Rot`,
/// clears the bone count and array and sets `Bip01Rot` to the point at
/// `011f426c`. Returns `this`.
pub fn rag_doll_data_rag_doll_data(e: &mut Engine, this: Ptr<RagDollData>) -> Ptr<RagDollData> {
    let base = this.addr();
    e.call(POINT_PREPARE, &args![base + 8]);
    e.set(this, RagDollData::iBoneCount, 0);
    e.set(this, RagDollData::pBoneArray, 0);
    for i in 0..3 {
        let word = e.mem.u32(ORIGIN + 4 * i);
        e.mem.set_u32(base + 8 + 4 * i, word);
    }
    this
}

// Translated from 004d9380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the bone array (`00401030`) and clears the count and the array.
/// The engine map gives it no name.
pub fn fn_004d9380(e: &mut Engine, this: Ptr<RagDollData>) {
    let bones = e.get(this, RagDollData::pBoneArray);
    e.call(FREE, &args![bones]);
    e.set(this, RagDollData::iBoneCount, 0);
    e.set(this, RagDollData::pBoneArray, 0);
}

// Translated from 004d94a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d94a0 - the exception-unwinding frame (FS:[0]) around the bone array's allocation is not translated.
/// Reads the current chunk (`thiscall`, the file; `RET 4`; the engine map
/// gives it no name):
/// - `XRGD`: the bone count is the chunk's size over 0x1c (as a byte); when
///   the size divides by that count, an array of that many bones (0x1c
///   bytes each, all ones on overflow) is allocated and each bone
///   constructed (`00401050` with `0055a400`; no array when the allocation
///   fails), the chunk read into it, and each bone byte-swapped
///   (`004d9280`) when the file needs it; otherwise the count goes back to
///   0. A chunk shorter than one bone makes the count 0 and the exe's `DIV`
///   by it faults; so does this translation (Rust's remainder by zero).
/// - Then, on the same chunk, `XRGB`: 12 bytes into `Bip01Rot`, each float
///   byte-swapped (`00401080` with 0) when the file needs it.
///
/// Returns 1 when either chunk was read.
pub fn fn_004d94a0(e: &mut Engine, this: Ptr<RagDollData>, file: Ptr) -> u8 {
    let base = this.addr();
    let mut read = 0u8;
    if e.call(GET_TES_CHUNK, &args![file]).u32() == CHUNK_XRGD {
        let size = e.call(CHUNK_SIZE, &args![file]).u32();
        e.set(this, RagDollData::iBoneCount, (size / BONE_SIZE) as u8);
        let count = u32::from(e.get(this, RagDollData::iBoneCount));
        // `DIV` by the count: a count of 0 faults, as Rust's `%` panics.
        let remainder = size % count;
        if remainder == 0 {
            let count = u32::from(e.get(this, RagDollData::iBoneCount));
            let bytes = count.checked_mul(BONE_SIZE).unwrap_or(u32::MAX);
            let block = e.call(ALLOCATE, &args![bytes]).u32();
            let bones = if block != 0 {
                e.call(
                    CONSTRUCT_ARRAY,
                    &args![block, BONE_SIZE, count, BONE_CONSTRUCT],
                );
                block
            } else {
                0
            };
            e.set(this, RagDollData::pBoneArray, bones);
            let bones = e.get(this, RagDollData::pBoneArray);
            e.call(GET_CHUNK_DATA, &args![file, bones, size]);
            if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                let mut index = 0u32;
                while index < u32::from(e.get(this, RagDollData::iBoneCount)) {
                    let bones = e.get(this, RagDollData::pBoneArray);
                    e.call(
                        BONE_SWAP,
                        &args![index.wrapping_mul(BONE_SIZE).wrapping_add(bones)],
                    );
                    index += 1;
                }
            }
            read = 1;
        } else {
            e.set(this, RagDollData::iBoneCount, 0);
        }
    }
    if e.call(GET_TES_CHUNK, &args![file]).u32() == CHUNK_XRGB {
        e.call(GET_CHUNK_DATA, &args![file, base + 8, 0xcu32]);
        if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
            for offset in [8, 0xc, 0x10] {
                e.call(FLOAT_SWAP, &args![base + offset, 0u32]);
            }
        }
        read = 1;
    }
    read
}

// Translated from 004d9910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RagDollData::UpdateReferenceFromData` (Xbox PDB; the reference; `RET
/// 4`). With a reference that has a 3D root (`0043fcd0`) and saved bones:
/// the root's slot `0xc` gives the object the bones start from (0 without
/// a root); for an actor (the reference's slot `0x100`) `004da230` runs
/// first; the bones are applied from that object down (`004d9dc0`,
/// counting the bones placed in a stack word). With a root, the root is
/// moved to the reference unless both `00439090` (on the root's `0043c490`,
/// with the reference's slot `0x1f4`) and `004d9ab0` (the root's rotation
/// `006a9540` against the reference's orientation, `GetOrientation`)
/// report false; moving sets the translate (`00440460` with slot `0x1f4`) and
/// the rotation (`0043fa80` with the orientation), resets the simulation
/// (`ResetSim` with 1) and updates the root (`00a59c60` with update data
/// from `0043d410(0.0, 0, 0)`). When a bone failed,
/// `bhkWorld::SetMotion(object, 1, 1, 0, 1)`.
///
/// True when the bones placed equal the count saved and none failed.
pub fn rag_doll_data_update_reference_from_data(
    e: &mut Engine,
    this: Ptr<RagDollData>,
    reference: Ptr,
) -> bool {
    e.with_stack(4, |e, placed| {
        e.mem.set_u32(placed.addr(), 0);
        let mut clean = 1u8;
        if !reference.is_null()
            && e.call(REFERENCE_ROOT, &args![reference]).u32() != 0
            && e.get(this, RagDollData::iBoneCount) != 0
        {
            let root = e.call(REFERENCE_ROOT, &args![reference]).u32();
            let object = if root != 0 {
                e.vcall(root, 0xc, &[]).u32()
            } else {
                0
            };
            if e.vcall(reference.addr(), 0x100, &[]).bool() {
                e.call(ACTOR_PREPARE, &args![this, reference]);
            }
            clean = fn_004d9dc0(e, this, Ptr::new(object), placed);
            if root != 0 {
                let position = e.vcall(reference.addr(), 0x1f4, &[]).u32();
                let node = e.call(NODE_43C490, &args![root]).u32();
                let mut move_root = true;
                if !e.call(COMPARE_439090, &args![node, position]).bool() {
                    move_root = e.with_stack(0x24, |e, matrix| {
                        let orientation = e.call(GET_ORIENTATION, &args![reference, matrix]).u32();
                        let rotation = e.call(NODE_ROTATION, &args![root]).u32();
                        fn_004d9ab0(e, Ptr::new(rotation), Ptr::new(orientation))
                    });
                }
                if move_root {
                    let position = e.vcall(reference.addr(), 0x1f4, &[]).u32();
                    e.call(NODE_SET_TRANSLATE, &args![root, position]);
                    e.with_stack(0x24, |e, matrix| {
                        let orientation = e.call(GET_ORIENTATION, &args![reference, matrix]).u32();
                        e.call(NODE_SET_ROTATE, &args![root, orientation]);
                    });
                    e.call(RESET_SIM, &args![root, 1u32]);
                    e.with_stack(0xc, |e, update| {
                        e.call(
                            UPDATE_DATA_CONSTRUCT,
                            &args![update, x87_store32(0.0), 0u32, 0u32],
                        );
                        e.call(NODE_UPDATE, &args![root, update]);
                    });
                }
            }
            if clean == 0 {
                e.call(SET_MOTION, &args![object, 1u32, 1u32, 0u32, 1u32]);
            }
        }
        let count = u32::from(e.get(this, RagDollData::iBoneCount));
        e.mem.u32(placed.addr()) == count && clean != 0
    })
}

// Translated from 004d9ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the two matrices differ: the negation of `004d9ae0` (`this`,
/// the other; `RET 4`) as 0 or 1. The engine map gives it no name.
pub fn fn_004d9ab0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    !ni_matrix3_operator_eq(e, this, other)
}

// Translated from 004d9ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::operator==` (the engine map's `NiMatrix3::operator_`; the
/// body compares): true when all nine floats are equal (`FUCOMPP`, so a
/// NaN is never equal and 0.0 equals -0.0), in order, stopping at the
/// first that is not.
pub fn ni_matrix3_operator_eq(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    for i in 0..9 {
        let mine = f32::from_bits(e.mem.u32(this.addr() + 4 * i));
        let theirs = f32::from_bits(e.mem.u32(other.addr() + 4 * i));
        if theirs != mine {
            return false;
        }
    }
    true
}

// Translated from 004d9da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bits 8 to 12 of the word at `this` (`>> 8 & 0x1f`). The engine map gives
/// it no name.
pub fn fn_004d9da0(e: &mut Engine, this: Ptr) -> u32 {
    (e.mem.u32(this.addr()) >> 8) & 0x1f
}

// Translated from 004d9dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the saved bones to a node and its children (`thiscall`; the
/// node, the address of the count of bones placed; `RET 8`; the engine map
/// gives it no name). With a node and bones left: when `004a8b00` gives
/// an object for the node, the value `004d9da0` reads from `0043b4f0` (with
/// a stack word) of the object's `006fa820` must match the next saved
/// bone's `iBoneID` (signed), else the result is 0 at once; the node gets
/// the bone's position (`00440460`) and the rotation built from its three
/// angles (`NiMatrix3::FromEulerAnglesXYZ` into a stack matrix,
/// `0043fa80`); the object gets `004d9fa0` with 1 and its slot `0xb0`
/// with (1, 0, 0), and the count goes up. Then the node is updated (its slot
/// `0xb8` with update data `0043d410(0.0, 0, 0)`) and each child's slot
/// `0xc` object, when there is one, is visited in turn (a failure makes the
/// result 0), stopping once all bones are placed. The result is 1 unless a
/// bone failed.
pub fn fn_004d9dc0(e: &mut Engine, this: Ptr<RagDollData>, node: Ptr, placed: Ptr) -> u8 {
    let mut result = 1u8;
    if node.is_null() {
        return result;
    }
    if e.mem.u32(placed.addr()) >= u32::from(e.get(this, RagDollData::iBoneCount)) {
        return result;
    }
    let body = e.call(NODE_4A8B00, &args![node]).u32();
    if body != 0 {
        let id = e.with_stack(4, |e, word| {
            let object = e.call(OBJECT_6FA820, &args![body]).u32();
            let info = e.call(OBJECT_43B4F0, &args![object, word]).u32();
            fn_004d9da0(e, Ptr::new(info))
        });
        let bone = |e: &Engine| {
            e.mem
                .u32(placed.addr())
                .wrapping_mul(BONE_SIZE)
                .wrapping_add(e.get(this, RagDollData::pBoneArray))
        };
        let saved = i32::from(e.mem.u8(bone(e)) as i8);
        if saved != id as i32 {
            return 0;
        }
        let position = bone(e) + 4;
        e.call(NODE_SET_TRANSLATE, &args![node, position]);
        e.with_stack(0x24, |e, matrix| {
            e.call(POINT_PREPARE, &args![matrix]);
            let x = x87_quiet_bits(e.mem.u32(bone(e) + 0x10));
            let y = x87_quiet_bits(e.mem.u32(bone(e) + 0x14));
            let z = x87_quiet_bits(e.mem.u32(bone(e) + 0x18));
            e.call(FROM_EULER, &args![matrix, x, y, z]);
            e.call(NODE_SET_ROTATE, &args![node, matrix]);
        });
        fn_004d9fa0(e, Ptr::new(body), 1);
        e.vcall(body, 0xb0, &args![1u32, 0u32, 0u32]);
        let count = e.mem.u32(placed.addr()).wrapping_add(1);
        e.mem.set_u32(placed.addr(), count);
    }
    e.with_stack(0xc, |e, update| {
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![update, x87_store32(0.0), 0u32, 0u32],
        );
        e.vcall(node.addr(), 0xb8, &args![update]);
    });
    let children = e.call(CHILD_COUNT, &args![node]).u32();
    let mut index = 0u32;
    while index < children {
        let child = e.call(CHILD_AT, &args![node, index]).u32();
        let object = if child != 0 {
            e.vcall(child, 0xc, &[]).u32()
        } else {
            0
        };
        if object != 0 && fn_004d9dc0(e, this, Ptr::new(object), placed) == 0 {
            result = 0;
        }
        if e.mem.u32(placed.addr()) >= u32::from(e.get(this, RagDollData::iBoneCount)) {
            break;
        }
        index += 1;
    }
    result
}

// Translated from 004d9fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004d9fc0` with the flag byte and the mask 0x40. The engine map gives it
/// no name.
pub fn fn_004d9fa0(e: &mut Engine, this: Ptr, flag: u8) {
    fn_004d9fc0(e, this, flag, 0x40);
}

// Translated from 004d9fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (nonzero flag byte) or clears the bits of `mask` in the 16-bit
/// word at `this + 0xc` (`RET 8`). The engine map gives it no name.
pub fn fn_004d9fc0(e: &mut Engine, this: Ptr, flag: u8, mask: u16) {
    let flags = e.mem.u16(this.addr() + 0xc);
    let flags = if flag != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.mem.set_u16(this.addr() + 0xc, flags);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004d_9330,
            rag_doll_data_rag_doll_data(Ptr<RagDollData>) -> Ptr<RagDollData>
        ),
        entry!(0x004d_9380, fn_004d9380(Ptr<RagDollData>)),
        entry!(0x004d_94a0, fn_004d94a0(Ptr<RagDollData>, Ptr) -> u8),
        entry!(
            0x004d_9910,
            rag_doll_data_update_reference_from_data(Ptr<RagDollData>, Ptr) -> bool
        ),
        entry!(0x004d_9ab0, fn_004d9ab0(Ptr, Ptr) -> bool),
        entry!(0x004d_9ae0, ni_matrix3_operator_eq(Ptr, Ptr) -> bool),
        entry!(0x004d_9da0, fn_004d9da0(Ptr) -> u32),
        entry!(0x004d_9dc0, fn_004d9dc0(Ptr<RagDollData>, Ptr, Ptr) -> u8),
        entry!(0x004d_9fa0, fn_004d9fa0(Ptr, u8)),
        entry!(0x004d_9fc0, fn_004d9fc0(Ptr, u8, u16)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine(zero: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in zero {
            e.register(*addr, |_, _| ret(0));
        }
        e
    }

    fn slot(table: u32, offset: u32) -> u32 {
        table + 0x10_0000 + offset
    }

    /// A vtable at `table` with 0x80 slots, doubles returning 0.
    fn vtable(e: &mut Engine, table: u32) {
        let slots: Vec<u32> = (0..0x80).map(|i| slot(table, 4 * i)).collect();
        for s in &slots {
            e.register(*s, |_, _| ret(0));
        }
        e.put_vtable(table, &slots);
    }

    /// An object with a vtable made by `vtable`.
    fn object(e: &mut Engine, table: u32) -> u32 {
        vtable(e, table);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, table);
        this
    }

    #[test]
    fn constructor_and_clear() {
        let mut e = engine(&[POINT_PREPARE, FREE]);
        e.map(ORIGIN, 0xc);
        for i in 0..3 {
            e.mem.set_u32(ORIGIN + 4 * i, 0x10 + i);
        }
        let this = e.mem.alloc(0x14);
        for i in 0..5 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        let (result, log) = run(&mut e, 0x004d_9330, &[this]);
        assert_eq!(result.u32(), this);
        assert_eq!(log, vec![(POINT_PREPARE, vec![this + 8])]);
        let words: Vec<u32> = (0..5).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0xffff_ff00, 0, 0x10, 0x11, 0x12]);
        e.mem.set_u8(this, 3);
        e.mem.set_u32(this + 4, 0x6000);
        let (_, log) = run(&mut e, 0x004d_9380, &[this]);
        assert_eq!(log, vec![(FREE, vec![0x6000])]);
        assert_eq!((e.mem.u8(this), e.mem.u32(this + 4)), (0, 0));
    }

    fn load_engine(tag: u32, size: u32, swap: bool) -> Engine {
        let mut e = engine(&[CONSTRUCT_ARRAY, GET_CHUNK_DATA, BONE_SWAP, FLOAT_SWAP]);
        e.register_double(GET_TES_CHUNK, move |_, _| ret(tag));
        e.register_double(CHUNK_SIZE, move |_, _| ret(size));
        e.register_double(FILE_NEEDS_SWAP, move |_, _| ret(u32::from(swap)));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e
    }

    #[test]
    fn xrgd_reads_the_bones() {
        let mut e = load_engine(CHUNK_XRGD, 0x38, true);
        let this = e.mem.alloc(0x14);
        let (result, log) = run(&mut e, 0x004d_94a0, &[this, 0x7000]);
        assert_eq!(result.u32(), 1);
        assert_eq!(e.mem.u8(this), 2);
        let bones = e.mem.u32(this + 4);
        assert_eq!(calls(&log, ALLOCATE), vec![vec![0x38]]);
        assert_eq!(
            calls(&log, CONSTRUCT_ARRAY),
            vec![vec![bones, BONE_SIZE, 2, BONE_CONSTRUCT]]
        );
        assert_eq!(calls(&log, GET_CHUNK_DATA), vec![vec![0x7000, bones, 0x38]]);
        assert_eq!(
            calls(&log, BONE_SWAP),
            vec![vec![bones], vec![bones + 0x1c]]
        );
        assert_eq!(calls(&log, GET_TES_CHUNK).len(), 2);
    }

    #[test]
    fn xrgd_with_a_partial_bone_is_dropped() {
        let mut e = load_engine(CHUNK_XRGD, 0x39, false);
        let this = e.mem.alloc(0x14);
        let (result, log) = run(&mut e, 0x004d_94a0, &[this, 0x7000]);
        assert_eq!(result.u32(), 0);
        assert_eq!(e.mem.u8(this), 0);
        assert!(calls(&log, ALLOCATE).is_empty());
        // A failed allocation leaves no array but still reads the chunk.
        let mut e = load_engine(CHUNK_XRGD, 0x1c, false);
        e.register(ALLOCATE, |_, _| ret(0));
        let this = e.mem.alloc(0x14);
        let (result, log) = run(&mut e, 0x004d_94a0, &[this, 0x7000]);
        assert_eq!(result.u32(), 1);
        assert!(calls(&log, CONSTRUCT_ARRAY).is_empty());
        assert_eq!(calls(&log, GET_CHUNK_DATA), vec![vec![0x7000, 0, 0x1c]]);
    }

    #[test]
    #[should_panic]
    fn xrgd_shorter_than_a_bone_divides_by_zero() {
        let mut e = load_engine(CHUNK_XRGD, 0x10, false);
        let this = e.mem.alloc(0x14);
        e.call(0x004d_94a0, &args![this, 0x7000u32]);
    }

    #[test]
    fn xrgb_reads_the_rotation() {
        let mut e = load_engine(CHUNK_XRGB, 0xc, true);
        let this = e.mem.alloc(0x14);
        let (result, log) = run(&mut e, 0x004d_94a0, &[this, 0x7000]);
        assert_eq!(result.u32(), 1);
        assert_eq!(
            calls(&log, GET_CHUNK_DATA),
            vec![vec![0x7000, this + 8, 0xc]]
        );
        assert_eq!(
            calls(&log, FLOAT_SWAP),
            vec![vec![this + 8, 0], vec![this + 0xc, 0], vec![this + 0x10, 0]]
        );
        let mut e = load_engine(0x1234, 0xc, true);
        let (result, log) = run(&mut e, 0x004d_94a0, &[this, 0x7000]);
        assert_eq!(result.u32(), 0);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn matrix_equality_and_difference() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x24);
        let b = e.mem.alloc(0x24);
        for i in 0..9 {
            e.mem.set_u32(a + 4 * i, (i as f32).to_bits());
            e.mem.set_u32(b + 4 * i, (i as f32).to_bits());
        }
        // 0.0 equals -0.0.
        e.mem.set_u32(b, (-0.0f32).to_bits());
        assert!(e.call(0x004d_9ae0, &args![a, b]).bool());
        assert_eq!(e.call(0x004d_9ab0, &args![a, b]).u32(), 0);
        e.mem.set_u32(b + 0x20, 9.0f32.to_bits());
        assert!(!e.call(0x004d_9ae0, &args![a, b]).bool());
        assert_eq!(e.call(0x004d_9ab0, &args![a, b]).u32(), 1);
        // A NaN is never equal, even to itself.
        assert!(e.call(0x004d_9ae0, &args![a, a]).bool());
        e.mem.set_u32(a + 4, f32::NAN.to_bits());
        assert!(!e.call(0x004d_9ae0, &args![a, a]).bool());
    }

    #[test]
    fn flag_helpers() {
        let mut e = Engine::new();
        let word = e.mem.alloc(0x10);
        e.mem.set_u32(word, 0x1234_5678);
        assert_eq!(e.call(0x004d_9da0, &args![word]).u32(), 0x16);
        e.mem.set_u16(word + 0xc, 0x8001);
        e.call(0x004d_9fa0, &args![word, 0x100u32]);
        assert_eq!(e.mem.u16(word + 0xc), 0x8001);
        e.call(0x004d_9fa0, &args![word, 2u32]);
        assert_eq!(e.mem.u16(word + 0xc), 0x8041);
        e.call(0x004d_9fc0, &args![word, 0u32, 0x1_8000u32]);
        assert_eq!(e.mem.u16(word + 0xc), 0x0041);
        e.call(0x004d_9fc0, &args![word, 1u32, 0x0300u32]);
        assert_eq!(e.mem.u16(word + 0xc), 0x0341);
    }

    const APPLY_CALLEES: [u32; 10] = [
        NODE_4A8B00,
        OBJECT_6FA820,
        OBJECT_43B4F0,
        NODE_SET_TRANSLATE,
        POINT_PREPARE,
        FROM_EULER,
        NODE_SET_ROTATE,
        UPDATE_DATA_CONSTRUCT,
        CHILD_COUNT,
        CHILD_AT,
    ];

    /// Ragdoll data with two saved bones (ids 5 and -3, angles 1, 2, 3 for
    /// the first).
    fn ragdoll(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x14);
        let bones = e.mem.alloc(0x38);
        e.mem.set_u8(this, 2);
        e.mem.set_u32(this + 4, bones);
        e.mem.set_u8(bones, 5);
        e.mem.set_u8(bones + 0x1c, 0xfd);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_u32(bones + 0x10 + 4 * i as u32, v.to_bits());
        }
        this
    }

    #[test]
    fn bones_are_applied_down_the_tree() {
        let mut e = engine(&APPLY_CALLEES);
        let this = ragdoll(&mut e);
        let bones = e.mem.u32(this + 4);
        let root = object(&mut e, 0x0f00_0000);
        let child = object(&mut e, 0x0f20_0000);
        let child_body = e.mem.alloc(0x10);
        let child_link = object(&mut e, 0x0f40_0000);
        e.register_double(slot(0x0f40_0000, 0xc), move |_, _| ret(child));
        e.register_double(NODE_4A8B00, move |_, a| {
            ret(if a[0] == child { child_body } else { 0x5000 })
        });
        // `0043b4f0` gives an info word whose bits 8..12 are the id: the
        // root's body is bone 5, the child's bone -3 (0x1d in five bits
        // does not match: the child fails).
        let info = e.mem.alloc(8);
        e.mem.set_u32(info, 5 << 8);
        e.register_double(OBJECT_43B4F0, move |_, _| ret(info));
        e.register_double(CHILD_COUNT, move |_, a| ret(u32::from(a[0] == root)));
        e.register_double(CHILD_AT, move |_, _| ret(child_link));
        // The root's body (0x5000) needs a vtable for slot 0xb0.
        vtable(&mut e, 0x0f60_0000);
        e.map(0x5000, 0x10);
        e.mem.set_u32(0x5000, 0x0f60_0000);
        let placed = e.mem.alloc(4);
        let (result, log) = run(&mut e, 0x004d_9dc0, &[this, root, placed]);
        assert_eq!(result.u32(), 0);
        assert_eq!(e.mem.u32(placed), 1);
        assert_eq!(calls(&log, NODE_SET_TRANSLATE), vec![vec![root, bones + 4]]);
        let euler = calls(&log, FROM_EULER);
        assert_eq!(euler.len(), 1);
        assert_eq!(
            euler[0][1..],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(calls(&log, NODE_SET_ROTATE), vec![vec![root, euler[0][0]]]);
        assert_eq!(e.mem.u16(0x5000 + 0xc), 0x40);
        assert_eq!(
            calls(&log, slot(0x0f60_0000, 0xb0)),
            vec![vec![0x5000, 1, 0, 0]]
        );
        assert_eq!(calls(&log, slot(0x0f00_0000, 0xb8)).len(), 1);
        // The child (its link's slot 0xc object) was visited and failed.
        assert_eq!(calls(&log, NODE_4A8B00), vec![vec![root], vec![child]]);
        assert!(calls(&log, slot(0x0f20_0000, 0xb8)).is_empty());
    }

    #[test]
    fn bones_stop_without_a_node_or_bones_left() {
        let mut e = engine(&APPLY_CALLEES);
        let this = ragdoll(&mut e);
        let placed = e.mem.alloc(4);
        let (result, log) = run(&mut e, 0x004d_9dc0, &[this, 0, placed]);
        assert_eq!(result.u32(), 1);
        assert!(log.is_empty());
        e.mem.set_u32(placed, 2);
        let (result, log) = run(&mut e, 0x004d_9dc0, &[this, 0x4000, placed]);
        assert_eq!(result.u32(), 1);
        assert!(log.is_empty());
    }

    const UPDATE_CALLEES: [u32; 13] = [
        REFERENCE_ROOT,
        ACTOR_PREPARE,
        NODE_43C490,
        COMPARE_439090,
        GET_ORIENTATION,
        NODE_ROTATION,
        NODE_SET_TRANSLATE,
        NODE_SET_ROTATE,
        RESET_SIM,
        UPDATE_DATA_CONSTRUCT,
        NODE_UPDATE,
        SET_MOTION,
        NODE_4A8B00,
    ];

    #[test]
    fn update_moves_the_root_and_reports() {
        let mut e = engine(&UPDATE_CALLEES);
        for addr in APPLY_CALLEES {
            if addr != NODE_4A8B00 {
                e.register(addr, |_, _| ret(0));
            }
        }
        let this = ragdoll(&mut e);
        let reference = object(&mut e, 0x0f00_0000);
        let root = object(&mut e, 0x0f20_0000);
        let start = object(&mut e, 0x0f40_0000);
        e.register_double(REFERENCE_ROOT, move |_, _| ret(root));
        e.register_double(slot(0x0f20_0000, 0xc), move |_, _| ret(start));
        e.register(slot(0x0f00_0000, 0x1f4), |_, _| ret(0x6100));
        e.register(slot(0x0f00_0000, SLOT_ACTOR_TEST), |_, _| ret(1));
        e.register(NODE_43C490, |_, _| ret(0x6200));
        e.register(GET_ORIENTATION, |_, a| ret(a[1]));
        e.register(NODE_ROTATION, |_, _| ret(0x6300));
        e.map(0x6300, 0x24);
        let (result, log) = run(&mut e, 0x004d_9910, &[this, reference]);
        // No body on any node: nothing placed, but nothing failed.
        assert!(!result.bool());
        assert_eq!(calls(&log, ACTOR_PREPARE), vec![vec![this, reference]]);
        assert_eq!(calls(&log, COMPARE_439090), vec![vec![0x6200, 0x6100]]);
        // The orientation (all zero) equals the rotation (all zero): no move.
        assert!(calls(&log, NODE_SET_TRANSLATE).is_empty());
        assert!(calls(&log, SET_MOTION).is_empty());
        // The positions differ: the root is moved.
        e.register(COMPARE_439090, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x004d_9910, &[this, reference]);
        assert_eq!(calls(&log, NODE_SET_TRANSLATE), vec![vec![root, 0x6100]]);
        let orientation = calls(&log, GET_ORIENTATION)[0][1];
        assert_eq!(calls(&log, NODE_SET_ROTATE), vec![vec![root, orientation]]);
        assert_eq!(calls(&log, RESET_SIM), vec![vec![root, 1]]);
        let update = calls(&log, UPDATE_DATA_CONSTRUCT)
            .into_iter()
            .last()
            .unwrap();
        assert_eq!(update[1..], [0, 0, 0]);
        assert_eq!(calls(&log, NODE_UPDATE), vec![vec![root, update[0]]]);
        // A failing bone sets the motion.
        e.register(NODE_4A8B00, |_, _| ret(0x5000));
        e.register(OBJECT_43B4F0, |e, _| {
            let info = e.mem.alloc(4);
            ret(info)
        });
        let (result, log) = run(&mut e, 0x004d_9910, &[this, reference]);
        assert!(!result.bool());
        assert_eq!(calls(&log, SET_MOTION), vec![vec![start, 1, 1, 0, 1]]);
    }

    #[test]
    fn update_with_every_bone_placed_succeeds() {
        let mut e = engine(&UPDATE_CALLEES);
        let this = e.mem.alloc(0x14);
        let reference = object(&mut e, 0x0f00_0000);
        e.register(REFERENCE_ROOT, |_, _| ret(0));
        // No root: nothing runs, and zero bones placed equals... the saved
        // count only when it is zero.
        assert!(e.call(0x004d_9910, &args![this, reference]).bool());
        e.mem.set_u8(this, 1);
        assert!(!e.call(0x004d_9910, &args![this, reference]).bool());
        assert!(!e.call(0x004d_9910, &args![this, 0u32]).bool());
    }

    /// The reference's actor test slot.
    const SLOT_ACTOR_TEST: u32 = 0x100;
}
