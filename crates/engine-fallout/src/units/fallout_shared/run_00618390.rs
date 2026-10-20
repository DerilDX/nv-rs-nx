//! `fallout shared/run_00618390` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00618580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00618580 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the topic form (`DIAL`, form type 0x45); `ECX` is the object and the byte parameter (`RET 4`) goes to
/// `+0x24`. `CreateFormOfType` (`00465110`) has no slot for type 0x45; this constructor registers the type itself
/// (`004f15a0(this, 0x45)`). The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0104_d19c`)
/// is `TESTopic`, so this is the constructor of `TESTopic`. It runs the `TESForm` constructor (`00483370`) and the component
/// constructors by address, stores the two vtable pointers (`+0`, `+0x18`), clears the two bytes at `+0x24` (the `memset`
/// wrapper `00403d30`) and then sets the byte at `+0x24` to the parameter, clears `+0x3c` and copies the float at `0101b268`
/// into `+0x28`. The exception-unwinding frame is not translated.
pub fn fn_00618580(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0104_d19c);
    e.mem.set_u32(base + 0x18, 0x0104_d188);
    e.call(0x0096_a2d0, &args![base + 0x2c]);
    e.call(0x0040_37b0, &args![base + 0x34]);
    e.call(0x0040_37b0, &args![base + 0x40]);
    e.call(0x0040_3d30, &args![base + 0x24, 0u32, 2u32]);
    e.mem.set_u32(base + 0x3c, 0);
    let value = e.global::<f32>(0x0101_b268);
    e.mem.set_f32(base + 0x28, value);
    e.mem.set_u8(base + 0x24, flag as u8);
    e.call(0x004f_15a0, &args![this, 0x45u32]);
    this
}

// Translated from 00618920 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00618940` of this unit, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00618940` is a function of this unit and returns nothing (see its
/// own doc).
pub fn fn_00618920(e: &mut Engine, this: Ptr) {
    fn_00618940(e, this);
}

// Translated from 00618940 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0061bcf0`, which stores the vtable `0104d448` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `0061bcf0`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_00618940(e: &mut Engine, this: Ptr) {
    e.call(0x0061_bcf0, &args![this]);
}

/// `005d8a70` (`ECX` = topic): the address of the per-quest entry list
/// (+0x2C).
const TOPIC_QUEST_LIST: u32 = 0x005d_8a70;
/// `006815c0` (`ECX` = node): the address of the node's item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = node): the next node, or 0.
const NODE_NEXT: u32 = 0x0072_6070;
/// `0063f7b0` (`ECX` = node): removes the node's item.
const NODE_REMOVE: u32 = 0x0063_f7b0;
/// `00717e50` (`ECX` = object): `this + 4` (in a per-quest entry, its
/// info array).
const PLUS_FOUR: u32 = 0x0071_7e50;
/// `0084e3a0` (`ECX` = object): the word at +0xC (an array's count; a
/// form's id).
const WORD_AT_0XC: u32 = 0x0084_e3a0;
/// `004037f0` (`ECX` = `BSStringT`, the text, a word): sets the string.
const STRING_SET: u32 = 0x0040_37f0;
/// The default-topic tables (see `run_00619f20`): counts per kind and the
/// 0xC-byte entries (topic, word, editor ID).
const TOPIC_COUNTS: u32 = 0x0119_a41c;
const TOPIC_ARRAYS: u32 = 0x0119_a43c;
/// The global `TESTopic::Load` leaves the last loaded topic in.
const LAST_LOADED_TOPIC: u32 = 0x011c_beb0;

const SIG_EDID: u32 = 0x4449_4445;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_FULL: u32 = 0x4c4c_5546;
const SIG_DATA: u32 = 0x4154_4144;
const SIG_QSTI: u32 = 0x4954_5351;
const SIG_INFC: u32 = 0x4346_4e49;
const SIG_INFX: u32 = 0x5846_4e49;
const SIG_PNAM: u32 = 0x4d41_4e50;
const SIG_TDUM: u32 = 0x4d55_4454;
const SIG_XIDX: u32 = 0x5844_4958;

