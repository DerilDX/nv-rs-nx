//! `fallout shared/teseffectshader.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00505c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00505c40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4f (0x170 bytes); `ECX` is the object.
/// The engine map names it `TESEffectShader::TESEffectShader` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0102_685c`) is `TESEffectShader`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00505c40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_685c);
    e.call(0x0050_64d0, &args![base + 0x18]);
    e.call(0x0048_e270, &args![base + 0x14c]);
    e.call(0x0048_e270, &args![base + 0x158]);
    e.call(0x0048_e270, &args![base + 0x164]);
    e.call(0x004f_15a0, &args![this, 0x4fu32]);
    e.call(0x0050_5db0, &args![this]);
    this
}

/// `006815c0` (`ECX`, answers `this`).
const IDENTITY_006815C0: u32 = 0x0068_15c0;
/// `00500940` (`ECX`): answers `this + 0x18` (here the shader data).
const DATA_00500940: u32 = 0x0050_0940;
/// The `NiPoint3` constructor (`00416870`, `ECX`, three float words), answers `this`.
const POINT3_CTOR: u32 = 0x0041_6870;

/// One store of `00506510`, in the exe's order.
enum Store {
    /// A byte constant.
    Byte(u32, u8),
    /// A word constant.
    Word(u32, u32),
    /// `FLD1`/`FSTP float`.
    One(u32),
    /// `FLDZ`/`FSTP float`.
    Zero(u32),
    /// `FLD float [global]`/`FSTP float` (a signalling NaN comes out quiet).
    Global(u32, u32),
    /// A stack `NiPoint3` made by `00416870` from three zeros, its three words copied.
    Point(u32),
}

/// The stores of `00506510`.
const EFFECT_SHADER_DEFAULTS: [Store; 72] = [
    Store::Byte(0x0, 0),
    Store::Word(0x4, 5),
    Store::Word(0x8, 1),
    Store::Word(0xc, 3),
    Store::Word(0x10, 0),
    Store::Zero(0x14),
    Store::Zero(0x18),
    Store::Zero(0x1c),
    Store::Zero(0x20),
    Store::Zero(0x24),
    Store::One(0x28),
    Store::Zero(0x2c),
    Store::Zero(0x30),
    Store::One(0x34),
    Store::Word(0x38, 0),
    Store::Zero(0x3c),
    Store::Zero(0x40),
    Store::Zero(0x44),
    Store::Zero(0x48),
    Store::Zero(0x4c),
    Store::One(0x50),
    Store::Word(0x5c, 6),
    Store::One(0x54),
    Store::One(0x58),
    Store::Word(0x60, 5),
    Store::Word(0x6c, 6),
    Store::Word(0x64, 1),
    Store::Word(0x68, 4),
    Store::Zero(0x70),
    Store::Zero(0x74),
    Store::Zero(0x78),
    Store::One(0x7c),
    Store::One(0x80),
    Store::One(0x84),
    Store::Zero(0x88),
    Store::Zero(0x8c),
    Store::Zero(0x90),
    Store::Point(0x94),
    Store::Point(0xa0),
    Store::One(0xac),
    Store::One(0xb0),
    Store::Zero(0xb4),
    Store::One(0xb8),
    Store::Word(0xbc, 0x00ff_ffff),
    Store::Word(0xc0, 0x00ff_ffff),
    Store::Word(0xc4, 0x00ff_ffff),
    Store::One(0xc8),
    Store::One(0xcc),
    Store::One(0xd0),
    Store::Zero(0xd4),
    Store::Global(0xd8, 0x0101_6248),
    Store::One(0xdc),
    Store::Zero(0xe0),
    Store::Zero(0xe4),
    Store::Zero(0xe8),
    Store::Zero(0xec),
    Store::Zero(0xf0),
    Store::Zero(0xf8),
    Store::Global(0xfc, 0x0101_7b78),
    Store::Global(0x100, 0x0102_3cd8),
    Store::Zero(0x104),
    Store::Zero(0x108),
    Store::Word(0x10c, 0x00ff_ffff),
    Store::Zero(0x110),
    Store::Word(0x114, 1),
    Store::Word(0x118, 1),
    Store::One(0x11c),
    Store::One(0x120),
    Store::One(0x124),
    Store::One(0x128),
    Store::One(0x12c),
    Store::One(0x130),
];

