//! `fallout shared/tesclass.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESClass` (Xbox PDB), 0x60 bytes on PC (`TESForm` is 0x10 smaller
    /// than in the PDB). Only the fields this unit reads.
    pub struct TESClass: 0x60 {
        /// `data.eTagSkills[0]` (Xbox PDB `CLASS_DATA`, 4 actor values).
        0x44 eTagSkills: u32,
        /// `data.cClassFlags` (Xbox PDB).
        0x54 cClassFlags: u8,
        /// `data.iServiceFlags` (Xbox PDB).
        0x58 iServiceFlags: i32,
        /// `data.cTrainingSkill` (Xbox PDB).
        0x5c cTrainingSkill: u8,
        /// `data.cTrainingLevel` (Xbox PDB).
        0x5d cTrainingLevel: u8,
    }
}

/// Whether an actor value index is a valid skill (`0047f060`, cdecl,
/// tesaiform.cpp).
const IS_VALID_SKILL: u32 = 0x0047_f060;
/// The form's virtual slot +0x48 (called with 2 when a field changes).
const SLOT_CHANGED: u32 = 0x48;

const GET_TES_FORM: u32 = 0x0047_2660;
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The current chunk's size (the word at `file + 0x25c`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// The byte at `file + 0x299` (non-zero: the data needs byte swapping).
const FILE_BYTE_299: u32 = 0x0040_1680;
const GET_CHUNK_DATA: u32 = 0x0047_2890;
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESForm::LoadData` (engine map): `__thiscall`, file, destination, size.
const TES_FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// The extra bytes the form's components add to a `DATA` chunk
/// (`00486620`, tesform.cpp).
const COMPONENT_DATA_SIZE: u32 = 0x0048_6620;
/// Byte swapping of the 0x1c-byte `CLASS_DATA` (`005f68d0`) and of the old
/// 0x18-byte `OLD_CLASS_DATA` (`005f6c90`), `__thiscall` on the data.
const SWAP_CLASS_DATA: u32 = 0x005f_68d0;
const SWAP_OLD_CLASS_DATA: u32 = 0x005f_6c90;
/// `TESDescription::LoadDescriptionData` (engine map), cdecl.
const LOAD_DESCRIPTION: u32 = 0x0048_2d50;
/// The full-name chunk reader (`00487050`), cdecl: name, file.
const LOAD_FULL_NAME: u32 = 0x0048_7050;
/// `TESTexture::LoadTextureChunk` (engine map), cdecl.
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// The attributes reader (`00480130`, tesattributes.cpp): `__thiscall` on
/// the attributes, the 7 bytes read.
const LOAD_ATTRIBUTES: u32 = 0x0048_0130;
/// The error printer (`005b5e40`, cdecl, format and arguments).
const PRINT_ERROR: u32 = 0x005b_5e40;
/// `"MASTERFILE: Class %s contains invalid data. Make sure all attributes
/// and skills are unique."`
const INVALID_CLASS_MESSAGE: u32 = 0x0104_8cf0;
/// The form type of a class record.
const FORM_TYPE_CLASS: u32 = 7;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_DESC: u32 = 0x4353_4544;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_ATTR: u32 = 0x5254_5441;

// Translated from 005f6e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESClass::SetTagSkill` (engine map): when `skill` is a valid skill
/// (`0047f060`) or -1, stores it as tag skill `index` (`this + 0x44 + 4 *
/// index`) and calls virtual slot +0x48 with 2. `__thiscall`, `RET 8`.
pub fn tes_class_set_tag_skill(e: &mut Engine, this: Ptr<TESClass>, index: i32, skill: i32) {
    if e.call(IS_VALID_SKILL, &args![skill]).bool() || skill == -1 {
        let at = this
            .addr()
            .wrapping_add((index as u32).wrapping_mul(4))
            .wrapping_add(0x44);
        e.mem.set_i32(at, skill);
        e.vcall(this.addr(), SLOT_CHANGED, &args![2u32]);
    }
}

// Translated from 005f6dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The training skill as an actor value index: `cTrainingSkill + 0x20`
/// (`ECX`, no stack words). The engine map gives it no name.
pub fn fn_005f6dd0(e: &mut Engine, this: Ptr<TESClass>) -> u32 {
    u32::from(e.get(this, TESClass::cTrainingSkill)) + 0x20
}

