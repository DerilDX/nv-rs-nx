//! `fallout shared/bgsbodypartdata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e4af0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e4af0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x57 (0x74 bytes); `ECX` is the object.
/// The engine map names it `BGSBodyPartData::BGSBodyPartData` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0104_5504`) is `BGSBodyPartData`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005e4af0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x18]);
    e.call(0x004f_9d90, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0104_5504);
    e.mem.set_u32(base + 0x18, 0x0104_54dc);
    e.mem.set_u32(base + 0x30, 0x0104_54c4);
    e.call(0x004f_15a0, &args![this, 0x57u32]);
    e.call(0x005e_4c40, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_4af0, fn_005e4af0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005e4af0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_4af0,
            size: 0x74,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x18)]),
                (0x004f_9d90, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x57)]),
                (0x005e_4c40, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_5504),
                (0x18, 4, 0x0104_54dc),
                (0x30, 4, 0x0104_54c4),
            ],
        });
    }
}
