//! `fallout shared/run_005963d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00598040 (decompiled, FalloutNV.exe 1.4.0.525)
/// A plain getter: the `float` at `+0x3c` of `this`, returned in `ST0`. The
/// engine map has no name for it; the translations that call it
/// (`animation.rs`, `inventorychanges.rs`) use it on a sequence object, which
/// this body does not confirm.
pub fn fn_00598040(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x3c)
}

// Translated from 00597610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00597610 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6f (0xc4 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_42ec`) is `MediaSet`, so this is the constructor of `MediaSet`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It also clears the global at `011ca9fc`.
/// The exception-unwinding frame is not translated.
pub fn fn_00597610(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_42ec);
    e.mem.set_u32(base + 0x18, 0x0103_42d8);
    e.call(0x0040_2d00, &args![base + 0x38]);
    e.call(0x0059_77d0, &args![base + 0x48]);
    e.call(0x0059_77d0, &args![base + 0x58]);
    e.call(0x0059_77d0, &args![base + 0x68]);
    e.call(0x0059_77d0, &args![base + 0x78]);
    e.call(0x0059_77d0, &args![base + 0x88]);
    e.call(0x0059_77d0, &args![base + 0x98]);
    e.call(0x004f_15a0, &args![this, 0x6fu32]);
    e.mem.set_u32(base + 0x44, 0xffff_ffff);
    e.call(0x0059_7850, &args![base + 0x48]);
    e.call(0x0059_7850, &args![base + 0x58]);
    e.call(0x0059_7850, &args![base + 0x68]);
    e.call(0x0059_7850, &args![base + 0x78]);
    e.call(0x0059_7850, &args![base + 0x88]);
    e.call(0x0059_7850, &args![base + 0x98]);
    e.mem.set_f32(base + 0xac, 0.0);
    e.mem.set_f32(base + 0xb0, 0.0);
    e.mem.set_f32(base + 0xb4, 0.0);
    e.mem.set_f32(base + 0xb8, 0.0);
    e.mem.set_u32(base + 0xbc, 0x0);
    e.mem.set_u32(base + 0xc0, 0x0);
    e.mem.set_u32(0x011c_a9fc, 0x0);
    e.mem.set_u32(base + 0x2c, 0x0);
    e.mem.set_u32(base + 0x28, 0x0);
    e.mem.set_u8(base + 0x30, 0x0);
    e.mem.set_u8(base + 0x31, 0x0);
    e.mem.set_u8(base + 0x33, 0x0);
    e.mem.set_u8(base + 0x34, 0x0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00598040, fn_00598040(Ptr) -> f32),
        entry!(0x0059_7610, fn_00597610(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_offset_0x3c() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x50);
        e.mem.set_f32(object + 0x3c, 1.5);
        e.mem.set_f32(object + 0x38, 9.0);
        assert_eq!(e.call(0x0059_8040, &args![object]).f32(), 1.5);
    }
}

#[cfg(test)]
mod ctor_tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00597610_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_7610,
            size: 0xc4,
            globals: &[(0x011c_a9fc, 0xdead_beef)],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0040_2d00, &[A::T(0x38)]),
                (0x0059_77d0, &[A::T(0x48)]),
                (0x0059_77d0, &[A::T(0x58)]),
                (0x0059_77d0, &[A::T(0x68)]),
                (0x0059_77d0, &[A::T(0x78)]),
                (0x0059_77d0, &[A::T(0x88)]),
                (0x0059_77d0, &[A::T(0x98)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6f)]),
                (0x0059_7850, &[A::T(0x48)]),
                (0x0059_7850, &[A::T(0x58)]),
                (0x0059_7850, &[A::T(0x68)]),
                (0x0059_7850, &[A::T(0x78)]),
                (0x0059_7850, &[A::T(0x88)]),
                (0x0059_7850, &[A::T(0x98)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_42ec),
                (0x18, 4, 0x0103_42d8),
                (0x44, 4, 0xffff_ffff),
                (0xac, 4, 0x0),
                (0xb0, 4, 0x0),
                (0xb4, 4, 0x0),
                (0xb8, 4, 0x0),
                (0xbc, 4, 0x0),
                (0xc0, 4, 0x0),
                (0x2c, 4, 0x0),
                (0x28, 4, 0x0),
                (0x30, 1, 0x0),
                (0x31, 1, 0x0),
                (0x33, 1, 0x0),
                (0x34, 1, 0x0),
            ],
        });
    }

    #[test]
    fn fn_00597610_clears_the_global() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        e.mem.map(0x011c_a9fc, 4);
        e.mem.set_u32(0x011c_a9fc, 0xdead_beef);
        for a in [
            0x0048_3370u32,
            0x0040_2d00,
            0x0059_77d0,
            0x0059_7850,
            0x004f_15a0,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call(0x0059_7610, &args![this]);
        assert_eq!(e.mem.u32(0x011c_a9fc), 0);
    }
}
