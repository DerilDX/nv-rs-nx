//! `fallout shared/run_0050fe50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005104f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005104f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x15 (0x90 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005104f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0048_cc50, &args![base + 0x5c]);
    e.call(0x0047_5350, &args![base + 0x68]);
    e.call(0x0051_0620, &args![base + 0x70]);
    e.mem.set_u32(base, 0x0102_9d5c);
    e.mem.set_u32(base + 0x30, 0x0102_9d44);
    e.mem.set_u32(base + 0x3c, 0x0102_9d20);
    e.mem.set_u32(base + 0x5c, 0x0102_9d0c);
    e.mem.set_u32(base + 0x68, 0x0102_9cf8);
    e.mem.set_u32(base + 0x70, 0x0102_9ce8);
    e.call(0x0040_37b0, &args![base + 0x88]);
    e.call(0x004f_15a0, &args![this, 0x15]);
    e.call(0x0051_0710, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 00510620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the component at `+0x70` of the form `005104f0` builds: it stores
/// the component's base vtable (`0x0102_9ee0`) and returns the object. The engine map
/// gives it no name, so the class is unconfirmed.
pub fn fn_00510620(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0102_9ee0);
    this
}

// Translated from 00510710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of the form `005104f0` builds (form type 0x15; class unconfirmed):
/// zeroes the words at `+0x74`, `+0x78`, `+0x80`, `+0x84` and `+0x7c`, then runs
/// `00438390` on the string at `+0x88` with the text at `0x0101_1584`.
pub fn fn_00510710(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x74, 0);
    e.mem.set_u32(base + 0x78, 0);
    e.mem.set_u32(base + 0x80, 0);
    e.mem.set_u32(base + 0x84, 0);
    e.mem.set_u32(base + 0x7c, 0);
    e.call(0x0043_8390, &args![base + 0x88, 0x0101_1584u32]);
}

// Translated from 005105f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00510640` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005105f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_0640, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_04f0, fn_005104f0(Ptr) -> Ptr),
        entry!(0x0051_0620, fn_00510620(Ptr) -> Ptr),
        entry!(0x0051_0710, fn_00510710(Ptr)),
        entry!(0x0051_05f0, fn_005105f0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005104f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_04f0,
            size: 0x90,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0048_cc50, &[A::T(0x5c)]),
                (0x0047_5350, &[A::T(0x68)]),
                (0x0051_0620, &[A::T(0x70)]),
                (0x0040_37b0, &[A::T(0x88)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x15)]),
                (0x0051_0710, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_9d5c),
                (0x30, 4, 0x0102_9d44),
                (0x3c, 4, 0x0102_9d20),
                (0x5c, 4, 0x0102_9d0c),
                (0x68, 4, 0x0102_9cf8),
                (0x70, 4, 0x0102_9ce8),
            ],
        });
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test;

    #[test]
    fn fn_00510620_stores_the_vtable() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_0620,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[],
            stores: &[(0x0, 4, 0x0102_9ee0)],
        });
    }

    #[test]
    fn fn_00510710_zeroes_the_fields_and_sets_the_string() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        e.mem.write(this, &[0xcd; 0x90]);
        e.register(0x0043_8390, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_0710, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0043_8390, vec![this + 0x88, 0x0101_1584])]);
        for offset in [0x74, 0x78, 0x7c, 0x80, 0x84] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x70), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(this + 0x88), 0xcdcd_cdcd);
    }
    #[test]
    fn at_005105f0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_0640, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_05f0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_05f0)
                .collect();
            let mut expected = vec![(0x0051_0640, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