// Translated from 005f6e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the training skill (`fn_005f6dd0`) is a valid skill
/// (`0047f060`); when it is not, resets it with `005f6df0(0x20)`. `ECX`, no
/// stack words. The engine map gives it no name.
pub fn fn_005f6e20(e: &mut Engine, this: Ptr<TESClass>) -> bool {
    let skill = fn_005f6dd0(e, this);
    if e.call(IS_VALID_SKILL, &args![skill]).bool() {
        true
    } else {
        e.call(0x005f_6df0, &args![this, 0x20u32]);
        false
    }
}

// Translated from 005f6e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESClass::IsGuard` (engine map): bit 1 of `cClassFlags`. `ECX`, no
/// stack words.
pub fn tes_class_is_guard(e: &mut Engine, this: Ptr<TESClass>) -> bool {
    e.get(this, TESClass::cClassFlags) & 2 != 0
}

/// `this + offset`, or null when `this` is null (the component casts).
fn component(this: u32, offset: u32) -> u32 {
    if this == 0 {
        0
    } else {
        this + offset
    }
}

/// The `DATA` chunk: `CLASS_DATA` (0x1c bytes, plus the components' extra
/// bytes when the form reads them itself) or the old 0x18-byte layout.
fn load_class_data(e: &mut Engine, this: Ptr<TESClass>, file: Ptr) {
    let base = this.addr();
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    let extra = e.call(COMPONENT_DATA_SIZE, &args![this]).u32();
    if size == extra.wrapping_add(0x1c) {
        e.call(TES_FORM_LOAD_DATA, &args![this, file, base + 0x44, 0x1cu32]);
        if e.call(FILE_BYTE_299, &args![file]).bool() {
            e.call(SWAP_CLASS_DATA, &args![base + 0x44]);
        }
        return;
    }
    let old = e.call(CHUNK_SIZE, &args![file]).u32() == 0x18 || {
        let size = e.call(CHUNK_SIZE, &args![file]).u32();
        let extra = e.call(COMPONENT_DATA_SIZE, &args![this]).u32();
        size == extra.wrapping_add(0x18)
    };
    if !old {
        e.call(GET_CHUNK_DATA, &args![file, base + 0x44, 0x1cu32]);
        if e.call(FILE_BYTE_299, &args![file]).bool() {
            e.call(SWAP_CLASS_DATA, &args![base + 0x44]);
        }
        return;
    }
    e.with_stack(0x18, |e, data| {
        let size = e.call(CHUNK_SIZE, &args![file]).u32();
        let extra = e.call(COMPONENT_DATA_SIZE, &args![this]).u32();
        if size == extra.wrapping_add(0x18) {
            e.call(TES_FORM_LOAD_DATA, &args![this, file, data, 0x18u32]);
        } else {
            e.call(GET_CHUNK_DATA, &args![file, data, 0x18u32]);
        }
        if e.call(FILE_BYTE_299, &args![file]).bool() {
            e.call(SWAP_OLD_CLASS_DATA, &args![data]);
        }
        let d = data.addr();
        let flags = e.mem.u8(d + 0xc);
        e.set(this, TESClass::cClassFlags, flags);
        let services = e.mem.i32(d + 0x10);
        e.set(this, TESClass::iServiceFlags, services);
        let level = e.mem.u8(d + 0x15);
        e.set(this, TESClass::cTrainingLevel, level);
        let skill = e.mem.u8(d + 0x14);
        e.set(this, TESClass::cTrainingSkill, skill);
        for i in 0..3 {
            let tag = e.mem.u32(d + 4 * i);
            e.mem.set_u32(base + 0x44 + 4 * i, tag);
        }
    });
}

