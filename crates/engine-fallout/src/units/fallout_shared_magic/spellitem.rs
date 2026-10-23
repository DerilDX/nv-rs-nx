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

// Translated from 0040e290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0040e2c0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0040e290(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_e2c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0058e890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0058e8c0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0058e890(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_e8c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e6440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e6470` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e6440(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_6470, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e7370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e73a0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e7370(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_73a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e8140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e8170` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e8140(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_8170, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e9560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e9590` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e9560(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_9590, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e9830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e9860` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e9830(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_9860, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The effect list clean-up (`enchantmentitem.rs`, `00409340`) and the
/// `MagicItemForm` destructor body (`00409210`), thiscall.
const REMOVE_EFFECTS_00409340: u32 = 0x0040_9340;
const MAGIC_ITEM_FORM_DESTRUCT: u32 = 0x0040_9210;

// Translated from 0040e210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether
/// `SpellItemData::iFlags` (+0x40) has any bit of `mask` (the whole stack
/// word is ANDed with the byte).
pub fn fn_0040e210(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    u32::from(e.mem.u8(this.addr() + 0x40)) & mask != 0
}

// Translated from 0040e280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the chunk tag `0x54495053`
/// (`"SPIT"`).
pub fn fn_0040e280(e: &mut Engine, this: Ptr) -> u32 {
    let _ = (e, this);
    0x5449_5053
}

// Translated from 0040e2c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040e2c0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the effect list's clean-up) is not translated.
/// The `SpellItem` destructor body (unnamed in the engine map): the three
/// `SpellItem` vtables (RTTI `.?AVSpellItem@@`), the effects removed
/// (`00409340`), then the `MagicItemForm` destructor body (`00409210`).
pub fn fn_0040e2c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0101_3f8c);
    e.mem.set_u32(base + 0x18, 0x0101_3f44);
    e.mem.set_u32(base + 0x24, 0x0101_3f28);
    e.call(REMOVE_EFFECTS_00409340, &args![this]);
    e.call(MAGIC_ITEM_FORM_DESTRUCT, &args![this]);
}

// Translated from 0040e330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `SpellItemData` start values (Xbox PDB
/// names, PC offsets): `iSpellType` (+0x34) 0, `iCostOverride` (+0x38) -1,
/// `eLevelOverride` (+0x3c) 0, `iFlags` (+0x40) 0.
pub fn fn_0040e330(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x34, 0);
    e.mem.set_u32(this.addr() + 0x38, 0xffff_ffff);
    e.mem.set_u32(this.addr() + 0x3c, 0);
    e.mem.set_u8(this.addr() + 0x40, 0);
}

// Translated from 0040e500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets `SpellItemData::iCostOverride` (+0x38).
pub fn fn_0040e500(e: &mut Engine, this: Ptr, cost: u32) {
    e.mem.set_u32(this.addr() + 0x38, cost);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_e160, spellitem_spellitem(Ptr) -> Ptr),
        entry!(0x0061_e760, fn_0061e760(Ptr, Ptr) -> Ptr),
        entry!(0x0040_e290, fn_0040e290(Ptr, u32) -> Ptr),
        entry!(0x0058_e890, fn_0058e890(Ptr, u32) -> Ptr),
        entry!(0x005e_6440, fn_005e6440(Ptr, u32) -> Ptr),
        entry!(0x005e_7370, fn_005e7370(Ptr, u32) -> Ptr),
        entry!(0x005e_8140, fn_005e8140(Ptr, u32) -> Ptr),
        entry!(0x005e_9560, fn_005e9560(Ptr, u32) -> Ptr),
        entry!(0x005e_9830, fn_005e9830(Ptr, u32) -> Ptr),
        entry!(0x0040_e210, fn_0040e210(Ptr, u32) -> bool),
        entry!(0x0040_e280, fn_0040e280(Ptr) -> u32),
        entry!(0x0040_e2c0, fn_0040e2c0(Ptr)),
        entry!(0x0040_e330, fn_0040e330(Ptr)),
        entry!(0x0040_e500, fn_0040e500(Ptr, u32)),
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
    #[test]
    fn at_0040e290_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_e2c0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_e290, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_e290)
                .collect();
            let mut expected = vec![(0x0040_e2c0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0058e890_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_e8c0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_e890, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_e890)
                .collect();
            let mut expected = vec![(0x0058_e8c0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e6440_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_6470, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_6440, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_6440)
                .collect();
            let mut expected = vec![(0x005e_6470, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e7370_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_73a0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_7370, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_7370)
                .collect();
            let mut expected = vec![(0x005e_73a0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e8140_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_8170, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_8140, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_8140)
                .collect();
            let mut expected = vec![(0x005e_8170, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e9560_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_9590, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_9560, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_9560)
                .collect();
            let mut expected = vec![(0x005e_9590, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e9830_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_9860, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_9830, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_9830)
                .collect();
            let mut expected = vec![(0x005e_9860, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod spell_data_tests {
    use super::*;

    #[test]
    fn flag_test_ands_the_whole_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x44);
        e.mem.set_u8(this + 0x40, 0x05);
        assert!(e.call(0x0040_e210, &args![this, 4u32]).bool());
        assert!(!e.call(0x0040_e210, &args![this, 0x102u32]).bool());
        assert!(e.call(0x0040_e210, &args![this, 0x101u32]).bool());
    }

    #[test]
    fn chunk_tag() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0040_e280, &args![0u32]).u32(), 0x5449_5053);
    }

    #[test]
    fn destructor_body_sets_the_vtables_and_unwinds() {
        let mut e = Engine::new();
        e.register(REMOVE_EFFECTS_00409340, |_, _| Ret::default());
        e.register(MAGIC_ITEM_FORM_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x44);
        e.call_log = Some(vec![]);
        e.call(0x0040_e2c0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0101_3f8c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0101_3f44);
        assert_eq!(e.mem.u32(this + 0x24), 0x0101_3f28);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![REMOVE_EFFECTS_00409340, MAGIC_ITEM_FORM_DESTRUCT]
        );
        assert!(log.iter().skip(1).all(|c| c.1 == vec![this]));
    }

    #[test]
    fn data_start_values_and_cost_override() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + 0x34, 9);
        e.mem.set_u8(this + 0x40, 9);
        e.call(0x0040_e330, &args![this]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u8(this + 0x40), 0);
        e.call(0x0040_e500, &args![this, 40u32]);
        assert_eq!(e.mem.u32(this + 0x38), 40);
    }
}
