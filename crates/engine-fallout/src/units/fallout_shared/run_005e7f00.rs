//! `fallout shared/run_005e7f00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e8060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e8060 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x31 (0x80 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005e8060(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x30]);
    e.call(0x0040_2d00, &args![base + 0x48]);
    e.call(0x0040_2c20, &args![base + 0x54]);
    e.call(0x0047_b4f0, &args![base + 0x60]);
    e.mem.set_u32(base, 0x0104_6874);
    e.mem.set_u32(base + 0x30, 0x0104_6850);
    e.mem.set_u32(base + 0x48, 0x0104_683c);
    e.mem.set_u32(base + 0x54, 0x0104_681c);
    e.mem.set_u32(base + 0x60, 0x0104_6808);
    e.call(0x0096_a2d0, &args![base + 0x74]);
    e.call(0x004f_15a0, &args![this, 0x31]);
    e.call(0x005e_8240, &args![this]);
    this
}

use crate::units::fallout_shared::bgsnote::BGSNote;

/// `BGSNote`'s vtable (RTTI `.?AVBGSNote@@`) and those of its bases at +0x30,
/// +0x48, +0x54 and +0x60.
const NOTE_VTABLE: u32 = 0x0104_6874;
const NOTE_MODEL_VTABLE: u32 = 0x0104_6850;
const NOTE_FULL_NAME_VTABLE: u32 = 0x0104_683c;
const NOTE_ICON_VTABLE: u32 = 0x0104_681c;
const NOTE_SOUNDS_VTABLE: u32 = 0x0104_6808;
/// The base destructors `005e8170` runs: the list (`0046ffb0`, +0x74), the
/// pickup/putdown sounds (`0047b520`, +0x60), the icon (`00402c40`, +0x54),
/// the full name (`00591300`, +0x48), the model (`00489070`, +0x30) and the
/// form base (`0050e9f0`).
const SIMPLE_LIST_DESTRUCT: u32 = 0x0046_ffb0;
const SOUNDS_DESTRUCT: u32 = 0x0047_b520;
const ICON_DESTRUCT: u32 = 0x0040_2c40;
const FULL_NAME_DESTRUCT: u32 = 0x0059_1300;
const MODEL_DESTRUCT: u32 = 0x0048_9070;
const FORM_BASE_DESTRUCT: u32 = 0x0050_e9f0;
/// `00470470` (`thiscall`): empties a `BSSimpleList`.
const SIMPLE_LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// `00401030`: free (`cdecl`).
const FREE: u32 = 0x0040_1030;
/// `004f29c0` (`thiscall`, `RET 4`): the deleting destructor `005e8270` calls
/// on a type-2 note's data.
const NOTE_DATA_DELETE: u32 = 0x004f_29c0;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni heap.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `BSFadeNode::BSFadeNode` (`00b4e150`).
const FADE_NODE_CONSTRUCT: u32 = 0x00b4_e150;
/// `00438170` (`thiscall`, `RET 4`): an `NiFixedString` from a C string;
/// answers `this`.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
/// `004381b0` (`thiscall`): the `NiFixedString` destructor.
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// `00a5b950` (`thiscall`, `RET 4`, the `NiFixedString`'s address): sets an
/// `NiObjectNET`'s name.
const SET_NAME: u32 = 0x00a5_b950;
/// The form vtable slots (byte offsets): `0x130` answers the name `005e7f90`
/// gives the node, `0x134` takes the editor id read, `0xe0` the file for
/// `OBND`.
const NAME_SLOT: u32 = 0x130;
const SET_EDITOR_ID_SLOT: u32 = 0x134;
const LOAD_BOUNDS_SLOT: u32 = 0xe0;
/// `TESFile` readers: `GetTESForm` (`00472660`), `GetTESChunk`
/// (`004726b0`), next chunk (`004726f0`), `GetChunkData` (`004727f0`,
/// `RET 4`), the sized overload (`00472890`, `RET 8`) and the chunk size
/// (`00401660`).
const GET_TES_FORM: u32 = 0x0047_2660;
const GET_TES_CHUNK: u32 = 0x0047_26b0;
const NEXT_CHUNK: u32 = 0x0047_26f0;
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESForm::LoadForm` (`00485110`, `RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
/// The component loaders (`cdecl`: component, file): full name (`00487050`),
/// `TESModel::LoadModelChunk` (`004892d0`), `TESTexture::LoadTextureChunk`
/// (`0048e3d0`) and `TESDescription::LoadDescriptionData` (`00482d50`).
const LOAD_FULL_NAME: u32 = 0x0048_7050;
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
const LOAD_DESCRIPTION: u32 = 0x0048_2d50;
/// `TESTexture::IsTextureChunk` (`0048e3b0`, `cdecl`, the tag).
const IS_TEXTURE_CHUNK: u32 = 0x0048_e3b0;
/// The pickup/putdown sound setters (`thiscall` on +0x60, `RET 4`):
/// `006ecd40` for `YNAM`, `00403550` for `ZNAM`.
const SET_PICKUP_SOUND: u32 = 0x006e_cd40;
const SET_PUTDOWN_SOUND: u32 = 0x0040_3550;
/// `005ae3d0` (`thiscall`, `RET 4`, the item's address): inserts in a list.
const LIST_INSERT: u32 = 0x005a_e3d0;
/// `TESSoundFile::SetSoundFile` (`00489100`, `RET 4`).
const SET_SOUND_FILE: u32 = 0x0048_9100;
/// `bgsnote.cpp` members the load calls by address: set the type
/// (`005e8f90`), the data of a type-2 note (`005e9160`) or a type-1 note
/// (`005e91d0`), and the setters `005e9130` (the type-0 sound), `005e92d0` (the type-3
/// speaker), `005e9270` (the type-3 topic; all `RET 4`).
const NOTE_SET_TYPE: u32 = 0x005e_8f90;
const NOTE_DATA_AS_FILE: u32 = 0x005e_9160;
const NOTE_DATA_AS_DESCRIPTION: u32 = 0x005e_91d0;
const NOTE_SET_SOUND: u32 = 0x005e_9130;
const NOTE_SET_SPEAKER: u32 = 0x005e_92d0;
const NOTE_SET_TOPIC: u32 = 0x005e_9270;
/// Chunk tags.
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_YNAM: u32 = 0x4d41_4e59;
const CHUNK_ZNAM: u32 = 0x4d41_4e5a;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_ONAM: u32 = 0x4d41_4e4f;
const CHUNK_XNAM: u32 = 0x4d41_4e58;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
const CHUNK_TNAM: u32 = 0x4d41_4e54;

