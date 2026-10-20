//! `fallout shared/run_004fa1a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Entries of 0x10 bytes (a `char` priority at +0, a flag byte at +1, an
//! owned C string at +4, a member constructed by `00488fd0` at +8) kept in
//! a sorted list at +0x1C of their owner, and a record of floats cleared by
//! its constructor.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401000` (`cdecl`, one word): allocates a block.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x0040_1030;
/// `0044a670` (`cdecl`): the length of a C string.
const STRING_LENGTH: u32 = 0x0044_a670;
/// `00406d30` (`cdecl`: destination, size, source): copies a C string.
const STRING_COPY: u32 = 0x0040_6d30;
/// `00408b20` (`cdecl`): compares two C strings.
const STRING_COMPARE: u32 = 0x0040_8b20;
/// `BSSimpleList<ActiveEffect_P>::Insert` (engine map), `007a7eb0`
/// (`thiscall`, `RET 8`: item, compare function): a sorted insert.
const LIST_INSERT_SORTED: u32 = 0x007a_7eb0;

// Translated from 004fa4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Orders two entries (`cdecl`; no Xbox PDB name; the compare function
/// [`fn_004fa610`] inserts with): the higher priority (signed `char` at +0)
/// first (-1 when `a`'s is higher, 1 when lower), otherwise the comparison
/// of their strings (+4, `00408b20`).
pub fn fn_004fa4b0(e: &mut Engine, a: Ptr, b: Ptr) -> i32 {
    let pa = e.mem.i8(a.addr());
    let pb = e.mem.i8(b.addr());
    if pa > pb {
        -1
    } else if pa < pb {
        1
    } else {
        let sa = e.mem.u32(a.addr() + 4);
        let sb = e.mem.u32(b.addr() + 4);
        e.call(STRING_COMPARE, &args![sa, sb]).i32()
    }
}

// Translated from 004fa500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fa500 - the C++ exception unwinding states (FS:[0] frame, handler 00f0b23b) are not translated
/// Adds an entry (no Xbox PDB name; `thiscall`, `RET 0xc`): nothing (null)
/// for a null or empty `name` or a zero `priority`; otherwise a new
/// 0x10-byte entry ([`fn_004fa5e0`]; null when the allocation fails, and
/// then the stores below write at low addresses as in the exe) gets the
/// priority, a copy of the name ([`fn_004fa670`]) and the flag, and goes
/// into the list ([`fn_004fa610`]). Returns the entry.
pub fn fn_004fa500(e: &mut Engine, this: Ptr, priority: i8, name: u32, flag: u8) -> u32 {
    if name == 0 || priority == 0 {
        return 0;
    }
    if e.call(STRING_LENGTH, &args![name]).u32() == 0 {
        return 0;
    }
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    let entry = if block == 0 {
        0
    } else {
        fn_004fa5e0(e, Ptr::new(block)).addr()
    };
    e.mem.set_i8(entry, priority);
    fn_004fa670(e, Ptr::new(entry), name);
    e.mem.set_u8(entry.wrapping_add(1), flag);
    fn_004fa610(e, this, Ptr::new(entry));
    entry
}

// Translated from 004fa5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entry constructor (no Xbox PDB name): priority 10, flag 0, no
/// string, the member at +8 constructed (`00488fd0`). Returns `this`.
pub fn fn_004fa5e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 10);
    e.mem.set_u8(this.addr() + 1, 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.call(0x0048_8fd0, &args![this.addr() + 8]);
    this
}

// Translated from 004fa610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts an entry into the sorted list at +0x1C (no Xbox PDB name;
/// `RET 4`): only an entry with a nonzero priority and a non-empty string,
/// ordered by [`fn_004fa4b0`]; answers whether it was inserted.
pub fn fn_004fa610(e: &mut Engine, this: Ptr, entry: Ptr) -> bool {
    if entry.addr() == 0 || e.mem.i8(entry.addr()) == 0 || e.mem.u32(entry.addr() + 4) == 0 {
        return false;
    }
    let name = e.mem.u32(entry.addr() + 4);
    if e.call(STRING_LENGTH, &args![name]).u32() == 0 {
        return false;
    }
    e.call(
        LIST_INSERT_SORTED,
        &args![this.addr() + 0x1c, entry, 0x004f_a4b0u32],
    );
    true
}

