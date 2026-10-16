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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_9080, enchantmentitem_enchantmentitem(Ptr) -> Ptr),
        entry!(0x0040_9340, fn_00409340(Ptr)),
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
}
