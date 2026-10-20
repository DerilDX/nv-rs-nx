//! `fallout shared/tesammo.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005026b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005026b0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESAmmo::TESAmmo` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x29 (0xdc bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn tesammo_tesammo(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0047_a130, &args![base + 0x68]);
    e.call(0x0048_e7a0, &args![base + 0x78]);
    e.call(0x0047_50b0, &args![base + 0x80]);
    e.call(0x0047_5350, &args![base + 0x88]);
    e.call(0x0047_b4f0, &args![base + 0x90]);
    e.call(0x0048_cc50, &args![base + 0x9c]);
    e.mem.set_u32(base, 0x0102_6064);
    e.mem.set_u32(base + 0x30, 0x0102_604c);
    e.mem.set_u32(base + 0x3c, 0x0102_6028);
    e.mem.set_u32(base + 0x5c, 0x0102_6008);
    e.mem.set_u32(base + 0x68, 0x0102_5ff4);
    e.mem.set_u32(base + 0x78, 0x0102_5fd4);
    e.mem.set_u32(base + 0x80, 0x0102_5fc0);
    e.mem.set_u32(base + 0x88, 0x0102_5fac);
    e.mem.set_u32(base + 0x90, 0x0102_5f98);
    e.mem.set_u32(base + 0x9c, 0x0102_5f84);
    e.call(0x0040_37b0, &args![base + 0xc4]);
    e.call(0x0040_37b0, &args![base + 0xcc]);
    e.call(0x0096_a2d0, &args![base + 0xd4]);
    e.call(0x004f_15a0, &args![this, 0x29]);
    e.call(0x0050_29f0, &args![this]);
    this
}

// Translated from 005029f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): the last step of the
/// form constructor `tesammo_tesammo` (`005026b0`). `ECX` is the object. It clears
/// the 8 bytes at `+0xa8` and the 0x14 bytes at `+0xb0` through `00403d30` (a
/// `cdecl` wrapper that forwards its destination, value and count to `_memset`),
/// then runs `004037f0` on the strings at `+0xc4` and `+0xcc` with two zero words
/// each. Returns nothing.
pub fn fn_005029f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0040_3d30, &args![base + 0xa8, 0u32, 8u32]);
    e.call(0x0040_3d30, &args![base + 0xb0, 0u32, 0x14u32]);
    e.call(0x0040_37f0, &args![base + 0xc4, 0u32, 0u32]);
    e.call(0x0040_37f0, &args![base + 0xcc, 0u32, 0u32]);
}

/// The form id (the word at `form + 0xc`, `0084e3a0`), the error printer
/// (`005b5e40`, cdecl) and the id-to-form helpers `InitItem` uses.
const FORM_ID: u32 = 0x0084_e3a0;
const PRINT_ERROR: u32 = 0x005b_5e40;
const GET_FILE: u32 = 0x0048_4e60;
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM: u32 = 0x0048_39c0;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// A list node's data address and the next node.
const NODE_DATA: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// The ammo's effect list (`00923000`, `__thiscall`).
const EFFECT_LIST: u32 = 0x0092_3000;

// Translated from 00503650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0xa8` (`ECX`, no stack words; other classes call this shared
/// body too, `TESRace` for its eye list). The engine map gives it no name.
pub fn fn_00503650(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xa8)
}

// Translated from 00502cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `form` (the ammo consumed) at `this + 0xbc`. `__thiscall`,
/// `RET 4`. The engine map gives it no name.
pub fn fn_00502cb0(e: &mut Engine, this: Ptr, form: u32) {
    e.mem.set_u32(this.addr() + 0xbc, form);
}

