//! `Havok SDK/hkpsimplecontactconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl: allocator, the array, element size).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkpConstraintData::getConstraintInfoUtil` (`00d21a60`, cdecl).
const GET_CONSTRAINT_INFO_UTIL: u32 = 0x00d2_1a60;
/// `hkpSimpleContactConstraintAtomUtil::allocateAtom` (`00d92510`, cdecl).
const ALLOCATE_ATOM: u32 = 0x00d9_2510;
/// The collision-response callbacks of the atoms in a contact constraint's chain (thiscall on
/// the atom): begin callbacks take five words, end callbacks six. The atom type (`u16` at
/// +0) selects them: `0x17`, `0x18`, `0x1a` and `0x1c`.
const BEGIN_CALLBACK_TYPE_17: u32 = 0x00d2_1f10;
const BEGIN_CALLBACK_TYPE_18: u32 = 0x00d2_1e00;
const BEGIN_CALLBACK_TYPE_1A: u32 = 0x00d2_1f50;
const BEGIN_CALLBACK_TYPE_1C: u32 = 0x00d2_2160;
const END_CALLBACK_TYPE_17: u32 = 0x00d2_1d10;
const END_CALLBACK_TYPE_18: u32 = 0x00d2_1d00;
const END_CALLBACK_TYPE_1A: u32 = 0x00d2_1fc0;
const END_CALLBACK_TYPE_1C: u32 = 0x00d2_2260;

// Translated from 00d927b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::getConstraintInfo` (Xbox PDB): calls
/// `hkpConstraintData::getConstraintInfoUtil(atom, u16[atom + 2], info)` with the atom at +0x28.
pub fn hkp_simple_contact_constraint_data_get_constraint_info(
    e: &mut Engine,
    this: Ptr,
    info: Ptr,
) {
    let atom = e.mem.u32(this.addr() + 0x28);
    let size = u32::from(e.mem.u16(atom + 2));
    e.call(GET_CONSTRAINT_INFO_UTIL, &args![atom, size, info]);
}

// Translated from 00d927d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::getRuntimeInfo` (Xbox PDB): clears the two words of the
/// runtime info `info`. Register value first: ECX (`this`) is never read; the first stack word
/// is never read either. Returns `info` (the exe leaves it in EAX).
pub fn hkp_simple_contact_constraint_data_get_runtime_info(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    info: Ptr,
) -> Ptr {
    e.mem.set_u32(info.addr(), 0);
    e.mem.set_u32(info.addr() + 4, 0);
    info
}

// Translated from 00d927f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::getSolverResults` (Xbox PDB): answers zero. Register value
/// first: ECX (`this`) is never read; the stack word is never read either.
pub fn hkp_simple_contact_constraint_data_get_solver_results(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
) -> u32 {
    0
}

// Translated from 00d92800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::isValid` (Xbox PDB): stores `1` in the byte `valid` points
/// to. Register value first: ECX (`this`) is never read. Returns `valid` (the exe leaves it in
/// EAX).
pub fn hkp_simple_contact_constraint_data_is_valid(e: &mut Engine, _this: Ptr, valid: Ptr) -> Ptr {
    e.mem.set_u8(valid.addr(), 1);
    valid
}

// Translated from 00d92810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::getType` (Xbox PDB): answers the constraint type `0xb`.
/// Register value first: ECX (`this`) is never read.
pub fn hkp_simple_contact_constraint_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0xb
}

// Translated from 00d92820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::getSimpleConstraintContactMgr` (Xbox PDB): the contact
/// manager this data is embedded in, at `this - 0x14`.
pub fn hkp_simple_contact_constraint_data_get_simple_constraint_contact_mgr(
    _e: &mut Engine,
    this: Ptr,
) -> Ptr {
    Ptr::new(this.addr().wrapping_sub(0x14))
}

