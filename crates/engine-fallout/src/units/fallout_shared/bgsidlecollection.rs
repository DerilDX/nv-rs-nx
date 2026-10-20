//! `fallout shared/bgsidlecollection.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSIdleCollection` (Xbox PDB), 0x10 bytes; the PC code uses the same
    /// offsets.
    pub struct BGSIdleCollection: 0x10 {
        /// `cIdleFlags` (Xbox PDB).
        0x04 cIdleFlags: u8,
        /// `cIdleCount` (Xbox PDB).
        0x05 cIdleCount: u8,
        /// `pIdleArray` (Xbox PDB): `TESIdleForm **`.
        0x08 pIdleArray: u32,
        /// `fTimerCheckForIdle` (Xbox PDB).
        0x0C fTimerCheckForIdle: f32,
    }
}

/// `00401000` / `00401030`: allocate / free (`cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `TESFile::GetTESChunk` (`004726b0`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::GetChunkData` (`004727f0`, `RET 4`).
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData` overload (`00472890`, `RET 8`: address, size).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `00401660` (`thiscall` on the file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680` (`thiscall` on the file): whether form ids read need fixing up.
const FILE_FIXES_IDS: u32 = 0x0040_1680;
/// `00401080` (`cdecl`: a form id's address, a byte): fixes up one form id.
const FIX_UP_FORM_ID: u32 = 0x0040_1080;
/// `00403d30` (`cdecl`: address, byte, count): fills memory.
const FILL_MEMORY: u32 = 0x0040_3d30;
/// `TESForm::GetFile` (`00484e60`, `RET 4`).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, `cdecl`: the id's address, the
/// file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, one word): the form of a form id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) and the type
/// descriptors `00479b00` passes.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_6a18;
/// `TESIdleForm::CheckIdleFile` (`005ff280`).
const CHECK_IDLE_FILE: u32 = 0x005f_f280;
/// `0084e3a0`: the word at +0xc (`TESForm::iFormID` on a form).
const WORD_AT_0C: u32 = 0x0084_e3a0;
/// The forms' vtable slot (byte offset) whose answer the warnings print as
/// `%s`.
const NAME_SLOT: u32 = 0x130;
/// `005b5e40` (`cdecl`, variable): prints a warning.
const PRINT_WARNING: u32 = 0x005b_5e40;
/// "MASTERFILE: Unable to find Idle (%08X) on form '%s' (%08X)."
/// (`0101aaa8`).
const MISSING_IDLE_FORMAT: u32 = 0x0101_aaa8;
/// "MASTERFILE: MBGSIdleCollection has an idle '%s' without an animation
/// file on form '%s' (%08X)." (`0101aae8`).
const IDLE_WITHOUT_FILE_FORMAT: u32 = 0x0101_aae8;
/// `00479ea0` (`thiscall`, `RET 4`): removes the idle at an index.
const REMOVE_IDLE_AT: u32 = 0x0047_9ea0;
/// `Actor::CanUseIdle` (`008b2010`, `RET 4`).
const ACTOR_CAN_USE_IDLE: u32 = 0x008b_2010;
/// `005ff430` (`thiscall` on the idle, `RET 0xc`: actor, target, 1).
const IDLE_CONDITIONS_PASS: u32 = 0x005f_f430;
/// `0047a110` (`RET 8`: index, element address) and `007cb2e0` (`RET 4`,
/// element address): the array's set-at and add.
const ARRAY_SET_AT: u32 = 0x0047_a110;
const ARRAY_ADD: u32 = 0x007c_b2e0;
/// Chunk tags.
const CHUNK_IDLF: u32 = 0x464c_4449;
const CHUNK_IDLA: u32 = 0x414c_4449;
const CHUNK_IDLB: u32 = 0x424c_4449;
const CHUNK_IDLC: u32 = 0x434c_4449;
const CHUNK_IDLN: u32 = 0x4e4c_4449;
const CHUNK_IDLT: u32 = 0x544c_4449;

