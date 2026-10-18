//! `fallout shared/magic/run_00402d00` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// The `BSStringT<char>` constructor (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// This class's vtable.
const VTABLE: u32 = 0x0101_1d54;

// Translated from 00402d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESFullName` (the engine map gives it no name; in the
/// Xbox PDB `TESFullName` is a `BaseFormComponent` followed by a `BSStringT`
/// at +4, and `TESQuest` constructs it at +0x30): runs the base constructor,
/// sets the vtable and constructs the string at `+4`. Returns `this`.
pub fn fn_00402d00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(STRING_CONSTRUCT, &args![this.byte_add(4)]);
    this
}

// Translated from 00402ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map gives it no name. `thiscall`, no stack arguments: sets the
/// fields of an object at `+0xb8` to -1, `+0xbc` to the byte 0, `+0xc0` to 0,
/// `+0xc4` to the `float` 0.0 and `+0xc8` to 0, then calls `00483710` on it.
/// Returns nothing.
pub fn fn_00402ea0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0xb8, 0xffff_ffff);
    e.mem.set_u8(base + 0xbc, 0);
    e.mem.set_u32(base + 0xc0, 0);
    e.mem.set_f32(base + 0xc4, 0.0);
    e.mem.set_u32(base + 0xc8, 0);
    e.call(0x0048_3710, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00402d00, fn_00402d00(Ptr) -> Ptr),
        entry!(0x00402ea0, fn_00402ea0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_base_then_the_vtable_then_the_string() {
        let mut e = Engine::new();
        e.register(BASE_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let back = e.call(0x0040_2d00, &args![object]).u32();
        assert_eq!(back, object);
        assert_eq!(e.mem.u32(object), VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (BASE_COMPONENT_CONSTRUCT, vec![object]),
                (STRING_CONSTRUCT, vec![object + 4]),
            ]
        );
    }
}

#[cfg(test)]
mod start_value_tests {
    use super::*;

    #[test]
    fn fn_00402ea0_sets_the_start_values_and_calls_00483710() {
        let mut e = Engine::new();
        e.register(0x0048_3710, |_, _| Ret::default());
        let this = e.mem.alloc(0xcc);
        e.mem.write(this, &[0xcd; 0xcc]);
        e.call_log = Some(vec![]);
        e.call(0x0040_2ea0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0048_3710, vec![this])]);
        let mut expected = vec![0xcd_u8; 0xcc];
        expected[0xb8..0xbc].copy_from_slice(&[0xff; 4]);
        expected[0xbc] = 0;
        expected[0xc0..0xcc].fill(0);
        assert_eq!(e.mem.bytes(this, 0xcc), expected);
    }
}