// Translated from 00d92830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::collisionResponseBeginCallback` (Xbox PDB): walks the atom
/// chain starting at the pointer at +0x10 of the object at `this + 0x24` (nothing when it is
/// null); while the atom's `u16` type is at least `0x17`, calls the begin callback of its type
/// on the atom (`0x17`: `00d21f10`, `0x18`: `00d21e00`, `0x1a`: `00d21f50`, `0x1c`:
/// `00d22160`; other types none) with the five stack words, then moves to the atom at +0x14.
pub fn hkp_simple_contact_constraint_data_collision_response_begin_callback(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
) {
    let owner = e.mem.u32(this.addr() + 0x24);
    let mut atom = e.mem.u32(owner + 0x10);
    if atom == 0 || e.mem.u16(atom) < 0x17 {
        return;
    }
    loop {
        let callback = match e.mem.u16(atom) {
            0x17 => Some(BEGIN_CALLBACK_TYPE_17),
            0x18 => Some(BEGIN_CALLBACK_TYPE_18),
            0x1a => Some(BEGIN_CALLBACK_TYPE_1A),
            0x1c => Some(BEGIN_CALLBACK_TYPE_1C),
            _ => None,
        };
        if let Some(callback) = callback {
            e.call(
                callback,
                &args![atom, word_1, word_2, word_3, word_4, word_5],
            );
        }
        atom = e.mem.u32(atom + 0x14);
        if e.mem.u16(atom) < 0x17 {
            break;
        }
    }
}

// Translated from 00d92f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::hkpSimpleContactConstraintData` (Xbox PDB, unnamed in the
/// map), `(owner, body_a, body_b)`: sets the vtable `0x010c_c934`, `1` in the `u16` at +6,
/// the empty inplace array at +0xc (data `this + 0x18`, size 0, capacity word `0x80000008`),
/// `owner` at +0x24, allocates the contact atom (`allocateAtom(1, byte[body_a + 0xd1],
/// byte[body_b + 0xd1], 0x3400 / ((sum of the bytes) * 4 + 0x34))`, the quotient as a 16-bit
/// value) into +0x28, clears its `u16` at +0x10, sets its `u16` at +0x12 to `3`, clears
/// its words +0x14 to +0x2c and stores the `u16` at atom + 2 in +0x2c. Returns `this`.
pub fn fn_00d92f20(e: &mut Engine, this: Ptr, owner: u32, body_a: u32, body_b: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x010c_c934);
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, base + 0x18);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0x8000_0008);
    e.mem.set_u32(base + 0x24, owner);
    let bytes_a = u32::from(e.mem.u8(body_a + 0xd1));
    let bytes_b = u32::from(e.mem.u8(body_b + 0xd1));
    let divisor = (bytes_b + bytes_a) * 4 + 0x34;
    let quotient = 0x3400 / divisor;
    let atom = e
        .call(
            ALLOCATE_ATOM,
            &args![1u32, bytes_a, bytes_b, quotient & 0xffff],
        )
        .u32();
    e.mem.set_u32(base + 0x28, atom);
    e.mem.set_u16(atom + 0x10, 0);
    e.mem.set_u16(atom + 0x12, 3);
    for offset in (0x14..0x30).step_by(4) {
        e.mem.set_u32(atom + offset, 0);
    }
    let size = u32::from(e.mem.u16(atom + 2));
    e.mem.set_u32(base + 0x2c, size);
    this
}

