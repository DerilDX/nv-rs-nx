//! `fallout shared/run_00589d30` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions here belong to `BGSAcousticSpace`: `00589d30` is the slot at
//! `+0x20` of the vtable `010320fc` (RTTI `.?AVBGSAcousticSpace@@`), which
//! the Xbox PDB's `TESForm` vtable lists as `Load`.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSAcousticSpace` (Xbox PDB, 0x64 bytes there): the fields sit 0x10
    /// lower on PC, `TESForm` being 0x18 bytes there.
    pub struct BGSAcousticSpace: 0x54 {
        /// `bIsInterior` (Xbox PDB +0x40).
        0x30 bIsInterior: bool,
        /// `eEnvType` (Xbox PDB +0x5C): `CELL_ENVIRO_TYPE`.
        0x4C eEnvType: u32,
        /// `iWallaPop` (Xbox PDB +0x60).
        0x50 iWallaPop: u32,
    }
}

/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
/// Sets (`true`) or clears form flag 0x8 (`00484ab0`, `RET 4`).
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The size of the current chunk (`00401660`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`): buffer and size.
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`): one word, the buffer.
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// Moves to the next chunk (`004726f0`, `bool`).
const NEXT_CHUNK: u32 = 0x0047_26f0;

/// The setters the `SNAM` chunks go to, in chunk order (each `RET 4`, one
/// word). The engine map gives their bodies other classes' names through
/// identical-code folding (`LowProcess::SetItemBeingUsed` for the first,
/// `LowProcess::SetGenericLocation` for the fifth); what field each one
/// writes is not confirmed here.
const SOUND_SETTERS: [u32; 5] = [
    0x008d_7dc0,
    0x0040_e500,
    0x0040_9480,
    0x0087_ce80,
    0x008d_8040,
];
/// The setter the `RDAT` chunk goes to (`00673400`, `RET 4`; the engine map
/// names its folded body `LowProcess::SetSecondGenericLocation`).
const RDAT_SETTER: u32 = 0x0067_3400;

/// Chunk signatures, as the words the exe compares.
const SIG_EDID: u32 = 0x4449_4445;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_ANAM: u32 = 0x4d41_4e41;
const SIG_INAM: u32 = 0x4d41_4e49;
const SIG_SNAM: u32 = 0x4d41_4e53;
const SIG_WNAM: u32 = 0x4d41_4e57;
const SIG_RDAT: u32 = 0x5441_4452;

// Translated from 00589d30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00589d30 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// `BGSAcousticSpace`'s `Load` (vtable slot `+0x20`): `TESForm::LoadForm`
/// (Xbox PDB, `00485110`), clears form flag 0x8 (`00484ab0`), then reads
/// every chunk of `file`:
/// - `EDID`: into a stack buffer of the chunk's size (`00401660`), read with
///   at most 0x200 bytes (`00472890`), given to vtable slot 0x134;
/// - `OBND`: vtable slot 0xE0 with the file;
/// - `ANAM`: a word given to `00589f50` (`eEnvType`);
/// - `INAM`: a word; `bIsInterior` is 1 when it is not zero;
/// - `SNAM`: a word (cleared first), the first five going to the five
///   setters in order, every later one to the first;
/// - `WNAM`: read straight into `iWallaPop` (`+0x50`);
/// - `RDAT`: a word (cleared first) given to `00673400`.
///
/// The `ANAM` word shares its stack slot with `SNAM` and `RDAT` and is not
/// cleared before the read, as in the exe. Returns true.
pub fn fn_00589d30(e: &mut Engine, this: Ptr<BGSAcousticSpace>, file: Ptr) -> bool {
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, false]);
    e.with_stack(8, |e, locals| {
        // `[EBP-0xC]` (cleared on entry) and `[EBP-0x14]`.
        let word_at = locals;
        let flag_at = locals.byte_add(4);
        let mut sounds_read = 0u32;
        loop {
            let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
            if chunk == 0 {
                break;
            }
            match chunk {
                SIG_INAM => {
                    e.call(GET_CHUNK_DATA, &args![file, flag_at]);
                    let interior = e.mem.u32(flag_at.addr()) != 0;
                    e.set(this, BGSAcousticSpace::bIsInterior, interior);
                }
                SIG_EDID => {
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    e.with_stack(size, |e, buffer| {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                        e.vcall(this.addr(), 0x134, &args![buffer]);
                    });
                }
                SIG_OBND => {
                    e.vcall(this.addr(), 0xe0, &args![file]);
                }
                SIG_ANAM => {
                    e.call(GET_CHUNK_DATA, &args![file, word_at]);
                    let value = e.mem.u32(word_at.addr());
                    fn_00589f50(e, this, value);
                }
                SIG_SNAM => {
                    e.mem.set_u32(word_at.addr(), 0);
                    e.call(GET_CHUNK_DATA, &args![file, word_at]);
                    let value = e.mem.u32(word_at.addr());
                    let setter = match sounds_read {
                        4 => SOUND_SETTERS[4],
                        3 => SOUND_SETTERS[3],
                        2 => SOUND_SETTERS[2],
                        1 => SOUND_SETTERS[1],
                        // Index 0 and every index past 4 (the exe's last
                        // `else`).
                        _ => SOUND_SETTERS[0],
                    };
                    e.call(setter, &args![this, value]);
                    sounds_read = sounds_read.wrapping_add(1);
                }
                SIG_WNAM => {
                    let walla = this.addr() + 0x50;
                    e.call(GET_CHUNK_DATA, &args![file, walla]);
                }
                SIG_RDAT => {
                    e.mem.set_u32(word_at.addr(), 0);
                    e.call(GET_CHUNK_DATA, &args![file, word_at]);
                    let value = e.mem.u32(word_at.addr());
                    e.call(RDAT_SETTER, &args![this, value]);
                }
                _ => {}
            }
            if !e.call(NEXT_CHUNK, &args![file]).bool() {
                break;
            }
        }
    });
    true
}

