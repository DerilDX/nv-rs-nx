//! `NiMain/nitexturingproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a6ab90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a6a0b0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x30 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_texturing_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a6_a0b0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, set_literal_fixed_string, ALLOCATE};

/// `011f49ac`: the pointer `00a6b410` stores the object it builds in.
const TEXTURING_PROPERTY_SLOT: u32 = 0x011f_49ac;
/// `00a6aa40` (`thiscall`, unit `nitexturingproperty.cpp`): constructs the object in the
/// block and answers it.
const TEXTURING_PROPERTY_CONSTRUCTOR: u32 = 0x00a6_aa40;
/// The static fixed strings `00a6b410` sets: (global, address of the literal), in order.
const TEXTURE_SLOT_NAMES: [(u32, u32); 10] = [
    (0x011f_49b0, 0x0108_03dc),
    (0x011f_49b4, 0x0103_c800),
    (0x011f_49b8, 0x0109_c5e4),
    (0x011f_49bc, 0x0109_c5dc),
    (0x011f_49c0, 0x0109_c5d4),
    (0x011f_49c4, 0x0109_c5cc),
    (0x011f_49c8, 0x0101_c29c),
    (0x011f_49cc, 0x0109_c5c0),
    (0x011f_49d0, 0x0101_e47c),
    (0x011f_49d4, 0x0109_c5b8),
];

// Translated from 00a6b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::_SDMInit` (Xbox PDB; run by the start-up `00a89cc0`, no
/// arguments): allocates 0x30 bytes (`00aa13e0`), constructs a texturing property in them
/// with `00a6aa40` and stores the object it answers, counted, in `011f49ac`, releasing the
/// object held there before (a failed allocation stores null); then sets ten static fixed
/// strings from literals (`TEXTURE_SLOT_NAMES`), each only when the global does not hold
/// the literal's address already.
pub fn ni_texturing_property_sdm_init(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x30u32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(TEXTURING_PROPERTY_CONSTRUCTOR, &args![block]).u32()
    };
    assign_ref_counted(e, TEXTURING_PROPERTY_SLOT, object);
    for (slot, literal) in TEXTURE_SLOT_NAMES {
        set_literal_fixed_string(e, slot, literal);
    }
}

/// `0096ae90` (`thiscall`, `RET 8`, no unit in the engine map): called with the part at
/// `+0x1c`, an index and the address of a word holding 0.
const SET_ELEMENT: u32 = 0x0096_ae90;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;