// Translated from 005e8240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears `cType`, the note data, `pSpeaker` and
/// `bHasBeenRead`.
pub fn fn_005e8240(e: &mut Engine, this: Ptr<BGSNote>) {
    e.set(this, BGSNote::cType, 0);
    e.set(this, BGSNote::pNoteData, 0);
    e.set(this, BGSNote::pSpeaker, 0);
    e.set(this, BGSNote::bHasBeenRead, 0);
}

// Translated from 005e8270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties `OwnerQuests` (`00470470`), then
/// releases the note data by type: type 1 frees it (`00401030`), type 2
/// deletes it (`004f29c0` with 1) when there is one; both clear it. Other
/// types keep it.
pub fn fn_005e8270(e: &mut Engine, this: Ptr<BGSNote>) {
    e.call(
        SIMPLE_LIST_REMOVE_ALL,
        &args![this.addr().wrapping_add(0x74)],
    );
    match e.get(this, BGSNote::cType) {
        1 => {
            let data = e.get(this, BGSNote::pNoteData);
            e.call(FREE, &args![data]);
            e.set(this, BGSNote::pNoteData, 0);
        }
        2 => {
            let data = e.get(this, BGSNote::pNoteData);
            if data != 0 {
                e.call(NOTE_DATA_DELETE, &args![data, 1u32]);
            }
            e.set(this, BGSNote::pNoteData, 0);
        }
        _ => {}
    }
}