// Translated from 00506510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range; `ECX` = the effect shader data, the
/// `TESEffectShader`'s +0x18): its default values, written in the exe's order: the flag byte
/// cleared, the blend and test modes (5, 1, 3, ...), the fill, edge and particle floats
/// (0.0, 1.0, or a copy of a float global), the two `NiPoint3`s at +0x94 and +0xa0 (zeros,
/// through a stack `NiPoint3` made by `00416870`), the white colour keys and the counts.
pub fn fn_00506510(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for store in EFFECT_SHADER_DEFAULTS.iter() {
        match store {
            Store::Byte(offset, value) => e.mem.set_u8(base + offset, *value),
            Store::Word(offset, value) => e.mem.set_u32(base + offset, *value),
            Store::One(offset) => e.mem.set_u32(base + offset, x87_store32(1.0)),
            Store::Zero(offset) => e.mem.set_u32(base + offset, x87_store32(0.0)),
            Store::Global(offset, global) => {
                let bits = x87_quiet_bits(e.global::<u32>(*global));
                e.mem.set_u32(base + offset, bits);
            }
            Store::Point(offset) => {
                let zero = x87_store32(0.0);
                e.with_stack(0xc, |e, point| {
                    let made = e.call(POINT3_CTOR, &args![point, zero, zero, zero]).u32();
                    for word in 0..3u32 {
                        let value = e.mem.u32(made + word * 4);
                        e.mem.set_u32(base + offset + word * 4, value);
                    }
                });
            }
        }
    }
}

// Translated from 005064d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: the effect shader data constructor (`ECX`, no stack words; the
/// `TESEffectShader` constructor runs it on its +0x18): `006815c0` on the two `NiPoint3`s
/// (+0x94, +0xa0), then the defaults ([`fn_00506510`]); answers `this`.
pub fn fn_005064d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(IDENTITY_006815C0, &args![this.addr() + 0x94]);
    e.call(IDENTITY_006815C0, &args![this.addr() + 0xa0]);
    fn_00506510(e, this);
    this
}

// Translated from 00505db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEffectShader`'s vtable (`0102685c`) slot +0x14, the `InitializeData` slot of the Xbox
/// PDB's `TESForm` (`ECX`): the defaults ([`fn_00506510`]) on the shader data `00500940`
/// answers.
pub fn fn_00505db0(e: &mut Engine, this: Ptr) {
    let data = e.call(DATA_00500940, &args![this]).u32();
    fn_00506510(e, Ptr::new(data));
}

// Translated from 00506390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX`): answers `this + 0x158` (the `TESEffectShader`'s second texture,
/// the one `Load` fills from `ICO2`).
pub fn fn_00506390(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x158)
}

// Translated from 00507b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, called from many classes (a folded getter): the `float` at +0x4c of
/// `this` (`ECX`), returned in `ST0` (`FLD dword`, so a signalling NaN comes out quiet).
pub fn fn_00507b00(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x4c)))
}

/// `TESFile::GetTESForm` (Xbox PDB, `00472660`, `ECX` = file): the record's form type (`AL`).
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `ECX` = form, one stack word: the file).
const LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`ECX`, one stack word): sets or clears form flag 8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`, `ECX` = file): the chunk's code, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, `ECX` = file, two stack words:
/// destination, size).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESTexture::LoadTextureChunk` (Xbox PDB, `0048e3d0`, `cdecl`: texture, file).
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// `00401680` (`ECX` = file): whether the file's data needs its byte order swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1680;
/// `00506850` (`ECX` = the shader data): swaps its byte order.
const SWAP_DATA_00506850: u32 = 0x0050_6850;
/// `TESForm::LoadData` (Xbox PDB, `004861f0`, `ECX` = form, three stack words: file,
/// destination, start).
const LOAD_DATA: u32 = 0x0048_61f0;
/// `004726f0` (`ECX` = file): steps to the next chunk; false at the record's end (`AL`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00ba52d0` (no words): a count kept at `012003ec` (0x1a, or 0x4e below a setting of 2).
const COUNT_00BA52D0: u32 = 0x00ba_52d0;
/// The doubles 1.0 (`01012070`) and 0.0 (`01012060`) the two scales are compared with.
const DOUBLE_ONE: u32 = 0x0101_2070;
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// Chunk codes `TESEffectShader::Load` reads (the four characters, first byte lowest).
const CHUNK_EDID: i32 = 0x4449_4445;
const CHUNK_ICO2: i32 = 0x324f_4349;
const CHUNK_NAM7: i32 = 0x374d_414e;
const CHUNK_DATA: i32 = 0x4154_4144;
const CHUNK_OBND: i32 = 0x444e_424f;
const CHUNK_ICON: i32 = 0x4e4f_4349;
/// The effect shader record's form type.
const FORM_TYPE_EFFECT_SHADER: u8 = 0x4f;

