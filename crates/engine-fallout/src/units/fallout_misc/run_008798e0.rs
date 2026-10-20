//! `fallout/misc/run_008798e0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `SeenData` vtable (`008799a0` stores the same address).
const SEEN_DATA_VTABLE: u32 = 0x0108_3fc4;
/// `memset`-shaped helper (`00403d30`, `cdecl`: destination, value, count).
const MEMSET_WRAPPER: u32 = 0x0040_3d30;

// Translated from 00879940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SeenData` constructor (no name in the engine map; the vtable is the
/// one `SeenData`'s destructor body `008799a0` restores): stores the vtable,
/// clears the 0x20 bytes of seen bits at +4 (`00403d30(this + 4, 0, 0x20)`)
/// and returns `this`.
pub fn fn_00879940(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SEEN_DATA_VTABLE);
    e.call(
        MEMSET_WRAPPER,
        &args![this.addr().wrapping_add(4), 0u32, 0x20u32],
    );
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0087_9940, fn_00879940(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00879940_sets_the_vtable_and_clears_the_bits() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.register(MEMSET_WRAPPER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_9940, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SEEN_DATA_VTABLE);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == MEMSET_WRAPPER).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this + 4, 0, 0x20]);
    }
}