// Translated from 005e8170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSNote` destructor body (unnamed in the engine map; its vtables by
/// RTTI): sets the five vtables, releases the note data (`005e8270`), then
/// destroys the bases in reverse order: the list (+0x74), the sounds
/// (+0x60), the icon (+0x54), the full name (+0x48), the model (+0x30) and
/// the form base.
// Translation incomplete: 005e8170 - the SEH frame (FS:[0], unwind of the bases) is not translated.
pub fn fn_005e8170(e: &mut Engine, this: Ptr<BGSNote>) {
    let base = this.addr();
    e.mem.set_u32(base, NOTE_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x30), NOTE_MODEL_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x48), NOTE_FULL_NAME_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x54), NOTE_ICON_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x60), NOTE_SOUNDS_VTABLE);
    fn_005e8270(e, this);
    e.call(SIMPLE_LIST_DESTRUCT, &args![base.wrapping_add(0x74)]);
    e.call(SOUNDS_DESTRUCT, &args![base.wrapping_add(0x60)]);
    e.call(ICON_DESTRUCT, &args![base.wrapping_add(0x54)]);
    e.call(FULL_NAME_DESTRUCT, &args![base.wrapping_add(0x48)]);
    e.call(MODEL_DESTRUCT, &args![base.wrapping_add(0x30)]);
    e.call(FORM_BASE_DESTRUCT, &args![this]);
}

// Translated from 005e7f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the stack word is never
/// read): a new `BSFadeNode` (0xe4 bytes from `00aa13e0`, constructed by
/// `00b4e150` unless the allocation failed) named after the note (vtable
/// `+0x130`, through a stack `NiFixedString`, `00a5b950`). Answers the node.
// Translation incomplete: 005e7f90 - the SEH frame (FS:[0], unwind of the allocation and the NiFixedString) is not translated.
pub fn fn_005e7f90(e: &mut Engine, this: Ptr<BGSNote>, _unused_1: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0xe4u32]).u32();
    let node = if block != 0 {
        e.call(FADE_NODE_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    let name = e.vcall(this.addr(), NAME_SLOT, &args![]).u32();
    e.with_stack(4, |e, fixed| {
        let string = e.call(FIXED_STRING_CONSTRUCT, &args![fixed, name]).u32();
        e.call(SET_NAME, &args![node, string]);
        e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
    });
    node
}

/// The component at `this + offset`, or null for a null note (the exe's
/// base-pointer adjustment).
fn component(this: Ptr<BGSNote>, offset: u32) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr().wrapping_add(offset)
    }
}

/// A dword chunk read into a cleared stack word.
fn read_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.call(GET_CHUNK_DATA, &args![file, word]);
        e.mem.u32(word.addr())
    })
}

