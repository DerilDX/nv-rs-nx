//! `fallout shared/tesresponse.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! One dialogue response (`TESResponse`): its text, the copy of another
//! response, `Load` (the `TRDT` chunk) with the byte swap of its words, and
//! `InitItem` (resolving the topic, sound and idle form ids, with the
//! master-file warnings when they are missing).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESResponse` (Xbox PDB), 0x2c bytes (the PC code allocates the
    /// same and uses the same offsets). Its first 0x18 bytes are
    /// `RESPONSE_DATA Data`, whose fields are declared here.
    pub struct TesResponse: 0x2c {
        /// `Data.eEmotion` (Xbox PDB): `DIALOGUE_EMOTION`.
        0x00 eEmotion: u32,
        /// `Data.iEmotionValue` (Xbox PDB).
        0x04 iEmotionValue: u32,
        /// `Data.pConversationTopic` (Xbox PDB): `TESTopic*` (the form id
        /// until `InitItem`).
        0x08 pConversationTopic: u32,
        /// `Data.ucResponseID` (Xbox PDB).
        0x0c ucResponseID: u8,
        /// `Data.pVoiceSound` (Xbox PDB): `TESSound*` (the form id until
        /// `InitItem`).
        0x10 pVoiceSound: u32,
        /// `Data.bUseEmotion` (Xbox PDB).
        0x14 bUseEmotion: u8,
        /// `cResponseText` (Xbox PDB): `BSStringT<char>`, 8 bytes.
        0x18 cResponseText: u32,
        /// `pSpeakerIdle` (Xbox PDB): `TESIdleForm*` (the form id until
        /// `InitItem`).
        0x20 pSpeakerIdle: u32,
        /// `pListenerIdle` (Xbox PDB): `TESIdleForm*` (the form id until
        /// `InitItem`).
        0x24 pListenerIdle: u32,
        /// `pNext` (Xbox PDB): `TESResponse*`.
        0x28 pNext: u32,
    }
}

