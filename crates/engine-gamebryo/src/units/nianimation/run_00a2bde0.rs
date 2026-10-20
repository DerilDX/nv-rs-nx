//! `NiAnimation/run_00a2bde0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a2c4c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3696` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2c4c0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3696) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d5c, 0x0);
    e.mem.set_u32(0x011f_3c3c, 0x0);
    e.mem.set_u32(0x011f_3bac, 0x0);
    e.mem.set_u8(0x011f_3696, 0x1);
    e.mem.set_u32(0x011f_3dec, 0xa2c420);
    e.mem.set_u32(0x011f_3a8c, 0xa2c490);
    e.mem.set_u32(0x011f_39fc, 0xa2bbf0);
    e.mem.set_u32(0x011f_396c, 0xa2be60);
    e.mem.set_u32(0x011f_38dc, 0xa2a600);
    e.mem.set_u8(0x011f_3767, 0x40);
    e.mem.set_u32(0x011f_3ccc, 0xa2bc60);
    e.mem.set_u32(0x011f_3b1c, 0xa2bde0);
    e.mem.set_u32(0x011f_37bc, 0xa2c130);
    e.mem.set_u32(0x011f_384c, 0xa2c1b0);
    1
}

// Translated from 00a2cb80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3697` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2cb80(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3697) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d2c, 0x0);
    e.mem.set_u32(0x011f_3c0c, 0x0);
    e.mem.set_u32(0x011f_3b7c, 0x0);
    e.mem.set_u8(0x011f_3697, 0x1);
    e.mem.set_u32(0x011f_3dbc, 0xa2caf0);
    e.mem.set_u32(0x011f_3a5c, 0xa2cb50);
    e.mem.set_u32(0x011f_39cc, 0xa2c5d0);
    e.mem.set_u32(0x011f_393c, 0xa2c660);
    e.mem.set_u32(0x011f_38ac, 0xa2a600);
    e.mem.set_u8(0x011f_375b, 0x1c);
    e.mem.set_u32(0x011f_3c9c, 0xa2c610);
    e.mem.set_u32(0x011f_3aec, 0xa2c550);
    e.mem.set_u32(0x011f_378c, 0xa2c7a0);
    e.mem.set_u32(0x011f_381c, 0xa2c890);
    1
}

// Translated from 00a2ce20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3698` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2ce20(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3698) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3dac, 0x0);
    e.mem.set_u32(0x011f_3c8c, 0x0);
    e.mem.set_u32(0x011f_3bfc, 0x0);
    e.mem.set_u8(0x011f_3698, 0x1);
    e.mem.set_u32(0x011f_3e3c, 0xa2cdc0);
    e.mem.set_u32(0x011f_3adc, 0xa2cc60);
    e.mem.set_u32(0x011f_3a4c, 0xa2cc10);
    e.mem.set_u32(0x011f_39bc, 0xa2cc90);
    e.mem.set_u32(0x011f_392c, 0xa2a600);
    e.mem.set_u8(0x011f_377b, 0x8);
    e.mem.set_u32(0x011f_3d1c, 0xa2cc40);
    e.mem.set_u32(0x011f_3b6c, 0xa2cc30);
    e.mem.set_u32(0x011f_380c, 0xa29680);
    e.mem.set_u32(0x011f_389c, 0xa2ccd0);
    1
}

// Translated from 00a2d140 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3699` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d140(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3699) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d7c, 0x0);
    e.mem.set_u32(0x011f_3c5c, 0x0);
    e.mem.set_u32(0x011f_3bcc, 0x0);
    e.mem.set_u32(0x011f_37dc, 0x0);
    e.mem.set_u8(0x011f_3699, 0x1);
    e.mem.set_u32(0x011f_3e0c, 0xa2cf60);
    e.mem.set_u32(0x011f_3aac, 0xa2cfd0);
    e.mem.set_u32(0x011f_3a1c, 0xa2ab10);
    e.mem.set_u32(0x011f_398c, 0xa2ced0);
    e.mem.set_u32(0x011f_38fc, 0xa2a600);
    e.mem.set_u8(0x011f_376f, 0x14);
    e.mem.set_u32(0x011f_3cec, 0xa2cf20);
    e.mem.set_u32(0x011f_3b3c, 0xa2a1a0);
    e.mem.set_u32(0x011f_386c, 0xa2d000);
    1
}