// Translated from 005e84e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSNote`'s load (unnamed in the engine map), `thiscall`, `RET 4`: the
/// file. False for a record that is not form type 0x31. Otherwise
/// `TESForm::LoadForm`, then each chunk until the tag is 0 or the record
/// ends: `EDID` (at most 0x200 bytes into a stack block of the chunk's
/// size, vtable `+0x134`), `OBND` (vtable `+0xe0`), `FULL`, `MODL`/`MODT`,
/// `ICON` (the components at +0x48, +0x30, +0x54), `YNAM`/`ZNAM` (the sound
/// setters on +0x60), `DATA` (the type byte, `005e8f90`), `ONAM` (an owner
/// quest into the list at +0x74), `XNAM` (a sound file into a stack block of
/// the chunk's size, given to the data's `TESSoundFile` when there is one).
/// Other tags depend on the type (`MOVSX`, 0 to 3; others ignored): type 0
/// `SNAM` (`005e9130`), type 1 `TNAM` (the description's data), type 2 a
/// texture chunk (the data's texture), type 3 `SNAM` (the speaker, `005e92d0`) or
/// `TNAM` (the topic, `005e9270`). Answers true.
// Translation incomplete: 005e84e0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_005e84e0(e: &mut Engine, this: Ptr<BGSNote>, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u32() != 0x31 {
        return false;
    }
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    loop {
        let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
        if tag == 0 {
            break;
        }
        match tag {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size.wrapping_add(15) & !15, |e, block| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, block, 0x200u32]);
                    e.vcall(base, SET_EDITOR_ID_SLOT, &args![block]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, LOAD_BOUNDS_SLOT, &args![file]);
            }
            CHUNK_FULL => {
                e.call(LOAD_FULL_NAME, &args![component(this, 0x48), file]);
            }
            CHUNK_MODL | CHUNK_MODT => {
                e.call(LOAD_MODEL_CHUNK, &args![component(this, 0x30), file]);
            }
            CHUNK_ICON => {
                e.call(LOAD_TEXTURE_CHUNK, &args![component(this, 0x54), file]);
            }
            CHUNK_YNAM => {
                let sound = read_word(e, file);
                e.call(SET_PICKUP_SOUND, &args![base.wrapping_add(0x60), sound]);
            }
            CHUNK_ZNAM => {
                let sound = read_word(e, file);
                e.call(SET_PUTDOWN_SOUND, &args![base.wrapping_add(0x60), sound]);
            }
            CHUNK_DATA => {
                e.with_stack(4, |e, byte| {
                    e.mem.set_u8(byte.addr(), 0);
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, byte, 1u32]);
                    let note_type = u32::from(e.mem.u8(byte.addr()));
                    e.call(NOTE_SET_TYPE, &args![this, note_type]);
                });
            }
            CHUNK_ONAM => {
                let quest = read_word(e, file);
                e.with_stack(4, |e, item| {
                    e.mem.set_u32(item.addr(), quest);
                    e.call(LIST_INSERT, &args![base.wrapping_add(0x74), item]);
                });
            }
            CHUNK_XNAM => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                if size != 0 {
                    e.with_stack(size.wrapping_add(15) & !15, |e, block| {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, block, size]);
                        if e.call(NOTE_DATA_AS_FILE, &args![this]).u32() != 0 {
                            let sound_file = e.call(NOTE_DATA_AS_FILE, &args![this]).u32();
                            e.call(SET_SOUND_FILE, &args![sound_file, block]);
                        }
                    });
                }
            }
            _ => match i32::from(e.mem.i8(base.wrapping_add(0x7c))) {
                0 => {
                    if tag == CHUNK_SNAM {
                        let sound = read_word(e, file);
                        e.call(NOTE_SET_SOUND, &args![this, sound]);
                    }
                }
                1 => {
                    if tag == CHUNK_TNAM {
                        let description = e.call(NOTE_DATA_AS_DESCRIPTION, &args![this]).u32();
                        if description != 0 {
                            e.call(LOAD_DESCRIPTION, &args![description, file]);
                        }
                    }
                }
                2 => {
                    if e.call(IS_TEXTURE_CHUNK, &args![tag]).bool() {
                        let texture = e.call(NOTE_DATA_AS_FILE, &args![this]).u32();
                        if texture != 0 {
                            e.call(LOAD_TEXTURE_CHUNK, &args![texture, file]);
                        }
                    }
                }
                3 => {
                    if tag == CHUNK_SNAM {
                        let speaker = read_word(e, file);
                        e.call(NOTE_SET_SPEAKER, &args![this, speaker]);
                    } else if tag == CHUNK_TNAM {
                        let topic = read_word(e, file);
                        e.call(NOTE_SET_TOPIC, &args![this, topic]);
                    }
                }
                _ => {}
            },
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
        entry!(0x005e_8060, fn_005e8060(Ptr) -> Ptr),
        entry!(0x005e_8240, fn_005e8240(Ptr<BGSNote>)),
        entry!(0x005e_8270, fn_005e8270(Ptr<BGSNote>)),
        entry!(0x005e_8170, fn_005e8170(Ptr<BGSNote>)),
        entry!(0x005e_7f90, fn_005e7f90(Ptr<BGSNote>, u32) -> u32),
        entry!(0x005e_84e0, fn_005e84e0(Ptr<BGSNote>, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005e8060_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_8060,
            size: 0x80,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x30)]),
                (0x0040_2d00, &[A::T(0x48)]),
                (0x0040_2c20, &[A::T(0x54)]),
                (0x0047_b4f0, &[A::T(0x60)]),
                (0x0096_a2d0, &[A::T(0x74)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x31)]),
                (0x005e_8240, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_6874),
                (0x30, 4, 0x0104_6850),
                (0x48, 4, 0x0104_683c),
                (0x54, 4, 0x0104_681c),
                (0x60, 4, 0x0104_6808),
            ],
        });
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FORM_VTABLE: u32 = 0x00ab_0000;
    const EDITOR_ID_FN: u32 = 0x00ab_1000;
    const BOUNDS_FN: u32 = 0x00ab_1004;
    const NAME_FN: u32 = 0x00ab_1008;

    type Chunks = Rc<RefCell<(usize, Vec<(u32, Vec<u8>)>)>>;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn note(e: &mut Engine) -> Ptr<BGSNote> {
        let n = e.new_object::<BGSNote>();
        e.mem.set_u32(n.addr(), FORM_VTABLE);
        n
    }

    fn vtable(e: &mut Engine) {
        let mut slots = vec![0u32; 0x4e];
        slots[0x134 / 4] = EDITOR_ID_FN;
        slots[0xe0 / 4] = BOUNDS_FN;
        slots[0x130 / 4] = NAME_FN;
        e.put_vtable(FORM_VTABLE, &slots);
        e.register(EDITOR_ID_FN, |_, _| Ret::default());
        e.register(BOUNDS_FN, |_, _| Ret::default());
        e.register(NAME_FN, |_, _| 0x0077_0000u32.into_ret());
    }

    #[test]
    fn fn_005e8240_clears_and_005e8270_releases_by_type() {
        let mut e = Engine::new();
        for addr in [SIMPLE_LIST_REMOVE_ALL, FREE, NOTE_DATA_DELETE] {
            e.register(addr, |_, _| Ret::default());
        }
        let n = note(&mut e);
        e.mem.write(n.addr() + 0x6c, &[0xff; 0x12]);
        e.call(0x005e_8240, &args![n]);
        assert_eq!(e.get(n, BGSNote::cType), 0);
        assert_eq!(e.get(n, BGSNote::bHasBeenRead), 0);
        assert_eq!(
            (e.get(n, BGSNote::pNoteData), e.get(n, BGSNote::pSpeaker)),
            (0, 0)
        );
        for (note_type, data, freed, deleted, kept) in [
            (1u8, 0x500u32, true, false, false),
            (2, 0x600, false, true, false),
            (2, 0, false, false, false),
            (3, 0x700, false, false, true),
        ] {
            e.set(n, BGSNote::cType, note_type);
            e.set(n, BGSNote::pNoteData, data);
            e.call_log = Some(vec![]);
            e.call(0x005e_8270, &args![n]);
            assert_eq!(
                calls_to(&e, SIMPLE_LIST_REMOVE_ALL),
                vec![vec![n.addr() + 0x74]]
            );
            assert_eq!(calls_to(&e, FREE) == vec![vec![data]], freed);
            assert_eq!(
                calls_to(&e, NOTE_DATA_DELETE) == vec![vec![data, 1]],
                deleted
            );
            assert_eq!(e.get(n, BGSNote::pNoteData) == data && data != 0, kept);
        }
    }

    #[test]
    fn fn_005e8170_destroys_in_reverse_order() {
        let mut e = Engine::new();
        for addr in [
            SIMPLE_LIST_REMOVE_ALL,
            SIMPLE_LIST_DESTRUCT,
            SOUNDS_DESTRUCT,
            ICON_DESTRUCT,
            FULL_NAME_DESTRUCT,
            MODEL_DESTRUCT,
            FORM_BASE_DESTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let n = note(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005e_8170, &args![n]);
        let a = n.addr();
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (SIMPLE_LIST_REMOVE_ALL, vec![a + 0x74]),
                (SIMPLE_LIST_DESTRUCT, vec![a + 0x74]),
                (SOUNDS_DESTRUCT, vec![a + 0x60]),
                (ICON_DESTRUCT, vec![a + 0x54]),
                (FULL_NAME_DESTRUCT, vec![a + 0x48]),
                (MODEL_DESTRUCT, vec![a + 0x30]),
                (FORM_BASE_DESTRUCT, vec![a]),
            ]
        );
        assert_eq!(
            [0, 0x30, 0x48, 0x54, 0x60].map(|o| e.mem.u32(a + o)),
            [
                NOTE_VTABLE,
                NOTE_MODEL_VTABLE,
                NOTE_FULL_NAME_VTABLE,
                NOTE_ICON_VTABLE,
                NOTE_SOUNDS_VTABLE
            ]
        );
    }

    #[test]
    fn fn_005e7f90_makes_a_named_fade_node() {
        let mut e = Engine::new();
        vtable(&mut e);
        e.register(NI_ALLOCATE, |_, _| 0x0066_0000u32.into_ret());
        e.register(FADE_NODE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1] + 1);
            a[0].into_ret()
        });
        e.register(FIXED_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(SET_NAME, |_, _| Ret::default());
        let n = note(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_7f90, &args![n, 0u32]).u32(), 0x0066_0000);
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0xe4]]);
        let fixed = calls_to(&e, FIXED_STRING_CONSTRUCT)[0].clone();
        assert_eq!(fixed[1], 0x0077_0000);
        assert_eq!(calls_to(&e, SET_NAME), vec![vec![0x0066_0000, fixed[0]]]);
        assert_eq!(calls_to(&e, FIXED_STRING_DESTRUCT), vec![vec![fixed[0]]]);
        // A failed allocation names a null node.
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_7f90, &args![n, 0u32]).u32(), 0);
        assert!(calls_to(&e, FADE_NODE_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&e, SET_NAME)[0][0], 0);
    }

    fn loader(record_type: u32, chunks: Vec<(u32, Vec<u8>)>) -> Engine {
        let mut e = Engine::new();
        vtable(&mut e);
        let state: Chunks = Rc::new(RefCell::new((0, chunks)));
        e.register_double(GET_TES_FORM, move |_, _| record_type.into_ret());
        let s = state.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.1.get(s.0).map_or(0, |c| c.0).into_ret()
        });
        let s = state.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.0 += 1;
            u32::from(s.0 < s.1.len()).into_ret()
        });
        let s = state.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            (s.1[s.0].1.len() as u32).into_ret()
        });
        let s = state.clone();
        e.register_double(GET_CHUNK_DATA_SIZED, move |e, a| {
            let data = s.borrow().1[s.borrow().0].1.clone();
            e.mem.write(a[1], &data[..data.len().min(a[2] as usize)]);
            Ret::default()
        });
        let s = state;
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = s.borrow().1[s.borrow().0].1.clone();
            e.mem.write(a[1], &data);
            Ret::default()
        });
        // The type setter stores the type as the exe's 005e8f90 would.
        e.register(NOTE_SET_TYPE, |e, a| {
            e.mem.set_u8(a[0] + 0x7c, a[1] as u8);
            Ret::default()
        });
        e.register(NOTE_DATA_AS_FILE, |_, _| 0x0088_0000u32.into_ret());
        e.register(NOTE_DATA_AS_DESCRIPTION, |_, _| 0x0099_0000u32.into_ret());
        e.register(IS_TEXTURE_CHUNK, |_, a| {
            u32::from(a[0] == 0x4f43_4943).into_ret()
        });
        for addr in [
            LOAD_FORM,
            LOAD_FULL_NAME,
            LOAD_MODEL_CHUNK,
            LOAD_TEXTURE_CHUNK,
            LOAD_DESCRIPTION,
            SET_PICKUP_SOUND,
            SET_PUTDOWN_SOUND,
            LIST_INSERT,
            SET_SOUND_FILE,
            NOTE_SET_SOUND,
            NOTE_SET_SPEAKER,
            NOTE_SET_TOPIC,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    fn word(v: u32) -> Vec<u8> {
        v.to_le_bytes().to_vec()
    }

    #[test]
    fn fn_005e84e0_loads_the_common_chunks() {
        let mut e = loader(
            0x31,
            vec![
                (CHUNK_EDID, b"Note\0".to_vec()),
                (CHUNK_OBND, vec![0; 12]),
                (CHUNK_FULL, b"N\0".to_vec()),
                (CHUNK_MODL, b"m.nif\0".to_vec()),
                (CHUNK_ICON, b"i.dds\0".to_vec()),
                (CHUNK_YNAM, word(0x11)),
                (CHUNK_ZNAM, word(0x22)),
                (CHUNK_DATA, vec![0]),
                (CHUNK_ONAM, word(0x33)),
                (CHUNK_XNAM, b"s.wav\0".to_vec()),
                (CHUNK_SNAM, word(0x44)),
            ],
        );
        let n = note(&mut e);
        let a = n.addr();
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_84e0, &args![n, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, EDITOR_ID_FN).len(), 1);
        assert_eq!(calls_to(&e, BOUNDS_FN), vec![vec![a, 0x9000]]);
        assert_eq!(calls_to(&e, LOAD_FULL_NAME), vec![vec![a + 0x48, 0x9000]]);
        assert_eq!(calls_to(&e, LOAD_MODEL_CHUNK), vec![vec![a + 0x30, 0x9000]]);
        assert_eq!(
            calls_to(&e, LOAD_TEXTURE_CHUNK),
            vec![vec![a + 0x54, 0x9000]]
        );
        assert_eq!(calls_to(&e, SET_PICKUP_SOUND), vec![vec![a + 0x60, 0x11]]);
        assert_eq!(calls_to(&e, SET_PUTDOWN_SOUND), vec![vec![a + 0x60, 0x22]]);
        assert_eq!(calls_to(&e, NOTE_SET_TYPE), vec![vec![a, 0]]);
        assert_eq!(calls_to(&e, LIST_INSERT)[0][0], a + 0x74);
        assert_eq!(calls_to(&e, SET_SOUND_FILE)[0][0], 0x0088_0000);
        assert_eq!(calls_to(&e, NOTE_SET_SOUND), vec![vec![a, 0x44]]);
        // Another record type: false.
        let mut e = loader(0x30, vec![]);
        let n = note(&mut e);
        assert!(!e.call(0x005e_84e0, &args![n, 0x9000u32]).bool());
    }

    #[test]
    fn fn_005e84e0_reads_the_typed_chunks() {
        // Type 1: TNAM is the description; SNAM is ignored.
        let mut e = loader(
            0x31,
            vec![
                (CHUNK_DATA, vec![1]),
                (CHUNK_SNAM, word(1)),
                (CHUNK_TNAM, vec![]),
            ],
        );
        let n = note(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005e_84e0, &args![n, 0x9000u32]);
        assert_eq!(
            calls_to(&e, LOAD_DESCRIPTION),
            vec![vec![0x0099_0000, 0x9000]]
        );
        assert!(calls_to(&e, NOTE_SET_SOUND).is_empty());
        // Type 2: texture chunks go to the image.
        let mut e = loader(
            0x31,
            vec![
                (CHUNK_DATA, vec![2]),
                (0x4f43_4943, vec![]),
                (CHUNK_TNAM, vec![]),
            ],
        );
        let n = note(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005e_84e0, &args![n, 0x9000u32]);
        assert_eq!(
            calls_to(&e, LOAD_TEXTURE_CHUNK),
            vec![vec![0x0088_0000, 0x9000]]
        );
        // Type 3: SNAM the speaker, TNAM the topic; a zero tag stops.
        let mut e = loader(
            0x31,
            vec![
                (CHUNK_DATA, vec![3]),
                (CHUNK_SNAM, word(5)),
                (CHUNK_TNAM, word(6)),
                (0, vec![]),
                (CHUNK_SNAM, word(7)),
            ],
        );
        let n = note(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005e_84e0, &args![n, 0x9000u32]);
        assert_eq!(calls_to(&e, NOTE_SET_SPEAKER), vec![vec![n.addr(), 5]]);
        assert_eq!(calls_to(&e, NOTE_SET_TOPIC), vec![vec![n.addr(), 6]]);
        // An empty XNAM reads nothing; a type above 3 ignores SNAM.
        let mut e = loader(
            0x31,
            vec![
                (CHUNK_XNAM, vec![]),
                (CHUNK_DATA, vec![0x80]),
                (CHUNK_SNAM, word(1)),
            ],
        );
        let n = note(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005e_84e0, &args![n, 0x9000u32]);
        assert!(calls_to(&e, NOTE_DATA_AS_FILE).is_empty());
        assert!(calls_to(&e, NOTE_SET_SOUND).is_empty());
    }
}