/// The size of `RESPONSE_DATA` (Xbox PDB), the bytes the copy moves.
const RESPONSE_DATA_SIZE: u32 = 0x18;
/// The empty string (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `00559450` (`ECX` = object): the word at +0 (a string's characters).
const WORD_AT_ZERO: u32 = 0x0055_9450;
/// `006815c0` (`ECX` = object): returns `ECX`.
const SELF: u32 = 0x0068_15c0;
/// The game's `memcpy` wrapper (`00401460`, cdecl: destination, source,
/// count).
const MEMCPY: u32 = 0x0040_1460;
/// `004037f0` (`ECX` = string; characters, a word; `RET 8`): sets the
/// string.
const STRING_SET: u32 = 0x0040_37f0;
/// `004037b0` (`ECX` = string): the string object's constructor.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `004037d0` (`ECX` = string): the string object's destructor.
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `007af430` (`ECX` = object): the word at +0x20.
const WORD_AT_0X20: u32 = 0x007a_f430;
/// `0050f9c0` (`ECX` = object, one word): stores it at +0x20.
const SET_WORD_AT_0X20: u32 = 0x0050_f9c0;
/// `0059bb30` (`ECX` = object): the word at +0x24.
const WORD_AT_0X24: u32 = 0x0059_bb30;
/// `007037c0` (`ECX` = object, one word): stores it at +0x24.
const SET_WORD_AT_0X24: u32 = 0x0070_37c0;
/// `TESFile::GetTESChunk` (`004726b0`, `ECX` = file): the chunk's tag.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::GetChunkData_ov3` (`00472890`, `ECX` = file; buffer, size;
/// `RET 8`).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401680` (`ECX` = file): whether the file's words need swapping.
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// `00401080` (cdecl: the word's address, a byte): swaps a word's bytes.
const BYTE_SWAP_WORD: u32 = 0x0040_1080;
/// The `TRDT` chunk tag.
const CHUNK_TRDT: u32 = 0x5444_5254;
/// `TESForm::GetFile` (`00484e60`, `ECX` = form, one word: the index).
const GET_FILE: u32 = 0x0048_4e60;
/// The `GetFile` index word the exe passes (-1).
const ANY_FILE: u32 = 0xffff_ffff;
/// `TESForm::AddCompileIndex` (`00485d50`, cdecl: the form id's address,
/// the file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl: a form id): the form with that id, or 0.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `00401170` (`ECX` = form): the form type byte at +4, zero-extended.
const FORM_TYPE: u32 = 0x0040_1170;
/// The form type the topic must have (0x45).
const TOPIC_TYPE: u32 = 0x45;
/// The form type the sound must have (0xd).
const SOUND_TYPE: u32 = 0x0d;
/// The form type the idles must have (0x48).
const IDLE_TYPE: u32 = 0x48;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TESForm` type descriptor (`01183028`).
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// The `TESTopicInfo` type descriptor (`01184738`).
const TES_TOPIC_INFO_TYPE: u32 = 0x0118_4738;
/// `0061a310` (cdecl: an info; `TESTopic::GetOwnerTopic` in the engine
/// map): the form the warning names as the topic.
const OWNER_TOPIC: u32 = 0x0061_a310;
/// `00619d00` (`ECX` = the topic, one word: the info; `RET 4`): the form
/// the warning names as the quest.
const OWNER_QUEST: u32 = 0x0061_9d00;
/// `TESTopicInfo::GetInfoDisplayText` (`0061e2b0`, `ECX` = info; the
/// string, a byte; `RET 8`).
const GET_INFO_DISPLAY_TEXT: u32 = 0x0061_e2b0;
/// `0084e3a0` (`ECX` = form): the word at +0xc (the form id).
const FORM_ID: u32 = 0x0084_e3a0;
/// The virtual slot the warning takes the topic's name from.
const TOPIC_NAME_SLOT: u32 = 0x130;
/// The virtual slot the warning takes the quest's name from.
const QUEST_NAME_SLOT: u32 = 0x138;
/// `sprintf` (`00ec623a`, cdecl).
const SPRINTF: u32 = 0x00ec_623a;
/// `005b5e40` (cdecl: format, words): the warning output.
const WARNING: u32 = 0x005b_5e40;
/// "TopicInfo %08X \"%s\" in Quest %s (%08x) Topic %s (%08x)" (`0104cae0`).
const FORMAT_INFO: u32 = 0x0104_cae0;
/// "MASTERFILE: Unable to find TESTopic (%08X) in TESResponse init for
/// %s." (`0104ca98`).
const FORMAT_NO_TOPIC: u32 = 0x0104_ca98;
/// "MASTERFILE: Unable to find TESSound (%08X) in TESResponse init for
/// %s." (`0104ca50`).
const FORMAT_NO_SOUND: u32 = 0x0104_ca50;
/// "MASTERFILE: Could not find speaker idle (%08X) in TESResponse init for
/// %s." (`0104ca00`).
const FORMAT_NO_SPEAKER_IDLE: u32 = 0x0104_ca00;
/// "MASTERFILE: Could not find listener idle (%08X) in TESResponse init
/// for %s." (`0104c9b0`).
const FORMAT_NO_LISTENER_IDLE: u32 = 0x0104_c9b0;
/// The size of the stack buffers the descriptions go in.
const DESCRIPTION_SIZE: u32 = 0x10c;
/// The size of the stack string object (`BSStringT<char>`).
const STRING_SIZE: u32 = 8;
/// `TESResponse::iStandardEmotionValue` (`011cbdf4`, a game setting; its
/// initializer `00f54410` names it "iStandardEmotionValue").
const STANDARD_EMOTION_VALUE: u32 = 0x011c_bdf4;
/// `0043d4d0` (`ECX` = setting): the address of its value.
const SETTING_VALUE: u32 = 0x0043_d4d0;
/// `006ecd40` (`ECX` = object, one word): stores it at +4.
const SET_WORD_AT_4: u32 = 0x006e_cd40;

// Translated from 00616c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the bytes of the response's (`ECX`) data words at +0, +4, +8 and
/// +0x10 (`00401080`, each with a zero byte). The engine map gives it no
/// name.
pub fn fn_00616c70(e: &mut Engine, this: Ptr<TesResponse>) {
    for offset in [0x00, 0x04, 0x08, 0x10] {
        e.call(BYTE_SWAP_WORD, &args![this.addr() + offset, 0u32]);
    }
}

// Translated from 00616de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESResponse::GetResponseText` (Xbox PDB; `ECX` = response): the
/// characters of the response text (+0x18, `00559450`), or the empty
/// string (`01011584`) when they are 0.
pub fn tes_response_get_response_text(e: &mut Engine, this: Ptr<TesResponse>) -> u32 {
    let text = this.addr() + 0x18;
    if e.call(WORD_AT_ZERO, &args![text]).u32() == 0 {
        EMPTY_STRING
    } else {
        e.call(WORD_AT_ZERO, &args![text]).u32()
    }
}

