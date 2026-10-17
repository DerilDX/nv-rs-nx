//! `fallout shared/magic/run_0040dee0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `Setting` (Xbox PDB) base vtable, set by the base constructors.
const SETTING_VTABLE: u32 = 0x0101_2120;
/// The game-setting vtable `0040e0b0` sets after the base constructor.
const GAME_SETTING_VTABLE: u32 = 0x0101_2114;
/// The game-setting collection (`00404a70`: created on first use).
const GAME_SETTINGS: u32 = 0x0040_4a70;
/// Slot of the collection's vtable that adds a setting (`00404de0` for the
/// game settings).
const COLLECTION_ADD_SLOT: u32 = 0x4;

// Translated from 0040e0b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a float game setting (`thiscall`, `RET 8`): the base
/// constructor [`fn_0040e130`] with the name and the value (passed on through
/// the x87 stack), the game-setting vtable (`01012114`), then adds itself to
/// the game-setting collection (`00404a70`, its vtable slot +4). Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_0040e0b0(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    fn_0040e130(e, this, name, x87_quiet_bits(value));
    e.mem.set_u32(this.addr(), GAME_SETTING_VTABLE);
    let collection = e.call(GAME_SETTINGS, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

// Translated from 0040e130 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `Setting` (Xbox PDB) base constructor for a float value (`thiscall`,
/// `RET 8`): the base vtable (`01012120`), the name at +8 and the value at +4
/// (`FLD float`/`FSTP float`). Returns `this`.
pub fn fn_0040e130(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.mem.set_u32(this.addr(), SETTING_VTABLE);
    e.mem.set_u32(this.addr() + 8, name);
    e.mem.set_u32(this.addr() + 4, x87_quiet_bits(value));
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040e0b0, fn_0040e0b0(Ptr, u32, u32) -> Ptr),
        entry!(0x0040e130, fn_0040e130(Ptr, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_base_keeps_name_and_value_and_quiets_a_signaling_nan() {
        let mut e = Engine::new();
        let s = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0040e130, &[s, 0x1234, 0x4040_0000]).u32(), s);
        assert_eq!(e.mem.u32(s), SETTING_VTABLE);
        assert_eq!(e.mem.u32(s + 8), 0x1234);
        assert_eq!(e.mem.u32(s + 4), 0x4040_0000);
        e.call(0x0040e130, &[s, 0x1234, 0x7f80_0001]);
        assert_eq!(e.mem.u32(s + 4), 0x7fc0_0001);
        e.call(0x0040e130, &[s, 0x1234, 0xffc0_0000]);
        assert_eq!(e.mem.u32(s + 4), 0xffc0_0000);
    }

    #[test]
    fn a_game_setting_adds_itself_to_the_collection() {
        let mut e = Engine::new();
        let collection = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(collection, 0x0090_0000);
        e.register_double(GAME_SETTINGS, move |_, _| collection.into_ret());
        e.register_double(0x0090_1000, |e, a| {
            assert_eq!(e.mem.u32(a[1]), GAME_SETTING_VTABLE);
            assert_eq!(e.mem.u32(a[1] + 4), 0x3f80_0000);
            Ret::default()
        });
        let s = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040e0b0, &[s, 0x55, 0x3f80_0000]).u32(), s);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, GAME_SETTINGS);
        assert_eq!(log[2], (0x0090_1000, vec![collection, s]));
        assert_eq!(e.mem.u32(s + 8), 0x55);
    }
}
