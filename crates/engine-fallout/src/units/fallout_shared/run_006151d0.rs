//! `fallout shared/run_006151d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006153c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006153c0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x68 (0x50 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0104_ba24`) is `TESReputation`, so this is the constructor of `TESReputation`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_006153c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0040_2c20, &args![base + 0x24]);
    e.call(0x0047_a130, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0104_ba24);
    e.mem.set_u32(base + 0x18, 0x0104_ba0c);
    e.mem.set_u32(base + 0x24, 0x0104_b9ec);
    e.mem.set_u32(base + 0x30, 0x0104_b9d8);
    e.call(0x0061_5560, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x68u32]);
    this
}

/// The race's body texture for a sex and index (`00613a20`, tesrace.cpp,
/// `__thiscall`, `RET 8`) and whether a texture has a path (`0048cee0`).
const BODY_TEXTURE: u32 = 0x0061_3a20;
const HAS_PATH: u32 = 0x0048_cee0;

// Translated from 00615560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the two floats at `this + 0x44` and `this + 0x48` to 0.0
/// (`FLDZ`/`FSTP`; `ECX`, no stack words). The constructor `fn_006153c0`
/// runs it. The engine map gives it no name.
pub fn fn_00615560(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x44, x87_store32(0.0));
    e.mem.set_u32(this.addr() + 0x48, x87_store32(0.0));
}

// Translated from 006155c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `this + 0x44` when `which` is 1, else the one at
/// `this + 0x48`, returned in ST0 (`FLD`, which quiets a signalling NaN).
/// `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_006155c0(e: &mut Engine, this: Ptr, which: i32) -> f64 {
    let at = if which == 1 { 0x44 } else { 0x48 };
    x87_load32(e.mem.u32(this.addr() + at))
}

