//! `fallout shared/run_005a7400` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005a77b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a77b0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6b (0x28 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_69dc`) is `TESRecipeCategory`, so this is the constructor of `TESRecipeCategory`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005a77b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_69dc);
    e.mem.set_u32(base + 0x18, 0x0103_69c8);
    e.call(0x004f_15a0, &args![this, 0x6bu32]);
    e.call(0x005a_7840, &args![this, 0x1u32]);
    this
}

// Translated from 005a7c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a7c50 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6a (0x5c bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_6b2c`) is `TESRecipe`, so this is the constructor of `TESRecipe`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005a7c50(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_6b2c);
    e.mem.set_u32(base + 0x18, 0x0103_6b18);
    e.call(0x0068_0890, &args![base + 0x34]);
    e.call(0x0096_a2d0, &args![base + 0x3c]);
    e.call(0x0096_a2d0, &args![base + 0x44]);
    e.call(0x0096_a2d0, &args![base + 0x4c]);
    e.call(0x004f_15a0, &args![this, 0x6au32]);
    this
}

// Translated from 005a7870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005a78a0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005a7870(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005a_78a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005a7d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005a7d40` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005a7d10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005a_7d40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005a7840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the bits of `flags` (a byte) in the recipe category's flag byte
/// at +0x24 (`TESRecipeCategory`, its constructor `005a77b0` clears bit 0).
pub fn fn_005a7840(e: &mut Engine, this: Ptr, flags: u8) {
    let value = e.mem.u8(this.addr() + 0x24);
    e.mem.set_u8(this.addr() + 0x24, value & !flags);
}

