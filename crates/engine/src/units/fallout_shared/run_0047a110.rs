//! `fallout shared/run_0047a110` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0047a130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0047a130 - the exception-unwinding frame (FS:[0]) is not translated.
/// The constructor of the form component `BGSMessageIcon` (the engine map
/// gives it no name; the RTTI name of the vtable it stores, `0101ab58`, is
/// `.?AVBGSMessageIcon@@`): runs the base constructor (`00402d30`), stores
/// the vtable at `+0`, runs `00402c20` on the member at `+4`, then `0047a200`
/// on the component. Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_0047a130(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_ab58);
    e.call(0x0040_2c20, &args![this.byte_add(4)]);
    e.call(0x0047_a200, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0047a130, fn_0047a130(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0047a130_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0047_a130,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2d30, &[A::T(0x0)]),
                (0x0040_2c20, &[A::T(0x4)]),
                (0x0047_a200, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_ab58)],
        });
    }
}
