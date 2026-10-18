//! `fallout shared/pathfinding/navmeshsearchhide.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006a9d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0106b7dc` (RTTI `.?AVNavMeshSearchHide@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `006a6b00`. One word, plain `RET`, returns nothing.
pub fn fn_006a9d60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_b7dc);
    e.call(0x006a_6b00, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006a_9d60, fn_006a9d60(Ptr))]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_006a9d60_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x006a_6b00, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006a_9d60, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x006a_6b00)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_b7dc);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_b7dc,
            "the vtable was stored before the call"
        );
    }
}
