//! `BSShader/run_00bd93d0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and this effect's vtable
/// (`010bc560`; no RTTI locator before it).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const VTABLE_010BC560: u32 = 0x010b_c560;

// Translated from 00bd93d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd93d0`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010bc560` and clears the 16-byte block at +0x58 (the bytes +0x58 to +0x5a, then the words
/// +0x5c to +0x64). Answers `this`.
pub fn fn_00bd93d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC560);
    for offset in [0x58u32, 0x59, 0x5a] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0x5cu32, 0x60, 0x64] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bd_93d0, fn_00bd93d0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bd93d0_builds_the_effect() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x68);
        e.mem.write(this, &[0xcc; 0x68]);
        assert_eq!(e.call(0x00bd_93d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC560);
        assert_eq!(e.mem.bytes(this + 0x58, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0x5b), 0xcc);
        assert_eq!(e.mem.bytes(this + 0x5c, 0xc), vec![0; 0xc]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(EFFECT_CONSTRUCT_00BA4080, vec![this])]
        );
    }
}