// Translated from 00502880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00502880 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The `TESAmmo` destructor body (the scalar deleting destructor
/// `fn_00502850` runs it): stores the vtables the constructor stores back
/// (`01026064` and the components' at +0x30 .. +0x9c), then destroys the
/// list at +0xd4 (`0046ffb0`), the strings at +0xcc and +0xc4
/// (`004037d0`), the components at +0x90 (`0047b520`), +0x88
/// (`00475380`), +0x80 (`004750e0`), +0x78 (`0048e7d0`), +0x68
/// (`0047a1a0`), +0x5c (`00402c40`), +0x3c (`0048a490`), +0x30
/// (`00591300`) and the base (`0050e9f0`). `ECX`, no stack words. The
/// exception-unwinding frame is not translated.
pub fn fn_00502880(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let vtables: [(u32, u32); 10] = [
        (0x00, 0x0102_6064),
        (0x30, 0x0102_604c),
        (0x3c, 0x0102_6028),
        (0x5c, 0x0102_6008),
        (0x68, 0x0102_5ff4),
        (0x78, 0x0102_5fd4),
        (0x80, 0x0102_5fc0),
        (0x88, 0x0102_5fac),
        (0x90, 0x0102_5f98),
        (0x9c, 0x0102_5f84),
    ];
    for (offset, vtable) in vtables {
        e.mem.set_u32(base + offset, vtable);
    }
    let destructors: [(u32, u32); 11] = [
        (0xd4, 0x0046_ffb0),
        (0xcc, 0x0040_37d0),
        (0xc4, 0x0040_37d0),
        (0x90, 0x0047_b520),
        (0x88, 0x0047_5380),
        (0x80, 0x0047_50e0),
        (0x78, 0x0048_e7d0),
        (0x68, 0x0047_a1a0),
        (0x5c, 0x0040_2c40),
        (0x3c, 0x0048_a490),
        (0x30, 0x0059_1300),
    ];
    for (offset, destructor) in destructors {
        e.call(destructor, &args![base + offset]);
    }
    e.call(0x0050_e9f0, &args![this]);
}

// Translated from 00502850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESAmmo` scalar deleting destructor: runs `fn_00502880` on `this`,
/// then frees `this` with `00401030` when bit 0 of `flags` is set. Returns
/// `this`. `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_00502850(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00502880(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The id at `slot` with the ammo file's compile index
/// (`TESForm::GetFile(-1)`, `TESForm::AddCompileIndex`), looked up
/// (`004839c0`); returns the (updated) id and the form.
fn resolve(e: &mut Engine, this: Ptr, slot: Ptr) -> (u32, u32) {
    let file = e.call(GET_FILE, &args![this, 0xffff_ffffu32]).u32();
    e.call(ADD_COMPILE_INDEX, &args![slot, file]);
    let id = e.mem.u32(slot.addr());
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    (id, form)
}

/// Prints a `MASTERFILE: Could not find ...` message with the id, the
/// ammo's name (virtual slot 0x130) and its form id.
fn report_missing(e: &mut Engine, this: Ptr, message: u32, id: u32) {
    let form_id = e.call(FORM_ID, &args![this]).u32();
    let name = e.vcall(this.addr(), 0x130, &args![]).u32();
    e.call(PRINT_ERROR, &args![message, id, name, form_id]);
}

