//! `fallout shared/run_00582be0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00582c20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00582c20 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form
/// type 0x41 (0xec bytes); `ECX` is the object. The engine map gives it no
/// name (a `range-sub` function), so the class is unconfirmed. It runs the
/// `TESForm` constructor (`00483370`) and the component constructors by
/// address, stores the three vtable pointers (`+0`, `+0x18`, `+0x24`),
/// registers the form type (`004f15a0`), and sets `+0x3c`, `+0x34` and
/// `+0x68` to null. It allocates a 0x10-byte object (`operator new`) and,
/// when the allocation succeeds, constructs it with `00589000(object,
/// 0x25)` and keeps the result at `+0x30` (null otherwise). It copies the
/// floats at `01016970` into `+0xa0` and `+0xa4` and the one at `01015f5c`
/// into `+0xa8` and `+0xac`, then calls `00583070` on the object.
pub fn fn_00582c20(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_e270, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0103_195c);
    e.mem.set_u32(base + 0x18, 0x0103_1948);
    e.mem.set_u32(base + 0x24, 0x0103_1928);
    e.call(0x0057_cd10, &args![base + 0x38]);
    e.call(0x0058_8fa0, &args![base + 0x50, 0x25u32]);
    e.call(0x0096_a2d0, &args![base + 0x60]);
    e.call(0x0063_3c90, &args![base + 0x6c, 0u32]);
    e.call(0x0068_15c0, &args![base + 0xa0]);
    e.call(0x0068_15c0, &args![base + 0xa8]);
    e.call(0x0058_8fd0, &args![base + 0xb0, 0x25u32]);
    e.call(0x0040_37b0, &args![base + 0xc0]);
    e.call(0x0048_e270, &args![base + 0xd4]);
    e.call(0x0048_e270, &args![base + 0xe0]);
    e.call(0x004f_15a0, &args![this, 0x41u32]);
    e.mem.set_u32(base + 0x3c, 0);
    let allocated = e.call(0x0040_1000, &args![0x10u32]).u32();
    let object = if allocated != 0 {
        e.call(0x0058_9000, &args![allocated, 0x25u32]).u32()
    } else {
        0
    };
    e.mem.set_u32(base + 0x30, object);
    e.mem.set_u32(base + 0x68, 0);
    e.mem.set_u32(base + 0x34, 0);
    let first = e.global::<f32>(0x0101_6970);
    e.mem.set_f32(base + 0xa4, first);
    e.mem.set_f32(base + 0xa0, first);
    let second = e.global::<f32>(0x0101_5f5c);
    e.mem.set_f32(base + 0xac, second);
    e.mem.set_f32(base + 0xa8, second);
    e.call(0x0058_3070, &args![this]);
    this
}

// Translated from 00582e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00582e30` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00582e00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_2e30, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00583040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `006fc980` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00583040(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006f_c980, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_2c20, fn_00582c20(Ptr) -> Ptr),
        entry!(0x0058_2e00, fn_00582e00(Ptr, u32) -> Ptr),
        entry!(0x0058_3040, fn_00583040(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00582c20_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_2c20,
            size: 0xec,
            globals: &[(0x0101_6970, 0x1000000), (0x0101_5f5c, 0x1000001)],
            returns: &[(0x0040_1000, 0x2000_0000), (0x0058_9000, 0x2000_1000)],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_e270, &[A::T(0x24)]),
                (0x0057_cd10, &[A::T(0x38)]),
                (0x0058_8fa0, &[A::T(0x50), A::V(0x25)]),
                (0x0096_a2d0, &[A::T(0x60)]),
                (0x0063_3c90, &[A::T(0x6c), A::V(0x0)]),
                (0x0068_15c0, &[A::T(0xa0)]),
                (0x0068_15c0, &[A::T(0xa8)]),
                (0x0058_8fd0, &[A::T(0xb0), A::V(0x25)]),
                (0x0040_37b0, &[A::T(0xc0)]),
                (0x0048_e270, &[A::T(0xd4)]),
                (0x0048_e270, &[A::T(0xe0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x41)]),
                (0x0040_1000, &[A::V(0x10)]),
                (0x0058_9000, &[A::V(0x2000_0000), A::V(0x25)]),
                (0x0058_3070, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_195c),
                (0x18, 4, 0x0103_1948),
                (0x24, 4, 0x0103_1928),
                (0x3c, 4, 0x0),
                (0x30, 4, 0x2000_1000),
                (0x68, 4, 0x0),
                (0x34, 4, 0x0),
                (0xa4, 4, 0x1000000),
                (0xa0, 4, 0x1000000),
                (0xac, 4, 0x1000001),
                (0xa8, 4, 0x1000001),
            ],
        });
    }

    #[test]
    fn fn_00582c20_without_memory_leaves_the_object_null() {
        // `operator new` answers null: the object is not constructed and
        // `+0x30` stays null.
        let mut e = Engine::new();
        let this = e.mem.alloc(0xec);
        e.mem.write(this, &[0xcd; 0xec]);
        e.mem.map(0x0101_6970, 4);
        e.mem.map(0x0101_5f5c, 4);
        for a in [
            0x0048_3370,
            0x0040_2d00,
            0x0048_e270,
            0x0057_cd10,
            0x0058_8fa0,
            0x0096_a2d0,
            0x0063_3c90,
            0x0068_15c0,
            0x0058_8fd0,
            0x0040_37b0,
            0x004f_15a0,
            0x0040_1000,
            0x0058_9000,
            0x0058_3070,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0058_2c20, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != 0x0058_9000));
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }
    #[test]
    fn at_00582e00_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_2e30, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_2e00, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_2e00)
                .collect();
            let mut expected = vec![(0x0058_2e30, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00583040_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x006f_c980, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_3040, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_3040)
                .collect();
            let mut expected = vec![(0x006f_c980, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