// Translated from 00a2d360 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f369a` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d360(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_369a) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d34, 0x0);
    e.mem.set_u32(0x011f_3c14, 0x0);
    e.mem.set_u32(0x011f_3b84, 0x0);
    e.mem.set_u8(0x011f_369a, 0x1);
    e.mem.set_u32(0x011f_3dc4, 0xa2a5a0);
    e.mem.set_u32(0x011f_3a64, 0xa2d230);
    e.mem.set_u32(0x011f_39d4, 0xa2d1e0);
    e.mem.set_u32(0x011f_3944, 0xa2cc90);
    e.mem.set_u32(0x011f_38b4, 0xa2a600);
    e.mem.set_u8(0x011f_375d, 0x8);
    e.mem.set_u32(0x011f_3ca4, 0xa2d200);
    e.mem.set_u32(0x011f_3af4, 0xa2d1d0);
    e.mem.set_u32(0x011f_3794, 0xa29680);
    e.mem.set_u32(0x011f_3824, 0xa2d260);
    1
}

// Translated from 00a2d610 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f369b` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d610(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_369b) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_369b, 0x1);
    e.mem.set_u32(0x011f_3ddc, 0xa2a8e0);
    e.mem.set_u32(0x011f_3a7c, 0xa2d470);
    e.mem.set_u32(0x011f_39ec, 0xa2a7b0);
    e.mem.set_u32(0x011f_395c, 0xa27040);
    e.mem.set_u32(0x011f_38cc, 0xa2a600);
    e.mem.set_u8(0x011f_3763, 0x10);
    e.call(0x00a2_d5b0, &args![0x1u32, 0x5u32]);
    1
}

// Translated from 00a2d810 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f369c` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d810(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_369c) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d64, 0x0);
    e.mem.set_u32(0x011f_3c44, 0x0);
    e.mem.set_u32(0x011f_3bb4, 0x0);
    e.mem.set_u8(0x011f_369c, 0x1);
    e.mem.set_u32(0x011f_3df4, 0xa2abc0);
    e.mem.set_u32(0x011f_3a94, 0xa2d690);
    e.mem.set_u32(0x011f_3a04, 0xa2ab10);
    e.mem.set_u32(0x011f_3974, 0xa2ab40);
    e.mem.set_u32(0x011f_38e4, 0xa2a600);
    e.mem.set_u8(0x011f_3769, 0x14);
    e.mem.set_u32(0x011f_3cd4, 0xa2cf20);
    e.mem.set_u32(0x011f_3b24, 0xa2d680);
    e.mem.set_u32(0x011f_37c4, 0xa2d670);
    e.mem.set_u32(0x011f_3854, 0xa2d6c0);
    1
}

// Translated from 00a2d5b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a2d5b0(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa291e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa2d3f0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa2d420);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa2a7e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa29680);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0xa2d4a0);
}

// Translated from 00a2e180 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reference-counted start-up hook of the animation unit (no Xbox PDB name; `this`, in `ECX`,
/// is returned unchanged): increments the counter at `011f36a0` and, when it was zero, registers
/// the unit's two functions (`00a2d8a0` and `00a2de70`) with the runtime's registration
/// (`00aa2080`, `cdecl`).
pub fn fn_00a2e180(e: &mut Engine, this: u32) -> u32 {
    let count = e.mem.u32(0x011f_36a0);
    e.mem.set_u32(0x011f_36a0, count.wrapping_add(1));
    if count == 0 {
        e.call(0x00aa_2080, &args![0x00a2_d8a0u32, 0x00a2_de70u32]);
    }
    this
}