// Translated from 00502a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAmmo::InitItem` (engine map): unless `004013e0` answers true, runs
/// the components' `InitItem`s (`TESScriptableForm` on +0x9c,
/// `BGSDestructibleObjectForm` on +0x88, `TESModelTextureSwap` on +0x3c,
/// `BGSPickupPutdownSounds` on +0x90), then resolves the projectile
/// (`008d80e0`, cast to the type at `01186228`, stored with `00502c90`),
/// the ammo consumed (`0097ae90`, stored with `fn_00502cb0`) and each id of
/// the effect list (`00923000`, cast to the type at `01186474` and stored
/// back in its node), printing the `MASTERFILE` message for each one not
/// found; then `00484ab0(1)`. `ECX`, no stack words.
pub fn tes_ammo_init_item(e: &mut Engine, this: Ptr) {
    let ammo = this.addr();
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    e.call(0x0048_cd90, &args![ammo + 0x9c, this]);
    e.call(0x0047_8be0, &args![ammo + 0x88, this]);
    e.call(0x0048_aa80, &args![ammo + 0x3c, this]);
    e.call(0x0047_b6a0, &args![ammo + 0x90, this]);
    e.with_stack(4, |e, slot| {
        let id = e.call(0x008d_80e0, &args![this]).u32();
        e.mem.set_u32(slot.addr(), id);
        if id != 0 {
            let (id, form) = resolve(e, this, slot);
            let projectile = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TES_FORM_TYPE, 0x0118_6228u32, 0u32],
                )
                .u32();
            e.call(0x0050_2c90, &args![this, projectile]);
            if e.call(0x008d_80e0, &args![this]).u32() == 0 {
                report_missing(e, this, 0x0102_6288, id);
            }
        }
        let id = e.call(0x0097_ae90, &args![this]).u32();
        e.mem.set_u32(slot.addr(), id);
        if id != 0 {
            let (id, form) = resolve(e, this, slot);
            fn_00502cb0(e, this, form);
            if e.call(0x0097_ae90, &args![this]).u32() == 0 {
                report_missing(e, this, 0x0102_6238, id);
            }
        }
        let mut node = e.call(EFFECT_LIST, &args![this]).u32();
        while node != 0 {
            let data = e.call(NODE_DATA, &args![node]).u32();
            if e.mem.u32(data) == 0 {
                break;
            }
            let data = e.call(NODE_DATA, &args![node]).u32();
            let id = e.mem.u32(data);
            e.mem.set_u32(slot.addr(), id);
            if id != 0 {
                let (id, form) = resolve(e, this, slot);
                let effect = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![form, 0u32, TES_FORM_TYPE, 0x0118_6474u32, 0u32],
                    )
                    .u32();
                if effect == 0 {
                    report_missing(e, this, 0x0102_61e8, id);
                }
                let data = e.call(NODE_DATA, &args![node]).u32();
                e.mem.set_u32(data, effect);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

// Translated from 00503a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00503a70 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESAmmo::BuildMenuString` (engine map): clears `out` and appends the
/// menu text of each effect in the effect list (`00923000`;
/// `TESAmmoEffect::BuildMenuString` into a 0x100-byte buffer, run twice
/// per effect as the exe does), the texts separated by `"\n"` (`00406d50`
/// with `size`). Empty texts are skipped. `__thiscall`, `RET 8`. The stack
/// cookie check is not translated.
pub fn tes_ammo_build_menu_string(e: &mut Engine, this: Ptr, out: u32, size: u32) {
    let mut appended = false;
    e.mem.set_u8(out, 0);
    e.with_stack(0x108, |e, text| {
        e.mem.set_u8(text.addr(), 0);
        let mut node = e.call(EFFECT_LIST, &args![this]).u32();
        while node != 0 {
            let data = e.call(NODE_DATA, &args![node]).u32();
            if e.mem.u32(data) == 0 {
                break;
            }
            let data = e.call(NODE_DATA, &args![node]).u32();
            let effect = e.mem.u32(data);
            e.call(0x0059_a1e0, &args![effect, text, 0x100u32]);
            let data = e.call(NODE_DATA, &args![node]).u32();
            let effect = e.mem.u32(data);
            node = e.call(NODE_NEXT, &args![node]).u32();
            e.call(0x0059_a1e0, &args![effect, text, 0x100u32]);
            if e.call(0x0044_a670, &args![text]).u32() != 0 {
                if appended {
                    e.call(0x0040_6d50, &args![out, size, 0x0101_2648u32]);
                    e.call(0x0040_6d50, &args![out, size, text]);
                } else {
                    e.call(0x0040_6d50, &args![out, size, text]);
                    appended = true;
                }
            }
        }
    });
}

/// Passes a named node of the clone, when there is one, to virtual slot
/// 0xe8 of the object `009611e0` gives for it (playercharacter.cpp).
fn pass_node(e: &mut Engine, node: u32) {
    if node != 0 {
        let object = e.call(0x0096_11e0, &args![node]).u32();
        e.vcall(object, 0xe8, &args![node]);
    }
}

// Translated from 00503820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00503820 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `TESAmmo::Clone3D` (engine map): `TESBoundObject::Clone3D` with
/// `reference` and `flag`; when the reference's count (its extra list's
/// `ExtraDataList::GetCount`, 1 without a reference) is below the arrow
/// limit (the word `0043d4d0` gives for `011c846c`), passes the arrow nodes
/// the count does not use to `pass_node`: for a count of 1 the node named
/// by `[011c6200]` (`005039e0`; virtual slot 0x9c of the clone finds it),
/// then `00c8ecf0` on the clone; otherwise each node `"Arrow%d"` from the
/// count up to the limit (a string made with `00406f60`, a fixed string
/// `00438170`). Returns the
/// clone. `__thiscall`, `RET 8`. The exception-unwinding frame is not
/// translated.
pub fn tes_ammo_clone_3d(e: &mut Engine, this: Ptr, reference: u32, flag: u8) -> u32 {
    let clone = e
        .call(0x0050_ee50, &args![this, reference, u32::from(flag)])
        .u32();
    let count = if reference != 0 {
        let extra = e.call(0x005d_43c0, &args![reference]).u32();
        i32::from(e.call(0x0041_8770, &args![extra]).u16() as i16)
    } else {
        1
    };
    if clone == 0 {
        return clone;
    }
    let limit = e.call(0x0043_d4d0, &args![0x011c_846cu32]).u32();
    if count >= e.mem.i32(limit) {
        return clone;
    }
    if count == 1 {
        let name = e.call(0x0050_39e0, &args![]).u32();
        let node = e.vcall(clone, 0x9c, &args![name]).u32();
        pass_node(e, node);
        e.call(0x00c8_ecf0, &args![clone]);
        return clone;
    }
    let mut index = count;
    loop {
        let limit = e.call(0x0043_d4d0, &args![0x011c_846cu32]).u32();
        if index >= e.mem.i32(limit) {
            break;
        }
        e.with_stack(0xc, |e, locals| {
            let string = locals.byte_add(4);
            let fixed = locals;
            e.call(0x0040_37b0, &args![string]);
            e.call(0x0040_6f60, &args![string, 0x0102_62ccu32, index]);
            let text = e.call(0x0055_9450, &args![string]).u32();
            let name = e.call(0x0043_8170, &args![fixed, text]).u32();
            let node = e.vcall(clone, 0x9c, &args![name]).u32();
            e.call(0x0043_81b0, &args![fixed]);
            pass_node(e, node);
            e.call(0x0040_37d0, &args![string]);
        });
        index = index.wrapping_add(1);
    }
    clone
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_26b0, tesammo_tesammo(Ptr) -> Ptr),
        entry!(0x0050_29f0, fn_005029f0(Ptr)),
        entry!(0x0050_3650, fn_00503650(Ptr) -> Ptr),
        entry!(0x0050_2cb0, fn_00502cb0(Ptr, u32)),
        entry!(0x0050_2880, fn_00502880(Ptr)),
        entry!(0x0050_2850, fn_00502850(Ptr, u32) -> Ptr),
        entry!(0x0050_2a50, tes_ammo_init_item(Ptr)),
        entry!(0x0050_3a70, tes_ammo_build_menu_string(Ptr, u32, u32)),
        entry!(0x0050_3820, tes_ammo_clone_3d(Ptr, u32, u8) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tesammo_tesammo_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_26b0,
            size: 0xdc,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0047_a130, &[A::T(0x68)]),
                (0x0048_e7a0, &[A::T(0x78)]),
                (0x0047_50b0, &[A::T(0x80)]),
                (0x0047_5350, &[A::T(0x88)]),
                (0x0047_b4f0, &[A::T(0x90)]),
                (0x0048_cc50, &[A::T(0x9c)]),
                (0x0040_37b0, &[A::T(0xc4)]),
                (0x0040_37b0, &[A::T(0xcc)]),
                (0x0096_a2d0, &[A::T(0xd4)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x29)]),
                (0x0050_29f0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_6064),
                (0x30, 4, 0x0102_604c),
                (0x3c, 4, 0x0102_6028),
                (0x5c, 4, 0x0102_6008),
                (0x68, 4, 0x0102_5ff4),
                (0x78, 4, 0x0102_5fd4),
                (0x80, 4, 0x0102_5fc0),
                (0x88, 4, 0x0102_5fac),
                (0x90, 4, 0x0102_5f98),
                (0x9c, 4, 0x0102_5f84),
            ],
        });
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;

    #[test]
    fn fn_005029f0_clears_the_blocks_and_the_strings() {
        let mut e = Engine::new();
        for address in [0x0040_3d30, 0x0040_37f0] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0050_29f0, &args![0x4000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0040_3d30, vec![0x40a8, 0, 8]),
                (0x0040_3d30, vec![0x40b0, 0, 0x14]),
                (0x0040_37f0, vec![0x40c4, 0, 0]),
                (0x0040_37f0, vec![0x40cc, 0, 0]),
            ]
        );
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const VTABLE: u32 = 0x0200_0000;
    const SLOTS: u32 = 0x0300_0000;

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

    fn engine(callees: &[u32]) -> Engine {
        let mut e = Engine::new();
        e.map(VTABLE, 0x200);
        for i in 0..0x80 {
            e.mem.set_u32(VTABLE + 4 * i, SLOTS + 4 * i);
            e.register(SLOTS + 4 * i, |_, _| Ret::default());
        }
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(NODE_DATA, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.call_log = Some(vec![]);
        e
    }

    /// A list of `{data, next}` nodes holding `items`.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for &item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    fn ammo(e: &mut Engine) -> u32 {
        let ammo = e.mem.alloc(0xdc);
        e.mem.set_u32(ammo, VTABLE);
        ammo
    }

    #[test]
    fn small_accessors() {
        let mut e = engine(&[]);
        assert_eq!(e.call(0x0050_3650, &[0x1000]).u32(), 0x10a8);
        let a = ammo(&mut e);
        e.call(0x0050_2cb0, &[a, 0x77]);
        assert_eq!(e.mem.u32(a + 0xbc), 0x77);
    }

    #[test]
    fn the_destructor_tears_down_the_components() {
        let callees = [
            0x0046_ffb0,
            0x0040_37d0,
            0x0047_b520,
            0x0047_5380,
            0x0047_50e0,
            0x0048_e7d0,
            0x0047_a1a0,
            0x0040_2c40,
            0x0048_a490,
            0x0059_1300,
            0x0050_e9f0,
            0x0040_1030,
        ];
        for flags in [0u32, 1] {
            let mut e = engine(&callees);
            let a = ammo(&mut e);
            assert_eq!(e.call(0x0050_2850, &[a, flags]).u32(), a);
            assert_eq!(e.mem.u32(a), 0x0102_6064);
            assert_eq!(e.mem.u32(a + 0x9c), 0x0102_5f84);
            let log: Vec<(u32, Vec<u32>)> =
                e.call_log.take().unwrap().into_iter().skip(1).collect();
            let mut expected = vec![
                (0x0046_ffb0, vec![a + 0xd4]),
                (0x0040_37d0, vec![a + 0xcc]),
                (0x0040_37d0, vec![a + 0xc4]),
                (0x0047_b520, vec![a + 0x90]),
                (0x0047_5380, vec![a + 0x88]),
                (0x0047_50e0, vec![a + 0x80]),
                (0x0048_e7d0, vec![a + 0x78]),
                (0x0047_a1a0, vec![a + 0x68]),
                (0x0040_2c40, vec![a + 0x5c]),
                (0x0048_a490, vec![a + 0x3c]),
                (0x0059_1300, vec![a + 0x30]),
                (0x0050_e9f0, vec![a]),
            ];
            if flags == 1 {
                expected.push((0x0040_1030, vec![a]));
            }
            assert_eq!(log, expected);
        }
    }

    fn init_engine(projectile: u32, consumed: u32, effects: &[u32]) -> (Engine, u32, u32) {
        let callees = [
            0x0048_cd90,
            0x0047_8be0,
            0x0048_aa80,
            0x0047_b6a0,
            0x0048_4ab0,
            PRINT_ERROR,
        ];
        let mut e = engine(&callees);
        let a = ammo(&mut e);
        let effects = list(&mut e, effects);
        e.register(0x0040_13e0, |_, _| ret(0));
        e.register_double(EFFECT_LIST, move |_, _| ret(effects));
        e.register(GET_FILE, |_, _| ret(0x0f11e));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // Ids with bit 0x80 set are not found.
        e.register(LOOKUP_FORM, |_, a| {
            ret(if a[0] & 0x80 != 0 { 0 } else { a[0] + 1 })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(FORM_ID, |_, _| ret(0x0abc));
        e.register(SLOTS + 0x130, |_, _| ret(0x0aae));
        // The projectile getter answers the id until the setter stores the
        // form (then the form); the consumed getter reads +0xbc once set.
        let projectile_slot = Rc::new(RefCell::new(projectile));
        let getter = projectile_slot.clone();
        e.register_double(0x008d_80e0, move |_, _| ret(*getter.borrow()));
        e.register_double(0x0050_2c90, move |_, a| {
            *projectile_slot.borrow_mut() = a[1];
            Ret::default()
        });
        // The consumed getter answers the id on its first call, then the
        // word at +0xbc.
        let calls = Rc::new(RefCell::new(0));
        e.register_double(0x0097_ae90, move |e, a| {
            *calls.borrow_mut() += 1;
            ret(if *calls.borrow() == 1 {
                consumed
            } else {
                e.mem.u32(a[0] + 0xbc)
            })
        });
        (e, a, effects)
    }

    #[test]
    fn init_item_resolves_everything() {
        let (mut e, a, effects) = init_engine(0x11, 0x22, &[0x33, 0x34]);
        e.call(0x0050_2a50, &[a]);
        assert_eq!(calls_to(&e, 0x0048_cd90), vec![vec![a + 0x9c, a]]);
        assert_eq!(calls_to(&e, 0x0047_8be0), vec![vec![a + 0x88, a]]);
        assert_eq!(calls_to(&e, 0x0048_aa80), vec![vec![a + 0x3c, a]]);
        assert_eq!(calls_to(&e, 0x0047_b6a0), vec![vec![a + 0x90, a]]);
        let casts = calls_to(&e, RT_DYNAMIC_CAST);
        assert_eq!(
            casts[0],
            vec![0x0100_0012, 0, TES_FORM_TYPE, 0x0118_6228, 0]
        );
        assert_eq!(calls_to(&e, 0x0050_2c90), vec![vec![a, 0x0100_0012]]);
        assert_eq!(e.mem.u32(a + 0xbc), 0x0100_0023);
        assert_eq!(casts[1][3], 0x0118_6474);
        assert_eq!(e.mem.u32(effects), 0x0100_0034);
        assert!(calls_to(&e, PRINT_ERROR).is_empty());
        assert_eq!(calls_to(&e, 0x0048_4ab0), vec![vec![a, 1]]);
    }

    #[test]
    fn init_item_reports_what_is_missing() {
        let (mut e, a, effects) = init_engine(0x81, 0x82, &[0x83, 0]);
        e.call(0x0050_2a50, &[a]);
        assert_eq!(
            calls_to(&e, PRINT_ERROR),
            vec![
                vec![0x0102_6288, 0x0100_0081, 0x0aae, 0x0abc],
                vec![0x0102_6238, 0x0100_0082, 0x0aae, 0x0abc],
                vec![0x0102_61e8, 0x0100_0083, 0x0aae, 0x0abc],
            ]
        );
        // The missing effect is stored as null; the null entry ends the walk.
        assert_eq!(e.mem.u32(effects), 0);
        assert_eq!(calls_to(&e, RT_DYNAMIC_CAST).len(), 2);
    }

    #[test]
    fn init_item_with_nothing_to_resolve() {
        let (mut e, a, _) = init_engine(0, 0, &[]);
        e.call(0x0050_2a50, &[a]);
        assert!(calls_to(&e, LOOKUP_FORM).is_empty());
        let (mut e, a, _) = init_engine(0x11, 0, &[]);
        e.register(0x0040_13e0, |_, _| ret(1));
        e.call(0x0050_2a50, &[a]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn the_menu_string_joins_the_effect_texts() {
        let mut e = engine(&[]);
        let a = ammo(&mut e);
        let effects = list(&mut e, &[0x0e1, 0x0e2, 0x0e3]);
        e.register_double(EFFECT_LIST, move |_, _| ret(effects));
        // The second effect has no text.
        e.register(0x0059_a1e0, |e, a| {
            let text: &[u8] = match a[0] {
                0x0e1 => b"Fire",
                0x0e2 => b"",
                _ => b"Ice",
            };
            e.mem.set_cstr(a[1], text);
            Ret::default()
        });
        e.register(0x0044_a670, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        let joined = Rc::new(RefCell::new(vec![]));
        let seen = joined.clone();
        e.register_double(0x0040_6d50, move |e, a| {
            seen.borrow_mut().extend(e.mem.cstr(a[2]));
            Ret::default()
        });
        e.map(0x0101_2648, 4);
        e.mem.set_cstr(0x0101_2648, b"\n");
        let out = e.mem.alloc(0x40);
        e.mem.set_u8(out, b'x');
        e.call(0x0050_3a70, &[a, out, 0x40]);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(*joined.borrow(), b"Fire\nIce");
        assert_eq!(calls_to(&e, 0x0059_a1e0).len(), 6);
        assert!(calls_to(&e, 0x0040_6d50)
            .iter()
            .all(|w| w[0] == out && w[1] == 0x40));
    }

    fn clone_engine(count: u16, limit: i32) -> (Engine, u32, u32) {
        let callees = [0x00c8_ecf0, 0x0040_37b0, 0x0040_37d0, 0x0043_81b0];
        let mut e = engine(&callees);
        let a = ammo(&mut e);
        let clone = e.mem.alloc(0x10);
        e.mem.set_u32(clone, VTABLE);
        e.register_double(0x0050_ee50, move |_, _| ret(clone));
        e.register(0x005d_43c0, |_, a| ret(a[0] + 4));
        e.register_double(0x0041_8770, move |_, _| ret(u32::from(count)));
        let limit_word = e.mem.alloc(4);
        e.mem.set_i32(limit_word, limit);
        e.register_double(0x0043_d4d0, move |_, _| ret(limit_word));
        e.register(0x0050_39e0, |_, _| ret(0x0a01));
        e.register(0x0040_6f60, |e, a| {
            let index = a[2];
            e.mem.set_u32(a[0], 0x0a00 + index);
            Ret::default()
        });
        e.register(0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0043_8170, |_, a| ret(a[1]));
        // Every name but 0x0a03 has a node.
        e.register(SLOTS + 0x9c, |_, a| {
            ret(if a[1] == 0x0a03 { 0 } else { a[1] + 0x1000 })
        });
        e.register(0x0096_11e0, |_, a| ret(a[0]));
        (e, a, clone)
    }

    #[test]
    fn a_single_arrow_passes_the_named_node() {
        let (mut e, a, clone) = clone_engine(1, 4);
        // The object for the node is the node itself; give it the vtable.
        e.register(0x0096_11e0, |_, _| ret(0x0400_0000));
        e.map(0x0400_0000, 4);
        e.mem.set_u32(0x0400_0000, VTABLE);
        assert_eq!(e.call(0x0050_3820, &[a, 0x5000, 0x101]).u32(), clone);
        assert_eq!(calls_to(&e, 0x0050_ee50), vec![vec![a, 0x5000, 1]]);
        assert_eq!(calls_to(&e, SLOTS + 0x9c), vec![vec![clone, 0x0a01]]);
        assert_eq!(calls_to(&e, SLOTS + 0xe8), vec![vec![0x0400_0000, 0x1a01]]);
        assert_eq!(calls_to(&e, 0x00c8_ecf0), vec![vec![clone]]);
    }

    #[test]
    fn more_arrows_pass_each_unused_numbered_node() {
        let (mut e, a, clone) = clone_engine(2, 5);
        e.register(0x0096_11e0, |_, _| ret(0x0400_0000));
        e.map(0x0400_0000, 4);
        e.mem.set_u32(0x0400_0000, VTABLE);
        assert_eq!(e.call(0x0050_3820, &[a, 0x5000, 0]).u32(), clone);
        let formats = calls_to(&e, 0x0040_6f60);
        assert_eq!(
            formats.iter().map(|w| w[2]).collect::<Vec<_>>(),
            vec![2, 3, 4]
        );
        assert!(formats.iter().all(|w| w[1] == 0x0102_62cc));
        // Node 3 is missing.
        assert_eq!(
            calls_to(&e, SLOTS + 0xe8),
            vec![vec![0x0400_0000, 0x1a02], vec![0x0400_0000, 0x1a04]]
        );
        assert_eq!(calls_to(&e, 0x0040_37b0).len(), 3);
        assert_eq!(calls_to(&e, 0x0040_37d0).len(), 3);
        assert!(calls_to(&e, 0x00c8_ecf0).is_empty());
    }

    #[test]
    fn enough_arrows_or_no_clone_change_nothing() {
        let (mut e, a, clone) = clone_engine(4, 4);
        assert_eq!(e.call(0x0050_3820, &[a, 0x5000, 0]).u32(), clone);
        assert!(calls_to(&e, SLOTS + 0x9c).is_empty());
        // Without a reference the count is 1.
        let (mut e, a, _) = clone_engine(9, 4);
        e.register(0x0096_11e0, |_, _| ret(0x0400_0000));
        e.map(0x0400_0000, 4);
        e.mem.set_u32(0x0400_0000, VTABLE);
        e.call(0x0050_3820, &[a, 0, 0]);
        assert!(calls_to(&e, 0x0041_8770).is_empty());
        assert_eq!(calls_to(&e, 0x00c8_ecf0).len(), 1);
        let (mut e, a, _) = clone_engine(1, 4);
        e.register(0x0050_ee50, |_, _| ret(0));
        assert_eq!(e.call(0x0050_3820, &[a, 0x5000, 0]).u32(), 0);
        assert!(calls_to(&e, 0x0043_d4d0).is_empty());
    }
}
