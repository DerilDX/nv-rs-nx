//! `(unplaced)/run_004019b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00401a10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00401a10 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x77 (0x20 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0101_15b4`) is `BGSHungerStage`, so this is the constructor of `BGSHungerStage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00401a10(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0101_15b4);
    e.call(0x0040_19f0, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x77u32]);
    e.call(0x0040_1b20, &args![this]);
    this
}

// Translated from 004020f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004020f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x78 (0x20 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0101_16fc`) is `BGSSleepDeprevationStage`, so this is the constructor of `BGSSleepDeprevationStage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004020f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0101_16fc);
    e.call(0x0040_19f0, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x78u32]);
    e.call(0x0040_1b20, &args![this]);
    this
}

// Translated from 00402820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00402820 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x2f (0xd8 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0101_1964`) is `AlchemyItem`, so this is the constructor of `AlchemyItem`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00402820(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_2b40, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x4c]);
    e.call(0x0040_2c20, &args![base + 0x6c]);
    e.call(0x0047_a130, &args![base + 0x78]);
    e.call(0x0048_cc50, &args![base + 0x88]);
    e.call(0x0048_eac0, &args![base + 0x94]);
    e.call(0x0047_9320, &args![base + 0x9c]);
    e.call(0x0047_5350, &args![base + 0xa4]);
    e.call(0x0047_b4f0, &args![base + 0xac]);
    e.mem.set_u32(base, 0x0101_1964);
    e.mem.set_u32(base + 0x30, 0x0101_191c);
    e.mem.set_u32(base + 0x3c, 0x0101_1900);
    e.mem.set_u32(base + 0x4c, 0x0101_18dc);
    e.mem.set_u32(base + 0x6c, 0x0101_18bc);
    e.mem.set_u32(base + 0x78, 0x0101_18a8);
    e.mem.set_u32(base + 0x88, 0x0101_1894);
    e.mem.set_u32(base + 0x94, 0x0101_1880);
    e.mem.set_u32(base + 0x9c, 0x0101_186c);
    e.mem.set_u32(base + 0xa4, 0x0101_1858);
    e.mem.set_u32(base + 0xac, 0x0101_1844);
    e.call(0x0040_2c20, &args![base + 0xcc]);
    e.call(0x004f_15a0, &args![this, 0x2fu32]);
    e.call(0x0040_2ea0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_1a10, fn_00401a10(Ptr) -> Ptr),
        entry!(0x0040_20f0, fn_004020f0(Ptr) -> Ptr),
        entry!(0x0040_2820, fn_00402820(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00401a10_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_1a10,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_19f0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x77)]),
                (0x0040_1b20, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_15b4)],
        });
    }

    #[test]
    fn fn_004020f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_20f0,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_19f0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x78)]),
                (0x0040_1b20, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_16fc)],
        });
    }

    #[test]
    fn fn_00402820_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_2820,
            size: 0xd8,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2b40, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x4c)]),
                (0x0040_2c20, &[A::T(0x6c)]),
                (0x0047_a130, &[A::T(0x78)]),
                (0x0048_cc50, &[A::T(0x88)]),
                (0x0048_eac0, &[A::T(0x94)]),
                (0x0047_9320, &[A::T(0x9c)]),
                (0x0047_5350, &[A::T(0xa4)]),
                (0x0047_b4f0, &[A::T(0xac)]),
                (0x0040_2c20, &[A::T(0xcc)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2f)]),
                (0x0040_2ea0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_1964),
                (0x30, 4, 0x0101_191c),
                (0x3c, 4, 0x0101_1900),
                (0x4c, 4, 0x0101_18dc),
                (0x6c, 4, 0x0101_18bc),
                (0x78, 4, 0x0101_18a8),
                (0x88, 4, 0x0101_1894),
                (0x94, 4, 0x0101_1880),
                (0x9c, 4, 0x0101_186c),
                (0xa4, 4, 0x0101_1858),
                (0xac, 4, 0x0101_1844),
            ],
        });
    }
}
