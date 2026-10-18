//! `fallout shared/magic/enchantmentitem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00409080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00409080 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `EnchantmentItem::EnchantmentItem` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x13 (0x44 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn enchantmentitem_enchantmentitem(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_9190, &args![this]);
    e.mem.set_u32(base, 0x0101_2ea4);
    e.mem.set_u32(base + 0x18, 0x0101_2e5c);
    e.mem.set_u32(base + 0x24, 0x0101_2e44);
    e.call(0x004f_15a0, &args![this, 0x13]);
    e.call(0x0040_9310, &args![this]);
    this
}

/// The function `fn_00409340` calls on the member (`00406340`).
const CALLEE_00406340: u32 = 0x0040_6340;

// Translated from 00409340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00406340` with `ECX` = `this + 0x24` and passes on nothing (the callee returns no value).
pub fn fn_00409340(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_00406340, &args![this.byte_add(0x24)]);
}

// Translated from 00409160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004092a0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00409160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_92a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00409270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00409210` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00409270(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_9210, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_9080, enchantmentitem_enchantmentitem(Ptr) -> Ptr),
        entry!(0x0040_9340, fn_00409340(Ptr)),
        entry!(0x0040_9160, fn_00409160(Ptr, u32) -> Ptr),
        entry!(0x0040_9270, fn_00409270(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn enchantmentitem_enchantmentitem_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_9080,
            size: 0x44,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_9190, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x13)]),
                (0x0040_9310, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_2ea4),
                (0x18, 4, 0x0101_2e5c),
                (0x24, 4, 0x0101_2e44),
            ],
        });
    }

    #[test]
    fn l68d_00409340_passes_this_plus_24_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00406340, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_9340, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00406340).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x24]);
    }
    #[test]
    fn at_00409160_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_92a0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_9160, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_9160)
                .collect();
            let mut expected = vec![(0x0040_92a0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00409270_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_9210, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_9270, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_9270)
                .collect();
            let mut expected = vec![(0x0040_9210, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
