//! `fallout/interface/menus/computersmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00470470` (`thiscall`): frees the nodes of the list it is given and clears its head;
/// `0046ffb0` (`thiscall`) forwards to it.
const LIST_REMOVE_ALL_00470470: u32 = 0x0047_0470;
const LIST_DESTROY_0046FFB0: u32 = 0x0046_ffb0;
/// `004037d0` (`thiscall`): the string destructor (`BSStringT`).
const STRING_DESTROY_004037D0: u32 = 0x0040_37d0;

// Translated from 007593e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::TextData::~TextData` (Xbox PDB name from the engine map; `thiscall`, plain
/// `RET`): empties the list at +0x20 (`00470470`), destroys it (`0046ffb0`), then
/// destroys the string at +0 (`004037d0`).
// Translation incomplete: 007593e0 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 for the members) is not translated.
pub fn font_text_data_destructor(e: &mut Engine, this: Ptr) {
    let list = this.addr().wrapping_add(0x20);
    e.call(LIST_REMOVE_ALL_00470470, &args![list]);
    e.call(LIST_DESTROY_0046FFB0, &args![list]);
    e.call(STRING_DESTROY_004037D0, &args![this]);
}

// Translated from 00759450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, plain `RET`): the float at +0x2c, loaded with `FLD` (which quiets a
/// signalling NaN) and returned in `ST0`.
pub fn fn_00759450(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x2c)))
}

// Translated from 00759d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, plain `RET`): the byte at +0x29 in `AL` (callers test `AL` only).
pub fn fn_00759d80(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x29))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0075_93e0, font_text_data_destructor(Ptr)),
        entry!(0x0075_9450, fn_00759450(Ptr) -> f32),
        entry!(0x0075_9d80, fn_00759d80(Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_text_data_destructor_frees_the_list_then_the_string() {
        let mut e = Engine::new();
        for callee in [
            LIST_REMOVE_ALL_00470470,
            LIST_DESTROY_0046FFB0,
            STRING_DESTROY_004037D0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0075_93e0, &args![0x4000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (LIST_REMOVE_ALL_00470470, vec![0x4020]),
                (LIST_DESTROY_0046FFB0, vec![0x4020]),
                (STRING_DESTROY_004037D0, vec![0x4000]),
            ]
        );
    }

    #[test]
    fn fn_00759450_returns_the_float_quieted() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x2c, 2.5f32.to_bits());
        assert_eq!(e.call(0x0075_9450, &args![this]).f32(), 2.5);
        e.mem.set_u32(this + 0x2c, 0xff80_0001);
        assert_eq!(
            e.call(0x0075_9450, &args![this]).f32().to_bits(),
            0xffc0_0001
        );
    }

    #[test]
    fn fn_00759d80_returns_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        assert_eq!(e.call(0x0075_9d80, &args![this]).u8(), 0);
        e.mem.set_u8(this + 0x29, 2);
        assert_eq!(e.call(0x0075_9d80, &args![this]).u8(), 2);
    }
}
