//! `fallout shared/bgsimpactdataset.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058e810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058e810 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5f (0x4c bytes); `ECX` is the object.
/// The engine map names it `BGSImpactDataSet::BGSImpactDataSet` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0103_323c`) is `BGSImpactDataSet`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0058e810(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x004f_9d90, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_323c);
    e.mem.set_u32(base + 0x18, 0x0103_3224);
    e.call(0x004f_15a0, &args![this, 0x5fu32]);
    e.call(0x0058_eb10, &args![this]);
    this
}

// Translated from 0058f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMap<BGSImpactData_P_BGSImpactData_P>` scalar deleting destructor (engine map name): runs the destructor body `0058f900` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0058_f900, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0058f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<BGSImpactData_P_BGSImpactData_P>` scalar deleting destructor (engine map name): runs the destructor body `0058f960` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_base_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0058_f960, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The impact data set override (`011ca834`), or null: `0058eeb0` sets it.
const OVERRIDE_SET: u32 = 0x011c_a834;
/// The default impact data set (`011ca830`).
const DEFAULT_SET: u32 = 0x011c_a830;
/// `0058f6f0` (`ECX` = the override set, two stack words: material, impact data).
const OVERRIDE_LOOKUP_0058F6F0: u32 = 0x0058_f6f0;
/// `0058f7f0` (`ECX` = a set, one stack word: a material): the set's impact data for it.
const SET_IMPACT_0058F7F0: u32 = 0x0058_f7f0;
/// `008374e0` (`cdecl`: material, impact data) and `00837500` (no words).
const SET_MATERIAL_IMPACT_008374E0: u32 = 0x0083_74e0;
const MATERIALS_SET_00837500: u32 = 0x0083_7500;
/// The game's `memset` wrapper (`00403d30`, `cdecl`: destination, byte, size).
const MEMSET: u32 = 0x0040_3d30;
/// `004013e0` (`ECX`): whether form flag 8 is set (`AL`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `00484ab0` (`ECX`, one stack word): sets or clears form flag 8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, `ECX`, one stack word).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, `cdecl`: address of a form id, file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, one word): the form of a form id, or null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@`, `.?AVBGSImpactDataSet@@`, `.?AVBGSImpactData@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_IMPACT_DATA_SET: u32 = 0x0118_62dc;
const TYPE_IMPACT_DATA: u32 = 0x0118_62fc;
/// `0084e3a0` (`ECX`): the word at +0xc (the form id).
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// The release build's logging stub (`005b5e40`, `cdecl`, variable words).
const LOG: u32 = 0x005b_5e40;
/// "MASTERFILE: Unable to find impact data (%08X) on impact data set %s (%08X)."
const MESSAGE_NO_IMPACT_DATA: u32 = 0x0103_3390;
/// The allocator (`00401000`, `cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `"DefaultImpactDataSet"`.
const DEFAULT_SET_NAME: u32 = 0x0103_3374;
/// The data handler (`011c3f2c`) and `TESDataHandler::AddFormToDataHandler` (Xbox PDB,
/// `004603b0`, `ECX` = data handler, one stack word).
const DATA_HANDLER: u32 = 0x011c_3f2c;
const ADD_FORM_TO_DATA_HANDLER: u32 = 0x0046_03b0;
/// The default set's form id.
const DEFAULT_SET_ID: u32 = 0x276;
/// The model loader (`011c3b3c`) and its `00443d30` (`ECX`, seven stack words).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_LOADER_QUEUE_00443D30: u32 = 0x0044_3d30;
/// `0048cee0` (`ECX` = a model): the length of its name.
const MODEL_NAME_LENGTH_0048CEE0: u32 = 0x0048_cee0;
/// `006733e0` (`ECX`): the word at +0x48 (here the impact data's texture set).
const WORD_48_006733E0: u32 = 0x0067_33e0;
/// `BGSTextureSet::QueueTextureSet` (Xbox PDB, `005930e0`, `ECX`, three stack words).
const QUEUE_TEXTURE_SET: u32 = 0x0059_30e0;
/// `TESFile::GetTESForm` (Xbox PDB, `00472660`, `ECX` = file): the record's form type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `ECX` = form, one stack word: the file).
const LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`, `ECX` = file): the chunk's code, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, `ECX` = file, two stack words:
/// destination, size).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `00401680` (`ECX` = file): whether the file's data needs its byte order swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1680;
/// `00401080` (`cdecl`: address of a word, a byte flag): swaps the word's byte order.
const SWAP_WORD: u32 = 0x0040_1080;
/// `004726f0` (`ECX` = file): steps to the next chunk; false at the record's end (`AL`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// Chunk codes the impact data set loader reads (the four characters, first byte lowest).
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
/// The impact data set's twelve material slots, from +0x1c.
const SLOTS: u32 = 0x1c;

// Translated from 0058e9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range; `ECX` and one stack word, `RET 4`): the impact
/// data of material slot `material` (`+0x1c + material * 4`); with an override set
/// (`011ca834`) its `0058f6f0` answer for the material and that data instead.
pub fn fn_0058e9d0(e: &mut Engine, this: Ptr, material: u32) -> u32 {
    let slot = this
        .addr()
        .wrapping_add(material.wrapping_mul(4))
        .wrapping_add(SLOTS);
    let set = e.global::<u32>(OVERRIDE_SET);
    if set != 0 {
        let data = e.mem.u32(slot);
        e.call(OVERRIDE_LOOKUP_0058F6F0, &args![set, material, data])
            .u32()
    } else {
        e.mem.u32(slot)
    }
}

// Translated from 0058eb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (the `BGSImpactDataSet` constructor `0058e810` runs it): clears the
/// twelve material slots (`memset` of 0x30 bytes at +0x1c).
pub fn fn_0058eb10(e: &mut Engine, this: Ptr) {
    e.call(MEMSET, &args![this.addr() + SLOTS, 0u32, 0x30u32]);
}

// Translated from 0058e8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one word): the impact data slot of a material code. The
/// exe's byte map and jump table in `.text` (`0058e9a4`, `0058e97c`) are these arms; codes
/// above 0x1f answer 4.
pub fn fn_0058e8f0(_e: &mut Engine, material: u32) -> u32 {
    match material {
        0 | 10 | 19 => 0,
        1 => 7,
        2 | 18 => 1,
        3 | 24 => 3,
        4 => 2,
        6 | 7 => 6,
        8 => 8,
        9 | 12 | 30 | 31 => 5,
        16 | 17 | 22 | 23 | 25 | 28 | 29 => 9,
        _ => 4,
    }
}

// Translated from 0058e8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSImpactDataSet` destructor body (unnamed in the map; `ECX`): stores the two vtables
/// (`0103323c` at +0, `01033224` at +0x18), then `TESForm::~TESForm` (`00483630`).
pub fn fn_0058e8c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0103_323c);
    e.mem.set_u32(this.addr() + 0x18, 0x0103_3224);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0058eeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one word): makes `set` the override set (`011ca834`), then