// Translated from 00618660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopic`'s vtable slot 0x130 (`GetEditorID` in the Xbox PDB's
/// `TESForm` vtable): the text of the editor ID string at +0x40
/// (`00559450`, called again for the result), or the empty string at
/// `01011584` when that is null.
pub fn fn_00618660(e: &mut Engine, this: Ptr) -> u32 {
    let text = this.addr() + 0x40;
    if e.call(0x0055_9450, &args![text]).u32() == 0 {
        0x0101_1584
    } else {
        e.call(0x0055_9450, &args![text]).u32()
    }
}

// Translated from 00619010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the topic's string at +0x34 (`004037f0` with `text`, 0).
pub fn fn_00619010(e: &mut Engine, this: Ptr, text: Ptr) {
    e.call(STRING_SET, &args![this.addr() + 0x34, text, 0u32]);
}

// Translated from 006184f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006184f0 - the /GS cookie check (00ec408c) is not translated; the copy from _RtlCheckStack12 (00ec5ec0, the _alloca) is a with_stack block of the length (its alignment is not modelled)
/// `TESTopic`'s vtable slot 0x134 (`SetEditorID` in the Xbox PDB): copies
/// `id` into a stack buffer of its length plus one (`0044a670`, then
/// `00406d30` with the buffer, the size and `id`); when the editor ID string
/// at +0x40 differs from it (`BSStringT::StrCmp`, `00408a80`, with 1,
/// nonzero), sets the string to the copy (`004037f0` with 0) and returns
/// true; an equal one gives false.
pub fn fn_006184f0(e: &mut Engine, this: Ptr, id: Ptr) -> bool {
    let size = e.call(0x0044_a670, &args![id]).u32().wrapping_add(1);
    e.with_stack(size, |e, copy| {
        e.call(0x0040_6d30, &args![copy, size, id]);
        let text = this.addr() + 0x40;
        if e.call(0x0040_8a80, &args![text, copy, 1u32]).u32() == 0 {
            return false;
        }
        e.call(STRING_SET, &args![text, copy, 0u32]);
        true
    })
}

