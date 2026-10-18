//! `fallout shared/bgstextureset.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005922e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005922e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `BGSTextureSet::BGSTextureSet` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 4 (0xa0 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn bgstextureset_bgstextureset(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0051_d190, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0103_3d1c);
    e.mem.set_u32(base + 0x30, 0x0103_3c7c);
    e.call(
        0x00ec_782f,
        &args![this, base + 0x38, 0xc, 0x6, 0x0048_e270, 0x0048_e2e0],
    );
    e.call(0x0059_2450, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x4]);
    this
}

// Translated from 00592380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (no Xbox name in the engine map): runs the
/// destructor body at `005923c0` and frees 0xa0 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn fn_00592380(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_23c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xa0u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_2380, fn_00592380(Ptr, u32) -> Ptr),
        entry!(0x0059_22e0, bgstextureset_bgstextureset(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn bgstextureset_bgstextureset_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_22e0,
            size: 0xa0,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0051_d190, &[A::T(0x30)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x0),
                        A::T(0x38),
                        A::V(0xc),
                        A::V(0x6),
                        A::V(0x0048_e270),
                        A::V(0x0048_e2e0),
                    ],
                ),
                (0x0059_2450, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4)]),
            ],
            stores: &[(0x0, 4, 0x0103_3d1c), (0x30, 4, 0x0103_3c7c)],
        });
    }

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
    fn fn_00592380_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0059_2380, 0x0059_23c0, 0xa0);
    }
}
