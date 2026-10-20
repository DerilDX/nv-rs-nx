//! `fallout/ai/run_00979510` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CombatAction`'s vtable (`0108cab8`, named by its RTTI locator).
const COMBAT_ACTION_VTABLE: u32 = 0x0108_cab8;
/// The list head constructor (`00680890`, `thiscall`).
const LIST_CONSTRUCT: u32 = 0x0068_0890;
/// The 12-byte table indexed by the action kind; its first word goes to
/// +0x28 (`011a4288`).
const KIND_TABLE: u32 = 0x011a_4288;
/// Returns the address of a list head's item (`006815c0`, `thiscall`).
const NODE_ITEM: u32 = 0x0068_15c0;
/// Removes the head of a list (`0063f7b0`, `thiscall`).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The list destructor (`007b9000`, `thiscall`).
const LIST_DESTRUCT: u32 = 0x007b_9000;
/// `operator delete` (`00401030`, `cdecl(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The four list heads of a `CombatAction`.
const LISTS: [u32; 4] = [0x04, 0x0c, 0x14, 0x1c];

// Translated from 009795c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float at +0xD4 of `this` (`FLD`/`FSTP`, so a signalling NaN is
/// stored quiet).
pub fn fn_009795c0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0xd4, x87_quiet_bits(value.to_bits()));
}

// Translated from 00979bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00979bd0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatAction` constructor (by the vtable it sets, `0108cab8`): the four
/// list heads at +0x04, +0x0C, +0x14, +0x1C (`00680890`), `kind` at +0x24
/// and the first word of entry `kind` of the 12-byte table at `011a4288` at
/// +0x28. Returns `this`.
pub fn fn_00979bd0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, COMBAT_ACTION_VTABLE);
    for offset in LISTS {
        e.call(LIST_CONSTRUCT, &args![base + offset]);
    }
    e.mem.set_u32(base + 0x24, kind);
    let index = e.mem.u32(base + 0x24);
    let word = e.mem.u32(index.wrapping_mul(0xc).wrapping_add(KIND_TABLE));
    e.mem.set_u32(base + 0x28, word);
    this
}

/// Frees the entry at the head of the list at `list` and removes the head
/// until the list is empty.
fn drain(e: &mut Engine, list: u32) {
    // The exe tests `list != 0` before every step; the address never
    // changes, so one test is the same.
    if list == 0 {
        return;
    }
    loop {
        let slot = e.call(NODE_ITEM, &args![list]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![list]).u32();
        let entry = e.mem.u32(slot);
        e.call(OPERATOR_DELETE, &args![entry]);
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
}

// Translated from 00979c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00979c80 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatAction` destructor body (by its vtable, `0108cab8`): frees every
/// entry of the lists at +0x14, +0x04, +0x0C and +0x1C (in that order),
/// then destroys the lists (+0x1C, +0x14, +0x0C, +0x04, `007b9000`).
pub fn fn_00979c80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, COMBAT_ACTION_VTABLE);
    for offset in [0x14u32, 0x04, 0x0c, 0x1c] {
        drain(e, base.wrapping_add(offset));
    }
    for offset in [0x1cu32, 0x14, 0x0c, 0x04] {
        e.call(LIST_DESTRUCT, &args![base.wrapping_add(offset)]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0097_95c0, fn_009795c0(Ptr, f32)),
        entry!(0x0097_9bd0, fn_00979bd0(Ptr, u32) -> Ptr),
        entry!(0x0097_9c80, fn_00979c80(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn float_setter_quiets_a_signalling_nan() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd8);
        e.call(0x0097_95c0, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this + 0xd4), 2.5);
        e.call(0x0097_95c0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0xd4), 0x7fc0_0001);
    }

    #[test]
    fn constructor_builds_the_lists_and_reads_the_kind_table() {
        let mut e = Engine::new();
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        e.map(0x011a_4000, 0x1000);
        e.set_global(KIND_TABLE + 3 * 0xc, 0x5555u32);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_9bd0, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), COMBAT_ACTION_VTABLE);
        assert_eq!(e.mem.u32(this + 0x24), 3);
        assert_eq!(e.mem.u32(this + 0x28), 0x5555);
        assert_eq!(
            calls(&e, LIST_CONSTRUCT),
            vec![
                vec![this + 4],
                vec![this + 0xc],
                vec![this + 0x14],
                vec![this + 0x1c]
            ]
        );
    }

    #[test]
    fn destructor_frees_all_four_lists() {
        let mut e = Engine::new();
        let empty = e.mem.alloc(4);
        e.register_double(NODE_ITEM, move |e, a| {
            let node = e.mem.u32(a[0]);
            if node == 0 {
                empty.into_ret()
            } else {
                node.into_ret()
            }
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let node = e.mem.u32(a[0]);
            let next = e.mem.u32(node + 4);
            e.mem.set_u32(a[0], next);
            Ret::default()
        });
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        for (i, offset) in LISTS.iter().enumerate() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, 0x6000 + i as u32);
            e.mem.set_u32(this + offset, node);
        }
        e.call_log = Some(vec![]);
        e.call(0x0097_9c80, &args![this]);
        assert_eq!(e.mem.u32(this), COMBAT_ACTION_VTABLE);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![0x6002], vec![0x6000], vec![0x6001], vec![0x6003]]
        );
        assert_eq!(
            calls(&e, LIST_DESTRUCT),
            vec![
                vec![this + 0x1c],
                vec![this + 0x14],
                vec![this + 0xc],
                vec![this + 4]
            ]
        );
    }
}
