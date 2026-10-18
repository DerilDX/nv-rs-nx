//! `fallout shared/testopicinfo.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A topic info's responses: the response list read back from the info's
//! record (`TESTopicInfo::GetResponseList`, kept in one shared list for the
//! last info asked without a list of its own), the list helpers (append,
//! clear, copy) and the info's display text (`GetInfoDisplayText`).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESResponse` (Xbox PDB), 0x2c bytes (the PC code allocates the
    /// same); the fields used here.
    pub struct TesResponse: 0x2c {
        /// `pNext` (Xbox PDB): `TESResponse*`.
        0x28 pNext: u32,
    }
}

/// The offset of `TESTopicInfo::iFileOffset` (Xbox PDB +0x5c; the PC code
/// reads +0x4c, `TESForm` being 0x10 smaller): the record's offset in its
/// file.
const INFO_FILE_OFFSET: u32 = 0x4c;
/// The shared response list (`011cc07c`, one word: the first response).
const SHARED_RESPONSES: u32 = 0x011c_c07c;
/// The info whose responses the shared list holds (`011cbf3c`).
const SHARED_RESPONSES_INFO: u32 = 0x011c_bf3c;
/// `0045cd60` (`ECX` = response): the word at +0x28 (`pNext`).
const RESPONSE_NEXT: u32 = 0x0045_cd60;
/// `00559450` (`ECX` = object): the word at +0 (a string's characters; a
/// response list's first response).
const WORD_AT_ZERO: u32 = 0x0055_9450;
/// `TESResponse::GetResponseText` (`00616de0`, `ECX` = response).
const GET_RESPONSE_TEXT: u32 = 0x0061_6de0;
/// `BSStringT<char>::operator+=` (`00404820`, `ECX` = string, one word: the
/// characters to append; `RET 4`).
const STRING_APPEND: u32 = 0x0040_4820;
/// `004037f0` (`ECX` = string; characters, a word; `RET 8`): sets the
/// string.
const STRING_SET: u32 = 0x0040_37f0;
/// `" | "` (`0104d7c8`), between the responses of the display text.
const RESPONSE_SEPARATOR: u32 = 0x0104_d7c8;
/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00401030` (cdecl: block): frees it.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The size of a `TESResponse`.
const RESPONSE_SIZE: u32 = 0x2c;
/// `00616ce0` (`ECX` = memory): constructs a response there, returns it.
const RESPONSE_CONSTRUCT: u32 = 0x0061_6ce0;
/// `00616dc0` (`ECX` = response): its destructor.
const RESPONSE_DESTRUCT: u32 = 0x0061_6dc0;
/// `006174d0` (`ECX` = response, one word: the response to copy).
const RESPONSE_COPY: u32 = 0x0061_74d0;
/// `TESResponse::Load` (`00617530`, `ECX` = response, one word: the file).
const RESPONSE_LOAD: u32 = 0x0061_7530;
/// `TESResponse::InitItem` (`00617580`, `ECX` = response, one word: the
/// info).
const RESPONSE_INIT_ITEM: u32 = 0x0061_7580;
/// `TESForm::GetFile` (`00484e60`, `ECX` = form, one word: an index).
const GET_FILE: u32 = 0x0048_4e60;
/// The `GetFile` index word the exe passes (-1).
const ANY_FILE: u32 = 0xffff_ffff;
/// `TESFile::GetThreadSafeFile` (`004739b0`, `ECX` = file).
const GET_THREAD_SAFE_FILE: u32 = 0x0047_39b0;
/// `TESFile::SetOffset` (`004723a0`, `ECX` = file, one word: the offset).
const SET_OFFSET: u32 = 0x0047_23a0;
/// `TESFile::GetTESForm` (`00472660`, `ECX` = file): the current record's
/// form type byte.
const GET_TES_FORM: u32 = 0x0047_2660;
/// The record form type the responses are read from (0x46).
const INFO_FORM_TYPE: u8 = 0x46;
/// `TESFile::GetTESChunk` (`004726b0`, `ECX` = file): the current chunk's
/// tag.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; zero at the end.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (`004727f0`, `ECX` = file, one word: the
/// destination of one word).
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov3` (`00472890`, `ECX` = file; destination,
/// size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESForm::AddCompileIndex` (`00485d50`, cdecl: address of a form id,
/// file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl: form id): the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `00401170` (`ECX` = form): the byte at +4 (`TESForm` form type).
const FORM_TYPE: u32 = 0x0040_1170;
/// The form type an idle must have (0x48).
const IDLE_FORM_TYPE: u32 = 0x48;
/// `0084e3a0` (`ECX` = form): the word at +0xc (`TESForm` form id).
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40` (cdecl: format, arguments...): writes a warning line.
const LOG_WARNING: u32 = 0x005b_5e40;
/// `"DEFAULT: Unable to find idle (%08X) for info (%08X)."` (`0104d7cc`).
const FORMAT_NO_IDLE: u32 = 0x0104_d7cc;
/// `0050f9c0` (`ECX` = response, one word: the idle): the `SNAM` idle's
/// setter.
const SET_SNAM_IDLE: u32 = 0x0050_f9c0;
/// `007037c0` (`ECX` = response, one word: the idle): the `LNAM` idle's
/// setter.
const SET_LNAM_IDLE: u32 = 0x0070_37c0;
/// The chunk tags.
const CHUNK_TRDT: u32 = 0x5444_5254;
const CHUNK_NAM1: u32 = 0x314d_414e;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
const CHUNK_LNAM: u32 = 0x4d41_4e4c;

// Translated from 0061cad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a response list (`ECX`, the word holding the first response):
/// each response is taken off (its next, `0045cd60`, read first) and
/// deleted (`0061cb30` with 1), then the head is zeroed. The engine map
/// gives it no name.
pub fn fn_0061cad0(e: &mut Engine, this: Ptr) {
    let mut response = e.mem.u32(this.addr());
    while response != 0 {
        let next = e.call(RESPONSE_NEXT, &args![response]).u32();
        if response != 0 {
            fn_0061cb30(e, Ptr::new(response), 1);
        }
        response = next;
    }
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 0061cb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The response's (`ECX`) scalar deleting destructor (one word: the flags;
/// `RET 4`): the destructor (`00616dc0`), then, with flag bit 0, the block
/// is freed (`00401030`). Returns the response. The engine map gives it no
/// name.
pub fn fn_0061cb30(e: &mut Engine, this: Ptr<TesResponse>, flags: u32) -> Ptr<TesResponse> {
    e.call(RESPONSE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0061cb60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061cb60 - the exception-unwinding frame (FS:[0]) that frees a response whose constructor throws is not translated.
/// Makes the response list (`ECX`) a copy of the chain starting at the
/// response given (one word; `RET 4`): the list is cleared (`0061cad0`),
/// then for each response of the chain (`0045cd60`) a new one is allocated
/// (0x2c bytes, `00401000`), constructed (`00616ce0`; 0 if the allocation
/// failed), copied from it (`006174d0`) and linked after the previous copy
/// (`0061cc40`), the first appended to the list (`0061cc60`). The engine
/// map gives it no name. The exception-unwinding frame is not translated.
pub fn fn_0061cb60(e: &mut Engine, this: Ptr, first: u32) {
    fn_0061cad0(e, this);
    let mut previous = 0u32;
    let mut source = first;
    while source != 0 {
        let memory = e.call(OPERATOR_NEW, &args![RESPONSE_SIZE]).u32();
        let copy = if memory != 0 {
            e.call(RESPONSE_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        e.call(RESPONSE_COPY, &args![copy, source]);
        if previous != 0 {
            fn_0061cc40(e, Ptr::new(previous), copy);
        } else {
            fn_0061cc60(e, this, copy);
        }
        previous = copy;
        source = e.call(RESPONSE_NEXT, &args![source]).u32();
    }
}

// Translated from 0061cc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word given (one word; `RET 4`) as the response's (`ECX`)
/// next (`pNext`, +0x28). The engine map gives it no name.
pub fn fn_0061cc40(e: &mut Engine, this: Ptr<TesResponse>, next: u32) {
    e.set(this, TesResponse::pNext, next);
}

// Translated from 0061cc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a response (one word; `RET 4`) to the response list (`ECX`):
/// stored as the first response of an empty list, else handed to the last
/// response's link (`0061cc90` on the first). The engine map gives it no
/// name.
pub fn fn_0061cc60(e: &mut Engine, this: Ptr, response: u32) {
    let first = e.mem.u32(this.addr());
    if first == 0 {
        e.mem.set_u32(this.addr(), response);
    } else {
        fn_0061cc90(e, Ptr::new(first), response);
    }
}

// Translated from 0061cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Links a response (one word; `RET 4`) after the last response of the
/// chain starting at `ECX` (followed through `0045cd60`): stored as its
/// `pNext` (+0x28). The engine map gives it no name.
pub fn fn_0061cc90(e: &mut Engine, this: Ptr<TesResponse>, response: u32) {
    let mut last = this.addr();
    while e.call(RESPONSE_NEXT, &args![last]).u32() != 0 {
        last = e.call(RESPONSE_NEXT, &args![last]).u32();
    }
    e.set(Ptr::<TesResponse>::new(last), TesResponse::pNext, response);
}

// Translated from 0061d230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the shared response list (`0061cad0` on `011cc07c`); `ECX` (an
/// info) is not read. The engine map gives it no name.
pub fn fn_0061d230(e: &mut Engine, _this: Ptr) {
    fn_0061cad0(e, Ptr::new(SHARED_RESPONSES));
}

// Translated from 0061e2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopicInfo::GetInfoDisplayText` (Xbox PDB; `ECX` = info; the string,
/// a byte; `RET 8`): empties the string (`004037f0` with 0, 0), then
/// appends the text (`TESResponse::GetResponseText`) of each response of
/// the info's list (`GetResponseList` with no list of its own, its first
/// response from `00559450`), with `" | "` after each response that has a
/// next; only the first response when `first_only` is nonzero.
pub fn tes_topic_info_get_info_display_text(
    e: &mut Engine,
    this: Ptr,
    string: Ptr,
    first_only: u8,
) {
    e.call(STRING_SET, &args![string, 0u32, 0u32]);
    let list = tes_topic_info_get_response_list(e, this, Ptr::NULL);
    let mut response = e.call(WORD_AT_ZERO, &args![list]).u32();
    while response != 0 {
        let text = e.call(GET_RESPONSE_TEXT, &args![response]).u32();
        e.call(STRING_APPEND, &args![string, text]);
        if e.call(RESPONSE_NEXT, &args![response]).u32() != 0 {
            e.call(STRING_APPEND, &args![string, RESPONSE_SEPARATOR]);
        }
        if first_only != 0 {
            break;
        }
        response = e.call(RESPONSE_NEXT, &args![response]).u32();
    }
}

// Translated from 0061e780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061e780 - the exception-unwinding frame (FS:[0]) around the response allocation and the security-cookie check (00ec408c) are not translated.
/// `TESTopicInfo::GetResponseList` (Xbox PDB; `ECX` = info, one word: a
/// response list or 0; `RET 4`): the list the responses go in is the one
/// given, or the shared list (`011cc07c`) for 0, and is returned.
/// - For the info whose responses the shared list holds (`011cbf3c`): a
///   given list becomes a copy of the shared one (`0061cb60`); nothing is
///   read.
/// - Otherwise, without a list of its own, the info becomes the shared
///   list's info and the list is cleared (`0061d230`). The info's record
///   is read again: from its file (`GetFile(-1)`, then
///   `GetThreadSafeFile`) at its file offset (+0x4c; nothing with no file
///   or a zero offset, or when `SetOffset` fails, or the record is not of
///   form type 0x46), chunk by chunk (`004726f0`) until a zero tag:
///   `TRDT` starts a new response (0x2c bytes, `00616ce0`, `Load` from the
///   file, `InitItem` with the info, appended by `0061cc60`); `NAM1`
///   (with a response) sets its text from the chunk (read into a stack
///   buffer of the chunk's size, `00401660`, `0061eb40`); `SNAM` and
///   `LNAM` (with a response) read a form id, give it the file's compile
///   index and look it up; a form of type 0x48 is the idle, else 0, and a
///   nonzero id without an idle warns with the id and the info's id; the
///   idle goes to `0050f9c0` (`SNAM`) or `007037c0` (`LNAM`).
///
/// The stack buffer is the `_alloca` (`00ec5ec0`) the exe makes. The
/// exception-unwinding frame and the security-cookie check are not
/// translated.
pub fn tes_topic_info_get_response_list(e: &mut Engine, this: Ptr, list: Ptr) -> Ptr {
    let target = if list.is_null() {
        Ptr::new(SHARED_RESPONSES)
    } else {
        list
    };
    if this.addr() == e.mem.u32(SHARED_RESPONSES_INFO) {
        if list.is_null() {
            return target;
        }
        let first = e.call(WORD_AT_ZERO, &args![SHARED_RESPONSES]).u32();
        fn_0061cb60(e, list, first);
        return list;
    }
    if list.is_null() {
        e.mem.set_u32(SHARED_RESPONSES_INFO, this.addr());
        fn_0061d230(e, this);
    }
    let file = e.call(GET_FILE, &args![this, ANY_FILE]).u32();
    if file == 0 || e.mem.u32(this.addr() + INFO_FILE_OFFSET) == 0 {
        return target;
    }
    let file = e.call(GET_THREAD_SAFE_FILE, &args![file]).u32();
    let offset = e.mem.u32(this.addr() + INFO_FILE_OFFSET);
    if e.call(SET_OFFSET, &args![file, offset]).u8() == 0 {
        return target;
    }
    if e.call(GET_TES_FORM, &args![file]).u8() != INFO_FORM_TYPE {
        return target;
    }
    let mut response = 0u32;
    loop {
        let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
        if tag == 0 {
            break;
        }
        match tag {
            CHUNK_TRDT => {
                let memory = e.call(OPERATOR_NEW, &args![RESPONSE_SIZE]).u32();
                response = if memory != 0 {
                    e.call(RESPONSE_CONSTRUCT, &args![memory]).u32()
                } else {
                    0
                };
                e.call(RESPONSE_LOAD, &args![response, file]);
                e.call(RESPONSE_INIT_ITEM, &args![response, this]);
                fn_0061cc60(e, target, response);
            }
            CHUNK_NAM1 if response != 0 => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
                    fn_0061eb40(e, Ptr::new(response), buffer);
                });
            }
            CHUNK_SNAM | CHUNK_LNAM if response != 0 => {
                let idle = read_idle(e, this, file);
                let setter = if tag == CHUNK_SNAM {
                    SET_SNAM_IDLE
                } else {
                    SET_LNAM_IDLE
                };
                e.call(setter, &args![response, idle]);
            }
            _ => {}
        }
        if e.call(NEXT_CHUNK, &args![file]).u8() == 0 {
            break;
        }
    }
    target
}

