//! `fallout shared/packagedatapatrol.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The patrol package data: a float at +0 and, at +4, the 0x10-byte
//! `PackageEventAction` block (`run_0067da20`, `packageeventaction`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0067da40` (`ECX` = the block at +4): zeroes it.
const ACTION_CLEAR: u32 = 0x0067_da40;
/// `0067da80` (`ECX` = the block at +4): releases it.
const ACTION_RELEASE: u32 = 0x0067_da80;
/// `0041fd00` (`ECX` = the block at +4, one word; `RET 4`).
const ACTION_SET_0041FD00: u32 = 0x0041_fd00;
/// `PackageEventAction::InitItem` (Xbox PDB name of `0067e190`; `ECX` = the
/// block at +4, the owner; `RET 4`).
const ACTION_INIT_ITEM: u32 = 0x0067_e190;

// Translated from 0067c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the action block at +4 (`0067da80`). The engine map gives it no
/// name.
pub fn fn_0067c670(e: &mut Engine, this: Ptr) {
    e.call(ACTION_RELEASE, &args![this.addr() + 4]);
}

// Translated from 0067c690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the float at `source` into `this` (`FLD`/`FSTP`, a signalling NaN
/// quieted) and zeroes the action block at +4 (`0067da40`); `RET 4`.
/// Returns `this`. The engine map gives it no name.
pub fn fn_0067c690(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let value = x87_quiet_bits(e.mem.u32(source.addr()));
    e.mem.set_u32(this.addr(), value);
    e.call(ACTION_CLEAR, &args![this.addr() + 4]);
    this
}

// Translated from 0067c6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0041fd00` with 3 on the action block at +4, then
/// `PackageEventAction::InitItem` on it with the owner (`RET 4`). The engine
/// map gives it no name.
pub fn fn_0067c6c0(e: &mut Engine, this: Ptr, owner: Ptr) {
    e.call(ACTION_SET_0041FD00, &args![this.addr() + 4, 3u32]);
    e.call(ACTION_INIT_ITEM, &args![this.addr() + 4, owner]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_c670, fn_0067c670(Ptr)),
        entry!(0x0067_c690, fn_0067c690(Ptr, Ptr) -> Ptr),
        entry!(0x0067_c6c0, fn_0067c6c0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
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
    fn patrol_data_forwards_to_its_action_block() {
        let mut e = Engine::new();
        for addr in [
            ACTION_CLEAR,
            ACTION_RELEASE,
            ACTION_SET_0041FD00,
            ACTION_INIT_ITEM,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        assert_eq!(
            log_of(&mut e, 0x0067_c670, &[0x4000]),
            vec![(ACTION_RELEASE, vec![0x4004])]
        );
        assert_eq!(
            log_of(&mut e, 0x0067_c6c0, &[0x4000, 0x5000]),
            vec![
                (ACTION_SET_0041FD00, vec![0x4004, 3]),
                (ACTION_INIT_ITEM, vec![0x4004, 0x5000]),
            ]
        );
        let this = e.mem.alloc(0x14);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x7f80_0003);
        assert_eq!(
            log_of(&mut e, 0x0067_c690, &[this, source]),
            vec![(ACTION_CLEAR, vec![this + 4])]
        );
        assert_eq!(e.mem.u32(this), 0x7fc0_0003);
        assert_eq!(e.call(0x0067_c690, &args![this, source]).u32(), this);
    }
}
