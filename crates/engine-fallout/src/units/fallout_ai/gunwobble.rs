//! `fallout/ai/gunwobble.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The twelve model-name slots (`+4 * index`) of the gun wobble table: the models themselves.
const MODELS: u32 = 0x011e_0110;
/// The twelve model-name slots: copies of each model's name.
const NAMES: u32 = 0x011e_00e0;
/// Set to 0 while the table is built and to 1 afterwards.
const READY: u32 = 0x0118_5520;

// Translated from 008d66b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d66b0 - the exception-unwinding frame (FS:[0]) and the security-cookie check (00ec408c) are not translated.
/// `InitGunWobble` (engine map name, `gunwobble.cpp`): clears the two tables of twelve words at
/// `0x011e0110` and `0x011e00e0`, then for each index builds a file name (`00406d00` with the
/// format at `0x010876e4` for index 0, the one at `0x010876c0` and the word `0x011977a4 + 4 *
/// index` for the others), loads the model (`00456a20`, then `00447080` on the object at
/// `0x011c3b3c`) into the first table and stores a heap copy of its name (`0043b1b0`) in the
/// second. Each model gets the name `00438170` makes of the file name (`00a5b950`). The flag at
/// `0x01185520` is 0 meanwhile and 1 at the end.
///
/// The 0x104 byte name buffer the game keeps on the stack is a heap block here. The exception
/// frame and the security cookie are not translated.
pub fn fn_008d66b0(e: &mut Engine) {
    e.call(0x0040_3d30, &args![MODELS, 0u32, 0xcu32]);
    e.call(0x0040_3d30, &args![NAMES, 0u32, 0xcu32]);
    e.mem.set_u8(READY, 0);
    let buffer = e.mem.alloc(0x104);
    for index in 0..12u32 {
        if index == 0 {
            e.call(
                0x0040_6d00,
                &args![buffer, 0x104u32, 0x0108_76e4u32, 0x0101_dccc_u32],
            );
        } else {
            let extra = e.global::<u32>(0x0119_77a4 + index * 4);
            e.call(
                0x0040_6d00,
                &args![buffer, 0x104u32, 0x0108_76c0u32, 0x0101_dccc_u32, extra],
            );
        }
        if e.call(0x0045_6a20, &args![buffer, 0u32, 6u32, 1u32]).u32() == 0 {
            continue;
        }
        let loader = e.global::<u32>(0x011c_3b3c);
        let model = e
            .call(
                0x0044_7080,
                &args![loader, buffer, 0u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
        e.mem.set_u32(MODELS + index * 4, model);
        let model = e.mem.u32(MODELS + index * 4);
        let node = e.call(0x0041_3f40, &args![model]).u32();
        let text = e.call(0x0043_b1b0, &args![node]).u32();
        let length = e.call(0x0044_a670, &args![text]).u32();
        let size = length.wrapping_add(1);
        let copy = e.call(0x0040_1000, &args![size]).u32();
        e.mem.set_u32(NAMES + index * 4, copy);
        let model = e.mem.u32(MODELS + index * 4);
        let node = e.call(0x0041_3f40, &args![model]).u32();
        let text = e.call(0x0043_b1b0, &args![node]).u32();
        e.call(0x0040_6d30, &args![copy, size, text]);
        let name = e.mem.alloc(4);
        let made = e.call(0x0043_8170, &args![name, buffer]).u32();
        let model = e.mem.u32(MODELS + index * 4);
        e.call(0x00a5_b950, &args![model, made]);
        e.call(0x0043_81b0, &args![name]);
        e.mem.free(name);
    }
    e.mem.set_u8(READY, 1);
    e.mem.free(buffer);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008d_66b0, fn_008d66b0())]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Maps the words the function reads and writes.
    fn map_globals(e: &mut Engine) {
        for (address, size) in [
            (MODELS, 0xc),
            (NAMES, 0xc),
            (READY, 1),
            (0x0119_77a4, 0x30),
            (0x011c_3b3c, 4),
        ] {
            e.map(address, size);
        }
    }

    fn double(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn run(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log = Some(vec![]);
        e.call(0x008d_66b0, &[]);
        let log = e.call_log.take().unwrap();
        log.into_iter().skip(1).collect()
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn builds_twelve_models_and_names() {
        let mut e = Engine::new();
        map_globals(&mut e);
        for (address, eax) in [
            (0x0040_3d30, 0),
            (0x0040_6d00, 0),
            (0x0045_6a20, 1),
            (0x0044_7080, 0x5000),
            (0x0041_3f40, 0x6000),
            (0x0043_b1b0, 0x7000),
            (0x0044_a670, 4),
            (0x0040_1000, 0x8000),
            (0x0040_6d30, 0),
            (0x0043_8170, 0x9000),
            (0x00a5_b950, 0),
            (0x0043_81b0, 0),
        ] {
            double(&mut e, address, eax);
        }
        e.set_global(0x0119_77a4 + 4, 0x1234u32);
        e.set_global(0x011c_3b3c, 0x4000u32);
        e.set_global(READY, 7u8);
        let log = run(&mut e);
        let formats = calls_to(&log, 0x0040_6d00);
        assert_eq!(formats.len(), 12);
        assert_eq!(formats[0][1..], [0x104, 0x0108_76e4, 0x0101_dccc]);
        assert_eq!(formats[1][1..], [0x104, 0x0108_76c0, 0x0101_dccc, 0x1234]);
        let loads = calls_to(&log, 0x0044_7080);
        assert_eq!(loads.len(), 12);
        assert_eq!(loads[0][0], 0x4000);
        assert_eq!(loads[0][2..], [0, 1, 0, 0, 0]);
        assert_eq!(calls_to(&log, 0x00a5_b950).len(), 12);
        assert_eq!(calls_to(&log, 0x0040_1000)[0], [5]);
        assert_eq!(e.mem.u32(MODELS + 11 * 4), 0x5000);
        assert_eq!(e.mem.u32(NAMES + 11 * 4), 0x8000);
        assert_eq!(e.mem.u8(READY), 1);
    }

    #[test]
    fn skips_models_that_do_not_load() {
        let mut e = Engine::new();
        map_globals(&mut e);
        for address in [0x0040_3d30, 0x0040_6d00, 0x0045_6a20] {
            double(&mut e, address, 0);
        }
        let log = run(&mut e);
        assert!(calls_to(&log, 0x0044_7080).is_empty());
        assert_eq!(e.mem.u32(MODELS), 0);
        assert_eq!(e.mem.u8(READY), 1);
    }
}
