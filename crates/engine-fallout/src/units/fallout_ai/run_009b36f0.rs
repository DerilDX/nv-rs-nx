//! `fallout/ai/run_009b36f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009c52f0` (named `Projectile::LoadGame` in the engine map): the base body the wrapper
/// forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LOAD_GAME_TARGET_009C52F0: u32 = 0x009c_52f0;

// Translated from 009b4cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable words 0108c420, 0108eac0 and 0108f6d0): forwards `this`
/// and one pointer to `009c52f0` (named `Projectile::LoadGame`; Xbox PDB
/// `?LoadGame@Projectile@@UAAXPAVBGSLoadFormBuffer@@@Z`, void). Returns nothing.
pub fn fn_009b4cf0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(LOAD_GAME_TARGET_009C52F0, &args![this, buffer]);
}

/// `009bc470` (`ECX` = projectile; in `fallout/ai/combat/projectile.cpp` by
/// the engine map).
const PROJECTILE_009BC470: u32 = 0x009b_c470;
/// `Projectile::Projectile` (engine map name of `009bbc70`; `ECX` = object,
/// one byte word; `RET 4`).
const PROJECTILE_CONSTRUCT: u32 = 0x009b_bc70;
/// `004f15a0` (`ECX` = form, the form type; `RET 4`).
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// `GrenadeProjectile`'s two vtables (RTTI names read through their
/// locators): the main one at +0 and the one at +0x18.
const GRENADE_VTABLE: u32 = 0x0108_f674;
const GRENADE_VTABLE_18: u32 = 0x0108_f668;
/// `00978f10` (`ECX` = object): a byte.
const OBJECT_00978F10: u32 = 0x0097_8f10;

// Translated from 009b3f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `009bc470` on `this`. The engine map gives it no name.
pub fn fn_009b3f80(e: &mut Engine, this: Ptr) {
    e.call(PROJECTILE_009BC470, &args![this]);
}

// Translated from 009b3fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b3fa0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs a `GrenadeProjectile` (the vtables it stores are that class's
/// by RTTI; one byte word, `RET 4`): `Projectile::Projectile` with the
/// byte, the two vtables (+0, +0x18), the byte at +0x150 cleared and the
/// form type 0x3e (`004f15a0`). Returns `this`. The engine map gives it no
/// name.
pub fn fn_009b3fa0(e: &mut Engine, this: Ptr, flag: u8) -> Ptr {
    let base = this.addr();
    e.call(PROJECTILE_CONSTRUCT, &args![this, u32::from(flag)]);
    e.mem.set_u32(base, GRENADE_VTABLE);
    e.mem.set_u32(base + 0x18, GRENADE_VTABLE_18);
    e.mem.set_u8(base + 0x150, 0);
    e.call(SET_FORM_TYPE, &args![this, 0x3eu32]);
    this
}

// Translated from 009b4d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the object's virtual slot `0x318` or else `00978f10` reports
/// true (each a byte). The engine map gives it no name.
pub fn fn_009b4d40(e: &mut Engine, this: Ptr) -> bool {
    if e.vcall(this.addr(), 0x318, &[]).bool() {
        return true;
    }
    e.call(OBJECT_00978F10, &args![this]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009b_4cf0, fn_009b4cf0(Ptr, Ptr)),
        entry!(0x009b_3f80, fn_009b3f80(Ptr)),
        entry!(0x009b_3fa0, fn_009b3fa0(Ptr, u8) -> Ptr),
        entry!(0x009b_4d40, fn_009b4d40(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_009b4cf0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LOAD_GAME_TARGET_009C52F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009b_4cf0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOAD_GAME_TARGET_009C52F0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod b027_tests {
    use super::*;

    fn log_of(e: &mut Engine, addr: u32, words: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log = Some(vec![]);
        e.call(addr, words);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect()
    }

    #[test]
    fn at_009b3f80_forwards() {
        let mut e = Engine::new();
        e.register(PROJECTILE_009BC470, |_, _| Ret::default());
        assert_eq!(
            log_of(&mut e, 0x009b_3f80, &[0x4000]),
            vec![(PROJECTILE_009BC470, vec![0x4000])]
        );
    }

    #[test]
    fn at_009b3fa0_constructs_a_grenade_projectile() {
        let mut e = Engine::new();
        e.register(PROJECTILE_CONSTRUCT, |_, _| Ret::default());
        e.register(SET_FORM_TYPE, |_, _| Ret::default());
        let this = e.mem.alloc(0x154);
        e.mem.set_u8(this + 0x150, 0xff);
        assert_eq!(
            log_of(&mut e, 0x009b_3fa0, &[this, 0x301]),
            vec![
                (PROJECTILE_CONSTRUCT, vec![this, 1]),
                (SET_FORM_TYPE, vec![this, 0x3e]),
            ]
        );
        assert_eq!(e.mem.u32(this), GRENADE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), GRENADE_VTABLE_18);
        assert_eq!(e.mem.u8(this + 0x150), 0);
        assert_eq!(e.call(0x009b_3fa0, &args![this, 0u32]).u32(), this);
    }

    #[test]
    fn at_009b4d40_asks_the_slot_then_00978f10() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x31c / 4];
        slots[0x318 / 4] = 0x0f00_1318;
        e.put_vtable(0x0f00_0000, &slots);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x0f00_0000);
        e.register(0x0f00_1318, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        e.register(OBJECT_00978F10, |_, _| Ret::default());
        assert!(!e.call(0x009b_4d40, &args![this]).bool());
        e.register(OBJECT_00978F10, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(e.call(0x009b_4d40, &args![this]).bool());
        let log = log_of(&mut e, 0x009b_4d40, &[this]);
        assert_eq!(log.len(), 2);
        e.register(0x0f00_1318, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let log = log_of(&mut e, 0x009b_4d40, &[this]);
        assert_eq!(log, vec![(0x0f00_1318, vec![this])]);
    }
}
