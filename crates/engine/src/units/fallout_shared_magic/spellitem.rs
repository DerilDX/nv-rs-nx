//! `fallout shared/magic/spellitem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0040e160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040e160 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `SpellItem::SpellItem` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x14 (0x44 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn spellitem_spellitem(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_9190, &args![this]);
    e.mem.set_u32(base, 0x0101_3f8c);
    e.mem.set_u32(base + 0x18, 0x0101_3f44);
    e.mem.set_u32(base + 0x24, 0x0101_3f28);
    e.call(0x004f_15a0, &args![this, 0x14]);
    e.call(0x0040_e330, &args![this]);
    this
}

/// `0061e780` (named `TESTopicInfo::GetResponseList` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const RESPONSE_LIST_TARGET_0061E780: u32 = 0x0061_e780;

// Translated from 0061e760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names this `TESTopicInfo::CloneResponseList` but the body forwards `this`
/// and one pointer to `0061e780` (named `TESTopicInfo::GetResponseList`; Xbox PDB
/// `?GetResponseList@TESTopicInfo@@IAAPAVResponseListWrapper@@PAV2@@Z` takes and returns a
/// `ResponseListWrapper *`). The callee returns its argument when it is non-null and the
/// address `011cc07c` otherwise (the two stores to its result slot before `MOV EAX`); the
/// wrapper returns that pointer.
pub fn fn_0061e760(e: &mut Engine, this: Ptr, wrapper: Ptr) -> Ptr {
    e.call(RESPONSE_LIST_TARGET_0061E780, &args![this, wrapper])
        .ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_e160, spellitem_spellitem(Ptr) -> Ptr),
        entry!(0x0061_e760, fn_0061e760(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn spellitem_spellitem_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_e160,
            size: 0x44,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_9190, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x14)]),
                (0x0040_e330, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_3f8c),
                (0x18, 4, 0x0101_3f44),
                (0x24, 4, 0x0101_3f28),
            ],
        });
    }

    #[test]
    fn forwarder_fn_0061e760_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(RESPONSE_LIST_TARGET_0061E780, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0061_e760, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == RESPONSE_LIST_TARGET_0061E780)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
}