/// for each material 0 to 9 hands `008374e0` the material and the set's impact data for it
/// (`0058f7f0`; 0 without a set), then `00837500`.
pub fn fn_0058eeb0(e: &mut Engine, set: u32) {
    e.set_global(OVERRIDE_SET, set);
    for material in 0..10u32 {
        let data = if set != 0 {
            e.call(SET_IMPACT_0058F7F0, &args![set, material]).u32()
        } else {
            0
        };
        e.call(SET_MATERIAL_IMPACT_008374E0, &args![material, data]);
    }
    e.call(MATERIALS_SET_00837500, &args![]);
}

// Translated from 0058edd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSImpactDataSet` `InitItem` (unnamed in the map; `ECX`): unless form flag 8 is set,
/// resolves each nonzero material slot (a form id until now) to its form cast to
/// `BGSImpactData`, after giving it the compile index of the set's file; a slot left null is
/// reported with the id, the set's name (vtable slot +0x130) and form id. Then form flag 8 is
/// set.
pub fn fn_0058edd0(e: &mut Engine, this: Ptr) {
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    e.with_stack(4, |e, id_at| {
        for material in 0..12u32 {
            let slot = this.addr() + SLOTS + material * 4;
            let id = e.mem.u32(slot);
            e.mem.set_u32(id_at.addr(), id);
            if id == 0 {
                continue;
            }
            let file = e.call(GET_FILE, &args![this, u32::MAX]).u32();
            e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
            let id = e.mem.u32(id_at.addr());
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let data = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_IMPACT_DATA, 0u32],
                )
                .u32();
            e.mem.set_u32(slot, data);
            if e.mem.u32(slot) != 0 {
                continue;
            }
            let form_id = e.call(FORM_ID_0084E3A0, &args![this]).u32();
            let name = e.vcall(this.addr(), 0x130, &args![]).u32();
            let id = e.mem.u32(id_at.addr());
            e.call(LOG, &args![MESSAGE_NO_IMPACT_DATA, id, name, form_id]);
        }
    });
    e.call(SET_FORM_FLAG_8, &args![this, 1u32]);
}