/// A scale float of the data at `at`: when it is in (0, 1] (`FCOMP` against the doubles,
/// NaN failing both tests) it becomes `00ba52d0`'s count (`FILD qword` of the zero-extended
/// word) times itself, stored as a float.
fn scale_by_count(e: &mut Engine, at: u32) {
    let value = x87_m32(e.mem.u32(at));
    if !(value <= e.global::<f64>(DOUBLE_ONE)) {
        return;
    }
    let value = x87_m32(e.mem.u32(at));
    if !(value > e.global::<f64>(DOUBLE_ZERO)) {
        return;
    }
    let count = e.call(COUNT_00BA52D0, &args![]).u32();
    let product = x87_mul(f64::from(count), x87_m32(e.mem.u32(at)));
    e.mem.set_u32(at, x87_store32(product));
}

// Translated from 00505e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00505e70 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// `TESEffectShader::Load` (Xbox PDB), `ECX` and one stack word, the file (`RET 4`). A
/// record that is not an effect shader (`TESFile::GetTESForm` not 0x4f) answers false.
/// Otherwise vtable slot +0x18, `TESForm::LoadForm`, form flag 8 cleared (`00484ab0`), then
/// each chunk (codes compared signed) until `GetTESChunk` answers 0 or `004726f0` false:
/// `EDID` (a stack buffer of the chunk's size, read with at most 0x200 bytes, given to vtable
/// slot +0x134), `OBND` (vtable slot +0xe0 with the file), `ICON`, `ICO2`, `NAM7` (the
/// textures at +0x14c, +0x158, +0x164), `DATA` (the shader data at +0x18 through
/// `TESForm::LoadData` with 0x134 bytes, byte-swapped by `00506850` before and after when the
/// file needs it). Then the two floats at +0x94 and +0x98 are scaled by `00ba52d0`'s count
/// when they are in (0, 1], and true.
pub fn tes_effect_shader_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let form_type = e.call(GET_TES_FORM, &args![file]).u32() as u8;
    if form_type != FORM_TYPE_EFFECT_SHADER {
        return false;
    }
    let base = this.addr();
    e.vcall(base, 0x18, &args![]);
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        let code = chunk as i32;
        if code > CHUNK_EDID {
            if code == CHUNK_OBND {
                e.vcall(base, 0xe0, &args![file]);
            } else if code == CHUNK_ICON {
                e.call(LOAD_TEXTURE_CHUNK, &args![base + 0x14c, file]);
            }
        } else if code == CHUNK_EDID {
            let size = e.call(CHUNK_SIZE, &args![file]).u32();
            e.with_stack(size, |e, name| {
                e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
                e.vcall(base, 0x134, &args![name]);
            });
        } else if code == CHUNK_ICO2 {
            e.call(LOAD_TEXTURE_CHUNK, &args![base + 0x158, file]);
        } else if code == CHUNK_NAM7 {
            e.call(LOAD_TEXTURE_CHUNK, &args![base + 0x164, file]);
        } else if code == CHUNK_DATA {
            if e.call(NEEDS_SWAP, &args![file]).bool() {
                e.call(SWAP_DATA_00506850, &args![base + 0x18]);
            }
            e.call(LOAD_DATA, &args![this, file, base + 0x18, 0x134u32]);
            if e.call(NEEDS_SWAP, &args![file]).bool() {
                e.call(SWAP_DATA_00506850, &args![base + 0x18]);
            }
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    scale_by_count(e, base + 0x94);
    scale_by_count(e, base + 0x98);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_5c40, fn_00505c40(Ptr) -> Ptr),
        entry!(0x0050_6510, fn_00506510(Ptr)),
        entry!(0x0050_64d0, fn_005064d0(Ptr) -> Ptr),
        entry!(0x0050_5db0, fn_00505db0(Ptr)),
        entry!(0x0050_6390, fn_00506390(Ptr) -> u32),
        entry!(0x0050_7b00, fn_00507b00(Ptr) -> f32),
        entry!(0x0050_5e70, tes_effect_shader_load(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00505c40_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_5c40,
            size: 0x170,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0050_64d0, &[A::T(0x18)]),
                (0x0048_e270, &[A::T(0x14c)]),
                (0x0048_e270, &[A::T(0x158)]),
                (0x0048_e270, &[A::T(0x164)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4f)]),
                (0x0050_5db0, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_685c)],
        });
    }
}

