//! `fallout shared/tesobjectbook.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESObjectBOOK` (vtable `0102a9c4`, RTTI `.?AVTESObjectBOOK@@`, form type
//! 0x19). Its components sit at the offsets the constructor `00514500`
//! gives them; the ones used here: `+0x30` (full name), `+0x3C` (model with
//! texture swaps), `+0x5C` (icon), `+0x68` (script), `+0x74`
//! (enchantment), `+0x94` (description), `+0x9C` (destructible object),
//! `+0xA4` (message icon), `+0xB4` (sounds); the book's data is 2 bytes at
//! `+0xC0` (flags, then the skill byte at `+0xC1`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESFile::GetTESForm` (Xbox PDB, `00472660`): the record's form type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
/// Sets (`true`) or clears form flag 0x8 (`00484ab0`, `RET 4`).
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// Tests form flag 0x8 (`004013e0`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The size of the current chunk (`00401660`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`): buffer and size.
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`): one word.
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov2` (Xbox PDB, `00472840`): one 16-bit word.
const GET_CHUNK_DATA_U16: u32 = 0x0047_2840;
/// Moves to the next chunk (`004726f0`, `bool`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESForm::LoadData` (`004861f0`, `RET 0xC`): file, buffer, size.
const LOAD_DATA: u32 = 0x0048_61f0;
/// The setter the form-id chunks go to (`006ecd40`, `RET 4`; the engine map
/// names the folded body `CombatProcedureAttackMelee::Initialize`).
const SET_FORM: u32 = 0x006e_cd40;
/// `ActorValue::ToActorValue` (Xbox PDB, `0066ec10`, cdecl, two words).
const TO_ACTOR_VALUE: u32 = 0x0066_ec10;

/// Chunk signatures, as the words the exe compares.
const SIG_ANAM: u32 = 0x4d41_4e41;
const SIG_DATA: u32 = 0x4154_4144;
const SIG_DESC: u32 = 0x4353_4544;
const SIG_DEST: u32 = 0x5453_4544;
const SIG_DSTD: u32 = 0x4454_5344;
const SIG_EAMT: u32 = 0x544d_4145;
const SIG_EDID: u32 = 0x4449_4445;
const SIG_EITM: u32 = 0x4d54_4945;
const SIG_ENAM: u32 = 0x4d41_4e45;
const SIG_FULL: u32 = 0x4c4c_5546;
const SIG_ICON: u32 = 0x4e4f_4349;
const SIG_MICO: u32 = 0x4f43_494d;
const SIG_MODD: u32 = 0x4444_4f4d;
const SIG_MODL: u32 = 0x4c44_4f4d;
const SIG_MODS: u32 = 0x5344_4f4d;
const SIG_MODT: u32 = 0x5444_4f4d;
const SIG_MOSD: u32 = 0x4453_4f4d;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_SCRI: u32 = 0x4952_4353;
const SIG_YNAM: u32 = 0x4d41_4e59;
const SIG_ZNAM: u32 = 0x4d41_4e5a;

/// A word read from the chunk into a cleared local.
fn read_word(e: &mut Engine, file: Ptr, local: Ptr) -> u32 {
    e.mem.set_u32(local.addr(), 0);
    e.call(GET_CHUNK_DATA, &args![file, local]);
    e.mem.u32(local.addr())
}