// Translated from 0058ea10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058ea10 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f109cb, state variable) is not translated
/// Unnamed in the map (no words): the default impact data set (`011ca830`) is form 0x276 cast
/// to `BGSImpactDataSet`; when there is none, a new one (`00401000` of 0x4c bytes, made by
/// `0058e810`, null when the allocation fails) takes the form id 0x276 (vtable slot +0x128
/// with 1), the editor id `"DefaultImpactDataSet"` (slot +0x134) and is filed in the data
/// handler (`TESDataHandler::AddFormToDataHandler`).
pub fn fn_0058ea10(e: &mut Engine) {
    let form = e.call(LOOKUP_FORM, &args![DEFAULT_SET_ID]).u32();
    let set = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_IMPACT_DATA_SET, 0u32],
        )
        .u32();
    e.set_global(DEFAULT_SET, set);
    if e.global::<u32>(DEFAULT_SET) != 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    let set = if block != 0 {
        fn_0058e810(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.set_global(DEFAULT_SET, set);
    let set = e.global::<u32>(DEFAULT_SET);
    e.vcall(set, 0x128, &args![DEFAULT_SET_ID, 1u32]);
    let set = e.global::<u32>(DEFAULT_SET);
    e.vcall(set, 0x134, &args![DEFAULT_SET_NAME]);
    let set = e.global::<u32>(DEFAULT_SET);
    let data_handler = e.global::<u32>(DATA_HANDLER);
    e.call(ADD_FORM_TO_DATA_HANDLER, &args![data_handler, set]);
}

// Translated from 0058ef10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: slot of the `BGSImpactDataSet`'s vtable at +0x18 (`ECX` is that +0x18,
/// two stack words, `RET 8`): for each of the twelve materials, the set's impact data
/// ([`fn_0058e9d0`] on `this - 0x18`); when there is one, its model (+0x18) is queued with
/// the model loader (`00443d30` with the two words, 1, 1, 0, 0) when it has a name
/// (`0048cee0`), and its texture set (`006733e0`), when there is one, queued too
/// (`BGSTextureSet::QueueTextureSet` with the two words and 0).
pub fn fn_0058ef10(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let set = Ptr::new(this.addr().wrapping_sub(0x18));
    for material in 0..12u32 {
        let data = fn_0058e9d0(e, set, material);
        if data == 0 {
            continue;
        }
        if e.call(MODEL_NAME_LENGTH_0048CEE0, &args![data + 0x18])
            .u32()
            != 0
        {
            let model = if data != 0 { data + 0x18 } else { 0 };
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(
                MODEL_LOADER_QUEUE_00443D30,
                &args![loader, model, first, second, 1u32, 1u32, 0u32, 0u32],
            );
        }
        let texture_set = e.call(WORD_48_006733E0, &args![data]).u32();
        if texture_set != 0 {
            e.call(QUEUE_TEXTURE_SET, &args![texture_set, first, second, 0u32]);
        }
    }
}

// Translated from 0058ec00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058ec00 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// The `BGSImpactDataSet` `Load` (unnamed in the map; `ECX` and one stack word, the file,
/// `RET 4`): false unless `TESFile::GetTESForm`'s whole `EAX` is 0x5f. Otherwise
/// `TESForm::LoadForm`, then each chunk: `EDID` (a stack buffer of the chunk's size, read
/// with at most 0x200 bytes, given to vtable slot +0x134), `OBND` (vtable slot +0xe0 with the
/// file), `DATA` (the twelve material slots, 0x30 bytes at +0x1c, each word byte-swapped when
/// the file needs it). Then true.
pub fn fn_0058ec00(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u32() != 0x5f {
        return false;
    }
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_DATA => {
                e.call(GET_CHUNK_DATA_SIZED, &args![file, base + SLOTS, 0x30u32]);
                if e.call(NEEDS_SWAP, &args![file]).bool() {
                    for material in 0..12u32 {
                        e.call(SWAP_WORD, &args![base + SLOTS + material * 4, 0u32]);
                    }
                }
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, name| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
                    e.vcall(base, 0x134, &args![name]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_e810, fn_0058e810(Ptr) -> Ptr),
        entry!(0x0058_f850, bs_map_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0058_f9a0, bs_map_base_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0058_e9d0, fn_0058e9d0(Ptr, u32) -> u32),
        entry!(0x0058_eb10, fn_0058eb10(Ptr)),
        entry!(0x0058_e8f0, fn_0058e8f0(u32) -> u32),
        entry!(0x0058_e8c0, fn_0058e8c0(Ptr)),
        entry!(0x0058_eeb0, fn_0058eeb0(u32)),
        entry!(0x0058_edd0, fn_0058edd0(Ptr)),
        entry!(0x0058_ea10, fn_0058ea10()),
        entry!(0x0058_ef10, fn_0058ef10(Ptr, u32, u32)),
        entry!(0x0058_ec00, fn_0058ec00(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0058e810_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_e810,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x004f_9d90, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5f)]),
                (0x0058_eb10, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_323c), (0x18, 4, 0x0103_3224)],
        });
    }
    #[test]
    fn at_0058f850_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_f900, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_f850, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_f850)
                .collect();
            let mut expected = vec![(0x0058_f900, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0058f9a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_f960, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_f9a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_f9a0)
                .collect();
            let mut expected = vec![(0x0058_f960, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod impact_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_0058e9d0_reads_a_slot_or_asks_the_override() {
        let mut e = engine();
        let set = e.mem.alloc(0x4c);
        e.mem.set_u32(set + 0x1c + 3 * 4, 0x0d33);
        assert_eq!(e.call(0x0058_e9d0, &args![set, 3u32]).u32(), 0x0d33);
        e.set_global(OVERRIDE_SET, 0x0777u32);
        e.register(OVERRIDE_LOOKUP_0058F6F0, |_, a| (a[2] + a[1]).into_ret());
        assert_eq!(e.call(0x0058_e9d0, &args![set, 3u32]).u32(), 0x0d36);
        assert_eq!(
            calls(&e, OVERRIDE_LOOKUP_0058F6F0),
            vec![vec![0x0777, 3, 0x0d33]]
        );
    }

    #[test]
    fn fn_0058eb10_clears_the_slots() {
        let mut e = engine();
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.call(0x0058_eb10, &args![0x5000u32]);
        assert_eq!(calls(&e, MEMSET), vec![vec![0x501c, 0, 0x30]]);
    }

    #[test]
    fn fn_0058e8f0_maps_materials() {
        let mut e = engine();
        let want = [
            0, 7, 1, 3, 2, 4, 6, 6, 8, 5, 0, 4, 5, 4, 4, 4, 9, 9, 1, 0, 4, 4, 9, 9, 3, 9, 4, 4, 9,
            9, 5, 5,
        ];
        for (material, slot) in want.iter().enumerate() {
            assert_eq!(
                e.call(0x0058_e8f0, &args![material as u32]).u32(),
                *slot,
                "material {material}"
            );
        }
        assert_eq!(e.call(0x0058_e8f0, &args![0x20u32]).u32(), 4);
        assert_eq!(e.call(0x0058_e8f0, &args![u32::MAX]).u32(), 4);
    }

    #[test]
    fn fn_0058e8c0_stores_the_vtables_then_destroys() {
        let mut e = engine();
        e.register(0x0048_3630, |_, _| Ret::default());
        let this = e.mem.alloc(0x4c);
        e.call(0x0058_e8c0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_323c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_3224);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]]);
    }

    #[test]
    fn fn_0058eeb0_hands_out_each_material() {
        let mut e = engine();
        e.register(SET_IMPACT_0058F7F0, |_, a| (0x100 + a[1]).into_ret());
        e.register(SET_MATERIAL_IMPACT_008374E0, |_, _| Ret::default());
        e.register(MATERIALS_SET_00837500, |_, _| Ret::default());
        e.call(0x0058_eeb0, &args![0x0888u32]);
        assert_eq!(e.global::<u32>(OVERRIDE_SET), 0x0888);
        let handed = calls(&e, SET_MATERIAL_IMPACT_008374E0);
        assert_eq!(handed.len(), 10);
        assert_eq!(handed[9], vec![9, 0x109]);
        assert_eq!(calls(&e, MATERIALS_SET_00837500).len(), 1);
        e.call(0x0058_eeb0, &args![0u32]);
        assert_eq!(e.global::<u32>(OVERRIDE_SET), 0);
        assert_eq!(calls(&e, SET_IMPACT_0058F7F0).len(), 10);
        assert_eq!(calls(&e, SET_MATERIAL_IMPACT_008374E0)[10], vec![0, 0]);
    }

    fn init_rig(flag_set: bool) -> (Engine, u32) {
        let mut e = engine();
        e.register_double(FORM_FLAG_8_SET, move |_, _| flag_set.into_ret());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        e.register(GET_FILE, |_, _| 0x0777u32.into_ret());
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // Ids ending in 1 have no form.
        e.register(LOOKUP_FORM, |_, a| {
            if a[0] & 0xf == 1 {
                0u32.into_ret()
            } else {
                (a[0] + 0x0900_0000).into_ret()
            }
        });
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(FORM_ID_0084E3A0, |_, _| 0x4444u32.into_ret());
        e.register(LOG, |_, _| Ret::default());
        e.put_vtable(0x0f00_f000, &[0; 0x4d]);
        e.mem.set_u32(0x0f00_f130, 0x0f00_f400);
        e.register(0x0f00_f400, |_, _| 0x0abc_0000u32.into_ret());
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this, 0x0f00_f000);
        e.mem.set_u32(this + 0x1c, 0x20);
        e.mem.set_u32(this + 0x1c + 11 * 4, 0x31);
        (e, this)
    }

    #[test]
    fn fn_0058edd0_resolves_the_slots() {
        let (mut e, this) = init_rig(false);
        e.call(0x0058_edd0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x0a00_0020);
        assert_eq!(e.mem.u32(this + 0x1c + 11 * 4), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(calls(&e, GET_FILE).len(), 2);
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[0],
            vec![0x0a00_0020, 0, TYPE_TES_FORM, TYPE_IMPACT_DATA, 0]
        );
        assert_eq!(
            calls(&e, LOG),
            vec![vec![
                MESSAGE_NO_IMPACT_DATA,
                0x0100_0031,
                0x0abc_0000,
                0x4444
            ]]
        );
        assert_eq!(calls(&e, SET_FORM_FLAG_8), vec![vec![this, 1]]);
        let (mut e, this) = init_rig(true);
        e.call(0x0058_edd0, &args![this]);
        assert!(calls(&e, GET_FILE).is_empty());
        assert!(calls(&e, SET_FORM_FLAG_8).is_empty());
    }

    fn default_rig(found: u32) -> Engine {
        let mut e = engine();
        e.register_double(LOOKUP_FORM, move |_, _| found.into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        // The constructor's callees.
        for addr in [0x0048_3370u32, 0x004f_9d90, 0x004f_15a0] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.put_vtable(0x0103_3000, &[0; 0x200]);
        e.mem.set_u32(0x0103_323c + 0x128, 0x0f01_0128);
        e.mem.set_u32(0x0103_323c + 0x134, 0x0f01_0134);
        e.register(0x0f01_0128, |_, _| Ret::default());
        e.register(0x0f01_0134, |_, _| Ret::default());
        e.set_global(DATA_HANDLER, 0x0555_0000u32);
        e.register(ADD_FORM_TO_DATA_HANDLER, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_0058ea10_keeps_a_loaded_default_set() {
        let mut e = default_rig(0x0d00_0000);
        e.call(0x0058_ea10, &args![]);
        assert_eq!(e.global::<u32>(DEFAULT_SET), 0x0d00_0000);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x0d00_0000, 0, TYPE_TES_FORM, TYPE_IMPACT_DATA_SET, 0]]
        );
        assert_eq!(calls(&e, LOOKUP_FORM), vec![vec![DEFAULT_SET_ID]]);
        assert!(calls(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn fn_0058ea10_makes_the_default_set() {
        let mut e = default_rig(0);
        e.call(0x0058_ea10, &args![]);
        let set = e.global::<u32>(DEFAULT_SET);
        assert_ne!(set, 0);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x4c]]);
        assert_eq!(e.mem.u32(set), 0x0103_323c);
        assert_eq!(calls(&e, 0x0f01_0128), vec![vec![set, DEFAULT_SET_ID, 1]]);
        assert_eq!(calls(&e, 0x0f01_0134), vec![vec![set, DEFAULT_SET_NAME]]);
        assert_eq!(
            calls(&e, ADD_FORM_TO_DATA_HANDLER),
            vec![vec![0x0555_0000, set]]
        );
    }

    #[test]
    fn fn_0058ef10_queues_models_and_texture_sets() {
        let mut e = engine();
        let set = e.mem.alloc(0x4c);
        let named = e.mem.alloc(0x50);
        let plain = e.mem.alloc(0x50);
        e.mem.set_u32(named + 0x48, 0x0ccc_0000);
        e.mem.set_u32(set + 0x1c, named);
        e.mem.set_u32(set + 0x1c + 5 * 4, plain);
        e.register_double(MODEL_NAME_LENGTH_0048CEE0, move |_, a| {
            u32::from(a[0] == named + 0x18).into_ret()
        });
        e.set_global(MODEL_LOADER, 0x0666_0000u32);
        e.register(MODEL_LOADER_QUEUE_00443D30, |_, _| Ret::default());
        e.register(WORD_48_006733E0, |e, a| e.mem.u32(a[0] + 0x48).into_ret());
        e.register(QUEUE_TEXTURE_SET, |_, _| Ret::default());
        e.call(0x0058_ef10, &args![set + 0x18, 0x11u32, 0x22u32]);
        assert_eq!(
            calls(&e, MODEL_LOADER_QUEUE_00443D30),
            vec![vec![0x0666_0000, named + 0x18, 0x11, 0x22, 1, 1, 0, 0]]
        );
        assert_eq!(
            calls(&e, QUEUE_TEXTURE_SET),
            vec![vec![0x0ccc_0000, 0x11, 0x22, 0]]
        );
        assert_eq!(calls(&e, WORD_48_006733E0), vec![vec![named], vec![plain]]);
    }

    type Script = Rc<RefCell<(Vec<(u32, u32)>, usize)>>;

    const FILE: u32 = 0x0e00_0000;

    fn load_rig(chunks: Vec<(u32, u32)>, swap: bool) -> (Engine, u32) {
        let mut e = engine();
        let script: Script = Rc::new(RefCell::new((chunks, 0)));
        e.register(GET_TES_FORM, |_, _| 0x5fu32.into_ret());
        e.register(LOAD_FORM, |_, _| Ret::default());
        let s = script.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.0.get(s.1).map_or(0, |c| c.0).into_ret()
        });
        let s = script.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            s.0[s.1].1.into_ret()
        });
        let s = script;
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.1 += 1;
            (s.1 < s.0.len()).into_ret()
        });
        e.register(GET_CHUNK_DATA_SIZED, |_, _| Ret::default());
        e.register(SWAP_WORD, |_, _| Ret::default());
        e.register_double(NEEDS_SWAP, move |_, _| swap.into_ret());
        e.put_vtable(0x0f01_1000, &[0; 0x4e]);
        e.mem.set_u32(0x0f01_10e0, 0x0f01_1100);
        e.mem.set_u32(0x0f01_1134, 0x0f01_1200);
        e.register(0x0f01_1100, |_, _| Ret::default());
        e.register(0x0f01_1200, |_, _| Ret::default());
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this, 0x0f01_1000);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_0058ec00_checks_the_whole_form_type_word() {
        let (mut e, this) = load_rig(vec![], false);
        e.register(GET_TES_FORM, |_, _| 0x15fu32.into_ret());
        assert!(!e.call(0x0058_ec00, &args![this, FILE]).bool());
        assert!(calls(&e, LOAD_FORM).is_empty());
    }

    #[test]
    fn fn_0058ec00_reads_each_chunk() {
        let chunks = vec![(CHUNK_EDID, 9), (CHUNK_OBND, 0xc), (CHUNK_DATA, 0x30)];
        let (mut e, this) = load_rig(chunks, true);
        assert!(e.call(0x0058_ec00, &args![this, FILE]).bool());
        assert_eq!(calls(&e, LOAD_FORM), vec![vec![this, FILE]]);
        let edid = calls(&e, 0x0f01_1200);
        assert_eq!(
            calls(&e, GET_CHUNK_DATA_SIZED),
            vec![vec![FILE, edid[0][1], 0x200], vec![FILE, this + 0x1c, 0x30]]
        );
        assert_eq!(calls(&e, 0x0f01_1100), vec![vec![this, FILE]]);
        let swapped = calls(&e, SWAP_WORD);
        assert_eq!(swapped.len(), 12);
        assert_eq!(swapped[11], vec![this + 0x1c + 11 * 4, 0]);
        let (mut e, this) = load_rig(vec![(CHUNK_DATA, 0x30)], false);
        assert!(e.call(0x0058_ec00, &args![this, FILE]).bool());
        assert!(calls(&e, SWAP_WORD).is_empty());
    }
}