/// `GetResponseList`'s `SNAM`/`LNAM` read (the same code in both cases):
/// the chunk's form id (in a stack word, 0 first) gets the file's compile
/// index and is looked up; the form is the idle if its type is 0x48. A
/// nonzero id without an idle warns with the id and the info's id.
fn read_idle(e: &mut Engine, info: Ptr, file: u32) -> u32 {
    e.with_stack(4, |e, id_slot| {
        e.mem.set_u32(id_slot.addr(), 0);
        e.call(GET_CHUNK_WORD, &args![file, id_slot]);
        e.call(ADD_COMPILE_INDEX, &args![id_slot, file]);
        let mut idle = 0u32;
        let form = e.call(LOOKUP_FORM, &args![e.mem.u32(id_slot.addr())]).u32();
        if form != 0 && e.call(FORM_TYPE, &args![form]).u32() == IDLE_FORM_TYPE {
            idle = form;
        }
        let id = e.mem.u32(id_slot.addr());
        if id != 0 && idle == 0 {
            let info_id = e.call(FORM_ID, &args![info]).u32();
            e.call(LOG_WARNING, &args![FORMAT_NO_IDLE, id, info_id]);
        }
        idle
    })
}

// Translated from 0061eb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the response's (`ECX`) text (`cResponseText`, +0x18) to the
/// characters given (one word; `RET 4`): `004037f0` with the characters
/// and 0. The engine map gives it no name.
pub fn fn_0061eb40(e: &mut Engine, this: Ptr<TesResponse>, text: Ptr) {
    e.call(STRING_SET, &args![this.addr() + 0x18, text, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_cad0, fn_0061cad0(Ptr)),
        entry!(
            0x0061_cb30,
            fn_0061cb30(Ptr<TesResponse>, u32) -> Ptr<TesResponse>
        ),
        entry!(0x0061_cb60, fn_0061cb60(Ptr, u32)),
        entry!(0x0061_cc40, fn_0061cc40(Ptr<TesResponse>, u32)),
        entry!(0x0061_cc60, fn_0061cc60(Ptr, u32)),
        entry!(0x0061_cc90, fn_0061cc90(Ptr<TesResponse>, u32)),
        entry!(0x0061_d230, fn_0061d230(Ptr)),
        entry!(
            0x0061_e2b0,
            tes_topic_info_get_info_display_text(Ptr, Ptr, u8)
        ),
        entry!(0x0061_e780, tes_topic_info_get_response_list(Ptr, Ptr) -> Ptr),
        entry!(0x0061_eb40, fn_0061eb40(Ptr<TesResponse>, Ptr)),
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

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Doubles that behave as the exe's bodies where the code follows
    /// links (`pNext` at +0x28, the word at +0), the rest record only.
    fn setup() -> Engine {
        let mut e = Engine::new();
        e.register(RESPONSE_NEXT, |e, a| ret(e.mem.u32(a[0] + 0x28)));
        e.register(WORD_AT_ZERO, |e, a| ret(e.mem.u32(a[0])));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(RESPONSE_CONSTRUCT, |_, a| ret(a[0]));
        for addr in [
            OPERATOR_DELETE,
            RESPONSE_DESTRUCT,
            RESPONSE_COPY,
            RESPONSE_LOAD,
            RESPONSE_INIT_ITEM,
            STRING_SET,
            STRING_APPEND,
            ADD_COMPILE_INDEX,
            LOG_WARNING,
            SET_SNAM_IDLE,
            SET_LNAM_IDLE,
            GET_CHUNK_DATA,
        ] {
            e.register(addr, |_, _| ret(0));
        }
        e.register(GET_RESPONSE_TEXT, |_, a| ret(a[0] + 0x18));
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.map(SHARED_RESPONSES_INFO, 4);
        e.map(SHARED_RESPONSES, 4);
        e
    }

    fn chain(e: &mut Engine, n: usize) -> Vec<u32> {
        let r: Vec<u32> = (0..n).map(|_| e.mem.alloc(RESPONSE_SIZE)).collect();
        for w in r.windows(2) {
            e.mem.set_u32(w[0] + 0x28, w[1]);
        }
        r
    }

    #[test]
    fn list_append_link_and_clear() {
        let mut e = setup();
        let head: Ptr = Ptr::new(e.mem.alloc(4));
        let r = chain(&mut e, 2);
        let extra = e.mem.alloc(RESPONSE_SIZE);
        e.call(0x0061_cc60, &args![head, r[0]]);
        assert_eq!(e.mem.u32(head.addr()), r[0]);
        e.mem.set_u32(r[1] + 0x28, 0);
        e.call(0x0061_cc60, &args![head, extra]);
        assert_eq!(e.mem.u32(r[1] + 0x28), extra);
        e.call(0x0061_cc40, &args![r[0], 0x1234u32]);
        assert_eq!(e.mem.u32(r[0] + 0x28), 0x1234);
        e.mem.set_u32(r[0] + 0x28, r[1]);

        e.call_log = Some(vec![]);
        e.call(0x0061_cad0, &args![head]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(head.addr()), 0);
        assert_eq!(
            calls(&log, RESPONSE_DESTRUCT),
            [vec![r[0]], vec![r[1]], vec![extra]]
        );
        assert_eq!(calls(&log, OPERATOR_DELETE).len(), 3);

        // The shared list, through 0061d230.
        e.mem.set_u32(SHARED_RESPONSES, r[0]);
        e.call(0x0061_d230, &args![0u32]);
        assert_eq!(e.mem.u32(SHARED_RESPONSES), 0);
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_zero() {
        let mut e = setup();
        let r = e.mem.alloc(RESPONSE_SIZE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_cb30, &args![r, 0u32]).u32(), r);
        assert_eq!(e.call(0x0061_cb30, &args![r, 3u32]).u32(), r);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, RESPONSE_DESTRUCT).len(), 2);
        assert_eq!(calls(&log, OPERATOR_DELETE), [vec![r]]);
    }

    #[test]
    fn copy_builds_a_new_chain() {
        let mut e = setup();
        let head: Ptr = Ptr::new(e.mem.alloc(4));
        let old = chain(&mut e, 1);
        e.mem.set_u32(head.addr(), old[0]);
        let source = chain(&mut e, 3);
        e.call_log = Some(vec![]);
        e.call(0x0061_cb60, &args![head, source[0]]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, RESPONSE_DESTRUCT), [vec![old[0]]]);
        let copies = calls(&log, RESPONSE_COPY);
        assert_eq!(copies.len(), 3);
        for (copy, src) in copies.iter().zip(&source) {
            assert_eq!(copy[1], *src);
        }
        assert_eq!(e.mem.u32(head.addr()), copies[0][0]);
        assert_eq!(e.mem.u32(copies[0][0] + 0x28), copies[1][0]);
        assert_eq!(e.mem.u32(copies[1][0] + 0x28), copies[2][0]);
    }

    #[test]
    fn response_text_is_set_from_the_chunk() {
        let mut e = setup();
        let r = e.mem.alloc(RESPONSE_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x0061_eb40, &args![r, 0x5555u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, STRING_SET), [vec![r + 0x18, 0x5555, 0]]);
    }

    #[test]
    fn display_text_joins_the_shared_lists_responses() {
        let mut e = setup();
        let info = e.mem.alloc(0x60);
        let r = chain(&mut e, 2);
        e.mem.set_u32(SHARED_RESPONSES_INFO, info);
        e.mem.set_u32(SHARED_RESPONSES, r[0]);
        let string = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0061_e2b0, &args![info, string, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, STRING_SET), [vec![string, 0, 0]]);
        assert_eq!(
            calls(&log, STRING_APPEND),
            [
                vec![string, r[0] + 0x18],
                vec![string, RESPONSE_SEPARATOR],
                vec![string, r[1] + 0x18],
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0061_e2b0, &args![info, string, 0x101u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, STRING_APPEND),
            [vec![string, r[0] + 0x18], vec![string, RESPONSE_SEPARATOR]]
        );
    }

    #[test]
    fn response_list_of_the_shared_info_is_copied() {
        let mut e = setup();
        let info = e.mem.alloc(0x60);
        let r = chain(&mut e, 1);
        e.mem.set_u32(SHARED_RESPONSES_INFO, info);
        e.mem.set_u32(SHARED_RESPONSES, r[0]);
        assert_eq!(
            e.call(0x0061_e780, &args![info, 0u32]).u32(),
            SHARED_RESPONSES
        );
        let list = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_e780, &args![info, list]).u32(), list);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, RESPONSE_COPY)[0][1], r[0]);
        assert!(calls(&log, GET_FILE).is_empty());
    }

    /// A record of chunks: (tag, word) pairs read in order by the doubles.
    fn file_with_chunks(e: &mut Engine, chunks: &[(u32, u32)], form_type: u8) -> u32 {
        let file = e.mem.alloc(0x18 + 8 * chunks.len() as u32);
        e.mem.set_u32(file, 0);
        e.mem.set_u8(file + 4, form_type);
        for (i, (tag, word)) in chunks.iter().enumerate() {
            e.mem.set_u32(file + 0x10 + 8 * i as u32, *tag);
            e.mem.set_u32(file + 0x14 + 8 * i as u32, *word);
        }
        e.register(GET_TES_FORM, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(GET_TES_CHUNK, |e, a| {
            let i = e.mem.u32(a[0]);
            ret(e.mem.u32(a[0] + 0x10 + 8 * i))
        });
        e.register(NEXT_CHUNK, |e, a| {
            let i = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], i);
            ret((e.mem.u32(a[0] + 0x10 + 8 * i) != 0) as u32)
        });
        e.register(CHUNK_SIZE, |e, a| {
            let i = e.mem.u32(a[0]);
            ret(e.mem.u32(a[0] + 0x14 + 8 * i))
        });
        e.register(GET_CHUNK_WORD, |e, a| {
            let file = a[0];
            let i = e.mem.u32(file);
            let word = e.mem.u32(file + 0x14 + 8 * i);
            e.mem.set_u32(a[1], word);
            ret(1)
        });
        file
    }

    #[test]
    fn response_list_reads_the_record() {
        let mut e = setup();
        // Idle form 0x77 has type 0x48; id 0x66 is another type; 0x55 is
        // missing.
        let idle = e.mem.alloc(0x10);
        e.mem.set_u8(idle + 4, 0x48);
        let other = e.mem.alloc(0x10);
        e.mem.set_u8(other + 4, 0x2a);
        e.register_double(LOOKUP_FORM, move |_, a| {
            ret(match a[0] {
                0x77 => idle,
                0x66 => other,
                _ => 0,
            })
        });
        let file = file_with_chunks(
            &mut e,
            &[
                (CHUNK_NAM1, 3), // before any response: ignored
                (CHUNK_TRDT, 0),
                (CHUNK_NAM1, 5),
                (CHUNK_SNAM, 0x77),
                (CHUNK_LNAM, 0x55),
                (CHUNK_TRDT, 0),
                (CHUNK_SNAM, 0x66),
                (CHUNK_LNAM, 0),
            ],
            0x46,
        );
        e.register_double(GET_FILE, move |_, _| ret(file));
        e.register(GET_THREAD_SAFE_FILE, |_, a| ret(a[0]));
        e.register(SET_OFFSET, |_, _| ret(1));
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(info + 0xc, 0x0001_2345);
        e.mem.set_u32(info + INFO_FILE_OFFSET, 0x400);
        e.mem.set_u32(SHARED_RESPONSES_INFO, 0x9999);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0061_e780, &args![info, 0u32]).u32(),
            SHARED_RESPONSES
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(SHARED_RESPONSES_INFO), info);
        assert_eq!(calls(&log, SET_OFFSET), [vec![file, 0x400]]);
        let loads = calls(&log, RESPONSE_LOAD);
        assert_eq!(loads.len(), 2);
        let (first, second) = (loads[0][0], loads[1][0]);
        assert_eq!(loads[0][1], file);
        assert_eq!(
            calls(&log, RESPONSE_INIT_ITEM),
            [vec![first, info], vec![second, info]]
        );
        assert_eq!(e.mem.u32(SHARED_RESPONSES), first);
        assert_eq!(e.mem.u32(first + 0x28), second);
        let data = calls(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!(data[0][2], 5);
        let sets = calls(&log, STRING_SET);
        assert_eq!(sets, [vec![first + 0x18, data[0][1], 0]]);
        assert_eq!(
            calls(&log, SET_SNAM_IDLE),
            [vec![first, idle], vec![second, 0]]
        );
        assert_eq!(
            calls(&log, SET_LNAM_IDLE),
            [vec![first, 0], vec![second, 0]]
        );
        assert_eq!(
            calls(&log, LOG_WARNING),
            [
                vec![FORMAT_NO_IDLE, 0x55, 0x0001_2345],
                vec![FORMAT_NO_IDLE, 0x66, 0x0001_2345],
            ]
        );
    }

    #[test]
    fn response_list_stops_without_a_readable_record() {
        let mut e = setup();
        let file = file_with_chunks(&mut e, &[(CHUNK_TRDT, 0)], 0x45);
        e.register_double(GET_FILE, move |_, _| ret(file));
        e.register(GET_THREAD_SAFE_FILE, |_, a| ret(a[0]));
        e.register(SET_OFFSET, |_, _| ret(1));
        let info = e.mem.alloc(0x60);
        let list = e.mem.alloc(4);
        // Not an info record.
        e.mem.set_u32(info + INFO_FILE_OFFSET, 0x400);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_e780, &args![info, list]).u32(), list);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, RESPONSE_LOAD).is_empty());
        // With a list of its own, the shared list is left alone.
        assert_eq!(e.mem.u32(SHARED_RESPONSES_INFO), 0);
        // No file offset: the file is not touched.
        e.mem.set_u32(info + INFO_FILE_OFFSET, 0);
        e.call_log = Some(vec![]);
        e.call(0x0061_e780, &args![info, list]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, GET_THREAD_SAFE_FILE).is_empty());
        // SetOffset fails.
        e.mem.set_u32(info + INFO_FILE_OFFSET, 0x400);
        e.register(SET_OFFSET, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0061_e780, &args![info, list]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, GET_TES_FORM).is_empty());
    }
}
