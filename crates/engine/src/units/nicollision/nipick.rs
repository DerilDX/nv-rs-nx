//! `NiCollision/nipick.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPick::Results::Results` (`00e98db0`, `this` plus two stack words, `RET 8`).
const RESULTS_CONSTRUCTOR: u32 = 0x00e9_8db0;
/// The vtable stored at `+0x18` (RTTI class `NiPick::Results`).
const VTABLE_RESULTS: u32 = 0x010f_16ec;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00e98f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPick::NiPick` (engine map; the two stack words go to the results constructor
/// `00e98db0`, which is `NiPick::Results` at `+0x18`): sets the default state, clears the
/// root `NiPointer` and returns `this`.
///
/// Fields (Xbox PDB names, PC offsets as in the code): `m_ePickType` +0 = 0,
/// `m_eSortType` +4 = 0, `m_eIntersectType` +8 = 1, `m_eCoordinateType` +0xc = 1,
/// `m_bFrontOnly` +0x10 = 1, `m_bObserveAppCullFlag` +0x11 = 0, `m_spRoot` +0x14,
/// `m_pickResults` +0x18 (vtable `010f16ec`; its `m_uiValidResults` at +0x28 = 0),
/// `m_pLastAddedRecord` +0x2c = 0, `m_bReturnTexture` .. `m_bReturnColor` +0x30..+0x33 = 0.
/// The root is cleared before the results are constructed, and then released again when it
/// is non-null: decrement the count at `+4` and, when it reaches 0, call the object's
/// virtual at slot `+4`; then store null.
pub fn ni_pick_ni_pick(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x14, 0);
    e.call(RESULTS_CONSTRUCTOR, &args![base + 0x18, first, second]);
    e.mem.set_u32(base + 0x18, VTABLE_RESULTS);
    e.mem.set_u32(base + 0x28, 0);
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 0x4, 0);
    e.mem.set_u32(base + 0x8, 1);
    e.mem.set_u32(base + 0xc, 1);
    e.mem.set_u8(base + 0x10, 1);
    e.mem.set_u8(base + 0x11, 0);
    let root = e.mem.u32(base + 0x14);
    if root != 0 {
        if e.call(INTERLOCKED_DECREMENT, &args![root + 4]).u32() == 0 {
            e.vcall(root, 4, &args![]);
        }
        e.mem.set_u32(base + 0x14, 0);
    }
    e.mem.set_u8(base + 0x30, 0);
    e.mem.set_u8(base + 0x31, 0);
    e.mem.set_u8(base + 0x32, 0);
    e.mem.set_u8(base + 0x33, 0);
    e.mem.set_u32(base + 0x2c, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_8f20, ni_pick_ni_pick(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    fn pick(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x40);
        for i in 0..0x10u32 {
            e.mem.set_u32(this + i * 4, 0xffff_ffff);
        }
        this
    }

    fn check_fields(e: &Engine, this: u32) {
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u8(this + 0x10), 1);
        assert_eq!(e.mem.u8(this + 0x11), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), VTABLE_RESULTS);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        // The results constructor's own words (+0x1c to +0x27) are left alone.
        assert_eq!(e.mem.u32(this + 0x1c), 0xffff_ffff);
    }

    #[test]
    fn sets_the_defaults_and_passes_the_words_to_the_results_constructor() {
        let mut e = Engine::new();
        e.register(RESULTS_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        e.register(INTERLOCKED_DECREMENT, |_, _| panic!("no root to release"));
        let this = pick(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_8f20, &args![this, 0u32, 8u32]).u32(), this);
        assert_eq!(
            calls_to(&mut e, RESULTS_CONSTRUCTOR),
            vec![vec![this + 0x18, 0, 8]]
        );
        check_fields(&e, this);
    }

    #[test]
    fn releases_a_root_the_results_constructor_left_and_calls_its_destructor_at_zero() {
        for (count, destroyed) in [(0u32, true), (3u32, false)] {
            let mut e = Engine::new();
            let root = e.mem.alloc(0x10);
            let vtable = e.mem.alloc(0x10);
            e.mem.set_u32(root, vtable);
            e.mem.set_u32(vtable + 4, 0x00aa_0001);
            e.register_double(RESULTS_CONSTRUCTOR, move |e, a| {
                // `a[0] - 4` is the root pointer at +0x14.
                e.mem.set_u32(a[0] - 4, root);
                Ret::default()
            });
            e.register_double(INTERLOCKED_DECREMENT, move |_, _| Ret {
                eax: count,
                ..Ret::default()
            });
            e.register(0x00aa_0001, |_, _| Ret::default());
            let this = pick(&mut e);
            e.call_log = Some(vec![]);
            e.call(0x00e9_8f20, &args![this, 0u32, 8u32]);
            assert_eq!(
                calls_to(&mut e, INTERLOCKED_DECREMENT),
                vec![vec![root + 4]]
            );
            assert_eq!(
                calls_to(&mut e, 0x00aa_0001),
                if destroyed { vec![vec![root]] } else { vec![] }
            );
            check_fields(&e, this);
        }
    }
}
