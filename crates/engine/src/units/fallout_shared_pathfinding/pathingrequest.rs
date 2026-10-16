//! `fallout shared/pathfinding/pathingrequest.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e25e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequest` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006e2620` and frees 0xb0 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn pathing_request_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006e_2620, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xb0u32]);
    }
    this
}

// Translated from 006e3a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (no Xbox name in the engine map): runs the
/// destructor body at `006e39d0` and frees 0x18 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn fn_006e3a50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006e_39d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x18u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_25e0, pathing_request_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_3a50, fn_006e3a50(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn pathing_request_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006e_25e0, 0x006e_2620, 0xb0);
    }

    #[test]
    fn fn_006e3a50_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006e_3a50, 0x006e_39d0, 0x18);
    }
}