// Translated from 006154b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006154b0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The `TESReputation` destructor body (the scalar deleting destructor
/// `fn_00615480` runs it): stores the vtables back (`0104ba24`, and the
/// components' at +0x18, +0x24, +0x30), runs `00483710` on `this` (an
/// empty function in the exe), the component destructors `0047a1a0`
/// (+0x30), `00402c40` (+0x24), `00591300` (+0x18) and
/// `TESForm::~TESForm` (`00483630`). `ECX`, no stack words. The
/// exception-unwinding frame is not translated.
pub fn fn_006154b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_ba24);
    e.mem.set_u32(base + 0x18, 0x0104_ba0c);
    e.mem.set_u32(base + 0x24, 0x0104_b9ec);
    e.mem.set_u32(base + 0x30, 0x0104_b9d8);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0047_a1a0, &args![base + 0x30]);
    e.call(0x0040_2c40, &args![base + 0x24]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00615480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESReputation` scalar deleting destructor: runs `fn_006154b0` on
/// `this`, then frees `this` with `00401030` when bit 0 of `flags` is set.
/// Returns `this`. `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_00615480(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006154b0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006151d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word `0043d4d0` gives the address of for the object at `011d5a50`
/// (`0043d4d0` is `__thiscall` on it, modelloader.cpp). No arguments. The
/// engine map gives it no name.
pub fn fn_006151d0(e: &mut Engine) -> u32 {
    let at = e.call(0x0043_d4d0, &args![0x011d_5a50u32]).u32();
    e.mem.u32(at)
}

// Translated from 00615300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body texture index of a body part: `part - 2` for parts 2..=4,
/// -1 for any other (cdecl). The engine map gives it no name.
pub fn fn_00615300(_e: &mut Engine, part: i32) -> i32 {
    let index = part.wrapping_sub(2);
    // `SUB` then `JS`, then a signed compare with 3.
    if !(0..3).contains(&index) {
        -1
    } else {
        index
    }
}

/// The body texture of `sex` and `index` when it has a path.
fn body_texture_with_path(e: &mut Engine, this: Ptr, sex: i32, index: i32) -> Option<u32> {
    if e.call(BODY_TEXTURE, &args![this, sex, index]).u32() == 0 {
        return None;
    }
    let texture = e.call(BODY_TEXTURE, &args![this, sex, index]).u32();
    if e.call(HAS_PATH, &args![texture]).u32() == 0 {
        return None;
    }
    Some(e.call(BODY_TEXTURE, &args![this, sex, index]).u32())
}

// Translated from 006151f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The race's body texture for `sex` (0 or 1) and body part `part`
/// (`fn_00615300` gives the index) that has a path (`0048cee0`), falling
/// back to the other sex's; 0 when neither has one or the arguments are
/// out of range. `__thiscall`, `RET 8`. The engine map gives it no name.
pub fn fn_006151f0(e: &mut Engine, this: Ptr, sex: i32, part: i32) -> u32 {
    let index = fn_00615300(e, part);
    if !(0..2).contains(&sex) || !(0..3).contains(&index) {
        return 0;
    }
    if let Some(texture) = body_texture_with_path(e, this, sex, index) {
        return texture;
    }
    let other = match sex {
        0 => 1,
        1 => 0,
        _ => return 0,
    };
    body_texture_with_path(e, this, other, index).unwrap_or(0)
}

// Translated from 00615330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00615330 - the /GS stack cookie check (00ec408c) is not translated.
/// Sets the race's string at `this + 0x50c` to `text` when it differs:
/// the text is copied into a stack buffer (`0044a670` its length,
/// `00406d30`), compared with `BSStringT<char>::StrCmp` (case-insensitive
/// flag 1); an equal string answers false, otherwise `004037f0` sets it and
/// it answers true. `__thiscall`, `RET 4`. The engine map gives it no
/// name. The stack cookie check is not translated.
pub fn fn_00615330(e: &mut Engine, this: Ptr, text: u32) -> bool {
    let length = e.call(0x0044_a670, &args![text]).u32().wrapping_add(1);
    // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
    let reserved = length.wrapping_add(15) & !15;
    e.with_stack(reserved, |e, copy| {
        e.call(0x0040_6d30, &args![copy, length, text]);
        let string = this.addr() + 0x50c;
        if e.call(0x0040_8a80, &args![string, copy, 1u32]).u32() == 0 {
            return false;
        }
        e.call(0x0040_37f0, &args![string, copy, 0u32]);
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_53c0, fn_006153c0(Ptr) -> Ptr),
        entry!(0x0061_5560, fn_00615560(Ptr)),
        entry!(0x0061_55c0, fn_006155c0(Ptr, i32) -> f64),
        entry!(0x0061_54b0, fn_006154b0(Ptr)),
        entry!(0x0061_5480, fn_00615480(Ptr, u32) -> Ptr),
        entry!(0x0061_51d0, fn_006151d0() -> u32),
        entry!(0x0061_5300, fn_00615300(i32) -> i32),
        entry!(0x0061_51f0, fn_006151f0(Ptr, i32, i32) -> u32),
        entry!(0x0061_5330, fn_00615330(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_006153c0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0061_53c0,
            size: 0x50,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0040_2c20, &[A::T(0x24)]),
                (0x0047_a130, &[A::T(0x30)]),
                (0x0061_5560, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x68)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_ba24),
                (0x18, 4, 0x0104_ba0c),
                (0x24, 4, 0x0104_b9ec),
                (0x30, 4, 0x0104_b9d8),
            ],
        });
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;

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

    #[test]
    fn the_two_floats() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x50);
        e.mem.set_f32(object + 0x44, 3.5);
        e.mem.set_f32(object + 0x48, -2.0);
        assert_eq!(e.call(0x0061_55c0, &[object, 1]).f64(), 3.5);
        assert_eq!(e.call(0x0061_55c0, &[object, 0]).f64(), -2.0);
        assert_eq!(e.call(0x0061_55c0, &[object, 2]).f64(), -2.0);
        // A signalling NaN comes back quiet.
        e.mem.set_u32(object + 0x44, 0x7f80_0001);
        let nan = e.call(0x0061_55c0, &[object, 1]).f64();
        assert_eq!((nan as f32).to_bits(), 0x7fc0_0001);
        e.call(0x0061_5560, &[object]);
        assert_eq!(e.mem.u32(object + 0x44), 0);
        assert_eq!(e.mem.u32(object + 0x48), 0);
    }

    #[test]
    fn the_destructor_runs_the_bases_and_deletes_on_flag_one() {
        let mut e = Engine::new();
        let callees = [
            0x0048_3710,
            0x0047_a1a0,
            0x0040_2c40,
            0x0059_1300,
            0x0048_3630,
            0x0040_1030,
        ];
        for callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0x50);
        for flags in [0u32, 1] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0061_5480, &[object, flags]).u32(), object);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![
                (0x0048_3710, vec![object]),
                (0x0047_a1a0, vec![object + 0x30]),
                (0x0040_2c40, vec![object + 0x24]),
                (0x0059_1300, vec![object + 0x18]),
                (0x0048_3630, vec![object]),
            ];
            if flags == 1 {
                expected.push((0x0040_1030, vec![object]));
            }
            assert_eq!(log[1..], expected[..]);
            assert_eq!(e.mem.u32(object), 0x0104_ba24);
            assert_eq!(e.mem.u32(object + 0x30), 0x0104_b9d8);
        }
    }

    #[test]
    fn the_word_of_the_global_object() {
        let mut e = Engine::new();
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 0x77);
        e.register_double(0x0043_d4d0, move |_, _| ret(word));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_51d0, &[]).u32(), 0x77);
        assert_eq!(calls_to(&e, 0x0043_d4d0), vec![vec![0x011d_5a50]]);
    }

    #[test]
    fn body_part_indices() {
        let mut e = Engine::new();
        let index = |e: &mut Engine, part: i32| e.call(0x0061_5300, &args![part]).i32();
        assert_eq!(index(&mut e, 1), -1);
        assert_eq!(index(&mut e, 2), 0);
        assert_eq!(index(&mut e, 4), 2);
        assert_eq!(index(&mut e, 5), -1);
        assert_eq!(index(&mut e, i32::MIN), -1);
    }

    /// The race double: texture `(sex, index)` is `0x100 * (sex + 1) +
    /// index`; textures listed in `with_path` have a path.
    fn race(with_path: &'static [u32]) -> Engine {
        let mut e = Engine::new();
        e.register(BODY_TEXTURE, |_, a| ret(0x100 * (a[1] + 1) + a[2]));
        e.register_double(HAS_PATH, move |_, a| {
            ret(u32::from(with_path.contains(&a[0])))
        });
        e
    }

    #[test]
    fn body_textures_fall_back_to_the_other_sex() {
        let mut e = race(&[0x101, 0x202]);
        let get = |e: &mut Engine, sex: i32, part: i32| {
            e.call(0x0061_51f0, &args![0x1000u32, sex, part]).u32()
        };
        assert_eq!(get(&mut e, 0, 3), 0x101);
        assert_eq!(get(&mut e, 1, 3), 0x101);
        assert_eq!(get(&mut e, 0, 4), 0x202);
        assert_eq!(get(&mut e, 0, 2), 0);
        assert_eq!(get(&mut e, 2, 3), 0);
        assert_eq!(get(&mut e, -1, 3), 0);
        assert_eq!(get(&mut e, 0, 9), 0);
    }

    #[test]
    fn the_name_string_is_set_when_it_differs() {
        let mut e = Engine::new();
        e.register(0x0044_a670, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register(0x0040_6d30, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(0x0040_37f0, |_, _| Ret::default());
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"Nord");
        let race = e.mem.alloc(0x524);
        for (equal, changed) in [(0u32, false), (1, true)] {
            e.register_double(0x0040_8a80, move |_, _| ret(equal));
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0061_5330, &[race, text]).bool(), changed);
            let compares = calls_to(&e, 0x0040_8a80);
            assert_eq!(compares[0][0], race + 0x50c);
            assert_eq!(compares[0][2], 1);
            assert_eq!(calls_to(&e, 0x0040_6d30)[0][1], 5);
            let sets = calls_to(&e, 0x0040_37f0);
            assert_eq!(sets.len(), usize::from(changed));
            if changed {
                assert_eq!(sets[0], vec![race + 0x50c, compares[0][1], 0]);
            }
        }
    }
}