// Translated from 005f6930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f6930 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESClass::Load` (engine map): `__thiscall`, `RET 4`, `file`. Answers
/// false when the record's form type is not 7; otherwise reads the form
/// header (`TESForm::LoadForm`) and every chunk, and answers true:
/// - `OBND`: virtual slot 0xe0 with the file;
/// - `DATA`: a chunk of 0x1c bytes plus `00486620`'s extra bytes is read
///   by `TESForm::LoadData` into `CLASS_DATA` at +0x44; a chunk of 0x18
///   bytes (or 0x18 plus the extra bytes, through `LoadData`) is the old
///   layout: read into a stack copy and moved over (the first three tag
///   skills, flags, services, training skill and level); any other size is
///   read as 0x1c bytes into +0x44. The byte-swap functions run when the
///   file asks for it (`00401680`);
/// - `DESC`: `TESDescription::LoadDescriptionData` on +0x24;
/// - `EDID`: a stack buffer of the chunk's size (at most 0x200 read) for
///   virtual slot 0x134;
/// - `FULL`: `00487050` on +0x18; `ICON`: `TESTexture::LoadTextureChunk`
///   on +0x2c (each null when `this` is null);
/// - `ATTR`: 7 bytes read and given to `00480130` on +0x38.
///
/// After the record, a class whose training skill is invalid
/// (`fn_005f6e20`) prints the invalid-data message with its name (virtual
/// slot 0x130). The stack cookie check is not translated.
pub fn tes_class_load(e: &mut Engine, this: Ptr<TESClass>, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u32() != FORM_TYPE_CLASS {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_DATA => load_class_data(e, this, file),
            CHUNK_DESC => {
                e.call(LOAD_DESCRIPTION, &args![component(base, 0x24), file]);
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            CHUNK_FULL => {
                e.call(LOAD_FULL_NAME, &args![component(base, 0x18), file]);
            }
            CHUNK_ICON => {
                e.call(LOAD_TEXTURE_CHUNK, &args![component(base, 0x2c), file]);
            }
            CHUNK_ATTR => {
                e.with_stack(8, |e, attributes| {
                    e.call(GET_CHUNK_DATA, &args![file, attributes, 7u32]);
                    e.call(LOAD_ATTRIBUTES, &args![base + 0x38, attributes]);
                });
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    if !fn_005f6e20(e, this) {
        let name = e.vcall(base, 0x130, &args![]).u32();
        e.call(PRINT_ERROR, &args![INVALID_CLASS_MESSAGE, name]);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x005f_6e80,
            tes_class_set_tag_skill(Ptr<TESClass>, i32, i32)
        ),
        entry!(0x005f_6dd0, fn_005f6dd0(Ptr<TESClass>) -> u32),
        entry!(0x005f_6e20, fn_005f6e20(Ptr<TESClass>) -> bool),
        entry!(0x005f_6e60, tes_class_is_guard(Ptr<TESClass>) -> bool),
        entry!(0x005f_6930, tes_class_load(Ptr<TESClass>, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const VTABLE: u32 = 0x0200_0000;
    const CHANGED: u32 = 0x0300_0000;
    const BOUNDS: u32 = 0x0300_0010;
    const EDITOR_ID: u32 = 0x0300_0020;
    const NAME: u32 = 0x0300_0030;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn class(e: &mut Engine) -> Ptr<TESClass> {
        e.map(VTABLE, 0x200);
        e.mem.set_u32(VTABLE + 0x48, CHANGED);
        e.mem.set_u32(VTABLE + 0xe0, BOUNDS);
        e.mem.set_u32(VTABLE + 0x134, EDITOR_ID);
        e.mem.set_u32(VTABLE + 0x130, NAME);
        for callee in [CHANGED, BOUNDS, EDITOR_ID] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(NAME, |_, _| ret(0x0bad));
        let this = e.new_object::<TESClass>();
        e.mem.set_u32(this.addr(), VTABLE);
        this
    }

    #[test]
    fn tag_skills_take_valid_skills_and_minus_one() {
        let mut e = Engine::new();
        let this = class(&mut e);
        e.register(IS_VALID_SKILL, |_, a| ret(u32::from(a[0] == 0x25)));
        e.call_log = Some(vec![]);
        e.call(0x005f_6e80, &args![this, 2i32, 0x25i32]);
        assert_eq!(e.mem.u32(this.addr() + 0x4c), 0x25);
        e.call(0x005f_6e80, &args![this, 3i32, -1i32]);
        assert_eq!(e.mem.i32(this.addr() + 0x50), -1);
        e.call(0x005f_6e80, &args![this, 0i32, 0x26i32]);
        assert_eq!(e.mem.u32(this.addr() + 0x44), 0);
        assert_eq!(calls_to(&e, CHANGED), vec![vec![this.addr(), 2]; 2]);
        assert_eq!(
            calls_to(&e, IS_VALID_SKILL),
            vec![vec![0x25], vec![0xffff_ffff], vec![0x26]]
        );
    }

    #[test]
    fn training_skill_is_an_actor_value_and_is_checked() {
        let mut e = Engine::new();
        let this = class(&mut e);
        e.set(this, TESClass::cTrainingSkill, 0xe0);
        assert_eq!(e.call(0x005f_6dd0, &args![this]).u32(), 0x100);
        e.register(IS_VALID_SKILL, |_, a| ret(u32::from(a[0] == 0x100)));
        e.register(0x005f_6df0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_6e20, &args![this]).bool());
        assert!(calls_to(&e, 0x005f_6df0).is_empty());
        e.set(this, TESClass::cTrainingSkill, 1);
        assert!(!e.call(0x005f_6e20, &args![this]).bool());
        assert_eq!(calls_to(&e, 0x005f_6df0), vec![vec![this.addr(), 0x20]]);
    }

    #[test]
    fn guards_have_flag_bit_1() {
        let mut e = Engine::new();
        let this = class(&mut e);
        e.set(this, TESClass::cClassFlags, 0xfd);
        assert!(!e.call(0x005f_6e60, &args![this]).bool());
        e.set(this, TESClass::cClassFlags, 2);
        assert!(e.call(0x005f_6e60, &args![this]).bool());
    }

    type Chunks = Rc<RefCell<Vec<(u32, u32)>>>;

    /// The chunk reads fill the destination with 0x10, 0x11, ... (at most
    /// 0x1c bytes: the double reads no more than the data chunks hold).
    fn fill(e: &mut Engine, dest: u32, size: u32) -> Ret {
        for i in 0..size.min(0x1c) {
            e.mem.set_u8(dest + i, 0x10 + i as u8);
        }
        Ret::default()
    }

    /// A file double: the chunks (signature, size) in order; the swap byte
    /// answers `swap`; the components add `extra` bytes; the training skill
    /// check answers `valid`.
    fn load_setup(
        form_type: u32,
        chunks: &[(u32, u32)],
        swap: u32,
        extra: u32,
        valid: bool,
    ) -> (Engine, Ptr<TESClass>, Ptr) {
        let mut e = Engine::new();
        let this = class(&mut e);
        let file: Ptr = Ptr::new(e.mem.alloc(0x300));
        e.register_double(GET_TES_FORM, move |_, _| ret(form_type));
        let queue: Chunks = Rc::new(RefCell::new(chunks.to_vec()));
        let q1 = queue.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(q1.borrow().first().map_or(0, |c| c.0))
        });
        let q2 = queue.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            ret(q2.borrow().first().map_or(0, |c| c.1))
        });
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut q = queue.borrow_mut();
            q.remove(0);
            ret(u32::from(!q.is_empty()))
        });
        e.register_double(FILE_BYTE_299, move |_, _| ret(swap));
        e.register_double(COMPONENT_DATA_SIZE, move |_, _| ret(extra));
        e.register_double(IS_VALID_SKILL, move |_, _| ret(u32::from(valid)));
        // The chunk reads fill the destination with 0x10, 0x11, ...
        e.register(GET_CHUNK_DATA, |e, a| fill(e, a[1], a[2]));
        e.register(TES_FORM_LOAD_DATA, |e, a| fill(e, a[2], a[3]));
        for callee in [
            TES_FORM_LOAD_FORM,
            SWAP_CLASS_DATA,
            SWAP_OLD_CLASS_DATA,
            LOAD_DESCRIPTION,
            LOAD_FULL_NAME,
            LOAD_TEXTURE_CHUNK,
            LOAD_ATTRIBUTES,
            PRINT_ERROR,
            0x005f_6df0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, this, file)
    }

    #[test]
    fn another_form_type_answers_false() {
        let (mut e, this, file) = load_setup(8, &[(CHUNK_DATA, 0x1c)], 0, 0, true);
        assert!(!e.call(0x005f_6930, &args![this, file]).bool());
        assert!(calls_to(&e, TES_FORM_LOAD_FORM).is_empty());
    }

    #[test]
    fn components_go_to_their_readers() {
        let chunks = [
            (CHUNK_OBND, 0xc),
            (CHUNK_EDID, 0x20),
            (CHUNK_DESC, 4),
            (CHUNK_FULL, 4),
            (CHUNK_ICON, 4),
            (CHUNK_ATTR, 7),
            (0x5858_5858, 1),
        ];
        let (mut e, this, file) = load_setup(7, &chunks, 0, 0, true);
        assert!(e.call(0x005f_6930, &args![this, file]).bool());
        let t = this.addr();
        let f = file.addr();
        assert_eq!(calls_to(&e, TES_FORM_LOAD_FORM), vec![vec![t, f]]);
        assert_eq!(calls_to(&e, BOUNDS), vec![vec![t, f]]);
        let reads = calls_to(&e, GET_CHUNK_DATA);
        assert_eq!(reads.len(), 2);
        assert_eq!(reads[0][2], 0x200);
        assert_eq!(calls_to(&e, EDITOR_ID), vec![vec![t, reads[0][1]]]);
        assert_eq!(reads[1][2], 7);
        assert_eq!(
            calls_to(&e, LOAD_ATTRIBUTES),
            vec![vec![t + 0x38, reads[1][1]]]
        );
        assert_eq!(calls_to(&e, LOAD_DESCRIPTION), vec![vec![t + 0x24, f]]);
        assert_eq!(calls_to(&e, LOAD_FULL_NAME), vec![vec![t + 0x18, f]]);
        assert_eq!(calls_to(&e, LOAD_TEXTURE_CHUNK), vec![vec![t + 0x2c, f]]);
        assert!(calls_to(&e, PRINT_ERROR).is_empty());
    }

    #[test]
    fn a_null_class_reads_components_into_null() {
        let chunks = [(CHUNK_DESC, 4), (CHUNK_FULL, 4), (CHUNK_ICON, 4)];
        let (mut e, _, file) = load_setup(7, &chunks, 0, 0, true);
        // `this` null: the training-skill read at the end needs memory at
        // 0x5c, so map the first page.
        e.map(0, 0x100);
        e.mem.set_u32(0, VTABLE);
        assert!(e.call(0x005f_6930, &args![Ptr::<()>::NULL, file]).bool());
        let f = file.addr();
        assert_eq!(calls_to(&e, LOAD_DESCRIPTION), vec![vec![0, f]]);
        assert_eq!(calls_to(&e, LOAD_FULL_NAME), vec![vec![0, f]]);
        assert_eq!(calls_to(&e, LOAD_TEXTURE_CHUNK), vec![vec![0, f]]);
    }

    #[test]
    fn current_data_with_component_bytes_goes_through_load_data() {
        let (mut e, this, file) = load_setup(7, &[(CHUNK_DATA, 0x1c + 6)], 1, 6, true);
        assert!(e.call(0x005f_6930, &args![this, file]).bool());
        let t = this.addr();
        assert_eq!(
            calls_to(&e, TES_FORM_LOAD_DATA),
            vec![vec![t, file.addr(), t + 0x44, 0x1c]]
        );
        assert_eq!(calls_to(&e, SWAP_CLASS_DATA), vec![vec![t + 0x44]]);
        assert_eq!(e.mem.u8(t + 0x5d), 0x10 + 0x19);
    }

    #[test]
    fn data_of_another_size_is_read_as_current_data() {
        let (mut e, this, file) = load_setup(7, &[(CHUNK_DATA, 0x1c)], 0, 8, true);
        assert!(e.call(0x005f_6930, &args![this, file]).bool());
        let t = this.addr();
        assert_eq!(
            calls_to(&e, GET_CHUNK_DATA),
            vec![vec![file.addr(), t + 0x44, 0x1c]]
        );
        assert!(calls_to(&e, TES_FORM_LOAD_DATA).is_empty());
        assert!(calls_to(&e, SWAP_CLASS_DATA).is_empty());
    }

    #[test]
    fn old_data_is_moved_into_the_current_layout() {
        let (mut e, this, file) = load_setup(7, &[(CHUNK_DATA, 0x18)], 1, 4, true);
        assert!(e.call(0x005f_6930, &args![this, file]).bool());
        let t = this.addr();
        let reads = calls_to(&e, GET_CHUNK_DATA);
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0][2], 0x18);
        assert_eq!(calls_to(&e, SWAP_OLD_CLASS_DATA), vec![vec![reads[0][1]]]);
        assert_eq!(e.mem.u32(t + 0x44), 0x1312_1110);
        assert_eq!(e.mem.u32(t + 0x4c), 0x1b1a_1918);
        assert_eq!(e.mem.u32(t + 0x50), 0);
        assert_eq!(e.mem.u8(t + 0x54), 0x1c);
        assert_eq!(e.mem.u32(t + 0x58), 0x2322_2120);
        assert_eq!(e.mem.u8(t + 0x5c), 0x24);
        assert_eq!(e.mem.u8(t + 0x5d), 0x25);
        // Old data with the component bytes goes through `LoadData`.
        let (mut e, this, file) = load_setup(7, &[(CHUNK_DATA, 0x18 + 4)], 0, 4, true);
        assert!(e.call(0x005f_6930, &args![this, file]).bool());
        let loads = calls_to(&e, TES_FORM_LOAD_DATA);
        assert_eq!(loads.len(), 1);
        assert_eq!(loads[0][0], this.addr());
        assert_eq!(loads[0][3], 0x18);
        assert!(calls_to(&e, SWAP_OLD_CLASS_DATA).is_empty());
    }

    #[test]
    fn an_invalid_training_skill_prints_the_message() {
        let (mut e, this, file) = load_setup(7, &[], 0, 0, false);
        assert!(e.call(0x005f_6930, &args![this, file]).bool());
        assert_eq!(
            calls_to(&e, PRINT_ERROR),
            vec![vec![INVALID_CLASS_MESSAGE, 0x0bad]]
        );
        assert_eq!(calls_to(&e, 0x005f_6df0), vec![vec![this.addr(), 0x20]]);
    }
}