/// `00a64900` (`cdecl`, two words, `NiStream::RegisterLoader`): registers a class name
/// (the address of a string in `.rdata`) with its creation function.
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// `00a64920` (`cdecl`, one word): registers a post-processing function.
const REGISTER_POST_PROCESS: u32 = 0x00a6_4920;
/// `00a421c0` (no stack words): the step `fn_00a2d8a0` ends with, by a jump.
const NEXT_STEP: u32 = 0x00a4_21c0;
/// Byte `011f369d`: set to 1 when `fn_00a2d8a0` has started.
const LOADERS_REGISTERED: u32 = 0x011f_369d;

/// The class registrations `fn_00a2d8a0` makes, in order: (class name string, creation
/// function).
const ANIMATION_LOADERS: [(u32, u32); 71] = [
    (0x0109_6060, 0x00a5_6b30),
    (0x0109_603c, 0x00a3_9700),
    (0x0109_6024, 0x00a5_65c0),
    (0x0109_6008, 0x00a5_5ff0),
    (0x0109_5fec, 0x00a4_0580),
    (0x0109_5fd0, 0x00a5_5a80),
    (0x0109_5fb0, 0x00a5_5470),
    (0x0109_5f90, 0x00a4_09d0),
    (0x0109_5f7c, 0x00a5_5300),
    (0x0109_5f70, 0x00a5_37f0),
    (0x0109_5f5c, 0x00a5_3500),
    (0x0109_5f40, 0x00a5_2fe0),
    (0x0109_5f2c, 0x00a5_2b60),
    (0x0109_5f1c, 0x00a5_1e10),
    (0x0109_5f00, 0x00a5_1c60),
    (0x0109_5ee0, 0x00a5_18f0),
    (0x0109_5ec0, 0x00a5_1620),
    (0x0109_5ea0, 0x00a5_13c0),
    (0x0109_5e7c, 0x00a5_0f00),
    (0x0109_5e60, 0x00a5_0ad0),
    (0x0109_5e44, 0x00a5_0840),
    (0x0109_5e24, 0x00a5_0180),
    (0x0109_5e18, 0x00a3_d150),
    (0x0109_5dfc, 0x00a4_fac0),
    (0x0109_5de8, 0x00a3_c130),
    (0x0109_5dd4, 0x00a2_f6c0),
    (0x0109_5dbc, 0x00a3_26c0),
    (0x0109_5da8, 0x00a4_f970),
    (0x0109_5d9c, 0x00a3_cd00),
    (0x0109_5d80, 0x00a4_11f0),
    (0x0109_5d6c, 0x00a3_c570),
    (0x0109_5d50, 0x00a4_f000),
    (0x0109_5d2c, 0x00a4_ecd0),
    (0x0109_5d14, 0x00a3_b550),
    (0x0109_5cfc, 0x00a3_6810),
    (0x0109_5cec, 0x00a4_e040),
    (0x0109_5cd8, 0x00a4_db60),
    (0x0109_5cc0, 0x00a4_d500),
    (0x0109_5ca8, 0x00a4_d0d0),
    (0x0109_5c94, 0x00a4_cde0),
    (0x0109_5c7c, 0x00a4_c5a0),
    (0x0109_5c60, 0x00a4_b790),
    (0x0109_5c54, 0x00a4_15a0),
    (0x0109_5c30, 0x00a3_0330),
    (0x0109_5c1c, 0x00a4_b270),
    (0x0109_5c08, 0x00a3_f4b0),
    (0x0109_5bf0, 0x00a4_9a30),
    (0x0109_5be4, 0x00a4_9020),
    (0x0109_5bc8, 0x00a4_8d70),
    (0x0109_5bb4, 0x00a4_8750),
    (0x0109_5ba8, 0x00a4_8110),
    (0x0109_5b9c, 0x00a4_7f20),
    (0x0109_5b84, 0x00a4_7380),
    (0x0109_5b74, 0x00a4_7150),
    (0x0109_5b60, 0x00a4_6b70),
    (0x0109_5b40, 0x00a4_6950),
    (0x0109_5b28, 0x00a3_6810),
    (0x0109_5b18, 0x00a4_e040),
    (0x0109_5b00, 0x00a4_03f0),
    (0x0109_5af0, 0x00a4_5f50),
    (0x0109_5ae4, 0x00a4_4ff0),
    (0x0109_5ad4, 0x00a4_4f10),
    (0x0109_5ac8, 0x00a4_4940),
    (0x0109_5ab8, 0x00a4_4380),
    (0x0109_5aa8, 0x00a4_3fb0),
    (0x0109_5a88, 0x00a4_3e40),
    (0x0109_5a6c, 0x00a4_3370),
    (0x0109_5a4c, 0x00a4_2f90),
    (0x0109_5a28, 0x00a3_59d0),
    (0x0109_5a10, 0x00a4_2ee0),
    (0x0109_5a04, 0x00a5_37f0),
];