// Translated from 00514870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectBOOK::InitItem` (Xbox PDB): unless form flag 0x8 is set
/// (`004013e0`), the components' `InitItem` with the book (`0048cd90` on
/// `+0x68`, `00483210` on `+0x74`, `00478be0` on `+0x9C`,
/// `TESModelTextureSwap::InitItem` `0048aa80` on `+0x3C`, `0047b6a0` on
/// `+0xB4`); a skill byte (`+0xC1`, signed) other than -1 whose actor value
/// (`ActorValue::ToActorValue`, `0066ec10`, with 2 and the byte) is below
/// 0x20 or above 0x2E becomes 0xFF. Finally form flag 0x8 is set.
pub fn tes_object_book_init_item(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    for (callee, offset) in [
        (0x0048_cd90, 0x68),
        (0x0048_3210, 0x74),
        (0x0047_8be0, 0x9c),
        (0x0048_aa80, 0x3c),
        (0x0047_b6a0, 0xb4),
    ] {
        e.call(callee, &args![base + offset, this]);
    }
    if e.mem.i8(base + 0xc1) != -1 {
        let skill = e.mem.u8(base + 0xc1);
        let value = e.call(TO_ACTOR_VALUE, &args![2u32, u32::from(skill)]).i32();
        if !(0x20..=0x2e).contains(&value) {
            e.mem.set_u8(base + 0xc1, 0xff);
        }
    }
    e.call(SET_FORM_FLAG_8, &args![this, true]);
}

// Translated from 00514930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00514930 - the /GS cookie check (00ec408c) is not translated; the __alloca_probe_16 (00ec5ec0) buffers are with_stack blocks of the chunk size (their 16-byte alignment padding, and the stack growing with each such chunk, are not modelled)
/// `TESObjectBOOK`'s `Load` (`RET 4`): false unless the record is of form
/// type 0x19. Then `TESForm::LoadForm` (Xbox PDB, `00485110`), form flag
/// 0x8 cleared, and every chunk of `file` (a component address is null
/// when `this` is, as in the exe):
/// - `EDID` (vtable slot 0x134, at most 0x200 bytes), `OBND` (slot 0xE0);
/// - `FULL` (`00487050` on `+0x30`), `MODL`/`MODD`/`MOSD`/`MODT`
///   (`TESModel::LoadModelChunk`, `004892d0`, on `+0x3C`), `MODS` (the
///   chunk read whole into a stack buffer and unpacked by `0048a7b0` on
///   `+0x3C` with its size and the form version `00403570`), `ICON`
///   (`0048e3d0` on `+0x5C`), `MICO` (`0047a290` on `+0xA4`), `DESC`
///   (`00482d50` on `+0x94`), `DSTD`/`DEST` (`004781e0` on `+0x9C`);
/// - form ids (each read into a cleared local): `SCRI` (`006ecd40` and
///   `0048cd90` with the book on `+0x68`), `ENAM`/`EITM` (`006ecd40` on
///   `+0x74`), `YNAM` (`006ecd40` on `+0xB4`), `ZNAM` (`00403550` on
///   `+0xB4`); `ANAM`/`EAMT` are 16-bit words (`00472840`) given to
///   `00483170` on `+0x74`;
/// - `DATA`: 2 bytes into `+0xC0` (`TESForm::LoadData`, `004861f0`).
///
/// Returns true.
pub fn fn_00514930(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    let component = |offset: u32| if base != 0 { base + offset } else { 0 };
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x19 {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, false]);
    e.with_stack(4, |e, word_at| loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            SIG_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            SIG_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            SIG_FULL => {
                e.call(0x0048_7050, &args![component(0x30), file]);
            }
            SIG_MODL | SIG_MODD | SIG_MOSD | SIG_MODT => {
                e.call(0x0048_92d0, &args![component(0x3c), file]);
            }
            SIG_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, size]);
                    let version = e.call(0x0040_3570, &args![file]).u16();
                    e.call(
                        0x0048_a7b0,
                        &args![base + 0x3c, buffer, size, u32::from(version)],
                    );
                });
            }
            SIG_ICON => {
                e.call(0x0048_e3d0, &args![component(0x5c), file]);
            }
            SIG_MICO => {
                e.call(0x0047_a290, &args![component(0xa4), file]);
            }
            SIG_SCRI => {
                let script = read_word(e, file, word_at);
                e.call(SET_FORM, &args![base + 0x68, script]);
                e.call(0x0048_cd90, &args![base + 0x68, this]);
            }
            SIG_ENAM | SIG_EITM => {
                let enchantment = read_word(e, file, word_at);
                e.call(SET_FORM, &args![base + 0x74, enchantment]);
            }
            SIG_ANAM | SIG_EAMT => {
                e.mem.set_u16(word_at.addr(), 0);
                e.call(GET_CHUNK_DATA_U16, &args![file, word_at]);
                let amount = e.mem.u16(word_at.addr());
                e.call(0x0048_3170, &args![base + 0x74, u32::from(amount)]);
            }
            SIG_DESC => {
                e.call(0x0048_2d50, &args![component(0x94), file]);
            }
            SIG_DSTD | SIG_DEST => {
                e.call(0x0047_81e0, &args![component(0x9c), file]);
            }
            SIG_YNAM => {
                let sound = read_word(e, file, word_at);
                e.call(SET_FORM, &args![base + 0xb4, sound]);
            }
            SIG_ZNAM => {
                let sound = read_word(e, file, word_at);
                e.call(0x0040_3550, &args![base + 0xb4, sound]);
            }
            SIG_DATA => {
                e.call(LOAD_DATA, &args![this, file, base + 0xc0, 2u32]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    });
    true
}