// Translated from 006174d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the response (`ECX`) a copy of the one given (one word; `RET 4`):
/// the 0x18 data bytes (`memcpy` from `006815c0` of the source), the text
/// (`004037f0` with the source's `GetResponseText` and 0), and the words
/// at +0x20 and +0x24 (the idles; `007af430`/`0050f9c0` and
/// `0059bb30`/`007037c0`). The engine map gives it no name.
pub fn fn_006174d0(e: &mut Engine, this: Ptr<TesResponse>, source: Ptr<TesResponse>) {
    let data = e.call(SELF, &args![source]).u32();
    e.call(MEMCPY, &args![this, data, RESPONSE_DATA_SIZE]);
    let text = tes_response_get_response_text(e, source);
    e.call(STRING_SET, &args![this.addr() + 0x18, text, 0u32]);
    let speaker = e.call(WORD_AT_0X20, &args![source]).u32();
    e.call(SET_WORD_AT_0X20, &args![this, speaker]);
    let listener = e.call(WORD_AT_0X24, &args![source]).u32();
    e.call(SET_WORD_AT_0X24, &args![this, listener]);
}

// Translated from 00617530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESResponse::Load` (Xbox PDB; `ECX` = response, one word: the file;
/// `RET 4`): with a file whose current chunk is `TRDT`, reads 0x18 bytes
/// into the response's data (`GetChunkData_ov3`) and swaps its words
/// (`00616c70`) when the file says so (`00401680`).
pub fn tes_response_load(e: &mut Engine, this: Ptr<TesResponse>, file: Ptr) {
    if file.is_null() {
        return;
    }
    if e.call(GET_TES_CHUNK, &args![file]).u32() != CHUNK_TRDT {
        return;
    }
    e.call(GET_CHUNK_DATA, &args![file, this, RESPONSE_DATA_SIZE]);
    if e.call(FILE_NEEDS_SWAP, &args![file]).u8() != 0 {
        fn_00616c70(e, this);
    }
}

/// `InitItem`'s resolution of one form id field (+`offset`): the id gets
/// the owner's file's compile index (`GetFile(-1)`, `AddCompileIndex` on a
/// stack copy), and the field becomes the form with that id when it has
/// `form_type`, else 0. Returns the id after `AddCompileIndex` (what the
/// warning prints) and whether the field is now 0.
fn resolve_field(
    e: &mut Engine,
    this: Ptr<TesResponse>,
    offset: u32,
    owner: Ptr,
    form_type: u32,
) -> (u32, bool) {
    let field = this.addr() + offset;
    let id = e.mem.u32(field);
    let id = e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), id);
        let file = e.call(GET_FILE, &args![owner, ANY_FILE]).u32();
        e.call(ADD_COMPILE_INDEX, &args![local, file]);
        e.mem.u32(local.addr())
    });
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    if form != 0 && e.call(FORM_TYPE, &args![form]).u32() == form_type {
        e.mem.set_u32(field, form);
    } else {
        e.mem.set_u32(field, 0);
    }
    (id, e.mem.u32(field) == 0)
}