// Translated from 004fa670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the entry's string (no Xbox PDB name; `RET 4`): the old one is
/// freed, and a non-null `name` is copied into a new block of its length
/// plus one (`00406d30`).
pub fn fn_004fa670(e: &mut Engine, this: Ptr, name: u32) {
    let old = e.mem.u32(this.addr() + 4);
    if old != 0 {
        e.call(FREE, &args![old]);
    }
    e.mem.set_u32(this.addr() + 4, 0);
    if name != 0 {
        let size = e.call(STRING_LENGTH, &args![name]).u32().wrapping_add(1);
        let block = e.call(ALLOCATE, &args![size]).u32();
        e.mem.set_u32(this.addr() + 4, block);
        let destination = e.mem.u32(this.addr() + 4);
        e.call(STRING_COPY, &args![destination, size, name]);
    }
}

// Translated from 004fa970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands every entry with a non-empty string to the object at `011c3b3c`
/// (no Xbox PDB name; `this` is a subobject at +0x18 of the owner whose
/// list `00409140` walks; `RET 8`): `004442c0(string, entry + 8, a, b, 1,
/// 1, 0, 0)` for each.
pub fn fn_004fa970(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    let mut node = e
        .call(0x0040_9140, &args![this.addr().wrapping_sub(0x18)])
        .u32();
    while node != 0 {
        let item_address = e.call(0x0068_15c0, &args![node]).u32();
        let entry = e.mem.u32(item_address);
        if entry != 0 && e.mem.u32(entry + 4) != 0 {
            let name = e.mem.u32(entry + 4);
            if e.call(STRING_LENGTH, &args![name]).u32() != 0 {
                let target = e.mem.u32(0x011c_3b3c);
                let name = e.mem.u32(entry + 4);
                e.call(
                    0x0044_42c0,
                    &args![target, name, entry + 8, a, b, 1u32, 1u32, 0u32, 0u32],
                );
            }
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
}

// Translated from 004faa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the float record (no Xbox PDB name): [`fn_004faa20`].
/// Returns `this`.
pub fn fn_004faa00(e: &mut Engine, this: Ptr) -> Ptr {
    fn_004faa20(e, this);
    this
}

// Translated from 004faa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the float record (no Xbox PDB name): the floats at +0, +4, +8,
/// +0x24, +0x28, +0x2C and +0x18 become 0.0, the words at +0xC, +0x10,
/// +0x14, +0x1C and +0x20 become 0, and +0x30 becomes 1, in the exe's
/// order.
pub fn fn_004faa20(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let zero = x87_store32(0.0);
    for offset in [0, 4, 8, 0x24, 0x28, 0x2c] {
        e.mem.set_u32(base + offset, zero);
    }
    for offset in [0xc, 0x10, 0x14] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x18, zero);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x30, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004fa4b0, fn_004fa4b0(Ptr, Ptr) -> i32),
        entry!(0x004fa500, fn_004fa500(Ptr, i8, u32, u8) -> u32),
        entry!(0x004fa5e0, fn_004fa5e0(Ptr) -> Ptr),
        entry!(0x004fa610, fn_004fa610(Ptr, Ptr) -> bool),
        entry!(0x004fa670, fn_004fa670(Ptr, u32)),
        entry!(0x004fa970, fn_004fa970(Ptr, u32, u32)),
        entry!(0x004faa00, fn_004faa00(Ptr) -> Ptr),
        entry!(0x004faa20, fn_004faa20(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Doubles for the CRT string helpers and the allocator, working on the
    /// engine's memory.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(STRING_LENGTH, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(FREE, |_, _| Ret::default());
        e.register(STRING_COPY, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        e.register(0x0048_8fd0, |_, _| Ret::default());
        e.register(LIST_INSERT_SORTED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(p, s);
        p
    }

    #[test]
    fn entries_order_by_priority_then_name() {
        let mut e = engine();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_i8(a, 5);
        e.mem.set_i8(b, -3);
        assert_eq!(e.call(0x004f_a4b0, &args![a, b]).i32(), -1);
        assert_eq!(e.call(0x004f_a4b0, &args![b, a]).i32(), 1);
        e.mem.set_i8(b, 5);
        e.mem.set_u32(a + 4, 0x111);
        e.mem.set_u32(b + 4, 0x222);
        e.register(STRING_COMPARE, |_, a| Ret {
            eax: (a[0] as i32 - a[1] as i32) as u32,
            ..Ret::default()
        });
        assert_eq!(e.call(0x004f_a4b0, &args![a, b]).i32(), -0x111);
    }

    #[test]
    fn add_builds_and_inserts_an_entry() {
        let mut e = engine();
        let owner = e.mem.alloc(0x20);
        let name = text(&mut e, b"Fx");
        assert_eq!(
            e.call(0x004f_a500, &args![owner, 0u32, name, 1u32]).u32(),
            0
        );
        assert_eq!(
            e.call(0x004f_a500, &args![owner, 3u32, 0u32, 1u32]).u32(),
            0
        );
        let empty = text(&mut e, b"");
        assert_eq!(
            e.call(0x004f_a500, &args![owner, 3u32, empty, 1u32]).u32(),
            0
        );
        assert!(calls(&e, ALLOCATE).is_empty());
        let entry = e.call(0x004f_a500, &args![owner, 3u32, name, 1u32]).u32();
        assert_ne!(entry, 0);
        assert_eq!((e.mem.i8(entry), e.mem.u8(entry + 1)), (3, 1));
        let copy = e.mem.u32(entry + 4);
        assert_ne!(copy, name);
        assert_eq!(e.mem.cstr(copy), b"Fx".to_vec());
        assert_eq!(calls(&e, 0x0048_8fd0), vec![vec![entry + 8]]);
        assert_eq!(
            calls(&e, LIST_INSERT_SORTED),
            vec![vec![owner + 0x1c, entry, 0x004f_a4b0]]
        );
    }

    #[test]
    fn insert_needs_a_priority_and_a_string() {
        let mut e = engine();
        let entry = e.mem.alloc(0x10);
        assert!(!e.call(0x004f_a610, &args![0x3000u32, 0u32]).bool());
        assert!(!e.call(0x004f_a610, &args![0x3000u32, entry]).bool());
        e.mem.set_i8(entry, 1);
        assert!(!e.call(0x004f_a610, &args![0x3000u32, entry]).bool());
        let empty = text(&mut e, b"");
        e.mem.set_u32(entry + 4, empty);
        assert!(!e.call(0x004f_a610, &args![0x3000u32, entry]).bool());
        let name = text(&mut e, b"a");
        e.mem.set_u32(entry + 4, name);
        assert!(e.call(0x004f_a610, &args![0x3000u32, entry]).bool());
        assert_eq!(calls(&e, LIST_INSERT_SORTED).len(), 1);
    }

    #[test]
    fn set_string_frees_the_old_copy() {
        let mut e = engine();
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 4, 0x7777);
        e.call(0x004f_a670, &args![entry, 0u32]);
        assert_eq!(calls(&e, FREE), vec![vec![0x7777]]);
        assert_eq!(e.mem.u32(entry + 4), 0);
        let name = text(&mut e, b"abc");
        e.call(0x004f_a670, &args![entry, name]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![4]]);
        assert_eq!(e.mem.cstr(e.mem.u32(entry + 4)), b"abc".to_vec());
    }

    #[test]
    fn entries_with_strings_go_to_the_handler() {
        let mut e = engine();
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(0x011c_3b3c, 0x8800);
        let named = e.mem.alloc(0x10);
        let name = text(&mut e, b"n");
        e.mem.set_u32(named + 4, name);
        let unnamed = e.mem.alloc(0x10);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, unnamed);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, named);
        e.mem.set_u32(first + 4, second);
        e.register_double(0x0040_9140, move |_, _| Ret {
            eax: first,
            ..Ret::default()
        });
        e.register(0x0068_15c0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x0072_6070, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(0x0044_42c0, |_, _| Ret::default());
        e.call(0x004f_a970, &args![0x5018u32, 7u32, 9u32]);
        assert_eq!(calls(&e, 0x0040_9140), vec![vec![0x5000]]);
        assert_eq!(
            calls(&e, 0x0044_42c0),
            vec![vec![0x8800, name, named + 8, 7, 9, 1, 1, 0, 0]]
        );
    }

    #[test]
    fn record_constructor_clears_floats_and_sets_the_last_word() {
        let mut e = engine();
        let this = e.mem.alloc(0x34);
        e.mem.write(this, &[0xff; 0x34]);
        assert_eq!(e.call(0x004f_aa00, &args![this]).u32(), this);
        for offset in (0..0x30).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x30), 1);
    }
}