// Translated from 00a2d8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the start-up function that `fn_00a2e180` registers; no
/// arguments, NiAnimation): once only (byte `011f369d`), registers the 71 class names of
/// `ANIMATION_LOADERS` with their creation functions through `00a64900`, registers the
/// post-processing function `00a42c80` through `00a64920`, and ends with a jump to
/// `00a421c0`.
pub fn fn_00a2d8a0(e: &mut Engine) {
    if e.mem.u8(LOADERS_REGISTERED) != 0 {
        return;
    }
    e.mem.set_u8(LOADERS_REGISTERED, 1);
    for (name, creator) in ANIMATION_LOADERS {
        e.call(REGISTER_LOADER, &args![name, creator]);
    }
    e.call(REGISTER_POST_PROCESS, &args![0x00a4_2c80u32]);
    e.call(NEXT_STEP, &args![]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2c4c0, fn_00a2c4c0() -> u32),
        entry!(0x00a2cb80, fn_00a2cb80() -> u32),
        entry!(0x00a2ce20, fn_00a2ce20() -> u32),
        entry!(0x00a2d140, fn_00a2d140() -> u32),
        entry!(0x00a2d360, fn_00a2d360() -> u32),
        entry!(0x00a2d610, fn_00a2d610() -> u32),
        entry!(0x00a2d810, fn_00a2d810() -> u32),
        entry!(0x00a2d5b0, fn_00a2d5b0(u32, u32)),
        entry!(0x00a2e180, fn_00a2e180(u32) -> u32),
        entry!(0x00a2_d8a0, fn_00a2d8a0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_call_registers_the_unit_and_every_call_returns_this() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.register(0x00aa_2080, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_e180, &args![0x1234u32]).u32(), 0x1234);
        assert_eq!(e.mem.u32(0x011f_36a0), 1);
        assert_eq!(e.call(0x00a2_e180, &args![0x5678u32]).u32(), 0x5678);
        assert_eq!(e.mem.u32(0x011f_36a0), 2);
        let log = e.call_log.take().unwrap();
        let registrations: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00aa_2080)
            .collect();
        assert_eq!(registrations.len(), 1);
        assert_eq!(registrations[0].1, vec![0x00a2_d8a0, 0x00a2_de70]);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn registers_every_loader_once_then_the_post_process_function() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        for address in [REGISTER_LOADER, REGISTER_POST_PROCESS, NEXT_STEP] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00a2_d8a0, &args![]);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.clone().unwrap().into_iter().skip(1).collect();
        assert_eq!(log.len(), 73);
        assert_eq!(log[0], (REGISTER_LOADER, vec![0x0109_6060, 0x00a5_6b30]));
        assert_eq!(log[70].0, REGISTER_LOADER);
        assert_eq!(log[71], (REGISTER_POST_PROCESS, vec![0x00a4_2c80]));
        assert_eq!(log[72], (NEXT_STEP, vec![]));
        assert_eq!(e.mem.u8(LOADERS_REGISTERED), 1);
        e.call_log = Some(vec![]);
        e.call(0x00a2_d8a0, &args![]);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }
}
