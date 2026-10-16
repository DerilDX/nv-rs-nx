//! `fallout shared/facegen/bsfacegenninode.cpp` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00662ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenNiNode` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00662b00` and frees 0xf0 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_face_gen_ni_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0066_2b00, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xf0u32]);
    }
    this
}

// Translated from 006629e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5bf8 without reading the receiver. The map names this body `BSFaceGenNiNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_006629e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5bf8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_29e0, fn_006629e0(Ptr) -> u32),
        entry!(0x0066_2ac0, bs_face_gen_ni_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
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
    fn bs_face_gen_ni_node_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0066_2ac0, 0x0066_2b00, 0xf0);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_006629e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_29e0, &args![0x1000u32]).u32(), 0x011d_5bf8);
        assert_eq!(e.call(0x0066_29e0, &args![0u32]).u32(), 0x011d_5bf8);
    }
}
