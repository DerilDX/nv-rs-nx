//! `NiMain/nishaderconstantmap.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{
    acquire_fixed_string, interlocked_increment, release_fixed_string, EMPTY_FIXED_STRING,
    GET_FIXED_STRING,
};

/// `011f6024`: the pointer to the first table (0x88 entries of 8 bytes: a number and a
/// fixed-string handle) that `00a9c2a0` builds.
const FIRST_TABLE: u32 = 0x011f_6024;
/// `011f6028`: the number of entries of the first table.
const FIRST_TABLE_COUNT: u32 = 0x011f_6028;
/// `011f602c`: the pointer to the second table (0x19 entries of 8 bytes).
const SECOND_TABLE: u32 = 0x011f_602c;
/// `011f6030`: the number of entries of the second table.
const SECOND_TABLE_COUNT: u32 = 0x011f_6030;
/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00401050` (`cdecl`: array, element size, count, constructor): constructs every element
/// of an array with the constructor.
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `00a9bdf0`: the element constructor handed to `00401050`.
const ENTRY_CONSTRUCTOR: u32 = 0x00a9_bdf0;
/// `00438170` (`thiscall`, `fixed_string_construct`): constructs a fixed-string handle from
/// a C string in the 4 bytes `this` points to and answers `this`.
const CONSTRUCT_FIXED_STRING: u32 = 0x0043_8170;

const INLINE_ENTRIES: [(u32, u32); 20] = [
    (0x0104_8858, 0),
    (0x010a_2074, 1),
    (0x010a_2068, 1),
    (0x010a_2060, 2),
    (0x010a_2050, 2),
    (0x010a_2048, 3),
    (0x010a_2040, 4),
    (0x0108_2c9c, 5),
    (0x010a_2034, 6),
    (0x010a_2028, 7),
    (0x010a_2018, 8),
    (0x010a_200c, 9),
    (0x010a_1ffc, 9),
    (0x010a_1ff0, 10),
    (0x010a_1fdc, 10),
    (0x010a_1fcc, 11),
    (0x010a_1fb8, 11),
    (0x010a_1fa4, 12),
    (0x010a_1f8c, 12),
    (0x010a_1f7c, 13),
];

const PAIR_ENTRIES: [(u32, u32); 116] = [
    (0x010a_1f68, 13),
    (0x010a_1f54, 14),
    (0x010a_1f3c, 14),
    (0x010a_1f2c, 15),
    (0x010a_1f18, 16),
    (0x010a_1f08, 17),
    (0x010a_1ef4, 18),
    (0x010a_1ee0, 19),
    (0x010a_1ec8, 20),
    (0x010a_1eb4, 21),
    (0x010a_1e9c, 21),
    (0x010a_1e84, 22),
    (0x010a_1e68, 22),
    (0x010a_1e50, 23),
    (0x010a_1e30, 23),
    (0x010a_1e14, 24),
    (0x010a_1df4, 24),
    (0x010a_1de4, 25),
    (0x010a_1dd0, 26),
    (0x010a_1db8, 27),
    (0x010a_1d9c, 28),
    (0x010a_1d88, 29),
    (0x010a_1d70, 29),
    (0x010a_1d58, 30),
    (0x010a_1d3c, 30),
    (0x010a_1d20, 31),
    (0x010a_1cfc, 31),
    (0x010a_1cdc, 32),
    (0x010a_1cb8, 32),
    (0x010a_1cac, 33),
    (0x010a_1ca0, 34),
    (0x010a_1c90, 35),
    (0x010a_1c80, 36),
    (0x010a_1c6c, 37),
    (0x010a_1c58, 38),
    (0x010a_1c3c, 39),
    (0x010a_1c1c, 40),
    (0x010a_1c08, 41),
    (0x010a_1bf4, 42),
    (0x010a_1bd8, 43),
    (0x010a_1bb8, 44),
    (0x010a_1ba4, 45),
    (0x010a_1b8c, 46),
    (0x010a_1b70, 47),
    (0x010a_1b50, 48),
    (0x010a_1b3c, 49),
    (0x010a_1b24, 50),
    (0x010a_1b08, 51),
    (0x010a_1ae8, 52),
    (0x010a_1ad4, 53),
    (0x010a_1ac0, 54),
    (0x010a_1aa4, 55),
    (0x010a_1a84, 56),
    (0x010a_1a70, 57),
    (0x010a_1a5c, 58),
    (0x010a_1a40, 59),
    (0x010a_1a20, 60),
    (0x010a_1a0c, 61),
    (0x010a_19f4, 62),
    (0x010a_19d8, 63),
    (0x010a_19b8, 64),
    (0x010a_19a4, 65),
    (0x010a_198c, 66),
    (0x010a_1970, 67),
    (0x010a_1950, 68),
    (0x010a_1944, 69),
    (0x010a_1938, 70),
    (0x010a_192c, 71),
    (0x010a_1920, 72),
    (0x010a_1910, 73),
    (0x0109_8dbc, 73),
    (0x010a_1900, 74),
    (0x0109_8dc4, 74),
    (0x010a_18ec, 75),
    (0x010a_18e0, 75),
    (0x010a_18cc, 76),
    (0x010a_18bc, 77),
    (0x010a_18ac, 77),
    (0x010a_18a4, 78),
    (0x010a_189c, 79),
    (0x010a_1890, 80),
    (0x010a_1884, 81),
    (0x010a_187c, 82),
    (0x010a_1870, 83),
    (0x010a_1864, 84),
    (0x010a_1858, 85),
    (0x010a_1844, 86),
    (0x010a_1834, 87),
    (0x010a_181c, 92),
    (0x010a_1804, 93),
    (0x010a_17e4, 94),
    (0x010a_17c0, 95),
    (0x010a_17ac, 88),
    (0x010a_1794, 89),
    (0x010a_1778, 90),
    (0x010a_1758, 91),
    (0x010a_174c, 96),
    (0x010a_173c, 97),
    (0x010a_1728, 98),
    (0x010a_1710, 99),
    (0x010a_1704, 100),
    (0x010a_16f8, 101),
    (0x010a_16ec, 102),
    (0x010a_16e0, 103),
    (0x010a_16cc, 104),
    (0x010a_16bc, 105),
    (0x010a_16b0, 106),
    (0x010a_16a4, 107),
    (0x010a_1694, 108),
    (0x010a_1684, 109),
    (0x010a_1678, 110),
    (0x010a_166c, 111),
    (0x010a_165c, 111),
    (0x010a_164c, 111),
    (0x010a_163c, 114),
    (0x010a_162c, 115),
];

const SECOND_PAIR_STRINGS: [u32; 12] = [
    0x010a_1624,
    0x010a_1614,
    0x010a_1604,
    0x010a_15f0,
    0x0109_8dc4,
    0x0109_8dbc,
    0x010a_18e0,
    0x010a_15e0,
    0x010a_15d0,
    0x010a_15c0,
    0x010a_15b0,
    0x010a_15a0,
];

const SECOND_CONSTRUCTED_STRINGS: [u32; 12] = [
    0x010a_1590,
    0x010a_1580,
    0x010a_1574,
    0x010a_155c,
    0x010a_1544,
    0x010a_1528,
    0x010a_150c,
    0x010a_14f8,
    0x010a_14e4,
    0x010a_14d8,
    0x010a_14bc,
    0x010a_14a0,
];

/// Table entry shape one (the first 20 entries of the first table): the handle for `string`
/// is made, held in a local, the entry's number is set and its handle replaced (counted),
/// and the local handles are released.
fn set_entry_inline(e: &mut Engine, slot: u32, string: u32, value: u32) {
    let result = e.call(GET_FIXED_STRING, &args![string]).u32();
    let mut local = e.mem.u32(EMPTY_FIXED_STRING);
    if local != result {
        interlocked_increment(e, result.wrapping_sub(8));
        if local != e.mem.u32(EMPTY_FIXED_STRING) {
            release_fixed_string(e, local);
        }
        local = result;
    }
    e.mem.set_u32(slot, value);
    if e.mem.u32(slot + 4) != local {
        acquire_fixed_string(e, local);
        let old = e.mem.u32(slot + 4);
        release_fixed_string(e, old);
        e.mem.set_u32(slot + 4, local);
    }
    release_fixed_string(e, local);
    release_fixed_string(e, result);
}

/// Table entry shape two: the handle for `string` is made and kept in a local, an entry is
/// made from `value` and the local's address (`00a9bda0`), assigned to the slot
/// (`00a9be30`), and the entry's handle and the local handle are released.
fn set_entry_pair(e: &mut Engine, slot: u32, string: u32, value: u32) {
    let name = e.call(GET_FIXED_STRING, &args![string]).u32();
    e.with_stack(0xc, |e, locals| {
        e.mem.set_u32(locals.addr(), name);
        let entry = locals.addr() + 4;
        let made = fn_00a9bda0(e, Ptr::new(entry), value, Ptr::new(locals.addr())).addr();
        fn_00a9be30(e, Ptr::new(slot), Ptr::new(made));
        let entry_handle = e.mem.u32(entry + 4);
        release_fixed_string(e, entry_handle);
        release_fixed_string(e, name);
    });
}

/// Table entry shape three: like shape two, but the local handle is constructed from the C
/// string by `00438170`.
fn set_entry_constructed(e: &mut Engine, slot: u32, string: u32, value: u32) {
    e.with_stack(0xc, |e, locals| {
        let text = e
            .call(CONSTRUCT_FIXED_STRING, &args![locals.addr(), string])
            .u32();
        let entry = locals.addr() + 4;
        let made = fn_00a9bda0(e, Ptr::new(entry), value, Ptr::new(text)).addr();
        fn_00a9be30(e, Ptr::new(slot), Ptr::new(made));
        let entry_handle = e.mem.u32(entry + 4);
        release_fixed_string(e, entry_handle);
        let local_handle = e.mem.u32(locals.addr());
        release_fixed_string(e, local_handle);
    });
}

/// Allocates a table of `count` 8-byte entries (a 4-byte count in front, the size the
/// exe's overflow-checked `count * 8 + 4` gives for these counts), constructs the entries
/// with `00a9bdf0`, records the count and the table in the globals and answers the table
/// (null when the allocation fails).
fn allocate_table(e: &mut Engine, count_slot: u32, table_slot: u32, count: u32) -> u32 {
    e.mem.set_u32(count_slot, count);
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, count * 8 + 4],
        )
        .u32();
    let table = if block == 0 {
        0
    } else {
        e.mem.set_u32(block, count);
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![block + 4, 8u32, count, ENTRY_CONSTRUCTOR],
        );
        block + 4
    };
    e.mem.set_u32(table_slot, table);
    table
}

// Translated from 00a9c2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::_SDMInit` (Xbox PDB; run by the start-up `00a89cc0`, no
/// arguments, 15104 bytes): builds two tables of 8-byte entries, each a number and a
/// fixed-string handle. The first (`011f6024`, 0x88 entries) is filled with the entries of
/// `INLINE_ENTRIES` (the first of them at the pointer just allocated) and `PAIR_ENTRIES`;
/// the second (`011f602c`, 0x19 entries) with the string at `01048858` (number 0), the
/// strings of `SECOND_PAIR_STRINGS` (numbers 1 to 12) and those of
/// `SECOND_CONSTRUCTED_STRINGS` (numbers 13 to 24). Every entry is written at the table
/// pointer read again from its global (the exe re-reads it), in the exe's order. The three
/// entry shapes differ only in how the handles are made and counted.
pub fn ni_shader_constant_map_sdm_init(e: &mut Engine) {
    let first = allocate_table(e, FIRST_TABLE_COUNT, FIRST_TABLE, 0x88);
    let (string, value) = INLINE_ENTRIES[0];
    set_entry_inline(e, first, string, value);
    for (index, (string, value)) in INLINE_ENTRIES.into_iter().enumerate().skip(1) {
        let slot = e.mem.u32(FIRST_TABLE) + 8 * index as u32;
        set_entry_inline(e, slot, string, value);
    }
    for (index, (string, value)) in PAIR_ENTRIES.into_iter().enumerate() {
        let slot = e.mem.u32(FIRST_TABLE) + 8 * (INLINE_ENTRIES.len() + index) as u32;
        set_entry_pair(e, slot, string, value);
    }
    let second = allocate_table(e, SECOND_TABLE_COUNT, SECOND_TABLE, 0x19);
    set_entry_pair(e, second, 0x0104_8858, 0);
    for (index, string) in SECOND_PAIR_STRINGS.into_iter().enumerate() {
        let slot = e.mem.u32(SECOND_TABLE) + 8 * (index as u32 + 1);
        set_entry_pair(e, slot, string, index as u32 + 1);
    }
    for (index, string) in SECOND_CONSTRUCTED_STRINGS.into_iter().enumerate() {
        let number = SECOND_PAIR_STRINGS.len() as u32 + 1 + index as u32;
        let slot = e.mem.u32(SECOND_TABLE) + 8 * number;
        set_entry_constructed(e, slot, string, number);
    }
}

// Translated from 00a9bda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nishaderconstantmap.cpp`; `thiscall`, `RET 8`):
/// constructs a table entry in `this` from `value` and the address `name` of a
/// fixed-string handle: the handle starts as the empty string's, the number is `value`,
/// and when the handle at `name` differs it is counted and replaces the empty one. Returns
/// `this`.
pub fn fn_00a9bda0(e: &mut Engine, this: Ptr, value: u32, name: Ptr) -> Ptr {
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(this.addr() + 4, empty);
    e.mem.set_u32(this.addr(), value);
    let handle = e.mem.u32(name.addr());
    if e.mem.u32(this.addr() + 4) != handle {
        acquire_fixed_string(e, handle);
        let current = e.mem.u32(this.addr() + 4);
        release_fixed_string(e, current);
        e.mem.set_u32(this.addr() + 4, handle);
    }
    this
}

// Translated from 00a9be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nishaderconstantmap.cpp`; `thiscall`, `RET 4`):
/// assigns the table entry `entry` to `this`: the number is copied, and when the handle
/// differs it is counted and replaces the old one (released). Returns `this`.
pub fn fn_00a9be30(e: &mut Engine, this: Ptr, entry: Ptr) -> Ptr {
    let value = e.mem.u32(entry.addr());
    e.mem.set_u32(this.addr(), value);
    let handle = e.mem.u32(entry.addr() + 4);
    if e.mem.u32(this.addr() + 4) != handle {
        acquire_fixed_string(e, handle);
        let current = e.mem.u32(this.addr() + 4);
        release_fixed_string(e, current);
        e.mem.set_u32(this.addr() + 4, handle);
    }
    this
}

// Translated from 00a9bdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the element constructor handed to `00401050` by
/// `ni_shader_constant_map_sdm_init`; `thiscall`, no stack word): the entry's handle
/// starts as the empty string's and its number as 0; since the handle is not 0, it is
/// replaced by what `00a5b690` answers for a null string and the old handle released.
/// Returns `this`.
pub fn fn_00a9bdf0(e: &mut Engine, this: Ptr) -> Ptr {
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(this.addr() + 4, empty);
    e.mem.set_u32(this.addr(), 0);
    let old = e.mem.u32(this.addr() + 4);
    if old != 0 {
        let handle = e.call(GET_FIXED_STRING, &args![0u32]).u32();
        e.mem.set_u32(this.addr() + 4, handle);
        release_fixed_string(e, old);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_c2a0, ni_shader_constant_map_sdm_init()),
        entry!(0x00a9_bda0, fn_00a9bda0(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00a9_bdf0, fn_00a9bdf0(Ptr) -> Ptr),
        entry!(0x00a9_be30, fn_00a9be30(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    type Handles = Rc<RefCell<HashMap<u32, u32>>>;

    /// The doubles for everything outside the unit: fixed strings are interned (count at
    /// `-8`), `00a9bda0` and `00a9be30` keep the counts the way the real ones must.
    fn engine() -> (Engine, Handles) {
        let mut e = engine_with_counting();
        let handles: Handles = Rc::default();
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |e, a| {
            let empty = e.mem.u32(EMPTY_FIXED_STRING);
            for index in 0..a[2] {
                e.mem.set_u32(a[0] + 8 * index + 4, empty);
            }
            Ret::default()
        });
        let table = handles.clone();
        e.register_double(GET_FIXED_STRING, move |e, a| {
            let handle = intern(e, &table, a[0]);
            e.mem.set_u32(handle - 8, e.mem.u32(handle - 8) + 1);
            handle.into_ret()
        });
        let table = handles.clone();
        e.register_double(CONSTRUCT_FIXED_STRING, move |e, a| {
            let handle = intern(e, &table, a[1]);
            e.mem.set_u32(handle - 8, e.mem.u32(handle - 8) + 1);
            e.mem.set_u32(a[0], handle);
            a[0].into_ret()
        });
        (e, handles)
    }

    fn intern(e: &mut Engine, table: &Handles, string: u32) -> u32 {
        if let Some(handle) = table.borrow().get(&string) {
            return *handle;
        }
        let handle = e.mem.alloc(0x20) + 8;
        table.borrow_mut().insert(string, handle);
        handle
    }

    #[test]
    fn builds_both_tables_with_balanced_counts() {
        let (mut e, handles) = engine();
        e.call_log = Some(vec![]);
        e.call(0x00a9_c2a0, &args![]);
        // 136 + 13 strings looked up, 12 constructed.
        assert_eq!(calls_to(&e, GET_FIXED_STRING).len(), 149);
        assert_eq!(calls_to(&e, CONSTRUCT_FIXED_STRING).len(), 12);
        let first = e.mem.u32(FIRST_TABLE);
        let second = e.mem.u32(SECOND_TABLE);
        assert_eq!(e.mem.u32(FIRST_TABLE_COUNT), 0x88);
        assert_eq!(e.mem.u32(SECOND_TABLE_COUNT), 0x19);
        assert_eq!(e.mem.u32(first - 4), 0x88);
        assert_eq!(e.mem.u32(second - 4), 0x19);
        let vector_calls = calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR);
        assert_eq!(
            vector_calls,
            vec![
                vec![first, 8, 0x88, ENTRY_CONSTRUCTOR],
                vec![second, 8, 0x19, ENTRY_CONSTRUCTOR]
            ]
        );
        let handle_of = |string: u32| handles.borrow()[&string];
        // Entries of the three shapes at their offsets.
        assert_eq!(e.mem.u32(first), 0);
        assert_eq!(e.mem.u32(first + 4), handle_of(0x0104_8858));
        for (index, (string, value)) in INLINE_ENTRIES.into_iter().enumerate() {
            assert_eq!(e.mem.u32(first + 8 * index as u32), value);
            assert_eq!(e.mem.u32(first + 8 * index as u32 + 4), handle_of(string));
        }
        for (index, (string, value)) in PAIR_ENTRIES.into_iter().enumerate() {
            let slot = first + 8 * (20 + index as u32);
            assert_eq!(e.mem.u32(slot), value);
            assert_eq!(e.mem.u32(slot + 4), handle_of(string));
        }
        assert_eq!(e.mem.u32(second), 0);
        assert_eq!(e.mem.u32(second + 4), handle_of(0x0104_8858));
        for (index, string) in SECOND_PAIR_STRINGS.into_iter().enumerate() {
            let slot = second + 8 * (index as u32 + 1);
            assert_eq!(e.mem.u32(slot), index as u32 + 1);
            assert_eq!(e.mem.u32(slot + 4), handle_of(string));
        }
        for (index, string) in SECOND_CONSTRUCTED_STRINGS.into_iter().enumerate() {
            let slot = second + 8 * (13 + index as u32);
            assert_eq!(e.mem.u32(slot), 13 + index as u32);
            assert_eq!(e.mem.u32(slot + 4), handle_of(string));
        }
        // Every handle ends up counted once per table slot that holds it.
        let mut holders: HashMap<u32, u32> = HashMap::new();
        for index in 0..0x88u32 {
            *holders.entry(e.mem.u32(first + 8 * index + 4)).or_default() += 1;
        }
        for index in 0..0x19u32 {
            *holders
                .entry(e.mem.u32(second + 8 * index + 4))
                .or_default() += 1;
        }
        for handle in handles.borrow().values() {
            assert_eq!(e.mem.u32(handle - 8), holders[handle], "{handle:#x}");
        }
    }
}

#[cfg(test)]
mod entry_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn made_entry_counts_a_new_handle_and_keeps_the_empty_one_uncounted() {
        let mut e = engine_with_counting();
        let entry = e.mem.alloc(8);
        let name = e.mem.alloc(4);
        let handle = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(name, handle);
        assert_eq!(
            e.call(0x00a9_bda0, &args![entry, 0x42u32, name]).u32(),
            entry
        );
        assert_eq!(e.mem.u32(entry), 0x42);
        assert_eq!(e.mem.u32(entry + 4), handle);
        assert_eq!(e.mem.u32(handle - 8), 1);
        // The empty handle stays uncounted and the entry holds it as is.
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        e.mem.set_u32(name, empty);
        e.call(0x00a9_bda0, &args![entry, 7u32, name]);
        assert_eq!(e.mem.u32(entry), 7);
        assert_eq!(e.mem.u32(entry + 4), empty);
    }

    #[test]
    fn assigned_entry_replaces_and_recounts_the_handle() {
        let mut e = engine_with_counting();
        let old = e.mem.alloc(0x20) + 8;
        let new = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(old - 8, 1);
        let slot = e.mem.alloc(8);
        e.mem.set_u32(slot + 4, old);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 9);
        e.mem.set_u32(entry + 4, new);
        assert_eq!(e.call(0x00a9_be30, &args![slot, entry]).u32(), slot);
        assert_eq!(e.mem.u32(slot), 9);
        assert_eq!(e.mem.u32(slot + 4), new);
        assert_eq!(e.mem.u32(new - 8), 1);
        assert_eq!(e.mem.u32(old - 8), 0);
        // The same handle again: nothing is counted.
        e.call(0x00a9_be30, &args![slot, entry]);
        assert_eq!(e.mem.u32(new - 8), 1);
    }

    #[test]
    fn element_constructor_replaces_the_empty_handle_with_the_null_string_handle() {
        let mut e = engine_with_counting();
        let handle = e.mem.alloc(0x20) + 8;
        e.register_double(GET_FIXED_STRING, move |_, a| {
            assert_eq!(a, &[0]);
            handle.into_ret()
        });
        let entry = e.mem.alloc(8);
        e.mem.write(entry, &[0xcc; 8]);
        assert_eq!(e.call(0x00a9_bdf0, &args![entry]).u32(), entry);
        assert_eq!(e.mem.u32(entry), 0);
        assert_eq!(e.mem.u32(entry + 4), handle);
    }
}