#[cfg(test)]
mod shader_data_tests {
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

    fn defaults_rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.map(0x0102_3000, 0x1000);
        e.mem.set_u32(0x0101_6248, 0x3f00_0000);
        e.mem.set_u32(0x0101_7b78, 0x4120_0000);
        // A signalling NaN comes out quiet.
        e.mem.set_u32(0x0102_3cd8, 0x7f80_0001);
        // The point constructor stores its three floats.
        e.register(POINT3_CTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            a[0].into_ret()
        });
        e.register(IDENTITY_006815C0, |_, a| a[0].into_ret());
        let data = e.mem.alloc(0x140);
        e.mem.write(data, &[0xaa; 0x140]);
        e.call_log = Some(vec![]);
        (e, data)
    }

    fn check_defaults(e: &Engine, data: u32) {
        let word = |offset: u32| e.mem.u32(data + offset);
        assert_eq!(e.mem.u8(data), 0);
        assert_eq!(e.mem.bytes(data + 1, 3), vec![0xaa; 3]);
        assert_eq!([word(4), word(8), word(0xc), word(0x10)], [5, 1, 3, 0]);
        assert_eq!(word(0x28), 0x3f80_0000);
        assert_eq!(word(0x50), 0x3f80_0000);
        assert_eq!(
            [word(0x5c), word(0x60), word(0x64), word(0x68), word(0x6c)],
            [6, 5, 1, 4, 6]
        );
        assert_eq!(e.mem.bytes(data + 0x88, 0x24), vec![0; 0x24]);
        assert_eq!(word(0xbc), 0x00ff_ffff);
        assert_eq!(word(0xd8), 0x3f00_0000);
        assert_eq!(word(0xf4), 0xaaaa_aaaa);
        assert_eq!(word(0xfc), 0x4120_0000);
        assert_eq!(word(0x100), 0x7fc0_0001);
        assert_eq!(word(0x10c), 0x00ff_ffff);
        assert_eq!([word(0x114), word(0x118)], [1, 1]);
        assert_eq!(word(0x130), 0x3f80_0000);
        assert_eq!(word(0x134), 0xaaaa_aaaa);
        let untouched: Vec<u32> = (0..0x134)
            .step_by(4)
            .filter(|offset| word(*offset) == 0xaaaa_aaaa)
            .collect();
        assert_eq!(untouched, vec![0xf4]);
    }

    #[test]
    fn fn_00506510_writes_the_defaults() {
        let (mut e, data) = defaults_rig();
        e.call(0x0050_6510, &args![data]);
        check_defaults(&e, data);
        let points = calls(&e, POINT3_CTOR);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0][1..], [0, 0, 0]);
    }

    #[test]
    fn fn_005064d0_constructs_the_points_then_the_defaults() {
        let (mut e, data) = defaults_rig();
        assert_eq!(e.call(0x0050_64d0, &args![data]).u32(), data);
        assert_eq!(
            calls(&e, IDENTITY_006815C0),
            vec![vec![data + 0x94], vec![data + 0xa0]]
        );
        check_defaults(&e, data);
    }

    #[test]
    fn fn_00505db0_writes_the_defaults_on_the_data() {
        let (mut e, data) = defaults_rig();
        e.register(DATA_00500940, |_, a| (a[0] + 0x18).into_ret());
        e.call(0x0050_5db0, &args![data - 0x18]);
        check_defaults(&e, data);
    }

    #[test]
    fn fn_00506390_and_00507b00_answer_their_fields() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0050_6390, &args![0x5000u32]).u32(), 0x5158);
        let this = e.mem.alloc(0x50);
        e.mem.set_f32(this + 0x4c, 3.25);
        assert_eq!(e.call(0x0050_7b00, &args![this]).f32(), 3.25);
        e.mem.set_u32(this + 0x4c, 0xff80_0004);
        assert_eq!(
            e.call(0x0050_7b00, &args![this]).f32().to_bits(),
            0xffc0_0004
        );
    }

    type Script = Rc<RefCell<(Vec<(u32, u32)>, usize)>>;

    const FILE: u32 = 0x0e00_0000;

    fn load_rig(chunks: Vec<(u32, u32)>, swap: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(DOUBLE_ONE, 1.0);
        e.mem.set_f64(DOUBLE_ZERO, 0.0);
        let script: Script = Rc::new(RefCell::new((chunks, 0)));
        e.register(GET_TES_FORM, |_, _| 0x4fu32.into_ret());
        e.register(LOAD_FORM, |_, _| Ret::default());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
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
        for addr in [
            GET_CHUNK_DATA_SIZED,
            LOAD_TEXTURE_CHUNK,
            SWAP_DATA_00506850,
            LOAD_DATA,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register_double(NEEDS_SWAP, move |_, _| swap.into_ret());
        e.register(COUNT_00BA52D0, |_, _| 0x1au32.into_ret());
        e.put_vtable(0x0f00_d000, &[0; 0x4e]);
        e.mem.set_u32(0x0f00_d018, 0x0f00_d300);
        e.mem.set_u32(0x0f00_d0e0, 0x0f00_d100);
        e.mem.set_u32(0x0f00_d134, 0x0f00_d200);
        for addr in [0x0f00_d100u32, 0x0f00_d200, 0x0f00_d300] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x170);
        e.mem.set_u32(this, 0x0f00_d000);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_00505e70_refuses_other_records() {
        let (mut e, this) = load_rig(vec![], false);
        e.register(GET_TES_FORM, |_, _| 0x50u32.into_ret());
        assert!(!e.call(0x0050_5e70, &args![this, FILE]).bool());
        assert!(calls(&e, 0x0f00_d300).is_empty());
    }

    #[test]
    fn fn_00505e70_reads_each_chunk() {
        let chunks = vec![
            (CHUNK_EDID as u32, 5),
            (CHUNK_OBND as u32, 0xc),
            (CHUNK_ICON as u32, 4),
            (CHUNK_ICO2 as u32, 4),
            (CHUNK_NAM7 as u32, 4),
            (CHUNK_DATA as u32, 0x134),
            (0x8000_0000, 0),
        ];
        let (mut e, this) = load_rig(chunks, true);
        assert!(e.call(0x0050_5e70, &args![this, FILE]).bool());
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .take(3)
            .map(|c| c.0)
            .collect();
        assert_eq!(order, vec![GET_TES_FORM, 0x0f00_d300, LOAD_FORM]);
        let edid = calls(&e, 0x0f00_d200);
        assert_eq!(
            calls(&e, GET_CHUNK_DATA_SIZED),
            vec![vec![FILE, edid[0][1], 0x200]]
        );
        assert_eq!(calls(&e, 0x0f00_d100), vec![vec![this, FILE]]);
        let textures: Vec<u32> = calls(&e, LOAD_TEXTURE_CHUNK)
            .iter()
            .map(|c| c[0] - this)
            .collect();
        assert_eq!(textures, vec![0x14c, 0x158, 0x164]);
        assert_eq!(
            calls(&e, LOAD_DATA),
            vec![vec![this, FILE, this + 0x18, 0x134]]
        );
        assert_eq!(calls(&e, SWAP_DATA_00506850), vec![vec![this + 0x18]; 2]);
        assert_eq!(calls(&e, NEXT_CHUNK).len(), 7);
        // Both scales were 0: unchanged, no count asked.
        assert!(calls(&e, COUNT_00BA52D0).is_empty());
    }

    #[test]
    fn fn_00505e70_scales_fractions_by_the_count() {
        for (value, scaled) in [
            (0.5f32, Some(13.0f32)),
            (1.0, Some(26.0)),
            (1.5, None),
            (0.0, None),
            (-0.5, None),
            (f32::NAN, None),
        ] {
            let (mut e, this) = load_rig(vec![], false);
            e.mem.set_f32(this + 0x94, value);
            e.mem.set_f32(this + 0x98, 0.25);
            assert!(e.call(0x0050_5e70, &args![this, FILE]).bool());
            match scaled {
                Some(want) => assert_eq!(e.mem.f32(this + 0x94), want),
                None => assert_eq!(e.mem.u32(this + 0x94), value.to_bits()),
            }
            assert_eq!(e.mem.f32(this + 0x98), 6.5);
        }
    }
}