/// The address of idle `index` in `pIdleArray`.
fn idle_slot(e: &Engine, this: Ptr<BGSIdleCollection>, index: u32) -> u32 {
    e.get(this, BGSIdleCollection::pIdleArray)
        .wrapping_add(index.wrapping_mul(4))
}

// Translated from 004797e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSIdleCollection::InitializeDataComponent` (Xbox PDB): clears the flags,
/// the count and the array pointer, and sets the timer to 0.0 (`FLDZ`).
pub fn bgs_idle_collection_initialize_data_component(e: &mut Engine, this: Ptr<BGSIdleCollection>) {
    e.set(this, BGSIdleCollection::cIdleFlags, 0);
    e.set(this, BGSIdleCollection::cIdleCount, 0);
    e.set(this, BGSIdleCollection::pIdleArray, 0);
    e.mem
        .set_u32(this.addr().wrapping_add(0xc), x87_store32(0.0));
}

// Translated from 00479810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSIdleCollection::ClearDataComponent` (Xbox PDB): frees the array
/// (`00401030`) and clears the count and the pointer.
pub fn bgs_idle_collection_clear_data_component(e: &mut Engine, this: Ptr<BGSIdleCollection>) {
    let array = e.get(this, BGSIdleCollection::pIdleArray);
    e.call(FREE, &args![array]);
    e.set(this, BGSIdleCollection::cIdleCount, 0);
    e.set(this, BGSIdleCollection::pIdleArray, 0);
}

// Translated from 00479e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): whether the chunk tag is
/// one of the idle collection's, `IDLF`, `IDLA`, `IDLB`, `IDLC`, `IDLN` or
/// `IDLT`.
pub fn fn_00479e20(_e: &mut Engine, tag: u32) -> bool {
    matches!(
        tag,
        CHUNK_IDLF | CHUNK_IDLA | CHUNK_IDLB | CHUNK_IDLC | CHUNK_IDLN | CHUNK_IDLT
    )
}

// Translated from 00479e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: 1 when bit 0 of `cIdleFlags` is set, else 0
/// (in `EAX`).
pub fn fn_00479e80(e: &mut Engine, this: Ptr<BGSIdleCollection>) -> u32 {
    u32::from(e.get(this, BGSIdleCollection::cIdleFlags) & 1 != 0)
}

