//! `fallout shared/run_0058e790` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0043d4d0` (`thiscall`, no stack word; unit `fallout shared/modelloader.cpp`): the address
/// of the integer value of the object it is called on.
const VALUE_ADDRESS: u32 = 0x0043_d4d0;
/// The objects whose values `0058e790` reads for modes 0, 1 and 2.
const MODE_0_SOURCE: u32 = 0x011c_dde0;
const MODE_1_SOURCE: u32 = 0x011c_d538;
const MODE_2_SOURCE: u32 = 0x011c_d3b4;

// Translated from 0058e790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): by the mode word at +0x40, the
/// value (`0043d4d0`) of `011cdde0` (0), `011cd538` (1) or `011cd3b4` (2); 0 for any other
/// mode.
pub fn fn_0058e790(e: &mut Engine, this: Ptr) -> u32 {
    let source = match e.mem.u32(this.addr() + 0x40) {
        0 => MODE_0_SOURCE,
        1 => MODE_1_SOURCE,
        2 => MODE_2_SOURCE,
        _ => return 0,
    };
    let address = e.call(VALUE_ADDRESS, &args![source]).u32();
    e.mem.u32(address)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0058_e790, fn_0058e790(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_value_of_the_modes_object() {
        let mut e = Engine::new();
        let values = e.mem.alloc(0xc);
        e.mem.set_u32(values, 10);
        e.mem.set_u32(values + 4, 11);
        e.mem.set_u32(values + 8, 12);
        e.register_double(VALUE_ADDRESS, move |_, a| {
            let index = match a[0] {
                MODE_0_SOURCE => 0,
                MODE_1_SOURCE => 1,
                _ => 2,
            };
            (values + 4 * index).into_ret()
        });
        let this = e.mem.alloc(0x44);
        for (mode, value) in [(0u32, 10u32), (1, 11), (2, 12), (3, 0)] {
            e.mem.set_u32(this + 0x40, mode);
            assert_eq!(e.call(0x0058_e790, &args![this]).u32(), value);
        }
    }
}
