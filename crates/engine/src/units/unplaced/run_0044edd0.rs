//! `(unplaced)/run_0044edd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Setting` (Xbox PDB) base constructor for a float value (`0040e130`).
const SETTING_CONSTRUCT_FLOAT: u32 = 0x0040_e130;
/// The INI setting vtable.
const INI_SETTING_VTABLE: u32 = 0x0101_7720;
/// The INI setting collection (`0044f560`: created on first use).
const INI_SETTINGS: u32 = 0x0044_f560;
/// Slot of the collection's vtable that adds a setting.
const COLLECTION_ADD_SLOT: u32 = 0x4;

/// A float passed through the x87 stack (`FLD float` then `FSTP float`):
/// the same bits, except that a signaling NaN comes out quiet.
fn x87_float_copy(bits: u32) -> u32 {
    let nan = bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0;
    if nan {
        bits | 0x0040_0000
    } else {
        bits
    }
}

// Translated from 0044f440 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a float INI setting (`thiscall`, `RET 8`): the base
/// constructor (`0040e130`) with the name and the value (passed on through
/// the x87 stack), the INI setting vtable (`01017720`), then adds itself to
/// the INI setting collection (`0044f560`, vtable slot +4). Returns `this`.
/// The exception-unwinding frame is not translated.
pub fn fn_0044f440(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.call(
        SETTING_CONSTRUCT_FLOAT,
        &args![this, name, x87_float_copy(value)],
    );
    e.mem.set_u32(this.addr(), INI_SETTING_VTABLE);
    let collection = e.call(INI_SETTINGS, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0044f440, fn_0044f440(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_float_ini_setting_adds_itself_to_the_collection() {
        let mut e = Engine::new();
        let collection = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(collection, 0x0090_0000);
        e.register_double(INI_SETTINGS, move |_, _| collection.into_ret());
        e.register_double(0x0090_1000, |e, a| {
            assert_eq!(e.mem.u32(a[1]), INI_SETTING_VTABLE);
            Ret::default()
        });
        let s = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044f440, &[s, 0x77, 0x7f80_0002]).u32(), s);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (SETTING_CONSTRUCT_FLOAT, vec![s, 0x77, 0x7fc0_0002])
        );
        assert_eq!(log[3], (0x0090_1000, vec![collection, s]));
    }
}