// Translated from 00d92fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::collisionResponseEndCallback` (Xbox PDB): collects the atom
/// chain (as in the begin callback, from the pointer at +0x10 of the object at `this + 0x24`;
/// nothing when it is null) into an inplace array of sixteen pointers on the stack (grown
/// through `00c90940` when full) while the atom's `u16` type is at least `0x17`, then calls
/// the end callback of each atom's type from the last atom to the first (`0x17`: `00d21d10`,
/// `0x18`: `00d21d00`, `0x1a`: `00d21fc0`, `0x1c`: `00d22260`; other types none) with
/// `(word_1, value, word_3, word_4, word_5, word_6)`, `value` through the x87 stack; the array
/// is emptied and, when its capacity word is not negative, freed through the allocator
/// (slot `0x10`, capacity masked with `0x3fffffff`, times 4).
pub fn hkp_simple_contact_constraint_data_collision_response_end_callback(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    value: f32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
) {
    let owner = e.mem.u32(this.addr() + 0x24);
    let first = e.mem.u32(owner + 0x10);
    if first == 0 {
        return;
    }
    e.with_stack(0x4c, |e, array| {
        let list = array.addr();
        e.mem.set_u32(list, list + 0xc);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, 0x8000_0010);
        let mut atom = first;
        if e.mem.u16(atom) >= 0x17 {
            loop {
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let capacity = e.mem.u32(list + 8) & 0x3fff_ffff;
                if e.mem.u32(list + 4) == capacity {
                    let allocator = e.mem.u32(router + 0x10);
                    e.call(ARRAY_RESERVE_MORE, &args![allocator, list, 4u32]);
                }
                let size = e.mem.u32(list + 4);
                let data = e.mem.u32(list);
                e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), atom);
                e.mem.set_u32(list + 4, size.wrapping_add(1));
                atom = e.mem.u32(atom + 0x14);
                if e.mem.u16(atom) < 0x17 {
                    break;
                }
            }
        }
        let mut index = e.mem.i32(list + 4).wrapping_sub(1);
        while index >= 0 {
            let data = e.mem.u32(list);
            let atom = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            let callback = match e.mem.u16(atom) {
                0x17 => Some(END_CALLBACK_TYPE_17),
                0x18 => Some(END_CALLBACK_TYPE_18),
                0x1a => Some(END_CALLBACK_TYPE_1A),
                0x1c => Some(END_CALLBACK_TYPE_1C),
                _ => None,
            };
            if let Some(callback) = callback {
                e.call(
                    callback,
                    &args![
                        atom,
                        word_1,
                        x87_quiet(value),
                        word_3,
                        word_4,
                        word_5,
                        word_6
                    ],
                );
            }
            index -= 1;
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(list + 8);
        e.mem.set_u32(list + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(list);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
    });
}