// Translated from 00479c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSIdleCollection::Load` (Xbox PDB), `thiscall`, `RET 4`: the file. Only
/// for the collection's own chunks (`00479e20`): `IDLF` reads the flags
/// byte; `IDLC` the count (one byte when the chunk is 1 byte, else the low
/// byte of a word) and, for a non-zero count, frees the old array and
/// allocates a cleared one of `count * 4` bytes (`MUL` overflow giving
/// `0xffffffff`); `IDLA` reads the form ids when there is an array and the
/// chunk is exactly `count * 4` bytes, fixing each up (`00401080`) when
/// `00401680` says so; `IDLT` reads the timer.
pub fn bgs_idle_collection_load(e: &mut Engine, this: Ptr<BGSIdleCollection>, file: Ptr) {
    let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
    if !fn_00479e20(e, tag) {
        return;
    }
    let base = this.addr();
    match tag {
        CHUNK_IDLF => {
            e.call(
                GET_CHUNK_DATA_SIZED,
                &args![file, base.wrapping_add(4), 1u32],
            );
        }
        CHUNK_IDLC => {
            if e.call(CHUNK_SIZE, &args![file]).u32() == 1 {
                e.call(
                    GET_CHUNK_DATA_SIZED,
                    &args![file, base.wrapping_add(5), 1u32],
                );
            } else {
                e.with_stack(4, |e, word| {
                    e.mem.set_u32(word.addr(), 0);
                    e.call(GET_CHUNK_DATA, &args![file, word]);
                    let count = e.mem.u8(word.addr());
                    e.set(this, BGSIdleCollection::cIdleCount, count);
                });
            }
            if e.get(this, BGSIdleCollection::cIdleCount) != 0 {
                let old = e.get(this, BGSIdleCollection::pIdleArray);
                if old != 0 {
                    e.call(FREE, &args![old]);
                }
                let count = u32::from(e.get(this, BGSIdleCollection::cIdleCount));
                let size = count.checked_mul(4).unwrap_or(0xffff_ffff);
                let array = e.call(ALLOCATE, &args![size]).u32();
                e.set(this, BGSIdleCollection::pIdleArray, array);
                let bytes = u32::from(e.get(this, BGSIdleCollection::cIdleCount)) << 2;
                let array = e.get(this, BGSIdleCollection::pIdleArray);
                e.call(FILL_MEMORY, &args![array, 0u32, bytes]);
            }
        }
        CHUNK_IDLA => {
            if e.get(this, BGSIdleCollection::pIdleArray) != 0 {
                let bytes = u32::from(e.get(this, BGSIdleCollection::cIdleCount)) << 2;
                if e.call(CHUNK_SIZE, &args![file]).u32() == bytes {
                    let array = e.get(this, BGSIdleCollection::pIdleArray);
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, array, bytes]);
                    if e.call(FILE_FIXES_IDS, &args![file]).bool() {
                        let mut index: i32 = 0;
                        while index < i32::from(e.get(this, BGSIdleCollection::cIdleCount)) {
                            let slot = idle_slot(e, this, index as u32);
                            e.call(FIX_UP_FORM_ID, &args![slot, 0u32]);
                            index = index.wrapping_add(1);
                        }
                    }
                }
            }
        }
        CHUNK_IDLT => {
            e.call(GET_CHUNK_DATA, &args![file, base.wrapping_add(0xc)]);
        }
        _ => {}
    }
}

// Translated from 00479b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSIdleCollection::InitItem` (Xbox PDB), `thiscall`, `RET 4`: the form
/// that owns the collection. Each form id in the array (the count is
/// re-read each round) gets the owner's file's compile index (`00484e60`
/// with -1, `00485d50`), is looked up (`004839c0`) and cast to an idle. A
/// found idle with an animation file (`005ff280`) replaces the id; one
/// without, or a missing one, prints its warning (names through vtable
/// `+0x130`, the owner's form id through `0084e3a0`) and is removed
/// (`00479ea0`), the index stepping back by one.
pub fn bgs_idle_collection_init_item(e: &mut Engine, this: Ptr<BGSIdleCollection>, owner: Ptr) {
    let mut index: u32 = 0;
    while index < u32::from(e.get(this, BGSIdleCollection::cIdleCount)) {
        e.with_stack(4, |e, id| {
            let slot = idle_slot(e, this, index);
            let value = e.mem.u32(slot);
            e.mem.set_u32(id.addr(), value);
            let file = e.call(GET_FILE, &args![owner, 0xffff_ffffu32]).u32();
            e.call(ADD_COMPILE_INDEX, &args![id, file]);
            let form_id = e.mem.u32(id.addr());
            let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
            let idle = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
                )
                .u32();
            if idle != 0 {
                if e.call(CHECK_IDLE_FILE, &args![idle]).bool() {
                    let slot = idle_slot(e, this, index);
                    e.mem.set_u32(slot, idle);
                } else {
                    let owner_id = e.call(WORD_AT_0C, &args![owner]).u32();
                    let owner_name = e.vcall(owner.addr(), NAME_SLOT, &args![]).u32();
                    let idle_name = e.vcall(idle, NAME_SLOT, &args![]).u32();
                    e.call(
                        PRINT_WARNING,
                        &args![IDLE_WITHOUT_FILE_FORMAT, idle_name, owner_name, owner_id],
                    );
                    e.call(REMOVE_IDLE_AT, &args![this, index]);
                    index = index.wrapping_sub(1);
                }
            } else {
                let owner_id = e.call(WORD_AT_0C, &args![owner]).u32();
                let owner_name = e.vcall(owner.addr(), NAME_SLOT, &args![]).u32();
                let form_id = e.mem.u32(id.addr());
                e.call(
                    PRINT_WARNING,
                    &args![MISSING_IDLE_FORMAT, form_id, owner_name, owner_id],
                );
                e.call(REMOVE_IDLE_AT, &args![this, index]);
                index = index.wrapping_sub(1);
            }
        });
        index = index.wrapping_add(1);
    }
}

