//! `Havok SDK/hkpconstraintatom.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d22030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpIgnoreModifierConstraintAtom::getConstraintInfo` (named by the exe's symbol table; the
/// atom type it handles, 0x1b, is the jump-table target of `00d22090`). `this` is an atom
/// (ECX); `info` is a four-word constraint-info record. Follows the chain through the word at
/// +0x14 while the type word is at least `0x17` (modifier atoms), remembering the word at
/// +0x12 of the last modifier; when the atom it ends on is not type `0x16` or `1`, asks
/// `00d21a60(atom, word, local record)` for that atom's sizes and subtracts the record's
/// words 1 to 3 from words 1 to 3 of `info`. Returns 0.
pub fn fn_00d22030(e: &mut Engine, this: Ptr, info: Ptr) -> u32 {
    let mut atom = this.addr();
    let mut word = 0u32;
    if e.mem.u16(atom) >= 0x17 {
        loop {
            word = u32::from(e.mem.u16(atom + 0x12));
            atom = e.mem.u32(atom + 0x14);
            if e.mem.u16(atom) < 0x17 {
                break;
            }
        }
    }
    let kind = e.mem.u16(atom);
    if kind != 0x16 && kind != 1 {
        e.with_stack(0x18, |e, local| {
            e.call(0x00d2_1a60, &args![atom, word, local]);
            for offset in [0xc, 8, 4] {
                let value = e.mem.u32(info.addr() + offset);
                let taken = e.mem.u32(local.addr() + offset);
                e.mem
                    .set_u32(info.addr() + offset, value.wrapping_sub(taken));
            }
        });
    }
    0
}

// Translated from 00d22090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpModifierConstraintAtom::addModifierDataToConstraintInfo` (Xbox PDB). `this` is the atom
/// (ECX), `info` a constraint-info record, `flags` points to a flag byte. By atom type word:
/// `0x17` sets flag 8; `0x18` adds `0x50` to `info` word 1 and sets flag 1; `0x19` sets flag
/// 0x10; `0x1a` adds `0x50` to word 1 and sets flag 4; `0x1b` runs `00d22030(atom, info)`; `0x1c`
/// adds `0xd0` to word 1 and sets flag 2; any other type does nothing. Returns 0 in every case.
pub fn hkp_modifier_constraint_atom_add_modifier_data_to_constraint_info(
    e: &mut Engine,
    this: Ptr,
    info: Ptr,
    flags: Ptr,
) -> u32 {
    let index = u32::from(e.mem.u16(this.addr())).wrapping_sub(0x17);
    let (add, flag) = match index {
        0 => (0, 8),
        1 => (0x50, 1),
        2 => (0, 0x10),
        3 => (0x50, 4),
        4 => return fn_00d22030(e, this, info),
        5 => (0xd0, 2),
        _ => return 0,
    };
    if add != 0 {
        let value = e.mem.u32(info.addr() + 4);
        e.mem.set_u32(info.addr() + 4, value.wrapping_add(add));
    }
    let byte = e.mem.u8(flags.addr());
    e.mem.set_u8(flags.addr(), byte | flag);
    0
}

