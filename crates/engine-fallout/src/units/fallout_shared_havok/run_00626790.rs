//! `fallout shared/havok/run_00626790` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00626790 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__cdecl(unused, count)`: a loop over `0..count` with an
/// empty body; it does nothing.
pub fn fn_00626790(_e: &mut Engine, _unused_0: u32, _count: i32) {}

// Translated from 006267c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__cdecl(destination, source)`: copies two words from
/// `source` to `destination`.
pub fn fn_006267c0(e: &mut Engine, destination: u32, source: u32) {
    for i in 0..2 {
        let word = e.mem.u32(source + i * 4);
        e.mem.set_u32(destination + i * 4, word);
    }
}

/// `004ae700` (`run_004ae6a0.rs`): `__fastcall`; the object's character proxy field
/// (`bhkCharacterProxy::operatorP` plus `+0x10`), or zero without a proxy.
const CHARACTER_PROXY_FIELD: u32 = 0x004a_e700;
/// `00559450` (unnamed in the engine map): `thiscall`, answers the dword at `this`.
const READ_FIRST_WORD: u32 = 0x0055_9450;
/// `00403550` (unnamed in the engine map): `thiscall`, `RET 4`; stores its word at `+8` of `this`.
const STORE_WORD_AT_8: u32 = 0x0040_3550;
/// Vtable slot (byte offset) `this` is called through at the end of `00626cb0`.
const SLOT_C4: u32 = 0xc4;

// Translated from 00626d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout shared/havok`). `thiscall`, `RET 4`: stores its word in
/// the member at `this + 0x14` through `00403550` (which writes `+8` of that member).
pub fn fn_00626d00(e: &mut Engine, this: Ptr, value: u32) {
    e.call(
        STORE_WORD_AT_8,
        &args![this.addr().wrapping_add(0x14), value],
    );
}

// Translated from 00626cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall`, `RET 4`: when `004ae700` finds a proxy field for
/// `this`, reads the stack argument back through `00559450` (the exe passes the address of the
/// argument word) and hands it to `00626d00` for that field; then calls virtual slot `+0xc4` of
/// `this` with no arguments.
pub fn fn_00626cb0(e: &mut Engine, this: Ptr, held: u32) {
    let proxy_field = e.call(CHARACTER_PROXY_FIELD, &args![this]).u32();
    if proxy_field != 0 {
        let value = e.with_stack(4, |e, word| {
            e.mem.set_u32(word.addr(), held);
            e.call(READ_FIRST_WORD, &args![word]).u32()
        });
        fn_00626d00(e, Ptr::new(proxy_field), value);
    }
    e.vcall(this.addr(), SLOT_C4, &args![]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_6cb0, fn_00626cb0(Ptr, u32)),
        entry!(0x0062_6d00, fn_00626d00(Ptr, u32)),
        entry!(0x00626790, fn_00626790(u32, i32)),
        entry!(0x006267c0, fn_006267c0(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00626790_has_no_effect() {
        let mut e = Engine::new();
        let block = e.mem.alloc(4);
        e.mem.set_u32(block, 5);
        e.call(0x0062_6790, &args![block, 10i32]);
        assert_eq!(e.mem.u32(block), 5);
    }

    #[test]
    fn fn_006267c0_copies_two_words() {
        let mut e = Engine::new();
        let source = e.mem.alloc(12);
        let destination = e.mem.alloc(12);
        for (i, word) in [1u32, 2, 3].iter().enumerate() {
            e.mem.set_u32(source + 4 * i as u32, *word);
        }
        e.call(0x0062_67c0, &args![destination, source]);
        assert_eq!(e.mem.u32(destination), 1);
        assert_eq!(e.mem.u32(destination + 4), 2);
        assert_eq!(e.mem.u32(destination + 8), 0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
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
    fn at_00626d00_stores_the_word_in_the_member_at_offset_0x14() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x0040_3550, |_, _| Ret::default());
        e.call(0x0062_6d00, &args![0x4000u32, 0x77u32]);
        assert_eq!(calls(&e, 0x0040_3550), vec![vec![0x4014, 0x77]]);
    }

    fn rig_for_626cb0(proxy_field: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        let mut slots = [0u32; 0x32];
        slots[0xc4 / 4] = 0x0f00_6000;
        let table = 0x0f00_5100;
        e.put_vtable(table, &slots);
        e.mem.set_u32(this, table);
        e.register(0x0f00_6000, |_, _| Ret::default());
        e.register_double(CHARACTER_PROXY_FIELD, move |_, _| proxy_field.into_ret());
        e.register(READ_FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0040_3550, |_, _| Ret::default());
        (e, this)
    }

    #[test]
    fn at_00626cb0_stores_the_argument_when_there_is_a_proxy_field() {
        let (mut e, this) = rig_for_626cb0(0x6000);
        e.call(0x0062_6cb0, &args![this, 0xfeed_0001u32]);
        assert_eq!(calls(&e, CHARACTER_PROXY_FIELD), vec![vec![this]]);
        assert_eq!(calls(&e, 0x0040_3550), vec![vec![0x6014, 0xfeed_0001]]);
        // The call through slot 0xc4 comes last, with only `this`.
        assert_eq!(calls(&e, 0x0f00_6000), vec![vec![this]]);
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(log.last().unwrap().0, 0x0f00_6000);
    }

    #[test]
    fn at_00626cb0_only_calls_the_slot_without_a_proxy_field() {
        let (mut e, this) = rig_for_626cb0(0);
        e.call(0x0062_6cb0, &args![this, 0xfeed_0001u32]);
        assert!(calls(&e, READ_FIRST_WORD).is_empty());
        assert!(calls(&e, 0x0040_3550).is_empty());
        assert_eq!(calls(&e, 0x0f00_6000), vec![vec![this]]);
    }
}