/// `InitItem`'s warning for a missing form: the owner cast to
/// `TESTopicInfo` (`__RTDynamicCast`), its topic (`0061a310`) and, with a
/// topic, quest (`00619d00`); its display text (first response only) in a
/// stack string; with `describe`, the description (`sprintf` of the info's
/// id, the text, the quest's name (slot 0x138) and id, the topic's name
/// (slot 0x130) and id; the empty string and 0 for a missing one) goes to
/// the warning with the id; without, the warning gets a stack buffer
/// nothing wrote (the exe's listener-idle warning passes an unwritten
/// buffer). The string is destroyed after.
fn warn_missing(e: &mut Engine, owner: Ptr, format: u32, id: u32, describe: bool) {
    let info = e
        .call(
            DYNAMIC_CAST,
            &args![owner, 0u32, TES_FORM_TYPE, TES_TOPIC_INFO_TYPE, 0u32],
        )
        .u32();
    let topic = e.call(OWNER_TOPIC, &args![info]).u32();
    let quest = if topic != 0 {
        e.call(OWNER_QUEST, &args![topic, info]).u32()
    } else {
        0
    };
    e.with_stack(STRING_SIZE, |e, string| {
        e.call(STRING_CONSTRUCT, &args![string]);
        e.call(GET_INFO_DISPLAY_TEXT, &args![info, string, 1u32]);
        e.with_stack(DESCRIPTION_SIZE, |e, buffer| {
            if describe {
                let topic_id = if topic != 0 {
                    e.call(FORM_ID, &args![topic]).u32()
                } else {
                    0
                };
                let topic_name = if topic != 0 {
                    e.vcall(topic, TOPIC_NAME_SLOT, &args![]).u32()
                } else {
                    EMPTY_STRING
                };
                let quest_id = if quest != 0 {
                    e.call(FORM_ID, &args![quest]).u32()
                } else {
                    0
                };
                let quest_name = if quest != 0 {
                    e.vcall(quest, QUEST_NAME_SLOT, &args![]).u32()
                } else {
                    EMPTY_STRING
                };
                let text = e.call(WORD_AT_ZERO, &args![string]).u32();
                let info_id = e.call(FORM_ID, &args![info]).u32();
                e.call(
                    SPRINTF,
                    &args![
                        buffer,
                        FORMAT_INFO,
                        info_id,
                        text,
                        quest_name,
                        quest_id,
                        topic_name,
                        topic_id
                    ],
                );
            }
            e.call(WARNING, &args![format, id, buffer]);
        });
        e.call(STRING_DESTRUCT, &args![string]);
    });
}

