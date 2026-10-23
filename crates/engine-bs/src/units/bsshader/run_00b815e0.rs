//! `BSShader/run_00b815e0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The two per-pass flag tables `00b815e0` reads (one word per pass).
const PASS_FLAGS_011FF248: u32 = 0x011f_f248;
const PASS_FLAGS_011FF120: u32 = 0x011f_f120;
/// The flag bytes it writes: bits 1 to 3 of the first table go to
/// `011ff118`..`011ff11a`, bits 1 to 6 of the second to `011ff110`..`011ff115`.
const FIRST_TABLE_BYTES: u32 = 0x011f_f118;
const SECOND_TABLE_BYTES: u32 = 0x011f_f110;
/// The nine objects whose byte at `+8` it sets (pointers at `011ff4a0`..`011ff4c0`).
const TOGGLE_OBJECTS: u32 = 0x011f_f4a0;

fn table_bit(e: &Engine, table: u32, pass: u32, bit: u32) -> u8 {
    let word = e.mem.u32(table.wrapping_add(pass.wrapping_mul(4)));
    ((word >> bit) & 1) as u8
}

fn toggle_object(e: &Engine, index: u32) -> u32 {
    e.mem.u32(TOGGLE_OBJECTS + 4 * index)
}

// Translated from 00b815e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, cdecl `(pass)`: copies the pass's flag bits
/// into the flag bytes and the toggle objects. The first three objects
/// get 1 (each when it exists, before the matching byte gets bit 1, 2 or 3
/// of the first table); the fourth gets bit 1 of the second table; then
/// bytes `011ff110`..`011ff115` get bits 1 to 6 of the second table, the
/// objects five to nine (when they exist) bits 2 to 6. Every bit is read
/// from the table again, as the code does.
pub fn fn_00b815e0(e: &mut Engine, pass: u32) {
    for (index, bit) in [(0u32, 1u32), (1, 2), (2, 3)] {
        let object = toggle_object(e, index);
        if object != 0 {
            e.mem.set_u8(object + 8, 1);
        }
        let value = table_bit(e, PASS_FLAGS_011FF248, pass, bit);
        e.mem.set_u8(FIRST_TABLE_BYTES + index, value);
    }
    let object = toggle_object(e, 3);
    if object != 0 {
        let value = table_bit(e, PASS_FLAGS_011FF120, pass, 1);
        e.mem.set_u8(object + 8, value);
    }
    for index in 0..6u32 {
        let value = table_bit(e, PASS_FLAGS_011FF120, pass, index + 1);
        e.mem.set_u8(SECOND_TABLE_BYTES + index, value);
        if index < 5 {
            let object = toggle_object(e, 4 + index);
            if object != 0 {
                let value = table_bit(e, PASS_FLAGS_011FF120, pass, index + 2);
                e.mem.set_u8(object + 8, value);
            }
        }
    }
}

// Translated from 00b81730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x124 of the vtable `010af9e0` (the one `WaterShader`'s
/// constructor stores at `00b86c1a`; `thiscall`, no stack words). For each of the 0x4a words
/// from `+0x6c` in order, calls the object's own slot +0x128 with the word.
pub fn fn_00b81730(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for index in 0..0x4au32 {
        let word = e.mem.u32(base + 0x6c + 4 * index);
        e.vcall(base, 0x128, &args![word]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b815e0, fn_00b815e0(u32)),
        entry!(0x00b8_1730, fn_00b81730(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_the_pass_bits_into_the_bytes_and_objects() {
        let mut e = Engine::new();
        e.map(0x011f_f000, 0x1000);
        let objects: Vec<u32> = (0..9).map(|_| e.mem.alloc(0xc)).collect();
        for (index, &object) in objects.iter().enumerate() {
            if index != 5 {
                e.set_global(TOGGLE_OBJECTS + 4 * index as u32, object);
            }
        }
        e.set_global(PASS_FLAGS_011FF248 + 8, 0b1010u32);
        e.set_global(PASS_FLAGS_011FF120 + 8, 0b101_0100u32);
        e.call(0x00b8_15e0, &args![2u32]);
        let bytes = |e: &Engine, at: u32, count: u32| -> Vec<u8> {
            (0..count).map(|index| e.mem.u8(at + index)).collect()
        };
        assert_eq!(bytes(&e, FIRST_TABLE_BYTES, 3), vec![1, 0, 1]);
        assert_eq!(bytes(&e, SECOND_TABLE_BYTES, 6), vec![0, 1, 0, 1, 0, 1]);
        let flags: Vec<u8> = objects.iter().map(|&object| e.mem.u8(object + 8)).collect();
        assert_eq!(flags, vec![1, 1, 1, 0, 1, 0, 1, 0, 1]);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    #[test]
    fn fn_00b81730_passes_each_word_to_slot_0x128() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x4b];
        slots[0x4a] = 0x0f00_0128;
        e.put_vtable(0x0f00_1000, &slots);
        e.register(0x0f00_0128, |_, _| Ret::default());
        let this = e.mem.alloc(0x6c + 0x4a * 4);
        e.mem.set_u32(this, 0x0f00_1000);
        for i in 0..0x4a {
            e.mem.set_u32(this + 0x6c + 4 * i, 0x100 + i);
        }
        e.call_log = Some(vec![]);
        e.call(0x00b8_1730, &args![this]);
        let seen: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0f00_0128)
            .map(|(_, w)| w)
            .collect();
        let expected: Vec<Vec<u32>> = (0..0x4a).map(|i| vec![this, 0x100 + i]).collect();
        assert_eq!(seen, expected);
    }
}