// Translated from 005a78a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a78a0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `TESRecipeCategory` (the vtables `005a77b0` stores):
/// puts back the vtables, destroys the full name at +0x18 (`00591300`) and
/// the `TESForm` base (`00483630`).
pub fn fn_005a78a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_69dc);
    e.mem.set_u32(base + 0x18, 0x0103_69c8);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005a7910 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a7910 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESRecipeCategory`'s record reader (`RET 4`: the `TESFile`; the engine
/// map has no name): false unless the form type byte is 0x6B; otherwise
/// `TESForm::LoadForm`, `00484ab0(0)` and each chunk: `EDID` (vtable
/// +0x134), `OBND` (vtable +0xE0), `FULL` (+0x18, `00487050`) and `DATA`
/// (one byte at +0x24, `TESForm::LoadData`, then `00483710` on it when the
/// file asks for swapped words). Returns true.
pub fn fn_005a7910(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let form_type = e.call(0x0047_2660, &args![file]).u8();
    if form_type != 0x6b {
        return false;
    }
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            // OBND
            0x444e_424f => {
                e.vcall(base, 0xe0, &args![file]);
            }
            // DATA
            0x4154_4144 => {
                e.call(0x0048_61f0, &args![this, file, base + 0x24, 1u32]);
                if e.call(0x0040_1680, &args![file]).bool() {
                    e.call(0x0048_3710, &args![base + 0x24]);
                }
            }
            // EDID
            0x4449_4445 => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(0x0047_2890, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            // FULL
            0x4c4c_5546 => {
                let full = if this.is_null() { 0 } else { base + 0x18 };
                e.call(0x0048_7050, &args![full, file]);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 005a7be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a recipe's 8-byte entry (the record reader `005a8110`
/// makes them): `00537e90(0)` on it. Returns `this`.
pub fn fn_005a7be0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0053_7e90, &args![this, 0u32]);
    this
}

// Translated from 005a7d40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a7d40 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `TESRecipe` (the vtables `005a7c50` stores): puts
/// back the vtables, clears the recipe (`005a80a0`), destroys the lists at
/// +0x4C, +0x44 and +0x3C (`0046ffb0`), the conditions at +0x34
/// (`006808b0`), the full name at +0x18 (`00591300`) and the `TESForm`
/// base (`00483630`).
pub fn fn_005a7d40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_6b2c);
    e.mem.set_u32(base + 0x18, 0x0103_6b18);
    e.call(0x005a_80a0, &args![this]);
    for list in [0x4cu32, 0x44, 0x3c] {
        e.call(0x0046_ffb0, &args![base + list]);
    }
    e.call(0x0068_08b0, &args![base + 0x34]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005a_77b0, fn_005a77b0(Ptr) -> Ptr),
        entry!(0x005a_7c50, fn_005a7c50(Ptr) -> Ptr),
        entry!(0x005a_7870, fn_005a7870(Ptr, u32) -> Ptr),
        entry!(0x005a_7d10, fn_005a7d10(Ptr, u32) -> Ptr),
        entry!(0x005a_7840, fn_005a7840(Ptr, u8)),
        entry!(0x005a_78a0, fn_005a78a0(Ptr)),
        entry!(0x005a_7910, fn_005a7910(Ptr, Ptr) -> bool),
        entry!(0x005a_7be0, fn_005a7be0(Ptr) -> Ptr),
        entry!(0x005a_7d40, fn_005a7d40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005a77b0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005a_77b0,
            size: 0x28,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6b)]),
                (0x005a_7840, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[(0x0, 4, 0x0103_69dc), (0x18, 4, 0x0103_69c8)],
        });
    }

    #[test]
    fn fn_005a7c50_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005a_7c50,
            size: 0x5c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0068_0890, &[A::T(0x34)]),
                (0x0096_a2d0, &[A::T(0x3c)]),
                (0x0096_a2d0, &[A::T(0x44)]),
                (0x0096_a2d0, &[A::T(0x4c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6a)]),
            ],
            stores: &[(0x0, 4, 0x0103_6b2c), (0x18, 4, 0x0103_6b18)],
        });
    }
    #[test]
    fn at_005a7870_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005a_78a0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005a_7870, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005a_7870)
                .collect();
            let mut expected = vec![(0x005a_78a0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005a7d10_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005a_7d40, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005a_7d10, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005a_7d10)
                .collect();
            let mut expected = vec![(0x005a_7d40, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod recipe_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_005a7840_clears_the_flag_bits() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        e.mem.set_u8(this + 0x24, 0xff);
        e.call(0x005a_7840, &args![this, 0x0105u32]);
        assert_eq!(e.mem.u8(this + 0x24), 0xfa);
    }

    #[test]
    fn the_destructors_destroy_their_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        quiet(
            &mut e,
            &[
                0x0059_1300,
                0x0048_3630,
                0x005a_80a0,
                0x0046_ffb0,
                0x0068_08b0,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x005a_78a0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_69dc);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_69c8);
        assert_eq!(calls(&e, 0x0059_1300), vec![vec![this + 0x18]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]]);

        e.call_log = Some(vec![]);
        e.call(0x005a_7d40, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_6b2c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_6b18);
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x005a_7d40)
            .collect();
        assert_eq!(
            order,
            vec![
                (0x005a_80a0, vec![this]),
                (0x0046_ffb0, vec![this + 0x4c]),
                (0x0046_ffb0, vec![this + 0x44]),
                (0x0046_ffb0, vec![this + 0x3c]),
                (0x0068_08b0, vec![this + 0x34]),
                (0x0059_1300, vec![this + 0x18]),
                (0x0048_3630, vec![this]),
            ]
        );
    }

    #[test]
    fn fn_005a7be0_starts_the_entry() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0053_7e90]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_7be0, &args![0x4000u32]).u32(), 0x4000);
        assert_eq!(calls(&e, 0x0053_7e90), vec![vec![0x4000, 0]]);
    }

    fn category(form_type: u32, swap: bool) -> (Engine, u32, u32, bool) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((
            vec![0x4449_4445u32, 0x444e_424f, 0x4c4c_5546, 0x4154_4144],
            0usize,
        )));
        e.register_double(0x0047_2660, move |_, _| ret(form_type));
        let c = chunks.clone();
        e.register_double(0x0047_26b0, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).copied().unwrap_or(0))
        });
        let c = chunks;
        e.register_double(0x0047_26f0, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        e.register(0x0040_1660, |_, _| ret(8));
        if swap {
            e.register(0x0040_1680, |_, _| ret(1));
        } else {
            e.register(0x0040_1680, |_, _| ret(0));
        }
        quiet(
            &mut e,
            &[
                0x7000_00e0,
                0x7000_0134,
                0x0047_2890,
                0x0048_5110,
                0x0048_4ab0,
                0x0048_61f0,
                0x0048_3710,
                0x0048_7050,
            ],
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x005a_7910, &args![this, file]).bool();
        (e, this, file, result)
    }

    #[test]
    fn fn_005a7910_reads_the_category() {
        let (e, this, file, result) = category(0x6b, true);
        assert!(result);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0x24, 1]]
        );
        assert_eq!(calls(&e, 0x0048_3710), vec![vec![this + 0x24]]);

        let (e, _, _, result) = category(0x6b, false);
        assert!(result);
        assert!(calls(&e, 0x0048_3710).is_empty());

        let (e, _, _, result) = category(0x6a, false);
        assert!(!result);
        assert!(calls(&e, 0x0048_5110).is_empty());
    }
}