// Translated from 006186d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006186d0 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESTopic`'s destructor body: stores its two vtables, then empties the
/// per-quest entry list: while the first node holds an entry, each info of
/// the entry's array (`00619f20`, for every index below the count read
/// once) is taken out (`0061bd20` on the array with the index and the
/// address of a zeroed stack word), gets `0061f340(0xFFFF)` and is deleted
/// (vtable slot 0x10 with 1); the entry is deleted (`00618890` with 1,
/// when not null) and the node's item removed (`0063f7b0`, the same node
/// again). Then `00483710`, the strings at +0x40 and +0x34 (`004037d0`),
/// the list (`0046ffb0` on +0x2C), the `TESFullName` (`00591300` on +0x18)
/// and the `TESForm` base (`TESForm::~TESForm`, `00483630`).
pub fn fn_006186d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_d19c);
    e.mem.set_u32(base + 0x18, 0x0104_d188);
    let list = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
    // The exe tests `list` once and then loops on the node's item: the node
    // stays the same while each pass removes its item.
    if list != 0 {
        loop {
            let at = e.call(NODE_ITEM, &args![list]).u32();
            if e.mem.u32(at) == 0 {
                break;
            }
            let at = e.call(NODE_ITEM, &args![list]).u32();
            let entry = e.mem.u32(at);
            let array = e.call(PLUS_FOUR, &args![entry]).u32();
            let count = e.call(WORD_AT_0XC, &args![array]).u32();
            let mut index = 0u32;
            while index < count {
                let info = e.call(0x0061_9f20, &args![entry, index]).u32();
                if info != 0 {
                    e.with_stack(4, |e, out| {
                        let array = e.call(PLUS_FOUR, &args![entry]).u32();
                        e.call(0x0061_bd20, &args![array, index, out]);
                    });
                    e.call(0x0061_f340, &args![info, 0xffffu32]);
                    e.vcall(info, 0x10, &args![1u32]);
                }
                index += 1;
            }
            if entry != 0 {
                fn_00618890(e, Ptr::new(entry), 1);
            }
            e.call(NODE_REMOVE, &args![list]);
        }
    }
    e.call(0x0048_3710, &args![this]);
    e.call(0x0040_37d0, &args![base + 0x40]);
    e.call(0x0040_37d0, &args![base + 0x34]);
    e.call(0x0046_ffb0, &args![base + 0x2c]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 006186a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopic`'s scalar deleting destructor: the destructor body
/// (`006186d0`), then frees `this` (`00401030`) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_006186a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006186d0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006188c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006188c0 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// A topic's per-quest entry destructor body: the member at +0x1C
/// (`0061bf50`), then the info array at +4 (`00618920`).
pub fn fn_006188c0(e: &mut Engine, this: Ptr) {
    e.call(0x0061_bf50, &args![this.addr() + 0x1c]);
    fn_00618920(e, this.byte_add(4));
}

// Translated from 00618890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The per-quest entry's scalar deleting destructor: the destructor body
/// (`006188c0`), then frees `this` (`00401030`) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_00618890(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006188c0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESFile::GetChunkData` into a stack word preset to `start`.
fn chunk_word(e: &mut Engine, file: Ptr, start: u32) -> u32 {
    e.with_stack(4, |e, at| {
        e.mem.set_u32(at.addr(), start);
        e.call(0x0047_27f0, &args![file, at]);
        e.mem.u32(at.addr())
    })
}

/// A form id read from the chunk and given the compile index of the
/// topic's file (`TESForm::GetFile(-1)`, `TESForm::AddCompileIndex`).
fn chunk_form_id(e: &mut Engine, this: Ptr, file: Ptr) -> u32 {
    e.with_stack(4, |e, at| {
        e.call(0x0047_27f0, &args![file, at]);
        let source = e.call(0x0048_4e60, &args![this, u32::MAX]).u32();
        e.call(0x0048_5d50, &args![at, source]);
        e.mem.u32(at.addr())
    })
}

// Translated from 00618aa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00618aa0 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from _RtlCheckStack12 (00ec5ec0, the _alloca) is a with_stack block of the chunk size (its alignment, and the stack growing with each EDID chunk, are not modelled)
/// `TESTopic`'s `Load` (vtable slot 0x20): false unless the record type
/// (a byte) is 0x45. Otherwise `TESForm::LoadForm`; each existing
/// per-quest entry gets `008454f0(1)` on its +0x1C; then every chunk:
/// - `EDID`, `OBND`, `FULL` (+0x18) as every form; `DATA`: 2 bytes into
///   +0x24; `PNAM`: a word into +0x28; `XIDX`: a word into +0x3C; `TDUM`:
///   up to 0x100 bytes into a stack buffer, set as the string at +0x34
///   (`00619010`).
/// - `QSTI`: a quest form id (compile index added); its entry
///   (`006195d0`, made when missing: second word 0) becomes the current
///   one, and its info array gets `00864280` with `0044ddc0(array) + 0x64`
///   and `00600ab0` with 10.
/// - `INFC`: an info form id (compile index added); with a current entry,
///   `006cf4c0` on its +0x1C with a stack record {id, 0, -1}.
/// - `INFX`: a word (preset -1); with a current entry whose list at +0x1C
///   has `n = 0044ddc0 - 1 >= 0` (signed), record `n` (`006a7af0`) gets the
///   word at +8, and the first earlier record `i` with the same first word
///   gets it too, after which record `n` is removed (`0061bf70` with `n`,
///   1).
///
/// After the chunks, a topic with a form id up to 0x7FF that is one of the
/// default topics (`0119a43c`, counts `0119a41c`, compared unsigned) takes
/// that kind as its byte at +0x24 and that entry's editor ID (vtable slot
/// 0x134). Finally clears form flag 0x8 (`00484ab0` with 0), stores the
/// topic in `011cbeb0` and returns true.
pub fn fn_00618aa0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let record = e.call(0x0047_2660, &args![file]).u8();
    let mut current = 0u32;
    if record != 0x45 {
        return false;
    }
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    let mut node = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
    while node != 0 {
        let at = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(at) == 0 {
            break;
        }
        let at = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(at);
        if entry != 0 {
            e.call(0x0084_54f0, &args![entry + 0x1c, 1u32]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            SIG_EDID => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            SIG_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            SIG_FULL => {
                let full_name = if this.is_null() { 0 } else { base + 0x18 };
                e.call(0x0048_7050, &args![full_name, file]);
            }
            SIG_QSTI => {
                let quest = chunk_form_id(e, this, file);
                let entry = e.call(0x0061_95d0, &args![this, quest, 0u32]).u32();
                if entry != 0 {
                    current = entry;
                    let array = e.call(PLUS_FOUR, &args![entry]).u32();
                    let size = e.call(0x0044_ddc0, &args![array]).u32();
                    let array = e.call(PLUS_FOUR, &args![entry]).u32();
                    e.call(0x0086_4280, &args![array, size.wrapping_add(0x64)]);
                    let array = e.call(PLUS_FOUR, &args![entry]).u32();
                    e.call(0x0060_0ab0, &args![array, 10u32]);
                }
            }
            SIG_INFC => {
                let info = chunk_form_id(e, this, file);
                if current != 0 {
                    e.with_stack(12, |e, record| {
                        let r = record.addr();
                        e.mem.set_u32(r + 4, 0);
                        e.mem.set_u32(r, info);
                        e.mem.set_u32(r + 8, u32::MAX);
                        e.call(0x006c_f4c0, &args![current + 0x1c, record]);
                    });
                }
            }
            SIG_INFX => {
                let value = chunk_word(e, file, u32::MAX);
                if current != 0 {
                    let list = current + 0x1c;
                    let last = e.call(0x0044_ddc0, &args![list]).u32().wrapping_sub(1) as i32;
                    if last >= 0 {
                        let record = e.call(0x006a_7af0, &args![list, last]).u32();
                        e.mem.set_u32(record + 8, value);
                        // The count read again (stored, not used).
                        e.call(0x0044_ddc0, &args![list]);
                        let mut index = 0i32;
                        while index < last {
                            let earlier = e.call(0x006a_7af0, &args![list, index]).u32();
                            let newest = e.call(0x006a_7af0, &args![list, last]).u32();
                            if e.mem.u32(earlier) == e.mem.u32(newest) {
                                let earlier = e.call(0x006a_7af0, &args![list, index]).u32();
                                e.mem.set_u32(earlier + 8, value);
                                e.call(0x0061_bf70, &args![list, last, 1u32]);
                                break;
                            }
                            index += 1;
                        }
                    }
                }
            }
            SIG_PNAM => {
                e.call(0x0047_27f0, &args![file, base + 0x28]);
            }
            SIG_TDUM => {
                e.with_stack(0x100, |e, buffer| {
                    e.call(0x0047_2890, &args![file, buffer, 0x100u32]);
                    fn_00619010(e, this, buffer);
                });
            }
            SIG_DATA => {
                e.call(0x0047_2890, &args![file, base + 0x24, 2u32]);
            }
            SIG_XIDX => {
                e.call(0x0047_27f0, &args![file, base + 0x3c]);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    if e.call(WORD_AT_0XC, &args![this]).u32() <= 0x7ff {
        for kind in 0..8u32 {
            let mut index = 0u32;
            while index < e.mem.u32(TOPIC_COUNTS + 4 * kind) {
                let table = e.mem.u32(TOPIC_ARRAYS + 4 * kind);
                let entry = index.wrapping_mul(0xc).wrapping_add(table);
                if e.mem.u32(entry) == base {
                    e.mem.set_u8(base + 0x24, kind as u8);
                    let table = e.mem.u32(TOPIC_ARRAYS + 4 * kind);
                    let name = e.mem.u32(index.wrapping_mul(0xc).wrapping_add(table) + 8);
                    e.vcall(base, 0x134, &args![name]);
                }
                index += 1;
            }
        }
    }
    e.call(0x0048_4ab0, &args![this, false]);
    e.set_global::<u32>(LAST_LOADED_TOPIC, base);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_8920, fn_00618920(Ptr)),
        entry!(0x0061_8940, fn_00618940(Ptr)),
        entry!(0x0061_8580, fn_00618580(Ptr, u32) -> Ptr),
        entry!(0x0061_8660, fn_00618660(Ptr) -> u32),
        entry!(0x0061_9010, fn_00619010(Ptr, Ptr)),
        entry!(0x0061_84f0, fn_006184f0(Ptr, Ptr) -> bool),
        entry!(0x0061_86d0, fn_006186d0(Ptr)),
        entry!(0x0061_86a0, fn_006186a0(Ptr, u32) -> Ptr),
        entry!(0x0061_88c0, fn_006188c0(Ptr)),
        entry!(0x0061_8890, fn_00618890(Ptr, u32) -> Ptr),
        entry!(0x0061_8aa0, fn_00618aa0(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn fn_00618580_builds_the_object() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        e.mem.write(this, &[0xcd; 0x48]);
        e.mem.map(0x0101_b268, 4);
        e.mem.set_u32(0x0101_b268, 0x0100_0001);
        for a in [
            0x0048_3370u32,
            0x0040_2d00,
            0x0096_a2d0,
            0x0040_37b0,
            0x0040_3d30,
            0x004f_15a0,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let ret = e.call(0x0061_8580, &args![this, 0x1234_5601u32]).u32();
        assert_eq!(ret, this);
        let log: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (0x0048_3370, vec![this]),
                (0x0040_2d00, vec![this + 0x18]),
                (0x0096_a2d0, vec![this + 0x2c]),
                (0x0040_37b0, vec![this + 0x34]),
                (0x0040_37b0, vec![this + 0x40]),
                (0x0040_3d30, vec![this + 0x24, 0, 2]),
                (0x004f_15a0, vec![this, 0x45]),
            ]
        );
        let mut want = vec![0xcd_u8; 0x48];
        let mut put = |off: usize, v: u32| want[off..off + 4].copy_from_slice(&v.to_le_bytes());
        put(0, 0x0104_d19c);
        put(0x18, 0x0104_d188);
        put(0x3c, 0);
        put(0x28, 0x0100_0001);
        want[0x24] = 0x01;
        assert_eq!(e.mem.bytes(this, 0x48), want);
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_00618920_runs_the_unit_function_and_through_it_the_destructor() {
        let mut e = Engine::new();
        e.register(0x0061_bcf0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0061_8920, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0061_bcf0), vec![vec![0x5000]]);
        // The call to the unit's own function is direct, not through the table.
        assert!(calls_to(&e, 0x0061_8940).is_empty());
    }

    #[test]
    fn fwd_a03_00618940_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0061_bcf0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0061_8940, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0061_bcf0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    #[test]
    fn b020_00618660_and_00619010_strings() {
        let mut e = Engine::new();
        e.register(0x0055_9450, |_, _| ret(0x7000));
        nop(&mut e, &[STRING_SET]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_8660, &args![0x3000u32]).u32(), 0x7000);
        assert_eq!(calls_to(&e, 0x0055_9450), vec![vec![0x3040], vec![0x3040]]);
        e.register(0x0055_9450, |_, _| ret(0));
        assert_eq!(e.call(0x0061_8660, &args![0x3000u32]).u32(), 0x0101_1584);
        e.call_log = Some(vec![]);
        e.call(0x0061_9010, &args![0x3000u32, 0x7100u32]);
        assert_eq!(
            calls(&mut e, 0x0061_9010),
            vec![(STRING_SET, vec![0x3034, 0x7100, 0])]
        );
    }

    #[test]
    fn b020_006184f0_sets_a_different_editor_id() {
        let mut e = Engine::new();
        e.register(0x0044_a670, |_, _| ret(5));
        nop(&mut e, &[0x0040_6d30, STRING_SET]);
        e.register(0x0040_8a80, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0061_84f0, &args![0x3000u32, 0x7000u32]).bool());
        let copy = calls_to(&e, 0x0040_6d30);
        assert_eq!(copy.len(), 1);
        assert_eq!(&copy[0][1..], &[6, 0x7000]);
        assert_eq!(calls_to(&e, 0x0040_8a80), vec![vec![0x3040, copy[0][0], 1]]);
        assert_eq!(calls_to(&e, STRING_SET), vec![vec![0x3040, copy[0][0], 0]]);
        e.register(0x0040_8a80, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0061_84f0, &args![0x3000u32, 0x7000u32]).bool());
        assert!(calls_to(&e, STRING_SET).is_empty());
    }

    #[test]
    fn b020_006186d0_and_006186a0_destroy_the_topic() {
        let mut e = Engine::new();
        // One entry with two infos (the second null).
        let topic = e.mem.alloc(0x48);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(topic + 0x2c, entry);
        e.register(TOPIC_QUEST_LIST, |_, a| ret(a[0] + 0x2c));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_REMOVE, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(PLUS_FOUR, |_, a| ret(a[0] + 4));
        e.register(WORD_AT_0XC, |_, _| ret(2));
        e.register(0x0061_9f20, |_, a| {
            ret(if a[1] == 0 { 0x0150_1000 } else { 0 })
        });
        e.put_vtable(0x0150_0000, &[0; 8]);
        e.mem.set_u32(0x0150_0010, 0x0000_1010);
        e.map(0x0150_1000, 0x10);
        e.mem.set_u32(0x0150_1000, 0x0150_0000);
        nop(
            &mut e,
            &[
                0x0061_bd20,
                0x0061_f340,
                0x0000_1010,
                0x0061_bf50,
                0x0061_bcf0,
                0x0040_1030,
                0x0048_3710,
                0x0040_37d0,
                0x0046_ffb0,
                0x0059_1300,
                0x0048_3630,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0061_86a0, &args![topic, 1u32]);
        assert_eq!(e.mem.u32(topic), 0x0104_d19c);
        assert_eq!(e.mem.u32(topic + 0x18), 0x0104_d188);
        let taken = calls_to(&e, 0x0061_bd20);
        assert_eq!(taken.len(), 1);
        assert_eq!(&taken[0][..2], &[entry + 4, 0]);
        assert_eq!(calls_to(&e, 0x0061_f340), vec![vec![0x0150_1000, 0xffff]]);
        assert_eq!(calls_to(&e, 0x0000_1010), vec![vec![0x0150_1000, 1]]);
        assert_eq!(calls_to(&e, 0x0061_bf50), vec![vec![entry + 0x1c]]);
        assert_eq!(calls_to(&e, 0x0061_bcf0), vec![vec![entry + 4]]);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![entry], vec![topic]]);
        assert_eq!(calls_to(&e, NODE_REMOVE), vec![vec![topic + 0x2c]]);
        assert_eq!(
            calls_to(&e, 0x0040_37d0),
            vec![vec![topic + 0x40], vec![topic + 0x34]]
        );
        assert_eq!(calls_to(&e, 0x0048_3630), vec![vec![topic]]);
    }

    #[test]
    fn b020_00618890_frees_on_bit_0() {
        let mut e = Engine::new();
        nop(&mut e, &[0x0061_bf50, 0x0061_bcf0, 0x0040_1030]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_8890, &args![0x5000u32, 0u32]).u32(), 0x5000);
        assert_eq!(
            calls(&mut e, 0x0061_8890),
            vec![(0x0061_bf50, vec![0x501c]), (0x0061_bcf0, vec![0x5004])]
        );
        e.call_log = Some(vec![]);
        e.call(0x0061_8890, &args![0x5000u32, 1u32]);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![0x5000]]);
    }

    /// A topic and a file handing out `chunks`.
    fn loading(chunks: Vec<u32>) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(0x0047_2660, |_, _| ret(0x45));
        let queue = Rc::new(RefCell::new(chunks));
        let q = queue.clone();
        e.register_double(0x0047_26b0, move |_, _| ret(q.borrow()[0]));
        e.register_double(0x0047_26f0, move |_, _| {
            let mut v = queue.borrow_mut();
            v.remove(0);
            ret(u32::from(!v.is_empty()))
        });
        e.register(0x0040_1660, |_, _| ret(8));
        e.register(0x0047_27f0, |e, a| {
            e.mem.set_u32(a[1], 0x0000_0123);
            Ret::default()
        });
        e.register(0x0048_4e60, |_, _| ret(0x77));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(TOPIC_QUEST_LIST, |_, a| ret(a[0] + 0x2c));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(WORD_AT_0XC, |_, _| ret(0x800));
        nop(
            &mut e,
            &[
                0x0048_5110,
                0x0047_2890,
                0x0048_7050,
                0x0084_54f0,
                0x0000_1134,
                0x0000_10e0,
                STRING_SET,
                0x0048_4ab0,
            ],
        );
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0134, 0x0000_1134);
        e.mem.set_u32(0x0150_00e0, 0x0000_10e0);
        e.map(0x0119_a000, 0x1000);
        e.map(0x011c_b000, 0x1000);
        let topic = e.mem.alloc(0x48);
        e.mem.set_u32(topic, 0x0150_0000);
        (e, topic)
    }

    #[test]
    fn b020_00618aa0_refuses_other_records() {
        let (mut e, topic) = loading(vec![]);
        e.register(0x0047_2660, |_, _| ret(0x46));
        assert!(!e.call(0x0061_8aa0, &args![topic, 0x9000u32]).bool());
    }

    #[test]
    fn b020_00618aa0_reads_the_chunks() {
        let (mut e, topic) = loading(vec![
            SIG_EDID, SIG_OBND, SIG_FULL, SIG_DATA, SIG_PNAM, SIG_XIDX, SIG_TDUM, SIG_INFC,
            SIG_QSTI, SIG_INFC, SIG_INFX, 0x1234,
        ]);
        // An existing entry (in the list's first node) gets 008454f0.
        let old = e.mem.alloc(0x20);
        e.mem.set_u32(topic + 0x2c, old);
        let entry = e.mem.alloc(0x40);
        e.register_double(0x0061_95d0, move |_, _| ret(entry));
        e.register(PLUS_FOUR, |_, a| ret(a[0] + 4));
        e.register(0x0044_ddc0, |_, _| ret(3));
        // Records of the list at +0x1C: 0xC bytes each at `records`.
        let records = e.mem.alloc(0x24);
        e.mem.set_u32(records, 0x55);
        e.mem.set_u32(records + 0xc, 0x66);
        e.mem.set_u32(records + 0x18, 0x66);
        e.register_double(0x006a_7af0, move |_, a| ret(records + 0xc * a[1]));
        nop(
            &mut e,
            &[0x0086_4280, 0x0060_0ab0, 0x006c_f4c0, 0x0061_bf70],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0061_8aa0, &args![topic, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, 0x0084_54f0), vec![vec![old + 0x1c, 1]]);
        assert_eq!(calls_to(&e, 0x0048_7050), vec![vec![topic + 0x18, 0x9000]]);
        let sized = calls_to(&e, 0x0047_2890);
        assert_eq!(sized[1], vec![0x9000, topic + 0x24, 2]);
        assert_eq!(sized[2][2], 0x100);
        assert_eq!(
            calls_to(&e, STRING_SET),
            vec![vec![topic + 0x34, sized[2][1], 0]]
        );
        let words = calls_to(&e, 0x0047_27f0);
        assert_eq!(words[0][1], topic + 0x28);
        assert_eq!(words[1][1], topic + 0x3c);
        // The first INFC comes before any QSTI: nothing.
        assert_eq!(calls_to(&e, 0x0061_95d0), vec![vec![topic, 0x0100_0123, 0]]);
        assert_eq!(calls_to(&e, 0x0086_4280), vec![vec![entry + 4, 3 + 0x64]]);
        assert_eq!(calls_to(&e, 0x0060_0ab0), vec![vec![entry + 4, 10]]);
        let added = calls_to(&e, 0x006c_f4c0);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], entry + 0x1c);
        // INFX: record 2 and the earlier record 1 with the same first word.
        assert_eq!(e.mem.u32(records + 0x18 + 8), 0x123);
        assert_eq!(e.mem.u32(records + 0xc + 8), 0x123);
        assert_eq!(e.mem.u32(records + 8), 0);
        assert_eq!(calls_to(&e, 0x0061_bf70), vec![vec![entry + 0x1c, 2, 1]]);
        assert_eq!(calls_to(&e, 0x0048_4ab0), vec![vec![topic, 0]]);
        assert_eq!(e.global::<u32>(LAST_LOADED_TOPIC), topic);
        // Form id 0x800: no default-topic check.
        assert!(calls_to(&e, 0x0000_1134).len() == 1);
    }

    #[test]
    fn b020_00618aa0_takes_a_default_topic_kind() {
        let (mut e, topic) = loading(vec![0x1234]);
        e.register(WORD_AT_0XC, |_, _| ret(0x7ff));
        let table = e.mem.alloc(0x18);
        e.mem.set_u32(TOPIC_COUNTS + 4 * 5, 2);
        e.mem.set_u32(TOPIC_ARRAYS + 4 * 5, table);
        e.mem.set_u32(table + 0xc, topic);
        e.mem.set_u32(table + 0xc + 8, 0x7300);
        e.call_log = Some(vec![]);
        e.call(0x0061_8aa0, &args![topic, 0x9000u32]);
        assert_eq!(e.mem.u8(topic + 0x24), 5);
        assert_eq!(calls_to(&e, 0x0000_1134), vec![vec![topic, 0x7300]]);
    }
}