// Translated from 00d93140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDynamicsCpIdMgr::newId` (Xbox PDB): hands out an id for `value`. Looks from the last
/// entry of the byte table (data +0, size +4) down for a free entry (`0xff`) and stores `value`
/// there; without one, grows the table when it is full (`00c90940(allocator, table, 1)`),
/// appends `value` and answers the new last index. Answers the index.
pub fn hkp_dynamics_cp_id_mgr_new_id(e: &mut Engine, this: Ptr, value: u8) -> i32 {
    let base = this.addr();
    let mut index = e.mem.i32(base + 4).wrapping_sub(1);
    if index >= 0 {
        let table = e.mem.u32(base);
        loop {
            let slot = table.wrapping_add(index as u32);
            if e.mem.u8(slot) == 0xff {
                e.mem.set_u8(slot, value);
                return index;
            }
            index = index.wrapping_sub(1);
            if index < 0 {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(base + 8) & 0x3fff_ffff;
    if e.mem.u32(base + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, this, 1u32]);
    }
    let slot = e.mem.u32(base).wrapping_add(e.mem.u32(base + 4));
    e.mem.set_u8(slot, value);
    let size = e.mem.u32(base + 4).wrapping_add(1);
    e.mem.set_u32(base + 4, size);
    size.wrapping_sub(1) as i32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d9_27b0,
            hkp_simple_contact_constraint_data_get_constraint_info(Ptr, Ptr)
        ),
        entry!(0x00d9_27d0, hkp_simple_contact_constraint_data_get_runtime_info(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00d9_27f0, hkp_simple_contact_constraint_data_get_solver_results(Ptr, u32) -> u32),
        entry!(0x00d9_2800, hkp_simple_contact_constraint_data_is_valid(Ptr, Ptr) -> Ptr),
        entry!(0x00d9_2810, hkp_simple_contact_constraint_data_get_type(Ptr) -> u32),
        entry!(0x00d9_2820, hkp_simple_contact_constraint_data_get_simple_constraint_contact_mgr(Ptr) -> Ptr),
        entry!(
            0x00d9_2830,
            hkp_simple_contact_constraint_data_collision_response_begin_callback(
                Ptr, u32, u32, u32, u32, u32
            )
        ),
        entry!(0x00d9_2f20, fn_00d92f20(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x00d9_2fd0,
            hkp_simple_contact_constraint_data_collision_response_end_callback(
                Ptr, u32, f32, u32, u32, u32, u32
            )
        ),
        entry!(0x00d9_3140, hkp_dynamics_cp_id_mgr_new_id(Ptr, u8) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn new_log() -> EventLog {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// A double at `address` that logs `[address, arguments...]`.
    fn log_calls(e: &mut Engine, address: u32, log: &EventLog) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            let mut entry = vec![address];
            entry.extend_from_slice(a);
            log.borrow_mut().push(entry);
            Ret::default()
        });
    }

    #[test]
    fn the_constraint_info_comes_from_the_atom() {
        let mut e = Engine::new();
        let log = new_log();
        log_calls(&mut e, GET_CONSTRAINT_INFO_UTIL, &log);
        let this = e.mem.alloc(0x40);
        let atom = e.mem.alloc(0x20);
        e.mem.set_u16(atom + 2, 0x321);
        e.mem.set_u32(this + 0x28, atom);
        e.call(0x00d9_27b0, &args![this, 0x5555u32]);
        assert_eq!(
            *log.borrow(),
            vec![vec![GET_CONSTRAINT_INFO_UTIL, atom, 0x321, 0x5555]]
        );
    }

    #[test]
    fn the_small_virtual_functions_answer_constants() {
        let mut e = Engine::new();
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info, 7);
        e.mem.set_u32(info + 4, 8);
        let got = e.call(0x00d9_27d0, &args![0x1000u32, 1u32, info]).u32();
        assert_eq!(got, info);
        assert_eq!(e.mem.u32(info), 0);
        assert_eq!(e.mem.u32(info + 4), 0);
        assert_eq!(e.call(0x00d9_27f0, &args![0x1000u32, 1u32]).u32(), 0);
        let valid = e.mem.alloc(0x10);
        let got = e.call(0x00d9_2800, &args![0x1000u32, valid]).u32();
        assert_eq!(got, valid);
        assert_eq!(e.mem.u8(valid), 1);
        assert_eq!(e.call(0x00d9_2810, &args![0x1000u32]).u32(), 0xb);
        assert_eq!(e.call(0x00d9_2820, &args![0x1014u32]).u32(), 0x1000);
    }

    /// A data object whose owner's chain has atoms of the given types (the last one ends the
    /// chain); returns `(this, atoms)`.
    fn chain(e: &mut Engine, types: &[u16]) -> (u32, Vec<u32>) {
        let this = e.mem.alloc(0x40);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x24, owner);
        let atoms: Vec<u32> = types
            .iter()
            .map(|atom_type| {
                let atom = e.mem.alloc(0x20);
                e.mem.set_u16(atom, *atom_type);
                atom
            })
            .collect();
        for pair in atoms.windows(2) {
            e.mem.set_u32(pair[0] + 0x14, pair[1]);
        }
        e.mem.set_u32(owner + 0x10, atoms[0]);
        (this, atoms)
    }

    #[test]
    fn the_begin_callback_visits_the_chain_in_order() {
        let mut e = Engine::new();
        let log = new_log();
        for address in [
            BEGIN_CALLBACK_TYPE_17,
            BEGIN_CALLBACK_TYPE_18,
            BEGIN_CALLBACK_TYPE_1A,
            BEGIN_CALLBACK_TYPE_1C,
        ] {
            log_calls(&mut e, address, &log);
        }
        let (this, atoms) = chain(&mut e, &[0x1c, 0x19, 0x18, 0x1b, 0x17, 0x1a, 0x16]);
        e.call(0x00d9_2830, &args![this, 1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![BEGIN_CALLBACK_TYPE_1C, atoms[0], 1, 2, 3, 4, 5],
                vec![BEGIN_CALLBACK_TYPE_18, atoms[2], 1, 2, 3, 4, 5],
                vec![BEGIN_CALLBACK_TYPE_17, atoms[4], 1, 2, 3, 4, 5],
                vec![BEGIN_CALLBACK_TYPE_1A, atoms[5], 1, 2, 3, 4, 5],
            ]
        );
        // An empty chain, or one whose first atom is below 0x17, calls nothing.
        log.borrow_mut().clear();
        let (this, _) = chain(&mut e, &[0x16]);
        e.call(0x00d9_2830, &args![this, 1u32, 2u32, 3u32, 4u32, 5u32]);
        let owner = e.mem.u32(this + 0x24);
        e.mem.set_u32(owner + 0x10, 0);
        e.call(0x00d9_2830, &args![this, 1u32, 2u32, 3u32, 4u32, 5u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_constructor_allocates_the_atom_and_sizes_it_from_the_bodies() {
        let mut e = Engine::new();
        let log = new_log();
        let atom = e.mem.alloc(0x40);
        e.mem.set_u16(atom + 2, 0x222);
        e.mem.set_u32(atom + 0x2c, 0xffff_ffff);
        let allocate_log = Rc::clone(&log);
        e.register_double(ALLOCATE_ATOM, move |_, a| {
            allocate_log.borrow_mut().push(a.to_vec());
            Ret {
                eax: atom,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x40);
        let body_a = e.mem.alloc(0x100);
        let body_b = e.mem.alloc(0x100);
        e.mem.set_u8(body_a + 0xd1, 3);
        e.mem.set_u8(body_b + 0xd1, 5);
        let got = e
            .call(0x00d9_2f20, &args![this, 0x7000u32, body_a, body_b])
            .u32();
        assert_eq!(got, this);
        // 0x3400 / ((3 + 5) * 4 + 0x34) = 13312 / 84 = 158.
        assert_eq!(*log.borrow(), vec![vec![1, 3, 5, 158]]);
        assert_eq!(e.mem.u32(this), 0x010c_c934);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), this + 0x18);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0008);
        assert_eq!(e.mem.u32(this + 0x24), 0x7000);
        assert_eq!(e.mem.u32(this + 0x28), atom);
        assert_eq!(e.mem.u32(this + 0x2c), 0x222);
        assert_eq!(e.mem.u16(atom + 0x12), 3);
        assert_eq!(e.mem.u16(atom + 0x10), 0);
        assert_eq!(e.mem.u32(atom + 0x2c), 0);
    }

    const FREE_DOUBLE: u32 = 0x0900_0051;

    /// Memory router with an allocator whose slot `0x10` logs `[0x10, data, size]`.
    fn install_allocator(e: &mut Engine, log: &EventLog) -> u32 {
        let mut slots = vec![0u32; 8];
        slots[4] = FREE_DOUBLE;
        e.put_vtable(0x0900_0400, &slots);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, 0x0900_0400);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(log);
        e.register_double(FREE_DOUBLE, move |_, a| {
            free_log.borrow_mut().push(vec![0x10, a[1], a[2]]);
            Ret::default()
        });
        allocator
    }

    #[test]
    fn the_end_callback_visits_the_chain_backwards_with_the_float() {
        let mut e = Engine::new();
        let log = new_log();
        install_allocator(&mut e, &log);
        for address in [
            END_CALLBACK_TYPE_17,
            END_CALLBACK_TYPE_18,
            END_CALLBACK_TYPE_1A,
            END_CALLBACK_TYPE_1C,
        ] {
            log_calls(&mut e, address, &log);
        }
        let (this, atoms) = chain(&mut e, &[0x1c, 0x19, 0x18, 0x17, 0x1a, 0x10]);
        let signalling = f32::from_bits(0x7f80_0001);
        e.call(
            0x00d9_2fd0,
            &args![this, 1u32, signalling, 3u32, 4u32, 5u32, 6u32],
        );
        let quiet = 0x7fc0_0001;
        assert_eq!(
            *log.borrow(),
            vec![
                vec![END_CALLBACK_TYPE_1A, atoms[4], 1, quiet, 3, 4, 5, 6],
                vec![END_CALLBACK_TYPE_17, atoms[3], 1, quiet, 3, 4, 5, 6],
                vec![END_CALLBACK_TYPE_18, atoms[2], 1, quiet, 3, 4, 5, 6],
                vec![END_CALLBACK_TYPE_1C, atoms[0], 1, quiet, 3, 4, 5, 6],
            ]
        );
        // No chain: nothing is called.
        log.borrow_mut().clear();
        let owner = e.mem.u32(this + 0x24);
        e.mem.set_u32(owner + 0x10, 0);
        e.call(
            0x00d9_2fd0,
            &args![this, 1u32, 2.0f32, 3u32, 4u32, 5u32, 6u32],
        );
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn a_long_chain_grows_the_pointer_array_and_frees_it() {
        let mut e = Engine::new();
        let log = new_log();
        install_allocator(&mut e, &log);
        e.register_double(END_CALLBACK_TYPE_17, |_, _| Ret::default());
        let grown = Rc::new(RefCell::new(0u32));
        let grown_in_double = Rc::clone(&grown);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            let buffer = e.mem.alloc(0x100);
            let old = e.mem.u32(a[1]);
            for word in 0..16 {
                let value = e.mem.u32(old + 4 * word);
                e.mem.set_u32(buffer + 4 * word, value);
            }
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, 0x4000_0040);
            *grown_in_double.borrow_mut() = buffer;
            Ret::default()
        });
        let mut types = vec![0x17u16; 17];
        types.push(0x10);
        let (this, _) = chain(&mut e, &types);
        e.call(
            0x00d9_2fd0,
            &args![this, 1u32, 2.0f32, 3u32, 4u32, 5u32, 6u32],
        );
        assert_ne!(*grown.borrow(), 0);
        assert_eq!(*log.borrow(), vec![vec![0x10, *grown.borrow(), 0x100]]);
    }

    #[test]
    fn a_new_id_reuses_the_last_free_entry_or_appends() {
        let mut e = Engine::new();
        let log = new_log();
        install_allocator(&mut e, &log);
        let table = e.mem.alloc(0x10);
        for (index, value) in [1u8, 0xff, 5, 0xff].into_iter().enumerate() {
            e.mem.set_u8(table + index as u32, value);
        }
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, table);
        e.mem.set_u32(this + 4, 4);
        e.mem.set_u32(this + 8, 0x4000_0004);
        assert_eq!(e.call(0x00d9_3140, &args![this, 9u32]).i32(), 3);
        assert_eq!(e.call(0x00d9_3140, &args![this, 8u32]).i32(), 1);
        assert_eq!(e.mem.u8(table + 3), 9);
        assert_eq!(e.mem.u8(table + 1), 8);
        // Full: the table grows (through the double) and the value is appended.
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            let bigger = e.mem.alloc(0x20);
            let old = e.mem.u32(a[1]);
            for byte in 0..4 {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(bigger + byte, value);
            }
            e.mem.set_u32(a[1], bigger);
            e.mem.set_u32(a[1] + 8, 0x4000_0010);
            Ret::default()
        });
        assert_eq!(e.call(0x00d9_3140, &args![this, 7u32]).i32(), 4);
        let table = e.mem.u32(this);
        assert_eq!(e.mem.u8(table + 4), 7);
        assert_eq!(e.mem.u32(this + 4), 5);
        // Room is left: no growth call is needed for the next one.
        assert_eq!(e.call(0x00d9_3140, &args![this, 6u32]).i32(), 5);
        // An empty table appends too.
        e.mem.set_u32(this + 4, 0);
        assert_eq!(e.call(0x00d9_3140, &args![this, 2u32]).i32(), 0);
    }
}