// Translated from 00514f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the book's data (`this + 0xC0`).
pub fn fn_00514f30(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0xc0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_4870, tes_object_book_init_item(Ptr)),
        entry!(0x0051_4930, fn_00514930(Ptr, Ptr) -> bool),
        entry!(0x0051_4f30, fn_00514f30(Ptr) -> u32),
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

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    #[test]
    fn data_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0051_4f30, &[0x1000]).u32(), 0x10c0);
        assert_eq!(e.call(0x0051_4f30, &[0xffff_ff80]).u32(), 0x40);
    }

    #[test]
    fn init_item_checks_the_skill() {
        let mut e = Engine::new();
        e.register(FORM_FLAG_8_SET, |_, _| ret(1));
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        e.call(0x0051_4870, &[this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);

        e.register(FORM_FLAG_8_SET, |_, _| ret(0));
        let inits = [
            0x0048_cd90,
            0x0048_3210,
            0x0047_8be0,
            0x0048_aa80,
            0x0047_b6a0,
        ];
        quiet(&mut e, &inits);
        quiet(&mut e, &[SET_FORM_FLAG_8]);
        // The actor value is the byte itself.
        e.register(TO_ACTOR_VALUE, |_, a| {
            assert_eq!(a[0], 2);
            ret(a[1])
        });
        for (skill, kept) in [
            (0x20u8, true),
            (0x2e, true),
            (0x1f, false),
            (0x2f, false),
            (0xff, true),
        ] {
            e.mem.set_u8(this + 0xc1, skill);
            e.call_log = Some(vec![]);
            e.call(0x0051_4870, &[this]);
            let log = e.call_log.take().unwrap();
            let expected = if kept { skill } else { 0xff };
            assert_eq!(e.mem.u8(this + 0xc1), expected, "{skill:#x}");
            let order: Vec<u32> = log
                .iter()
                .filter(|(a, _)| inits.contains(a))
                .map(|(_, w)| w[0] - this)
                .collect();
            assert_eq!(order, vec![0x68, 0x74, 0x9c, 0x3c, 0xb4]);
            assert_eq!(
                calls_to(&log, TO_ACTOR_VALUE).len(),
                usize::from(skill != 0xff)
            );
            assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![this, 1]]);
        }
    }

    #[test]
    fn load_reads_every_chunk_kind() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        let file = 0x0500_0000u32;
        e.register(GET_TES_FORM, |_, _| ret(0x18));
        assert!(!e.call(0x0051_4930, &[this, file]).bool());

        e.register(GET_TES_FORM, |_, _| ret(0x19));
        let mut chunks = vec![
            SIG_EDID, SIG_OBND, SIG_FULL, SIG_MODL, SIG_MODD, SIG_MOSD, SIG_MODT, SIG_MODS,
            SIG_ICON, SIG_MICO, SIG_SCRI, SIG_ENAM, SIG_EITM, SIG_ANAM, SIG_EAMT, SIG_DESC,
            SIG_DSTD, SIG_DEST, SIG_YNAM, SIG_ZNAM, SIG_DATA, 0x1234,
        ];
        chunks.reverse();
        let chunks = Rc::new(RefCell::new(chunks));
        let c = chunks.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(c.borrow_mut().pop().unwrap_or(0))
        });
        let c = chunks.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            ret(u32::from(!c.borrow().is_empty()))
        });
        e.register(CHUNK_SIZE, |_, _| ret(8));
        let n = Rc::new(RefCell::new(0x500u32));
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            let mut k = n.borrow_mut();
            *k += 1;
            e.mem.set_u32(a[1], *k);
            Ret::default()
        });
        e.register(GET_CHUNK_DATA_U16, |e, a| {
            assert_eq!(e.mem.u16(a[1]), 0);
            e.mem.set_u16(a[1], 0x66);
            Ret::default()
        });
        e.register(0x0040_3570, |_, _| ret(0xffff_000f));
        quiet(
            &mut e,
            &[
                LOAD_FORM,
                SET_FORM_FLAG_8,
                GET_CHUNK_DATA_SIZED,
                0x7000_0000,
                0x7000_0004,
                0x0048_7050,
                0x0048_92d0,
                0x0048_a7b0,
                0x0048_e3d0,
                0x0047_a290,
                SET_FORM,
                0x0048_cd90,
                0x0048_3170,
                0x0048_2d50,
                0x0047_81e0,
                0x0040_3550,
                LOAD_DATA,
            ],
        );
        let mut slots = vec![0u32; 0x138 / 4];
        slots[0x134 / 4] = 0x7000_0000;
        slots[0xe0 / 4] = 0x7000_0004;
        e.put_vtable(0x0600_0000, &slots);
        e.mem.set_u32(this, 0x0600_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_4930, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOAD_FORM), vec![vec![this, file]]);
        assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![this, 0]]);
        let sized = calls_to(&log, GET_CHUNK_DATA_SIZED);
        assert_eq!((sized[0][2], sized[1][2]), (0x200, 8));
        assert_eq!(calls_to(&log, 0x7000_0000).len(), 1);
        assert_eq!(calls_to(&log, 0x7000_0004), vec![vec![this, file]]);
        assert_eq!(calls_to(&log, 0x0048_7050), vec![vec![this + 0x30, file]]);
        assert_eq!(
            calls_to(&log, 0x0048_92d0),
            vec![vec![this + 0x3c, file]; 4]
        );
        assert_eq!(
            calls_to(&log, 0x0048_a7b0),
            vec![vec![this + 0x3c, sized[1][1], 8, 0xf]]
        );
        assert_eq!(calls_to(&log, 0x0048_e3d0), vec![vec![this + 0x5c, file]]);
        assert_eq!(calls_to(&log, 0x0047_a290), vec![vec![this + 0xa4, file]]);
        assert_eq!(
            calls_to(&log, SET_FORM),
            vec![
                vec![this + 0x68, 0x501],
                vec![this + 0x74, 0x502],
                vec![this + 0x74, 0x503],
                vec![this + 0xb4, 0x504],
            ]
        );
        assert_eq!(calls_to(&log, 0x0048_cd90), vec![vec![this + 0x68, this]]);
        assert_eq!(
            calls_to(&log, 0x0048_3170),
            vec![vec![this + 0x74, 0x66]; 2]
        );
        assert_eq!(calls_to(&log, 0x0048_2d50), vec![vec![this + 0x94, file]]);
        assert_eq!(
            calls_to(&log, 0x0047_81e0),
            vec![vec![this + 0x9c, file]; 2]
        );
        assert_eq!(calls_to(&log, 0x0040_3550), vec![vec![this + 0xb4, 0x505]]);
        assert_eq!(
            calls_to(&log, LOAD_DATA),
            vec![vec![this, file, this + 0xc0, 2]]
        );
    }
}