// Translated from 00a6aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitexturingproperty.cpp`; the body of a constructor,
/// `thiscall`, no stack word): runs the base constructor `00a5c200`, sets the vtable
/// `0109c514` and the part at `this + 0x1c` (vtable `0109c394`, then `0109c5ac`, count
/// words 9, 0, 0 and 1, a 0x24-byte buffer from `00aa1070` at `+0x20`), clears the word at
/// `+0x2c`, calls `0096ae90` for the indexes 0 to 8 with a word holding 0, then sets the
/// flag word at `+0x18`: bit 0 when one of the entries of the buffer from index 1 up to
/// the count at `+0x26` is non-zero, bit 2 always, and bits 1 and 3 cleared.
pub fn fn_00a6aa40(e: &mut Engine, this: Ptr) -> Ptr {
    let object = this.addr();
    e.call(
        crate::units::nimain::nimainsdm::OBJECT_BASE_CONSTRUCTOR,
        &args![this],
    );
    e.mem.set_u32(object, 0x0109_c514);
    let part = object + 0x1c;
    e.mem.set_u16(object + 0x18, 0);
    e.mem.set_u16(part + 8, 9);
    e.mem.set_u32(part, 0x0109_c394);
    e.mem.set_u16(part + 0xe, 1);
    e.mem.set_u16(part + 0xa, 0);
    e.mem.set_u16(part + 0xc, 0);
    let buffer = e.call(ALLOCATE_BLOCK, &args![0x24u32]).u32();
    e.mem.set_u32(part + 4, buffer);
    e.mem.set_u32(part, 0x0109_c5ac);
    e.mem.set_u32(object + 0x2c, 0);
    for index in 0..9u32 {
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), 0);
            e.call(SET_ELEMENT, &args![part, index, local]);
        });
    }
    let mut flags = u32::from(e.mem.u16(object + 0x18) & 0xf00f);
    e.mem.set_u16(object + 0x18, flags as u16);
    flags = u32::from(e.mem.u16(object + 0x18));
    let count = u32::from(e.mem.u16(object + 0x26));
    let mut found = false;
    if count > 1 {
        let mut slot = e.mem.u32(object + 0x20).wrapping_add(4);
        let mut index = 1u32;
        loop {
            if e.mem.u32(slot) != 0 {
                found = true;
                break;
            }
            index += 1;
            slot = slot.wrapping_add(4);
            if index >= count {
                break;
            }
        }
    }
    if found {
        flags |= 1;
    } else {
        flags &= 0xfffe;
    }
    let word = (flags as u16 & 0xfff5) | 4;
    e.mem.set_u16(object + 0x18, flags as u16);
    e.mem.set_u16(object + 0x18, word);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6ab90, ni_texturing_property_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a6_b410, ni_texturing_property_sdm_init()),
        entry!(0x00a6_aa40, fn_00a6aa40(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_texturing_property_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a6_a0b0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a6ab90, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a6ab90, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x30]]);
        assert_eq!(calls_to(&e, 0x00a6_a0b0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_and_sets_the_ten_static_strings() {
        let run = run_default_instance(0x00a6_b410, TEXTURING_PROPERTY_SLOT, |e| {
            e.register(TEXTURING_PROPERTY_CONSTRUCTOR, |_, a| a[0].into_ret());
            e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
            let empty = e.mem.u32(EMPTY_FIXED_STRING);
            for (slot, _) in TEXTURE_SLOT_NAMES {
                e.mem.set_u32(slot, empty);
            }
            // The fourth global already holds its literal's address.
            e.mem.set_u32(0x011f_49bc, 0x0109_c5dc);
        });
        assert_ne!(run.object, 0);
        assert_eq!(run.engine.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
        for (slot, literal) in TEXTURE_SLOT_NAMES {
            let expected = if slot == 0x011f_49bc {
                literal
            } else {
                literal + 0x1000
            };
            assert_eq!(run.engine.mem.u32(slot), expected);
        }
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a6_b410, TEXTURING_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
            e.register(GET_FIXED_STRING, |_, a| a[0].into_ret());
            let empty = e.mem.u32(EMPTY_FIXED_STRING);
            for (slot, _) in TEXTURE_SLOT_NAMES {
                e.mem.set_u32(slot, empty);
            }
        });
        assert_eq!(run.object, 0);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    /// Runs the constructor with `0096ae90` setting the count word to `count` and the
    /// buffer entries to `entries` (from index 1).
    fn construct(count: u16, entries: &[u32]) -> (Engine, u32, Vec<Vec<u32>>) {
        let mut e = engine_with_counting();
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xcc; 0x40]);
        let entries = entries.to_vec();
        e.register_double(SET_ELEMENT, move |e, a| {
            if a[1] == 8 {
                // The last call: the count and entries become visible.
                e.mem.set_u16(a[0] + 0xa, count);
                let buffer = e.mem.u32(a[0] + 4);
                for (index, value) in entries.iter().enumerate() {
                    e.mem.set_u32(buffer + 4 * (index as u32 + 1), *value);
                }
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_aa40, &args![this]).u32(), this);
        let calls = calls_to(&e, SET_ELEMENT);
        (e, this, calls)
    }

    #[test]
    fn sets_the_part_and_calls_the_element_setter_for_nine_indexes() {
        let (e, this, calls) = construct(1, &[]);
        assert_eq!(calls_to(&e, OBJECT_BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x0109_c514);
        assert_eq!(e.mem.u32(this + 0x1c), 0x0109_c5ac);
        assert_eq!(e.mem.u16(this + 0x1c + 8), 9);
        assert_eq!(e.mem.u16(this + 0x1c + 0xe), 1);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        let indexes: Vec<u32> = calls.iter().map(|c| c[1]).collect();
        assert_eq!(indexes, (0..9).collect::<Vec<u32>>());
        assert!(calls.iter().all(|c| c[0] == this + 0x1c));
    }

    #[test]
    fn flag_word_follows_the_buffer_entries() {
        // count 1: nothing to scan: bit 0 cleared, bit 2 set.
        let (e, this, _) = construct(1, &[]);
        assert_eq!(e.mem.u16(this + 0x18), 0x0004);
        // count 3, all entries zero: same.
        let (e, this, _) = construct(3, &[0, 0]);
        assert_eq!(e.mem.u16(this + 0x18), 0x0004);
        // count 3, the second entry set: bit 0 set.
        let (e, this, _) = construct(3, &[0, 7]);
        assert_eq!(e.mem.u16(this + 0x18), 0x0005);
    }
}