// Translated from 00d22120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpModifierConstraintAtom::addAllModifierDataToConstraintInfo` (Xbox PDB), cdecl
/// `(atom, info, flags)`: while the atom's type word is at least `0x17` it runs
/// `addModifierDataToConstraintInfo(atom, info, flags)` and moves on to the atom at +0x14,
/// returning the OR of the results.
pub fn hkp_modifier_constraint_atom_add_all_modifier_data_to_constraint_info(
    e: &mut Engine,
    atom: Ptr,
    info: Ptr,
    flags: Ptr,
) -> u32 {
    let mut result = 0;
    let mut atom = atom.addr();
    if e.mem.u16(atom) >= 0x17 {
        loop {
            result |= hkp_modifier_constraint_atom_add_modifier_data_to_constraint_info(
                e,
                Ptr::new(atom),
                info,
                flags,
            );
            atom = e.mem.u32(atom + 0x14);
            if e.mem.u16(atom) < 0x17 {
                break;
            }
        }
    }
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_2030, fn_00d22030(Ptr, Ptr) -> u32),
        entry!(0x00d2_2090, hkp_modifier_constraint_atom_add_modifier_data_to_constraint_info(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d2_2120, hkp_modifier_constraint_atom_add_all_modifier_data_to_constraint_info(Ptr, Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    #[test]
    fn modifier_types_update_the_info_and_flags() {
        let mut e = test_engine();
        let info = e.mem.alloc(0x10);
        let flags = e.mem.alloc(4);
        let atom = e.mem.alloc(0x20);
        for (kind, add, flag) in [
            (0x17u16, 0u32, 8u8),
            (0x18, 0x50, 1),
            (0x19, 0, 0x10),
            (0x1a, 0x50, 4),
            (0x1c, 0xd0, 2),
        ] {
            e.mem.set_u32(info + 4, 7);
            e.mem.set_u8(flags, 0x40);
            e.mem.set_u16(atom, kind);
            let got = e.call(0x00d2_2090, &args![atom, info, flags]).u32();
            assert_eq!(got, 0);
            assert_eq!(e.mem.u32(info + 4), 7 + add);
            assert_eq!(e.mem.u8(flags), 0x40 | flag);
        }
        // Other types do nothing.
        e.mem.set_u8(flags, 0);
        e.mem.set_u16(atom, 0x1d);
        e.call(0x00d2_2090, &args![atom, info, flags]);
        e.mem.set_u16(atom, 3);
        e.call(0x00d2_2090, &args![atom, info, flags]);
        assert_eq!(e.mem.u8(flags), 0);
    }

    #[test]
    fn the_ignore_modifier_subtracts_the_sizes_of_the_atom_it_ends_on() {
        let mut e = test_engine();
        let calls: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&calls);
        e.register_double(0x00d2_1a60, move |e, a| {
            sink.borrow_mut().push((a[0], a[1]));
            e.mem.set_u32(a[2] + 4, 1);
            e.mem.set_u32(a[2] + 8, 2);
            e.mem.set_u32(a[2] + 0xc, 3);
            Ret::default()
        });
        let info = e.mem.alloc(0x10);
        for (i, v) in [100u32, 100, 100, 100].into_iter().enumerate() {
            e.mem.set_u32(info + 4 * i as u32, v);
        }
        let last = e.mem.alloc(0x20);
        e.mem.set_u16(last, 5);
        let modifier = e.mem.alloc(0x20);
        e.mem.set_u16(modifier, 0x1b);
        e.mem.set_u16(modifier + 0x12, 0x77);
        e.mem.set_u32(modifier + 0x14, last);
        let got = e.call(0x00d2_2030, &args![modifier, info]).u32();
        assert_eq!(got, 0);
        assert_eq!(*calls.borrow(), vec![(last, 0x77)]);
        assert_eq!(e.mem.u32(info), 100);
        assert_eq!(e.mem.u32(info + 4), 99);
        assert_eq!(e.mem.u32(info + 8), 98);
        assert_eq!(e.mem.u32(info + 0xc), 97);
        // Ending on type 0x16 or 1 asks nothing.
        calls.borrow_mut().clear();
        e.mem.set_u16(last, 0x16);
        e.call(0x00d2_2030, &args![modifier, info]);
        e.mem.set_u16(last, 1);
        e.call(0x00d2_2030, &args![modifier, info]);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn all_modifiers_walks_the_chain() {
        let mut e = test_engine();
        let info = e.mem.alloc(0x10);
        let flags = e.mem.alloc(4);
        let end = e.mem.alloc(0x20);
        e.mem.set_u16(end, 2);
        let second = e.mem.alloc(0x20);
        e.mem.set_u16(second, 0x19);
        e.mem.set_u32(second + 0x14, end);
        let first = e.mem.alloc(0x20);
        e.mem.set_u16(first, 0x17);
        e.mem.set_u32(first + 0x14, second);
        e.call(0x00d2_2120, &args![first, info, flags]);
        assert_eq!(e.mem.u8(flags), 0x18);
        // A plain atom is left alone.
        e.mem.set_u8(flags, 0);
        e.call(0x00d2_2120, &args![end, info, flags]);
        assert_eq!(e.mem.u8(flags), 0);
    }
}
