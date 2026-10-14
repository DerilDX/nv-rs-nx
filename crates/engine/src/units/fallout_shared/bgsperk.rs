//! `fallout shared/bgsperk.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005eaab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005eaab0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x56 (0x50 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0104_6ec4`) is `BGSPerk`, so this is the constructor of `BGSPerk`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005eaab0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.call(0x0040_2c20, &args![base + 0x2c]);
    e.mem.set_u32(base, 0x0104_6ec4);
    e.mem.set_u32(base + 0x18, 0x0104_6eac);
    e.mem.set_u32(base + 0x24, 0x0104_6e94);
    e.mem.set_u32(base + 0x2c, 0x0104_6e74);
    e.call(0x0068_0890, &args![base + 0x40]);
    e.call(0x0096_a2d0, &args![base + 0x48]);
    e.call(0x004f_15a0, &args![this, 0x56u32]);
    e.call(0x005e_ac70, &args![this]);
    this
}

// Translated from 005eb680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at `this + 8`, zero-extended. The engine map has no
/// name for it; the condition item's `InitItem` (`006819e0`) calls it.
pub fn fn_005eb680(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u16(this.addr() + 8))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_aab0, fn_005eaab0(Ptr) -> Ptr),
        entry!(0x005e_b680, fn_005eb680(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005eb680_answers_the_word_at_8() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xc);
        e.mem.set_u32(object + 8, 0xffff_8001);
        assert_eq!(e.call(0x005e_b680, &[object]).u32(), 0x8001);
    }

    #[test]
    fn fn_005eaab0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_aab0,
            size: 0x50,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0040_2c20, &[A::T(0x2c)]),
                (0x0068_0890, &[A::T(0x40)]),
                (0x0096_a2d0, &[A::T(0x48)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x56)]),
                (0x005e_ac70, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_6ec4),
                (0x18, 4, 0x0104_6eac),
                (0x24, 4, 0x0104_6e94),
                (0x2c, 4, 0x0104_6e74),
            ],
        });
    }
}