// Translated from 00617580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00617580 - the exception-unwinding frame (FS:[0]) that destroys the stack strings and the security-cookie check (00ec408c) are not translated.
/// `TESResponse::InitItem` (Xbox PDB; `ECX` = response, one word: the
/// owning form, the info; `RET 4`). Each nonzero form id field is resolved
/// with the owner's file's compile index: the topic (+8, form type 0x45),
/// the sound (+0x10, 0xd), the speaker idle (+0x20, 0x48) and the listener
/// idle (+0x24, 0x48); a field left 0 warns ("MASTERFILE: ...", with the
/// id) with a description of the info, its quest and topic (none for the
/// listener idle, which passes an unwritten buffer). Then, with a nonzero
/// emotion (+0, `00559450`), the emotion value (+4, `006ecd40`) becomes
/// `iStandardEmotionValue` (`011cbdf4`, through `0043d4d0`). The
/// exception-unwinding frame and the security-cookie check are not
/// translated.
pub fn tes_response_init_item(e: &mut Engine, this: Ptr<TesResponse>, owner: Ptr) {
    let fields = [
        (0x08, TOPIC_TYPE, FORMAT_NO_TOPIC, true),
        (0x10, SOUND_TYPE, FORMAT_NO_SOUND, true),
        (0x20, IDLE_TYPE, FORMAT_NO_SPEAKER_IDLE, true),
        (0x24, IDLE_TYPE, FORMAT_NO_LISTENER_IDLE, false),
    ];
    for (offset, form_type, format, describe) in fields {
        if e.mem.u32(this.addr() + offset) == 0 {
            continue;
        }
        let (id, missing) = resolve_field(e, this, offset, owner, form_type);
        if missing {
            warn_missing(e, owner, format, id, describe);
        }
    }
    if e.call(WORD_AT_ZERO, &args![this]).u32() != 0 {
        let value = e.call(SETTING_VALUE, &args![STANDARD_EMOTION_VALUE]).u32();
        let value = e.mem.u32(value);
        e.call(SET_WORD_AT_4, &args![this, value]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00616c70, fn_00616c70(Ptr<TesResponse>)),
        entry!(
            0x00616de0,
            tes_response_get_response_text(Ptr<TesResponse>) -> u32
        ),
        entry!(0x006174d0, fn_006174d0(Ptr<TesResponse>, Ptr<TesResponse>)),
        entry!(0x00617530, tes_response_load(Ptr<TesResponse>, Ptr)),
        entry!(0x00617580, tes_response_init_item(Ptr<TesResponse>, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn response(e: &mut Engine) -> u32 {
        e.mem.alloc(0x2c)
    }

    #[test]
    fn the_data_words_are_swapped() {
        let mut e = Engine::new();
        e.register(BYTE_SWAP_WORD, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let p = response(&mut e);
        e.call(0x0061_6c70, &args![p]);
        assert_eq!(
            calls(&e, BYTE_SWAP_WORD),
            vec![
                vec![p, 0],
                vec![p + 4, 0],
                vec![p + 8, 0],
                vec![p + 0x10, 0]
            ]
        );
    }

    #[test]
    fn the_text_is_the_string_or_empty() {
        let mut e = Engine::new();
        e.register(WORD_AT_ZERO, |e, a| ret(e.mem.u32(a[0])));
        let p = response(&mut e);
        e.mem.set_u32(p + 0x18, 0);
        assert_eq!(e.call(0x0061_6de0, &args![p]).u32(), EMPTY_STRING);
        e.mem.set_u32(p + 0x18, 0x1234);
        assert_eq!(e.call(0x0061_6de0, &args![p]).u32(), 0x1234);
    }

    #[test]
    fn the_copy_moves_data_text_and_idles() {
        let mut e = Engine::new();
        e.register(SELF, |_, a| ret(a[0]));
        e.register(MEMCPY, |_, a| ret(a[0]));
        e.register(WORD_AT_ZERO, |e, a| ret(e.mem.u32(a[0])));
        e.register(STRING_SET, |_, _| ret(1));
        e.register(WORD_AT_0X20, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(WORD_AT_0X24, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        e.register(SET_WORD_AT_0X20, |_, _| ret(0));
        e.register(SET_WORD_AT_0X24, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let this = response(&mut e);
        let source = response(&mut e);
        e.mem.set_u32(source + 0x18, 0);
        e.mem.set_u32(source + 0x20, 0x111);
        e.mem.set_u32(source + 0x24, 0x222);
        e.call(0x0061_74d0, &args![this, source]);
        assert_eq!(calls(&e, MEMCPY), vec![vec![this, source, 0x18]]);
        assert_eq!(
            calls(&e, STRING_SET),
            vec![vec![this + 0x18, EMPTY_STRING, 0]]
        );
        assert_eq!(calls(&e, SET_WORD_AT_0X20), vec![vec![this, 0x111]]);
        assert_eq!(calls(&e, SET_WORD_AT_0X24), vec![vec![this, 0x222]]);
    }

    #[test]
    fn load_reads_trdt_and_swaps_when_asked() {
        let mut e = Engine::new();
        e.map(0x0b00_0000, 0x10);
        e.register(GET_TES_CHUNK, |e, _| ret(e.mem.u32(0x0b00_0000)));
        e.register(GET_CHUNK_DATA, |_, _| ret(1));
        e.register(FILE_NEEDS_SWAP, |e, _| ret(e.mem.u32(0x0b00_0004)));
        e.register(BYTE_SWAP_WORD, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let p = response(&mut e);
        let file = e.mem.alloc(4);
        // No file: nothing.
        e.call(0x0061_7530, &args![p, 0u32]);
        assert!(calls(&e, GET_TES_CHUNK).is_empty());
        // Another chunk: nothing read.
        e.mem.set_u32(0x0b00_0000, 0x4d414e31);
        e.call(0x0061_7530, &args![p, file]);
        assert!(calls(&e, GET_CHUNK_DATA).is_empty());
        // TRDT without swapping.
        e.mem.set_u32(0x0b00_0000, CHUNK_TRDT);
        e.call(0x0061_7530, &args![p, file]);
        assert_eq!(calls(&e, GET_CHUNK_DATA), vec![vec![file, p, 0x18]]);
        assert!(calls(&e, BYTE_SWAP_WORD).is_empty());
        // TRDT with swapping.
        e.mem.set_u32(0x0b00_0004, 1);
        e.call(0x0061_7530, &args![p, file]);
        assert_eq!(calls(&e, BYTE_SWAP_WORD).len(), 4);
    }

    /// Doubles for `InitItem`: forms 0x5000_0000 + type found by id
    /// `0x100 + type` (the compile index adds 0x0100_0000 to the id).
    fn init_engine() -> Engine {
        let mut e = Engine::new();
        e.register(GET_FILE, |_, _| ret(0x6000));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            ret(0)
        });
        e.register(LOOKUP_FORM, |_, a| {
            let id = a[0] & 0x00ff_ffff;
            if (0x100..0x200).contains(&id) {
                ret(0x5000_0000 + (id - 0x100))
            } else {
                ret(0)
            }
        });
        e.register(FORM_TYPE, |_, a| ret(a[0] - 0x5000_0000));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0] + 1));
        e.register(OWNER_TOPIC, |_, a| {
            ret(if a[0] == 0x9001 { 0x9100 } else { 0 })
        });
        e.register(OWNER_QUEST, |_, _| ret(0x9200));
        e.register(STRING_CONSTRUCT, |_, a| ret(a[0]));
        e.register(STRING_DESTRUCT, |_, _| ret(0));
        e.register(GET_INFO_DISPLAY_TEXT, |e, a| {
            e.mem.set_u32(a[1], 0x7777);
            ret(0)
        });
        e.register(WORD_AT_ZERO, |e, a| ret(e.mem.u32(a[0])));
        e.register(FORM_ID, |_, a| ret(a[0] + 0x10));
        e.register(SPRINTF, |_, _| ret(0));
        e.register(WARNING, |_, _| ret(0));
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        e.register(SET_WORD_AT_4, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            ret(0)
        });
        e.map(0x9100, 0x400);
        e.mem.set_u32(0x9100, 0x9180);
        e.mem.set_u32(0x9180 + TOPIC_NAME_SLOT, 0x00ab_0001);
        e.mem.set_u32(0x9200, 0x9280);
        e.mem.set_u32(0x9280 + QUEST_NAME_SLOT, 0x00ab_0002);
        e.register(0x00ab_0001, |_, _| ret(0x5151));
        e.register(0x00ab_0002, |_, _| ret(0x5252));
        e.map(STANDARD_EMOTION_VALUE, 8);
        e.mem.set_u32(STANDARD_EMOTION_VALUE + 4, 100);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn init_item_resolves_forms_of_the_right_type() {
        let mut e = init_engine();
        let p = response(&mut e);
        e.mem.set_u32(p, 0);
        e.mem.set_u32(p + 4, 50);
        e.mem.set_u32(p + 8, 0x100 + TOPIC_TYPE);
        e.mem.set_u32(p + 0x10, 0x100 + SOUND_TYPE);
        e.mem.set_u32(p + 0x20, 0x100 + IDLE_TYPE);
        e.mem.set_u32(p + 0x24, 0);
        e.call(0x0061_7580, &args![p, 0x9000u32]);
        assert_eq!(e.mem.u32(p + 8), 0x5000_0000 + TOPIC_TYPE);
        assert_eq!(e.mem.u32(p + 0x10), 0x5000_0000 + SOUND_TYPE);
        assert_eq!(e.mem.u32(p + 0x20), 0x5000_0000 + IDLE_TYPE);
        assert_eq!(e.mem.u32(p + 0x24), 0);
        assert!(calls(&e, WARNING).is_empty());
        // A neutral emotion keeps its value.
        assert_eq!(e.mem.u32(p + 4), 50);
        assert_eq!(calls(&e, LOOKUP_FORM).len(), 3);
        assert_eq!(calls(&e, GET_FILE), vec![vec![0x9000, ANY_FILE]; 3]);
    }

    #[test]
    fn init_item_warns_for_missing_forms_and_sets_the_emotion_value() {
        let mut e = init_engine();
        let p = response(&mut e);
        e.mem.set_u32(p, 3);
        e.mem.set_u32(p + 4, 50);
        // Wrong type for the topic; not found for the others.
        e.mem.set_u32(p + 8, 0x100 + SOUND_TYPE);
        e.mem.set_u32(p + 0x10, 0x300);
        e.mem.set_u32(p + 0x20, 0x100 + TOPIC_TYPE);
        e.mem.set_u32(p + 0x24, 0x301);
        // The info 0x9001 has a topic and quest.
        e.call(0x0061_7580, &args![p, 0x9000u32]);
        for offset in [8, 0x10, 0x20, 0x24] {
            assert_eq!(e.mem.u32(p + offset), 0, "{offset:#x}");
        }
        let warnings = calls(&e, WARNING);
        assert_eq!(warnings.len(), 4);
        let formats: Vec<_> = warnings.iter().map(|w| (w[0], w[1])).collect();
        assert_eq!(
            formats,
            vec![
                (FORMAT_NO_TOPIC, 0x0100_0100 + SOUND_TYPE),
                (FORMAT_NO_SOUND, 0x0100_0300),
                (FORMAT_NO_SPEAKER_IDLE, 0x0100_0100 + TOPIC_TYPE),
                (FORMAT_NO_LISTENER_IDLE, 0x0100_0301),
            ]
        );
        // Three descriptions, none for the listener idle.
        let sprintfs = calls(&e, SPRINTF);
        assert_eq!(sprintfs.len(), 3);
        assert_eq!(
            sprintfs[0][1..].to_vec(),
            vec![FORMAT_INFO, 0x9011, 0x7777, 0x5252, 0x9210, 0x5151, 0x9110]
        );
        assert_eq!(warnings[0][2], sprintfs[0][0]);
        assert_eq!(
            calls(&e, GET_INFO_DISPLAY_TEXT).len(),
            4,
            "the listener idle reads the text too"
        );
        assert_eq!(calls(&e, STRING_DESTRUCT).len(), 4);
        assert_eq!(calls(&e, OWNER_QUEST)[0], vec![0x9100, 0x9001]);
        // A nonzero emotion takes iStandardEmotionValue.
        assert_eq!(e.mem.u32(p + 4), 100);
        assert_eq!(calls(&e, SET_WORD_AT_4), vec![vec![p, 100]]);
    }

    #[test]
    fn init_item_describes_an_info_without_topic() {
        let mut e = init_engine();
        let p = response(&mut e);
        e.mem.set_u32(p, 0);
        e.mem.set_u32(p + 8, 0);
        e.mem.set_u32(p + 0x10, 0x300);
        e.mem.set_u32(p + 0x20, 0);
        e.mem.set_u32(p + 0x24, 0);
        // The info 0x9101 has no topic: no quest either.
        e.call(0x0061_7580, &args![p, 0x9100u32]);
        assert!(calls(&e, OWNER_QUEST).is_empty());
        let sprintfs = calls(&e, SPRINTF);
        assert_eq!(
            sprintfs[0][1..].to_vec(),
            vec![
                FORMAT_INFO,
                0x9111,
                0x7777,
                EMPTY_STRING,
                0,
                EMPTY_STRING,
                0
            ]
        );
        assert!(calls(&e, SET_WORD_AT_4).is_empty());
    }
}
