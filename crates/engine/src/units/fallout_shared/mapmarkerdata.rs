//! `fallout shared/mapmarkerdata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The text of a string object (`00559450`: the word at its `+0`).
const STRING_TEXT: u32 = 0x0055_9450;
/// The empty string the exe answers for a string without text
/// (`01011584`).
const EMPTY_TEXT: u32 = 0x0101_1584;

// Translated from 00408da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The text of the string at `this + 4`, or the exe's empty string
/// (`01011584`) when it has none. The engine map names it
/// `MapMarkerData::GetLocationName` (Xbox PDB); the body is shared by
/// identical-code folding (`TESForm`'s full-name getter `0048274c` calls it
/// too), so it is named by address: it reads `cFullName` of a
/// `TESFullName` (Xbox PDB, `+4`), which is `LocationName.cFullName` for a
/// `MapMarkerData`.
pub fn fn_00408da0(e: &mut Engine, this: Ptr) -> u32 {
    let string = this.addr() + 4;
    if e.call(STRING_TEXT, &args![string]).u32() == 0 {
        EMPTY_TEXT
    } else {
        e.call(STRING_TEXT, &args![string]).u32()
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00408da0, fn_00408da0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_text_or_the_empty_string() {
        let mut e = Engine::new();
        e.register(STRING_TEXT, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let name = e.mem.alloc(0xc);
        e.mem.set_u32(name + 4, 0x7000);
        assert_eq!(e.call(0x0040_8da0, &[name]).u32(), 0x7000);
        e.mem.set_u32(name + 4, 0);
        assert_eq!(e.call(0x0040_8da0, &[name]).u32(), EMPTY_TEXT);
        e.call_log = Some(vec![]);
        e.call(0x0040_8da0, &[name]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .skip(1)
            .all(|(a, w)| *a == STRING_TEXT && w == &[name + 4]));
    }
}
