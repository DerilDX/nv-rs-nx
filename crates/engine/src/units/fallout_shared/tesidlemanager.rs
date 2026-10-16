//! `fallout shared/tesidlemanager.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005ffbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00600b90`, which stores the vtable `0104a060` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00600b90`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_005ffbd0(e: &mut Engine, this: Ptr) {
    e.call(0x0060_0b90, &args![this]);
}

// Translated from 00600d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00600d90`, a destructor with an exception frame
/// that stores the vtable `0104a070` and, when the byte at `+0x10` is set, frees the items
/// of the chains in the table at `+8`, and returns nothing. Parameters: one word, `this`;
/// the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `00600d90`'s disassembly ends with its exception-frame unlink and returns no value
/// (Ghidra types it void).
pub fn fn_00600d70(e: &mut Engine, this: Ptr) {
    e.call(0x0060_0d90, &args![this]);
}

// Translated from 00600b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0104a040` (RTTI `.?AV?$BSTCaseInsensitiveStringMap@PAVIDLE_ANIM_ROOT@@@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00600d70`. One word, plain `RET`, returns nothing.
pub fn fn_00600b00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0104_a040);
    fn_00600d70(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060_0b00, fn_00600b00(Ptr)),
        entry!(0x005f_fbd0, fn_005ffbd0(Ptr)),
        entry!(0x0060_0d70, fn_00600d70(Ptr)),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_005ffbd0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0060_0b90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_fbd0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0060_0b90), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_00600d70_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0060_0d90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0060_0d70, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0060_0d90), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_00600b00_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0060_0d90, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0060_0b00, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0060_0d90)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0104_a040);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0104_a040,
            "the vtable was stored before the call"
        );
    }
}