// Translated from 00589f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in `eEnvType` (`+0x4C`); `RET 4`.
pub fn fn_00589f50(e: &mut Engine, this: Ptr<BGSAcousticSpace>, value: u32) {
    e.set(this, BGSAcousticSpace::eEnvType, value);
}

// Translated from 0058a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bIsInterior` (`+0x30`).
pub fn fn_0058a200(e: &mut Engine, this: Ptr<BGSAcousticSpace>) -> bool {
    e.get(this, BGSAcousticSpace::bIsInterior)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_9d30, fn_00589d30(Ptr<BGSAcousticSpace>, Ptr) -> bool),
        entry!(0x0058_9f50, fn_00589f50(Ptr<BGSAcousticSpace>, u32)),
        entry!(0x0058_a200, fn_0058a200(Ptr<BGSAcousticSpace>) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn load_reads_every_chunk_kind() {
        let mut e = Engine::new();
        // Chunks in reverse (popped from the end).
        let chunks = Rc::new(RefCell::new(vec![
            SIG_ANAM,
            SIG_SNAM,
            SIG_SNAM,
            SIG_SNAM,
            SIG_SNAM,
            SIG_SNAM,
            SIG_SNAM,
            SIG_RDAT,
            SIG_WNAM,
            0x1234_5678,
            SIG_INAM,
            SIG_INAM,
            SIG_OBND,
            SIG_EDID,
        ]));
        let c = chunks.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(c.borrow_mut().pop().unwrap_or(0))
        });
        e.register(NEXT_CHUNK, |_, _| ret(1));
        e.register(CHUNK_SIZE, |_, _| ret(6));
        e.register(GET_CHUNK_DATA_SIZED, |e, a| {
            e.mem.set_cstr(a[1], b"Name");
            Ret::default()
        });
        // Read k gives 0x100 + k, except the second, which gives 0.
        let n = Rc::new(RefCell::new(0u32));
        let nn = n.clone();
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let mut k = nn.borrow_mut();
            *k += 1;
            let value = if *k == 2 { 0 } else { 0x100 + *k };
            e.mem.set_u32(a[1], value);
            Ret::default()
        });
        for addr in [
            LOAD_FORM,
            SET_FORM_FLAG_8,
            RDAT_SETTER,
            0x7000_0000,
            0x7000_0004,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        for addr in SOUND_SETTERS {
            e.register(addr, |_, _| Ret::default());
        }
        let mut slots = vec![0u32; 0x138 / 4];
        slots[0x134 / 4] = 0x7000_0000;
        slots[0xe0 / 4] = 0x7000_0004;
        e.put_vtable(0x0600_0000, &slots);
        let this: Ptr<BGSAcousticSpace> = e.new_object();
        e.mem.set_u32(this.addr(), 0x0600_0000);
        let file = 0x0500_0000u32;
        e.call_log = Some(vec![]);
        assert!(e.call(0x0058_9d30, &[this.addr(), file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOAD_FORM), vec![vec![this.addr(), file]]);
        assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![this.addr(), 0]]);
        assert_eq!(calls_to(&log, GET_CHUNK_DATA_SIZED)[0][2], 0x200);
        assert_eq!(calls_to(&log, 0x7000_0000).len(), 1);
        assert_eq!(calls_to(&log, 0x7000_0004), vec![vec![this.addr(), file]]);
        // INAM 0x101 then 0: the second clears the flag.
        assert!(!e.get(this, BGSAcousticSpace::bIsInterior));
        // WNAM is read straight into +0x50.
        let reads = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(reads[2], vec![file, this.addr() + 0x50]);
        assert_eq!(e.get(this, BGSAcousticSpace::iWallaPop), 0x103);
        assert_eq!(calls_to(&log, RDAT_SETTER), vec![vec![this.addr(), 0x104]]);
        // Six SNAM: setters 0..4, then the sixth goes to the first.
        for (i, addr) in SOUND_SETTERS.iter().enumerate() {
            let got = calls_to(&log, *addr);
            let mut expected = vec![vec![this.addr(), 0x105 + i as u32]];
            if i == 0 {
                expected.push(vec![this.addr(), 0x10a]);
            }
            assert_eq!(got, expected, "setter {i}");
        }
        assert_eq!(e.get(this, BGSAcousticSpace::eEnvType), 0x10b);
        // An INAM that is not zero sets the flag; a missing next chunk
        // stops the walk.
        chunks.borrow_mut().extend([SIG_OBND, SIG_INAM]);
        e.register(NEXT_CHUNK, |_, _| ret(0));
        assert!(e.call(0x0058_9d30, &[this.addr(), file]).bool());
        assert!(e.get(this, BGSAcousticSpace::bIsInterior));
        assert_eq!(chunks.borrow().len(), 1);
    }

    #[test]
    fn env_type_setter_and_interior_getter() {
        let mut e = Engine::new();
        let this: Ptr<BGSAcousticSpace> = e.new_object();
        e.call(0x0058_9f50, &args![this, 0xdead_beefu32]);
        assert_eq!(e.mem.u32(this.addr() + 0x4c), 0xdead_beef);
        assert!(!e.call(0x0058_a200, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x30, 1);
        assert!(e.call(0x0058_a200, &args![this]).bool());
    }
}