/// Whether the actor can use the idle and its conditions pass for the
/// target (`008b2010`, then `005ff430` with 1), as `00479fb0` and
/// `0047a030` ask.
fn idle_usable(e: &mut Engine, idle: u32, actor: Ptr, target: u32) -> bool {
    e.call(ACTOR_CAN_USE_IDLE, &args![actor, idle]).bool()
        && e.call(IDLE_CONDITIONS_PASS, &args![idle, actor, target, 1u32])
            .bool()
}

// Translated from 00479fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: actor, target): whether
/// any idle of the collection is usable by the actor for the target; false
/// without an actor.
pub fn fn_00479fb0(e: &mut Engine, this: Ptr<BGSIdleCollection>, actor: Ptr, target: u32) -> bool {
    if actor.is_null() {
        return false;
    }
    let mut index: i32 = 0;
    while index < i32::from(e.get(this, BGSIdleCollection::cIdleCount)) {
        let idle = e.mem.u32(idle_slot(e, this, index as u32));
        if idle != 0 && idle_usable(e, idle, actor, target) {
            return true;
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 0047a030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSIdleCollection::GetIdlesForActor` (engine map name), `thiscall`,
/// `RET 0xc`: actor, target, an output array (may be null). Counts the
/// usable idles; with an array, each is stored at the count's index
/// (`0047a110`) while that is below the word at the array's +0xc
/// (`0084e3a0`, read once), else added (`007cb2e0`), from a stack copy. 0
/// without an actor.
pub fn bgs_idle_collection_get_idles_for_actor(
    e: &mut Engine,
    this: Ptr<BGSIdleCollection>,
    actor: Ptr,
    target: u32,
    array: Ptr,
) -> u32 {
    let mut found: u32 = 0;
    if actor.is_null() {
        return found;
    }
    let mut capacity = 0;
    if !array.is_null() {
        capacity = e.call(WORD_AT_0C, &args![array]).u32();
    }
    e.with_stack(4, |e, element| {
        let mut index: i32 = 0;
        while index < i32::from(e.get(this, BGSIdleCollection::cIdleCount)) {
            let idle = e.mem.u32(idle_slot(e, this, index as u32));
            e.mem.set_u32(element.addr(), idle);
            if idle != 0 && idle_usable(e, idle, actor, target) {
                if !array.is_null() {
                    if found < capacity {
                        e.call(ARRAY_SET_AT, &args![array, found, element]);
                    } else {
                        e.call(ARRAY_ADD, &args![array, element]);
                    }
                }
                found = found.wrapping_add(1);
            }
            index = index.wrapping_add(1);
        }
    });
    found
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0047_97e0,
            bgs_idle_collection_initialize_data_component(Ptr<BGSIdleCollection>)
        ),
        entry!(
            0x0047_9810,
            bgs_idle_collection_clear_data_component(Ptr<BGSIdleCollection>)
        ),
        entry!(0x0047_9e20, fn_00479e20(u32) -> bool),
        entry!(0x0047_9e80, fn_00479e80(Ptr<BGSIdleCollection>) -> u32),
        entry!(
            0x0047_9c40,
            bgs_idle_collection_load(Ptr<BGSIdleCollection>, Ptr)
        ),
        entry!(
            0x0047_9b00,
            bgs_idle_collection_init_item(Ptr<BGSIdleCollection>, Ptr)
        ),
        entry!(
            0x0047_9fb0,
            fn_00479fb0(Ptr<BGSIdleCollection>, Ptr, u32) -> bool
        ),
        entry!(
            0x0047_a030,
            bgs_idle_collection_get_idles_for_actor(Ptr<BGSIdleCollection>, Ptr, u32, Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FORM_VTABLE: u32 = 0x00ab_0000;
    const NAME_FN: u32 = 0x00ab_1000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn collection(e: &mut Engine, idles: &[u32]) -> Ptr<BGSIdleCollection> {
        let this = e.new_object::<BGSIdleCollection>();
        let array = e.mem.alloc(4 * idles.len() as u32 + 4);
        for (i, v) in idles.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *v);
        }
        e.set(this, BGSIdleCollection::cIdleCount, idles.len() as u8);
        e.set(this, BGSIdleCollection::pIdleArray, array);
        this
    }

    #[test]
    fn initialize_and_clear_data_component() {
        let mut e = Engine::new();
        e.register(FREE, |_, _| Ret::default());
        let this = collection(&mut e, &[1, 2]);
        e.set(this, BGSIdleCollection::cIdleFlags, 7);
        e.set(this, BGSIdleCollection::fTimerCheckForIdle, 3.0);
        let array = e.get(this, BGSIdleCollection::pIdleArray);
        e.call_log = Some(vec![]);
        e.call(0x0047_9810, &args![this]);
        assert_eq!(calls_to(&e, FREE), vec![vec![array]]);
        assert_eq!(e.get(this, BGSIdleCollection::cIdleCount), 0);
        assert_eq!(e.get(this, BGSIdleCollection::pIdleArray), 0);
        assert_eq!(e.get(this, BGSIdleCollection::cIdleFlags), 7);
        e.call(0x0047_97e0, &args![this]);
        assert_eq!(e.get(this, BGSIdleCollection::cIdleFlags), 0);
        assert_eq!(
            e.get(this, BGSIdleCollection::fTimerCheckForIdle).to_bits(),
            0
        );
    }

    #[test]
    fn fn_00479e20_knows_the_six_tags_and_00479e80_reads_bit_0() {
        let mut e = Engine::new();
        for tag in [
            CHUNK_IDLF, CHUNK_IDLA, CHUNK_IDLB, CHUNK_IDLC, CHUNK_IDLN, CHUNK_IDLT,
        ] {
            assert!(e.call(0x0047_9e20, &args![tag]).bool());
        }
        for tag in [0u32, 0x4444_4c49, 0x464c_4448, 0x5444_4f4d] {
            assert!(!e.call(0x0047_9e20, &args![tag]).bool());
        }
        let this = collection(&mut e, &[]);
        e.set(this, BGSIdleCollection::cIdleFlags, 3);
        assert_eq!(e.call(0x0047_9e80, &args![this]).u32(), 1);
        e.set(this, BGSIdleCollection::cIdleFlags, 2);
        assert_eq!(e.call(0x0047_9e80, &args![this]).u32(), 0);
    }

    type Chunk = Rc<RefCell<(u32, Vec<u8>)>>;

    fn loader() -> (Engine, Chunk) {
        let mut e = Engine::new();
        let chunk: Chunk = Rc::new(RefCell::new((0, vec![])));
        let c = chunk.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| c.borrow().0.into_ret());
        let c = chunk.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            (c.borrow().1.len() as u32).into_ret()
        });
        let c = chunk.clone();
        e.register_double(GET_CHUNK_DATA_SIZED, move |e, a| {
            let data = c.borrow().1.clone();
            e.mem.write(a[1], &data[..data.len().min(a[2] as usize)]);
            Ret::default()
        });
        let c = chunk.clone();
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = c.borrow().1.clone();
            e.mem.write(a[1], &data);
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        e.register(FILL_MEMORY, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(FILE_FIXES_IDS, |_, _| 1u32.into_ret());
        e.register(FIX_UP_FORM_ID, |_, _| Ret::default());
        (e, chunk)
    }

    #[test]
    fn load_reads_flags_count_array_and_timer() {
        let (mut e, chunk) = loader();
        let this = e.new_object::<BGSIdleCollection>();
        e.call_log = Some(vec![]);
        *chunk.borrow_mut() = (CHUNK_IDLF, vec![5]);
        e.call(0x0047_9c40, &args![this, 0x9000u32]);
        assert_eq!(e.get(this, BGSIdleCollection::cIdleFlags), 5);
        // A one-byte count makes a cleared array.
        *chunk.borrow_mut() = (CHUNK_IDLC, vec![2]);
        e.call(0x0047_9c40, &args![this, 0x9000u32]);
        assert_eq!(e.get(this, BGSIdleCollection::cIdleCount), 2);
        let array = e.get(this, BGSIdleCollection::pIdleArray);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![8]]);
        assert_eq!(calls_to(&e, FILL_MEMORY), vec![vec![array, 0, 8]]);
        assert!(calls_to(&e, FREE).is_empty());
        // A four-byte count frees the old array first.
        *chunk.borrow_mut() = (CHUNK_IDLC, vec![3, 0, 0, 0]);
        e.call(0x0047_9c40, &args![this, 0x9000u32]);
        assert_eq!(e.get(this, BGSIdleCollection::cIdleCount), 3);
        assert_eq!(calls_to(&e, FREE), vec![vec![array]]);
        // The ids: only with the exact size, each fixed up.
        *chunk.borrow_mut() = (CHUNK_IDLA, vec![1; 8]);
        e.call(0x0047_9c40, &args![this, 0x9000u32]);
        assert!(calls_to(&e, FIX_UP_FORM_ID).is_empty());
        *chunk.borrow_mut() = (CHUNK_IDLA, vec![1; 12]);
        e.call(0x0047_9c40, &args![this, 0x9000u32]);
        let array = e.get(this, BGSIdleCollection::pIdleArray);
        assert_eq!(e.mem.u32(array + 8), 0x0101_0101);
        assert_eq!(
            calls_to(&e, FIX_UP_FORM_ID),
            vec![vec![array, 0], vec![array + 4, 0], vec![array + 8, 0]]
        );
        *chunk.borrow_mut() = (CHUNK_IDLT, 2.5f32.to_bits().to_le_bytes().to_vec());
        e.call(0x0047_9c40, &args![this, 0x9000u32]);
        assert_eq!(e.get(this, BGSIdleCollection::fTimerCheckForIdle), 2.5);
        // A zero count keeps the array; another tag does nothing.
        e.call_log = Some(vec![]);
        *chunk.borrow_mut() = (CHUNK_IDLC, vec![0]);
        e.call(0x0047_9c40, &args![this, 0x9000u32]);
        *chunk.borrow_mut() = (0x4449_4445, vec![0]);
        e.call(0x0047_9c40, &args![this, 0x9000u32]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert_eq!(calls_to(&e, CHUNK_SIZE).len(), 1);
    }

    #[test]
    fn init_item_resolves_ids_and_drops_bad_ones() {
        let mut e = Engine::new();
        let removed: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let r = removed.clone();
        e.register_double(REMOVE_IDLE_AT, move |e, a| {
            // Shift the following ids down, as a removal does.
            r.borrow_mut().push(a[1]);
            let count = u32::from(e.mem.u8(a[0] + 5));
            let array = e.mem.u32(a[0] + 8);
            for i in a[1]..count - 1 {
                let next = e.mem.u32(array + 4 * (i + 1));
                e.mem.set_u32(array + 4 * i, next);
            }
            e.mem.set_u8(a[0] + 5, (count - 1) as u8);
            Ret::default()
        });
        e.register(GET_FILE, |_, _| 0x7000u32.into_ret());
        e.register(ADD_COMPILE_INDEX, |_, _| Ret::default());
        // Ids 1 and 2 are idles at 0x0050_0000 + id; 3 is missing.
        e.register(LOOKUP_FORM, |_, a| {
            if a[0] < 3 {
                (0x0050_0000 + a[0]).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        // Idle 2 has no animation file.
        e.register(CHECK_IDLE_FILE, |_, a| {
            u32::from(a[0] != 0x0050_0002).into_ret()
        });
        e.register(WORD_AT_0C, |_, _| 0x00ff_0001u32.into_ret());
        e.register(NAME_FN, |_, a| (a[0] + 0x10).into_ret());
        e.register(PRINT_WARNING, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x4d];
        slots[0x130 / 4] = NAME_FN;
        e.put_vtable(FORM_VTABLE, &slots);
        e.map(0x0050_0000, 0x10);
        e.mem.set_u32(0x0050_0002, FORM_VTABLE);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner, FORM_VTABLE);
        let this = collection(&mut e, &[1, 3, 2, 1]);
        e.call_log = Some(vec![]);
        e.call(0x0047_9b00, &args![this, owner]);
        assert_eq!(removed.borrow().as_slice(), [1, 1]);
        assert_eq!(e.get(this, BGSIdleCollection::cIdleCount), 2);
        let array = e.get(this, BGSIdleCollection::pIdleArray);
        assert_eq!([e.mem.u32(array), e.mem.u32(array + 4)], [0x0050_0001; 2]);
        assert_eq!(
            calls_to(&e, PRINT_WARNING),
            vec![
                vec![MISSING_IDLE_FORMAT, 3, owner + 0x10, 0x00ff_0001],
                vec![
                    IDLE_WITHOUT_FILE_FORMAT,
                    0x0050_0012,
                    owner + 0x10,
                    0x00ff_0001
                ],
            ]
        );
        assert_eq!(calls_to(&e, GET_FILE)[0], vec![owner, 0xffff_ffff]);
    }

    fn usable_engine() -> Engine {
        let mut e = Engine::new();
        // Idles at even addresses can be used; conditions pass for target 7.
        e.register(ACTOR_CAN_USE_IDLE, |_, a| {
            u32::from(a[1] % 2 == 0).into_ret()
        });
        e.register(IDLE_CONDITIONS_PASS, |_, a| {
            u32::from(a[2] == 7 && a[3] == 1).into_ret()
        });
        e.register(WORD_AT_0C, |_, _| 1u32.into_ret());
        e.register(ARRAY_SET_AT, |_, _| Ret::default());
        e.register(ARRAY_ADD, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_00479fb0_finds_a_usable_idle() {
        let mut e = usable_engine();
        let this = collection(&mut e, &[0, 0x101, 0x102]);
        assert!(e.call(0x0047_9fb0, &args![this, 0x9000u32, 7u32]).bool());
        assert!(!e.call(0x0047_9fb0, &args![this, 0x9000u32, 8u32]).bool());
        assert!(!e.call(0x0047_9fb0, &args![this, 0u32, 7u32]).bool());
        let odd = collection(&mut e, &[0x101]);
        assert!(!e.call(0x0047_9fb0, &args![odd, 0x9000u32, 7u32]).bool());
    }

    #[test]
    fn get_idles_for_actor_counts_and_fills_the_array() {
        let mut e = usable_engine();
        let this = collection(&mut e, &[0x102, 0x101, 0, 0x104, 0x106]);
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0047_a030, &args![this, 0x9000u32, 7u32, array])
                .u32(),
            3
        );
        assert_eq!(calls_to(&e, WORD_AT_0C), vec![vec![array]]);
        let set_at = calls_to(&e, ARRAY_SET_AT);
        assert_eq!((set_at.len(), set_at[0][1]), (1, 0));
        assert_eq!(calls_to(&e, ARRAY_ADD).len(), 2);
        // Without an array only the count; without an actor 0.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0047_a030, &args![this, 0x9000u32, 7u32, 0u32])
                .u32(),
            3
        );
        assert!(calls_to(&e, ARRAY_ADD).is_empty());
        assert_eq!(
            e.call(0x0047_a030, &args![this, 0u32, 7u32, array]).u32(),
            0
        );
    }
}
