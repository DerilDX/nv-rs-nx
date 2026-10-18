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

// Translated from 0044f440 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a float INI setting (`thiscall`, `RET 8`): the base
/// constructor (`0040e130`) with the name and the value (passed on through
/// the x87 stack), the INI setting vtable (`01017720`), then adds itself to
/// the INI setting collection (`0044f560`, vtable slot +4). Returns `this`.
/// The exception-unwinding frame is not translated.
pub fn fn_0044f440(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.call(
        SETTING_CONSTRUCT_FLOAT,
        &args![this, name, x87_quiet_bits(value)],
    );
    e.mem.set_u32(this.addr(), INI_SETTING_VTABLE);
    let collection = e.call(INI_SETTINGS, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

// Translated from 0044f3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044f400` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044f3b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_f400, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044f440, fn_0044f440(Ptr, u32, u32) -> Ptr),
        entry!(0x0044_f3b0, fn_0044f3b0(Ptr, u32) -> Ptr),
    ]
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
    #[test]
    fn at_0044f3b0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_f400, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_f3b0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_f3b0)
                .collect();
            let mut expected = vec![(0x0044_f400, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
